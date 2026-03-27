use std::sync::Arc;
use std::thread;

// Arc = Atomic Reference Counted
// 여러 스레드에서 하나의 데이터를 안전하게 공유하기 위한 스마트 포인터
// 내부적으로 ref count 증가/감소가 atomic 연산으로 처리되어 thread-safe하다.
// 단, Arc 자체는 "소유권 공유"만 해결하며, 내부 데이터 변경까지 안전하게 해주지는 않는다.
// 데이터를 수정하려면 Arc<Mutex<T>> 같은 형태를 사용해야 한다.

pub fn run() {
    let s = Arc::new(String::from("hello"));

    let mut handles = vec![];

    for i in 0..3 {
        let ss = Arc::clone(&s);

        let handle = thread::spawn(move || {
            println!("thread {i}: {ss}");
        });

        handles.push(handle);
    }

    // 여기서 한 번에 join → 진짜 병렬 실행
    for handle in handles {
        handle.join().unwrap();
    }

    println!("main: {s}");
}
