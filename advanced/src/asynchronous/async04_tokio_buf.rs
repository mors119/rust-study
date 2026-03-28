use std::io;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
/*
std::io::BufRead 트레잇 |	tokio::io::AsyncdBufReadExt 트레잇 |	메모리 버퍼 이용한 Read
std::io::BufReader 구조체 |	tokio::io::BufReader |	AsyncBufRead를 구현한 구조체
std::io::BufWrite 트레잇 |	tokio::io::AsyncBufWriteExt |	메모리 버퍼 이용한 Write
std::io::BufWriter 구조체 |	tokio::io::BufWriter |	AsynceBufWrite를 구현한 구조체
*/

#[tokio::main]
pub async fn run() {
    file_read().await.unwrap();
    file_write().await.unwrap();
}

// BufReader는 읽어드릴 데이터를 모아서 한 번에 요청하기 때문에 분산된 데이터의 경우 AsyncRead보다 호출 횟수가 적어 효율이 좋다.
// AsyncRead는 큰 파일을 한번만 읽을 떄 (호출 1)이기 때문에 더 좋다고 할 수 있다.
async fn file_read() -> io::Result<()> {
    let f = File::open("hello.txt").await?; // 1) 파일 핸들러 얻는다. 
    let mut reader = BufReader::new(f); // 2) BufReader 객체 생성  
    let mut buf = String::new();

    // 3) AsyncBufReader의 메서드 사용
    // 한 줄만 읽기.
    reader.read_line(&mut buf).await?;

    // 여러 줄 읽기
    loop {
        let num_bytes = reader.read_line(&mut buf).await?;
        if num_bytes == 0 {
            break;
        }
    }

    println!("{}", buf);
    Ok(())
}

async fn file_write() -> io::Result<()> {
    let f = File::create("tmp.txt").await?; // 핸들러 얻기   
    let mut writer = BufWriter::new(f); // 객체 생성
    writer.write(b"first line\nBye\n").await?; // 쓰기
    writer.flush().await?; // flush로 마무리

    Ok(())
}
