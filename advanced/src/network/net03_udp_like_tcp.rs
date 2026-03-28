use std::io;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

/*
이 예제에
중복 패킷 처리
순서 어긋남 처리
여러 메시지 연속 전송
서버가 같은 seq를 두 번 받았을 때 중복 처리
체크섬 / 손상 검증
sliding window
을 추가하면 실뢰성을 더 얻을 수 있다.
*/

const SERVER_ADDR: &str = "127.0.0.1:40000";

/// 서버 실행
fn udp_reliable_server() -> io::Result<()> {
    // 서버용 UDP 소켓 생성
    let socket = UdpSocket::bind(SERVER_ADDR)?;
    println!("[server] listening on {}", SERVER_ADDR);

    let mut buf = [0u8; 1024];

    loop {
        // recv_from(...)
        // - UDP 데이터그램 1개 수신
        // - 반환값: (읽은 바이트 수, 보낸 사람 주소)
        let (n, client_addr) = socket.recv_from(&mut buf)?;

        let raw = String::from_utf8_lossy(&buf[..n]);
        println!("[server] received from {}: {}", client_addr, raw);

        // DATA|seq|message 형식 파싱
        let parts: Vec<&str> = raw.splitn(3, '|').collect();

        // 형식이 잘못되면 무시
        if parts.len() != 3 {
            println!("[server] invalid packet format");
            continue;
        }

        let packet_type = parts[0]; // DATA 또는 ACK 같은 타입
        let seq = parts[1]; // sequence 번호
        let message = parts[2]; // 실제 메시지

        if packet_type != "DATA" {
            println!("[server] unsupported packet type");
            continue;
        }

        println!("[server] seq={}, message={}", seq, message);

        // 서버가 ACK 전송
        let ack_packet = format!("ACK|{}", seq); // seq 번호의 메시지를 잘 받았다고 응답 (TCP는 OS가 해줌)

        // send_to(...)
        // - 특정 주소(client_addr)로 ACK 전송
        socket.send_to(ack_packet.as_bytes(), client_addr)?;
        println!("[server] sent ack: {}", ack_packet);
    }
}

/// 클라이언트 실행
fn udp_reliable_client() -> io::Result<()> {
    // 클라이언트용 UDP 소켓 생성
    // :0 -> OS가 사용 가능한 임시 포트 자동 할당
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    println!("[client] bound on {}", socket.local_addr()?);

    // read timeout 설정
    // - recv_from()이 무한정 기다리지 않고
    // - 일정 시간 지나면 timeout 에러 반환
    socket.set_read_timeout(Some(Duration::from_secs(1)))?;

    let seq = 1; // 내가 보낸 메시지 번호를 식별하기 위해 사용
    let message = "hello reliable udp";
    let data_packet = format!("DATA|{}|{}", seq, message);

    let mut buf = [0u8; 1024];

    // 최대 3번 재전송 시도
    for attempt in 1..=3 {
        println!("[client] send attempt {}: {}", attempt, data_packet);

        // 서버로 데이터 전송
        socket.send_to(data_packet.as_bytes(), SERVER_ADDR)?;

        // ACK 기다리기
        match socket.recv_from(&mut buf) {
            Ok((n, from_addr)) => {
                let raw = String::from_utf8_lossy(&buf[..n]);
                println!("[client] received from {}: {}", from_addr, raw);

                let parts: Vec<&str> = raw.splitn(2, '|').collect();

                // ACK|1 형식 확인
                if parts.len() == 2 && parts[0] == "ACK" && parts[1] == seq.to_string() {
                    println!("[client] ack matched. delivery success");
                    return Ok(());
                } else {
                    println!("[client] invalid ack packet");
                }
            }
            Err(e) => {
                // timeout 포함 모든 recv 에러 처리
                println!("[client] no ack received: {}", e);
                println!("[client] retry...");
            }
        }
    }

    Err(io::Error::new(
        io::ErrorKind::TimedOut,
        "failed to receive ack after retries",
    ))
}

pub fn run() {
    // 서버는 계속 대기하므로 별도 스레드에서 실행
    let _server_handle = thread::spawn(|| {
        if let Err(e) = udp_reliable_server() {
            eprintln!("[server] error: {}", e);
        }
    });

    // 서버가 먼저 뜰 시간을 잠깐 줌
    thread::sleep(Duration::from_millis(300));

    // 클라이언트 실행
    if let Err(e) = udp_reliable_client() {
        eprintln!("[client] error: {}", e);
    }

    // 출력이 보이도록 잠깐 대기
    thread::sleep(Duration::from_millis(300));
}
