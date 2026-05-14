use std::fs::File;
use std::io::{self, Read};

pub fn run() {
    // panic!("This is a panic message!"); // 프로그램이 패닉 상태에 빠지고 이 메시지가 출력됩니다.

    // Result<T, E> 열거형을 사용하여 오류를 처리할 수 있습니다. Result는 Ok(T)와 Err(E) 두 가지 배리언트를 가집니다.
    // let greeting_file_result = std::fs::File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file, // 파일이 성공적으로 열렸을 때
    //     Err(error) => {
    //         panic!("파일을 열 수 없습니다: {:?}", error); // 오류가 발생했을 때 패닉 상태에 빠집니다.
    //     }
    // };

    // 서로 다른 종류의 오류에 대해 다른 처리를 할 수도 있습니다. 예를 들어, 파일이 존재하지 않는 경우에는 새 파일을 생성하고, 다른 종류의 오류가 발생한 경우에는 패닉 상태에 빠지도록 할 수 있습니다.
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         std::io::ErrorKind::NotFound => match std::fs::File::create("hello.txt") {
    //             Ok(fc) => fc, // 파일이 성공적으로 생성되었을 때
    //             Err(e) => panic!("파일을 생성할 수 없습니다: {:?}", e), // 파일 생성 중 오류가 발생했을 때 패닉 상태에 빠집니다.
    //         },
    //         other_error => panic!("파일을 열 수 없습니다: {:?}", other_error), // 다른 종류의 오류가 발생했을 때 패닉 상태에 빠집니다.
    //     },
    // };

    let _greeting_file = File::open("hello.txt").unwrap(); // 파일이 열리지 않으면 프로그램이 패닉 상태에 빠집니다. 
    let _greeting_file2 = File::open("hello.txt").expect("파일을 열 수 없습니다."); // 파일이 열리지 않으면 이 메시지와 함께 프로그램이 패닉 상태에 빠집니다.
    let _greeting_file3 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == io::ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("파일을 생성할 수 없습니다: {:?}", error);
            })
        } else {
            panic!("파일을 열 수 없습니다: {:?}", error);
        }
    }); // 파일이 열리지 않으면, 오류의 종류에 따라 새 파일을 생성하거나 패닉 상태에 빠집니다.

    // 함수에서 Result<T, E>를 반환하여 오류를 호출자에게 전파할 수 있습니다. 호출자는 이 결과를 match 표현식이나 다른 방법으로 처리할 수 있습니다.
    match error_propagation() {
        Ok(username) => println!("파일 내용: {}", username),
        Err(e) => println!("오류 발생: {:?}", e),
    }
}

fn error_propagation() -> Result<String, io::Error> {
    let mut username = String::new();
    // let mut username_file = File::open("username.txt")?; // 파일이 열리지 않으면 오류가 호출자에게 전파됩니다.
    // username_file.read_to_string(&mut username)?; // 파일 내용을 읽는 중에 오류가 발생하면 호출자에게 전파됩니다.

    File::open("username.txt")?.read_to_string(&mut username)?; // 위의 두 줄을 한 줄로 줄일 수 있습니다. 파일이 열리지 않거나 읽는 중에 오류가 발생하면 호출자에게 전파됩니다.
    Ok(username) // 파일 내용이 성공적으로 읽어졌을 때 Ok로 감싸서 반환됩니다.
}
