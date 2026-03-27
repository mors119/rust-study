use std::fs::{self, OpenOptions};
use std::io::{self, BufWriter, Read, Write};
use std::path::Path;

pub fn run() {
    if let Err(e) = file_copy() {
        eprintln!("{}", e);
    }
    if let Err(e) = file_delete() {
        eprintln!("{}", e);
    }
    if let Err(e) = file_merge_and_delete() {
        eprintln!("{}", e);
    }
    if let Err(e) = move_or_rename_file() {
        eprintln!("{}", e);
    }
}

// 파일 복사
fn file_copy() -> io::Result<()> {
    fs::copy("log/output.log", "log/copy.log")?;
    Ok(())
}

// 파일 삭제
fn file_delete() -> io::Result<()> {
    // fs::remove_file("log/output.log")?;
    Ok(())
}

// 파일을 옮겨 붙이고 파일 삭제
fn file_merge_and_delete() -> io::Result<()> {
    // 1. source 파일 읽기
    let mut src = fs::File::open("log/output.log")?;

    let mut contents = Vec::new();
    src.read_to_end(&mut contents)?;

    // 2. destination 파일 append 모드로 열기
    let dest = OpenOptions::new()
        .create(true) // 없으면 생성
        .append(true) // 있으면 뒤에 추가
        .open("log/copy.log")?;

    let mut writer = BufWriter::new(dest);

    writer.write_all(b"\n")?; // 줄 구분
    writer.write_all(&contents)?; // 내용 추가
    writer.flush()?; // 반드시 flush

    // 3. 원본 삭제
    fs::remove_file("log/output.log")?;

    Ok(())
}

// 파일 이동 / 이름 변경
fn move_or_rename_file() -> io::Result<()> {
    let src = Path::new("log/output.log");
    let dst = Path::new("log/output_backup.log");

    // 실무스타일 에러 처리
    if !src.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "source file not found",
        ));
    }

    if dst.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "destination file already exists",
        ));
    }

    fs::rename(src, dst)?;

    println!("파일 이동/이름 변경 완료: {:?} -> {:?}", src, dst);

    Ok(())
}
