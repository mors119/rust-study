use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Write};

pub fn run() {
    // 한번만 쓰기
    if let Err(e) = byte_arr() {
        eprintln!("{}", e);
    }
    // 여러번 쓰기
    if let Err(e) = buf_write() {
        eprintln!("{}", e);
    }
    if let Err(e) = file_open() {
        eprintln!("{}", e);
    }
    if let Err(e) = practical_file_open() {
        eprintln!("{}", e);
    }
}

fn byte_arr() -> io::Result<()> {
    let mut f = File::create("hello.txt")?;
    // write 메서드도 사용 가능하지만 write의 경우 가능한 만큼만 작성하고 작업이 중단되므로 리턴되는 길이값을 가지고 loop를 통해 쓰기 작업을 마무리해야한다.
    // u8 타입의 배열값을 파일에 쓸 때는 파일 핸들러에서 직접 write_all메서드를 사용
    f.write_all(b"Hello Rust")?; // f.write_all("Hello Rust".as_bytes())와 같음
    Ok(())
}

// 파일에 여러 번 값을 써야한다면 BufWriter로 파일을 감싼 후 buf.write_all을 하는 것이 수행속도 면에서 좋다.
fn buf_write() -> io::Result<()> {
    let f = File::create("hello.txt")?; // write_all은 mut가 필요하지만 buffer write로 감싼 경우는 필요하지 않음.
    let mut buf = BufWriter::new(f);

    buf.write_all(b"hello rust\n")?; // 아직 쓰기 작업이 이뤄지지 않고, flush를 한 후에 쓰기가 일어난다.
    buf.write_all("헬로 러스트".as_bytes())?;
    let _ = buf.flush()?;

    Ok(())
}

// 파일을 오픈해서 쓰기 작업
fn file_open() -> io::Result<()> {
    // Open Option 인스턴스를 받고 (read, write(새 작업만 적용), create, append(기존 작업에 추가)) 모드를 선택 후 파일 열기
    let f = File::options().append(true).open("hello.txt")?; // OpenOption::new()
    let mut buf = BufWriter::new(f);

    buf.write_all(b"\n")?;
    buf.write_all(b"hello rust 2\n")?;
    buf.write_all("헬로 러스트 2".as_bytes())?;
    let _ = buf.flush()?;

    Ok(())
}

// 실무적인
fn practical_file_open() -> io::Result<()> {
    let f = OpenOptions::new()
        .create(true) // 없으면 생성
        .append(true) // 있으면 기존 내용 뒤에 추가
        .open("hello.txt")?;
    /*
    로그 파일은 계속 쌓는 구조: .create(true).append(true)
    기존 내용 삭제 후 새로 쓰기: .create(true).write(true).truncate(true)
    설정 파일 같이 읽고 쓰기: .read(true).write(true).create(true)
    */

    // bufWriter는 보통 buf 대신 writer라고 많이 씀
    let mut writer = BufWriter::new(f);

    writer.write_all(b"\n")?;
    writer.write_all(b"hello rust 2\n")?;
    writer.write_all("헬로 러스트 2".as_bytes())?;
    writeln!(writer, "hello rust 2")?;
    writeln!(writer, "헬로 러스트 2")?; // ! 실무에서 writeln!이 더 많이 사용됨.

    writer.flush()?; // 반드시 에러 전파

    Ok(())
}
