use std::io;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

/*
* TCP는
```rust
let listener = TcpListener::bind(...)?;
let stream = TcpStream::connect(...)?;
```
이런 식으로 연결 객체(stream) 를 다룬다.

* UDP는
```rust
let socket = UdpSocket::bind(...)?;
socket.send_to(...)?;
socket.recv_from(...)?;
```
소켓 하나로 보내고 받는다.
*/

const SERVER_ADDR: &str = "127.0.0.1:12345";

/// UDP 서버 실행
fn udp_server_main() -> io::Result<()> {
    // UdpSocket::bind(...)
    // - 지정한 주소에 UDP 소켓 바인딩
    let socket = UdpSocket::bind(SERVER_ADDR)?;
    println!("UDP server is running on {}", SERVER_ADDR);

    let mut buf = [0u8; 1024]; // 수신 버퍼

    loop {
        // recv_from(&mut buf)
        // - 데이터를 받고
        // - (읽은 바이트 수, 보낸 쪽 주소)를 반환
        let (n, client_addr) = socket.recv_from(&mut buf)?;

        // 실제 읽은 바이트만 사용
        let received = &buf[..n];
        let msg = String::from_utf8_lossy(received);

        println!("Client {} sent ({} bytes): {}", client_addr, n, msg);

        // send_to(...)
        // - 특정 주소로 데이터 전송
        socket.send_to(b"Message received from UDP server", client_addr)?;
    }
}

/// 문자열을 보내는 UDP 클라이언트
fn udp_client_send_message() -> io::Result<()> {
    // "0.0.0.0:0" 바인딩
    // - OS가 사용 가능한 임시 포트를 자동 할당
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    println!("UDP client bound on {}", socket.local_addr()?);

    // send_to(...)
    // - 서버 주소로 데이터 전송
    socket.send_to("헬로우 UDP 러스트".as_bytes(), SERVER_ADDR)?;

    let mut buf = [0u8; 1024];

    // recv_from(...)
    // - 서버 응답 수신
    let (n, from_addr) = socket.recv_from(&mut buf)?;

    if n > 0 {
        let reply = String::from_utf8_lossy(&buf[..n]);
        println!("UDP client received from {}: {}", from_addr, reply);
    }

    Ok(())
}

/// 바이트 1개를 보내는 UDP 클라이언트
fn udp_client_send_one_byte() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    println!("UDP one-byte client bound on {}", socket.local_addr()?);

    // &[1]
    // - 바이트 1개짜리 데이터
    socket.send_to(&[1], SERVER_ADDR)?;

    let mut buf = [0u8; 1024];
    let (n, from_addr) = socket.recv_from(&mut buf)?;

    if n > 0 {
        let reply = String::from_utf8_lossy(&buf[..n]);
        println!("UDP one-byte client received from {}: {}", from_addr, reply);
    }

    Ok(())
}

pub fn run() {
    // UDP 서버는 계속 대기하므로 별도 스레드에서 실행
    let _server_handle = thread::spawn(|| {
        if let Err(e) = udp_server_main() {
            eprintln!("UDP server error: {}", e);
        }
    });

    // 서버가 먼저 실행될 시간을 잠깐 줌
    thread::sleep(Duration::from_millis(300));

    println!("\n--- udp_client_send_message ---");
    if let Err(e) = udp_client_send_message() {
        eprintln!("udp_client_send_message error: {}", e);
    }

    println!("\n--- udp_client_send_one_byte ---");
    if let Err(e) = udp_client_send_one_byte() {
        eprintln!("udp_client_send_one_byte error: {}", e);
    }

    // 예제 출력이 보이도록 잠깐 대기
    thread::sleep(Duration::from_millis(300));
}
