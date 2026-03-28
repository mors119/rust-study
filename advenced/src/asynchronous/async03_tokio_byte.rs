use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt}; //  f.read(&mut buf)와 같이 파일을 read 하려면, 이 read 메서드에 대한 트레잇인 AsyncReadExt를 use 해야함.

/*
파일을 한 번에 읽고 쓰는 것은 tokio::fs를 사용하면 되겠지만, 정해진 크기 만큼씩 읽기/쓰기를 하려면 tokio::io 모듈을 이용
*/
#[tokio::main]
pub async fn run() {
    // 일부러 다양한 오류 처리 방법 사용
    async_read_ext().await.unwrap_or_else(|err| {
        eprintln!("read failed: {err}");
    });

    match file_lines().await {
        Ok(()) => println!("read success"),
        Err(err) => eprintln!("read failed: {err}"),
    }

    match write_lines().await {
        Ok(()) => println!("write success"),
        Err(err) => eprintln!("write failed: {err}"),
    }
}

// AsyncReadExt: 바이트 단위 파일 읽기
async fn async_read_ext() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = File::open("hello.txt").await?;
    let mut buf = [0; 10]; // 버퍼 크기 지정

    let n = f.read(&mut buf).await?; // 지정된 크기 만큼만 읽기
    println!("The bytes: {:?}", &buf[..n]);

    let txt = String::from_utf8(buf[..n].to_vec())?; // 바이너리 가져와서 utf8로 변경
    println!("txt: \n{}", txt);

    Ok(())
}

// AsyncReadExt: 파일 내 라인 수 세기
async fn file_lines() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = fs::File::open("hello.txt").await?;

    let mut buf = vec![0; 10]; // 실무에서는 8 * 1024(8KB) 정도씩 읽음.
    let mut number_of_lines = 0;
    loop {
        let len = file.read(&mut buf).await?;
        if len == 0 {
            break;
        }
        for &b in &buf[..len] {
            if b == b'\n' {
                number_of_lines += 1;
            }
        }
    }
    println!("File has {} lines.", number_of_lines);
    Ok(())
}

// AsyncWriteExt: 라인 단위 쓰기
async fn write_lines() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("tmp.txt").await?;

    file.write_all(b"First line.\n").await?;
    file.write_all(b"Second line.\n").await?;
    file.write_all(b"Third line.\n").await?;

    // write를 한 후 flush()를 해줘야 한다.
    file.flush().await?;

    Ok(())
}
