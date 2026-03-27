use std::process::ExitCode;

use tokio::fs;

/*
dyn: dynamic dispatch, runtime에 어떤 타입인지 보고 맞는 함수 호출ㅇ
static dispatch (기본): 컴파일 타임 결정, 빠름, 코드가 여러 개 생성됨
dynamic dispatch (dyn): 런타임 결정, 약간 느림 (vtable lookup), 대신 유연함

* tokio::fs 함수
ex) tokio::fs::read_to_string("file.txt").await?;

tokio::fs::read_to_string: 파일 전체 읽기, String으로 바로 변환
tokio::fs::read: Vec<u8>로 읽음, 바이너리 파일용
tokio::fs::write: 파일 생성 or 덮어쓰기, 기존 내용 사라짐

* tokio::fs::File 함수
ex) File::create("file.txt").await?;

File::open: 읽기 전용
File::create: 파일 없으면 생성, 있으면 덮어쓰기


* tokio::io::AsyncReadExt 함수
ex) file.read(&mut buf).await

file.read: 읽기

* tokio::io::AsyncWriteExt 함수
ex) file.write_all(b"hello").await?;

file.write_all: 쓰기

* tokio::fs::OpenOptions 함수
ex) OpenOptions::new().read(true).write(true).open("file.txt")

.read(true): 읽기 가능
.write(true): 쓰기 가능
.create(true): 파일 없으면 생성
.create_new(true): 파일 없을 때만 생성 (이미 있으면 에러)
.append(true): 파일 끝에 추가
.truncate(true): 파일 내용 초기화 (비움)
*/

#[tokio::main]
pub async fn run() {
    if let Err(e) = read_string().await {
        eprintln!("read_string error: {e}");
    }

    if let Err(e) = read_bytes().await {
        eprintln!("read_bytes error: {e}");
    }

    if let Err(e) = write_file().await {
        eprintln!("write_file error: {e}");
    }
}

// main에 대해 Result 타입으로 리턴하게 할 때는 Result의 첫 번째 인자를 ExitCode로 해줘야 한다.
// 지금은 파일 구조 상 내용을 옮겼을 뿐
async fn _read_to_string() -> Result<ExitCode, std::io::Error> {
    // ❗ ExitCode는 메인에서만 사용함.
    // tokio::fs는 파일 I/O가 본질적으로 blocking이라는 점을 감싸서,
    // async context에서 사용할 수 있게 해준다.
    // 내부적으로는 runtime이 적절한 방식으로 blocking 작업을 처리한다.
    let txt = fs::read_to_string("hello.txt").await?;
    println!("File contens: \n{}", txt);
    Ok(ExitCode::from(0))
}

// 어떠한 Error 타입도 받을 수 있게 하려면 Result<(), Box<dyn std::error::Error>>로
// 더 일반적인 형태다.
async fn read_string() -> Result<(), Box<dyn std::error::Error>> {
    let txt = fs::read_to_string("hello.txt").await?;
    println!("File contens: \n{}", txt);
    Ok(())
}

async fn read_bytes() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read("hello.txt").await?; // Result<Vec<u8>>을 리턴
    let txt = String::from_utf8(contents)?;
    println!("File contens: \n{}", txt);
    Ok(())
}

// 루트 폴더에 'tmp.txt' 파일이 생성되고, 그 안에 "Hello world!"가 쓰여진다.
async fn write_file() -> Result<(), Box<dyn std::error::Error>> {
    fs::write("tmp.txt", b"Hello world!").await?;
    Ok(())
}
