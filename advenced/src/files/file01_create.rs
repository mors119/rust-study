use std::fs::File;
use std::io::{self, Read, Write};

pub fn run() {
    if let Err(e) = write_mode() {
        eprintln!("error: {}", e);
    }

    if let Err(e) = read_write_mode() {
        eprintln!("error: {}", e);
    }
}

fn write_mode() -> io::Result<()> {
    let mut f = File::create("hello.txt")?; // 파일을 생성 - 파일이 있으면 파일을 삭제하고 새로 생성
    f.write_all(b"Hello Rust")?;

    Ok(())
}

// '쓰기' 뿐만 아니라 '읽기'도 하려면 create가 아니라 create_new 메서드를 사용
fn read_write_mode() -> io::Result<()> {
    let mut f = File::create_new("hello.txt")?; // 파일을 생성 - 파일이 있으면 에러
    f.write_all(b"Hello Rust")?;

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    println!("{}", buf);

    Ok(())
}
