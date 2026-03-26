use std::io::{self, BufRead, Read};

pub fn run() {
    mono_thread();
}

// 단일 쓰레드
fn mono_thread() {
    let stdin = std::io::stdin(); // 콘손 입력용 핸들러
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("input error");

    let input: Vec<String> = buf.split(' ').map(|s| s.to_string()).collect(); // 공백으로 문자를 나누고 vec에 담기
    let n = input[0].trim().parse::<usize>().unwrap(); // 숫자형으로 전환
    let k = input[1].trim().parse::<usize>().unwrap();

    println!("n = {}", n);
    println!("k = {}", k);
}

// 멀티 쓰레드
fn _multi_thread() {
    let mut buf = String::new();
    // 콘솔 핸들러를 얻은 후 lock을 걸지 않으면, 중간에 콘솔 핸들러를 다른 쓰레드에 빼앗길 수도 있다.
    let len = io::stdin()
        .lock() // 핸들에 Lock 걸기
        .read_line(&mut buf)
        .expect("reading console input error.");

    let input: Vec<String> = buf.split(' ').map(|s| s.to_string()).collect();
    let n = input[0].trim().parse::<usize>().unwrap();
    let k = input[1].trim().parse::<usize>().unwrap();

    println!("n = {}", n);
    println!("k = {}", k);
}

// 여러 줄 읽기
fn _read_lines() {
    let lines = io::stdin().lock().lines(); // 

    for line in lines {
        let s = line.unwrap();
        if s.eq("xxx") {
            return;
        }
        println!("input: {}", s);
    }
}

// 빠르게 여러 줄 읽기
fn _fast_read_lines() {
    let mut sum = 0;
    let mut cnt = 0;

    let mut handle = io::stdin().lock();
    let mut buf = String::new();
    let mut eof = false;

    while !eof {
        match handle.read_line(&mut buf) {
            //(2)
            Ok(0) => {
                eof = true;
            }
            Ok(_) => {
                let n = buf.trim().parse::<i32>().expect("parsing error");
                buf.clear();
                sum += n;
                cnt += 1;
            }
            Err(error) => {
                panic!("something wrong:{}", error);
            }
        }
    }
    println!(
        "(cnt, sum, avg)=({},{},{})",
        cnt,
        sum,
        sum as f32 / cnt as f32
    );
}

// 2차원 배열 만들기
fn _arr() {
    let mut buf = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut buf)
        .expect("input error"); //(2)

    let mut grid: Vec<Vec<i32>> = Vec::new(); // (3)
    for line in buf.lines() {
        // (4)
        let v: Vec<i32> = line
            .trim()
            .split(' ')
            .filter_map(|w| w.parse().ok())
            .collect(); // (5)
        grid.push(v);
    }
    println!("{:?}", grid);
}
