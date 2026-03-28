use std::fs::File;
use std::io::{self, BufWriter, Read, Write};

pub fn run() {
    if let Err(e) = only_read() {
        eprintln!("{}", e);
    }
    if let Err(e) = only_read_error_handling() {
        eprintln!("{}", e);
    }
    if let Err(e) = read_write_mode() {
        eprintln!("{}", e);
    }
}

// 읽기 전용으로 열기
fn only_read() -> io::Result<()> {
    let mut f = File::open("hello.txt")?;

    let mut buf = String::new();
    // 파일을 오픈해서 파일내용을 모두 String으로 읽음.
    f.read_to_string(&mut buf)?;
    println!("{}", buf);

    Ok(())
}

// 파일 처리 시 에러도 처리하기
fn only_read_error_handling() -> io::Result<()> {
    let mut f = match File::open("hello3.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => {
                println!("File not found");
                return Ok(());
            }
            _ => return Err(error),
        },
    };

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    println!("{}", buf);
    Ok(())
}

// 읽고 쓰기 모드로 파일 열기
fn read_write_mode() -> io::Result<()> {
    let f = File::options().read(true).write(true).open("hello.txt")?;
    /*
    로그 파일은 계속 쌓는 구조: .create(true).append(true)
    기존 내용 삭제 후 새로 쓰기: .create(true).write(true).truncate(true)
    설정 파일 같이 읽고 쓰기: .read(true).write(true).create(true)
    */
    let mut buf = BufWriter::new(f);

    buf.write_all(b"\n")?;
    buf.write_all(b"hello rust\n")?;
    buf.write_all("헬로 러스트".as_bytes())?;
    let _ = buf.flush();

    Ok(())
}
