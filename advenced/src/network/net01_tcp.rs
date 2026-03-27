use std::io::{self, Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

/*
tcp udp 간단 정리

* TCP: 연결을 맺고 통신하는 방식

특징: 데이터가 순서대로 도착하도록 보장, 중간에 유실되면 재전송, 안정적인 대신 상대적으로 무겁고 느릴 수 있음

주로 쓰는 곳: 웹 브라우저와 서버 통신, 파일 전송, 로그인, 채팅, API 요청

Rust 타입: TcpListener, TcpStream

* UDP: 연결 없이 그냥 데이터그램(datagram, 독립적인 패킷) 을 보내는 방식

특징: 빠르고 가벼움, 순서 보장 없음, 유실 가능, 중복 가능, 재전송 보장 없음

주로 쓰는 곳: 실시간 게임, 스트리밍, 음성 통화, DNS

Rust 타입: UdpSocket
*/

// 서버가 바인딩할 기본 주소
const IP: &str = "127.0.0.1:12345";

/// 서버 시작 함수
fn server_main() -> io::Result<()> {
    // TcpListener::bind(...)
    // - 지정한 IP:PORT에 서버 소켓 생성
    let listener = TcpListener::bind(IP)?;
    println!("Server is running on {}", IP);

    // incoming()
    // - 클라이언트 연결이 들어올 때마다 TcpStream을 반환
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Client connected");

                // 각 클라이언트를 별도 스레드에서 처리
                thread::spawn(|| {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("client error: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("connection failed: {}", e);
            }
        }
    }

    Ok(())
}

/// 클라이언트 1개 처리 함수
fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0u8; 1024]; // 수신 버퍼

    loop {
        // read(&mut buf)
        // - 스트림에서 데이터를 읽음
        // - 반환값: 실제 읽은 바이트 수
        let n = stream.read(&mut buf)?;

        // n == 0
        // - 상대방이 연결 종료
        if n == 0 {
            println!("Client disconnected");
            break;
        }

        // 실제 읽은 부분만 사용
        let received = &buf[..n];

        // from_utf8_lossy(...)
        // - UTF-8이 아니어도 깨지지 않게 문자열로 변환
        let msg = String::from_utf8_lossy(received);
        println!("Client sent ({} bytes): {}", n, msg);

        // 서버 응답 전송
        stream.write_all(b"Message received from server")?;
    }

    Ok(())
}

/// 일반 문자열 메시지를 보내는 클라이언트
fn client_send_message() -> io::Result<()> {
    let mut stream = TcpStream::connect(IP)?;
    println!("Client connected to {}", IP);

    // 문자열 전송
    stream.write_all("헬로우 러스트".as_bytes())?;

    // shutdown(Shutdown::Write)
    // - "나는 더 이상 보낼 데이터 없음"을 알림
    // - 서버 쪽 read()가 이후 0을 받아 연결 종료를 인식할 수 있음
    stream.shutdown(Shutdown::Write)?;

    let mut buf = [0u8; 1024];

    // 서버 응답 1회 읽기
    let n = stream.read(&mut buf)?;
    if n > 0 {
        let reply = String::from_utf8_lossy(&buf[..n]);
        println!("Server replied: {}", reply);
    }

    Ok(())
}

/// 바이트 1개만 보내는 간단한 클라이언트
fn client_send_one_byte() -> io::Result<()> {
    let mut stream = TcpStream::connect(IP)?;
    println!("One-byte client connected to {}", IP);

    // write_all(&[1])
    // - 바이트 1개 전송
    stream.write_all(&[1])?;

    // 더 이상 보낼 데이터가 없음을 알림
    stream.shutdown(Shutdown::Write)?;

    let mut buf = [0u8; 128];
    let n = stream.read(&mut buf)?;

    if n > 0 {
        let reply = String::from_utf8_lossy(&buf[..n]);
        println!("One-byte client received: {}", reply);
    }

    Ok(())
}

/// 여러 주소 후보를 순서대로 시도하는 클라이언트
fn client_connect_candidates() -> io::Result<()> {
    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 8080)), // 일부러 실패할 수 있는 주소
        SocketAddr::from(([127, 0, 0, 1], 12345)), // 실제 서버 주소
    ];

    // TcpStream::connect(&addrs[..])
    // - 슬라이스에 담긴 주소들을 순서대로 시도
    let mut stream = TcpStream::connect(&addrs[..])?;
    println!("Connected by candidate address list");

    stream.write_all(b"hello from candidate client")?;
    stream.shutdown(Shutdown::Write)?;

    let mut buf = [0u8; 1024];
    let n = stream.read(&mut buf)?;

    if n > 0 {
        let reply = String::from_utf8_lossy(&buf[..n]);
        println!("Candidate client received: {}", reply);
    }

    Ok(())
}

pub fn run() {
    // 서버는 계속 대기하므로 별도 스레드에서 실행
    let _server_handle = thread::spawn(|| {
        if let Err(e) = server_main() {
            eprintln!("Server error: {}", e);
        }
    });

    // 서버가 먼저 실행될 시간을 잠깐 줌
    thread::sleep(Duration::from_millis(300));

    println!("\n--- client_send_message ---");
    if let Err(e) = client_send_message() {
        eprintln!("client_send_message error: {}", e);
    }

    println!("\n--- client_send_one_byte ---");
    if let Err(e) = client_send_one_byte() {
        eprintln!("client_send_one_byte error: {}", e);
    }

    println!("\n--- client_connect_candidates ---");
    if let Err(e) = client_connect_candidates() {
        eprintln!("client_connect_candidates error: {}", e);
    }

    // 서버 스레드는 계속 실행 중이므로 예제 종료 전 잠깐 대기
    thread::sleep(Duration::from_millis(300));
}
