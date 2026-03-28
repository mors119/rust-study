use std::thread;

pub fn run() {
    let v = vec![1, 2, 3, 4];
    // thread::spawn은 새로운 스레드에서 실행되기 때문에,
    // 클로저가 참조(&v)를 캡처하면 v의 수명이 보장되지 않는다.
    // 그래서 move를 사용해서 v의 소유권을 스레드로 이동시켜야 한다.
    let handle = thread::spawn(move || {
        println!("{:?}", &v);
    });

    let _ = handle.join().unwrap();

    // move로 인해 v의 소유권이 스레드로 이동했기 때문에,
    // 메인 스레드에서는 더 이상 v를 사용할 수 없다.
    // println!("v: {:?}", v); // ! 에러
}
