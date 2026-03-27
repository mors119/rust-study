use std::fs::File;
use std::io::{self, Read, Seek, Write};

pub fn run() {
    if let Err(e) = create_and_read() {
        eprintln!("{}", e);
    }
    if let Err(e) = read_u8_arr() {
        eprintln!("{}", e);
    }
    if let Err(e) = read_string() {
        eprintln!("{}", e);
    }
    if let Err(e) = specify_len() {
        eprintln!("{}", e);
    }
    if let Err(e) = seek_from() {
        eprintln!("{}", e);
    }
}

// 파일 (생성)작성 후 읽기
fn create_and_read() -> io::Result<()> {
    let mut f = File::create_new("hello2.txt")?;
    f.write_all(b"Hello Rust")?;

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    println!("{}", buf);

    Ok(())
}

/*
read(buf): 파일로부터 읽을 수 있을 만큼 데이터를 읽는 메서드
read_exact(buf): 지정한 길이만큼 읽는다.
read_to_end(buf): u8로 파일내 데이터에 대해서 EOF를 만날 때까지 read(buf) 메서드를 계속 호출, 파일의 모든 내용이 읽는 것을 보장
read_to_string(buf): String으로 파일의 모든 내용이 읽는 것을 보장
*/

// u8 배열로 읽기
fn read_u8_arr() -> io::Result<()> {
    let mut f = File::open("hello.txt")?; // open 메서드는 파일을 read 모드로 연다 (쓰기 작업 불가)
    let mut data = vec![];

    f.read_to_end(&mut data)?; // data에 파일 내에 있던 모든 값이 읽혀진다. 이때 읽혀지는 데이터 타입은 u8
    Ok(())
}

// 파일을 String으로 읽기
fn read_string() -> io::Result<()> {
    let mut f = File::open("hello.txt")?;
    let mut buf = String::new();

    f.read_to_string(&mut buf)?;
    println!("{}", buf);

    Ok(())
}
// 지정한 길이 만큼 읽기
fn specify_len() -> io::Result<()> {
    let mut f = File::open("hello.txt")?;

    let mut buf = [0; 4];
    f.read_exact(&mut buf)?;

    println!("{:?}", buf); // [48, 49, 50, 51]
    Ok(())
}

// 지정한 위치의 데이터 읽기 (SeekFrom enum에는 start, end, current가 있다.)
fn seek_from() -> io::Result<()> {
    let mut f = File::open("hello.txt")?;
    f.seek(io::SeekFrom::Start(4))?;

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    println!("{:?}", buf);
    Ok(())
}
