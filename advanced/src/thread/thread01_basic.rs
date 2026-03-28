use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;

/*
프로세스는 CPU에서 수행되는 프로그램의 단위이고 운영체제(OS)에서 관리되는 것이고, 프로세스를 이루는 항목을 구분해보면 [코드, 데이터, 힙, 스택]로 이루어져 있다.

쓰레드는 프로세스 안에서 다시 나눌 수 있는 제어 흐름이고, 프로세스 안에서 여러 쓰레드로 나눠서 작업을 수행할 수 있다.
쓰레드들은 부모가 되는 프로세스의 [코드, 데이터, 힙]을 서로 공유하고, '스택'만을 따로 갖는 구조다.
*/

pub fn run() {
    thread_create();
    thread_join();
    read_and_sum();
}

fn thread_create() {
    // thread 생성
    thread::spawn(|| {
        for i in 1..=9 {
            print!("t{} ", i);
        }
    });

    // 현재 실행 중인 main thread를 잠시 멈춰서
    // 다른 thread가 먼저 실행될 기회를 줌
    thread::sleep(Duration::from_millis(100)); // 실무에서 쓰지 않음.

    for i in 1..=9 {
        print!("m{} ", i);
    }
}

// 해당 스레드가 종료될 때까지 기다리고 다음 실행하기
fn thread_join() {
    println!("");

    // pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    // 그래서 join으로 확인할 수 있다.
    let handle = thread::spawn(|| {
        for i in 1..=9 {
            print!("t{i} ");
        }
    });

    // handle 종료를 보장
    let _ = handle.join().unwrap(); // 정확하게 t 끝날 때까지 대기

    for i in 1..=9 {
        print!("m{i} ");
    }
}

//  텍스트 파일을 읽고 그 안에 있는 내용을 화면에 출력하는 것을 스레드에게 맡기고, 그 사이에 1~10000까지의 모든 홀수를 합한 결과를 출력
fn read_and_sum() {
    //1. Thread: 파일 읽기
    let handle = thread::spawn(|| {
        let file = File::open("Cargo.toml").expect("file open error");
        let reader = BufReader::new(file);
        for line in reader.lines() {
            println!("{}", line.unwrap());
        }
    });

    //2. Main process: 10000까지 홀수를 찾아 더하기
    let mut sum = 0;
    for i in 1..=10000 {
        if i % 2 == 1 {
            sum += i;
        }
    }
    println!("sum = {}\n", sum);

    // join은 출력 순서가 아니라,
    // spawned thread가 끝날 때까지 기다리는 역할이다.
    let _ = handle.join();
}
