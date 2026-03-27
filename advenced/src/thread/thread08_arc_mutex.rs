use std::sync::{Arc, Mutex};
use std::thread;

// Mutex라는 것이 힙을 사용하는 것이기에 여러 쓰레드가 해당 Mutex를 얻어내기 위해서는 동일한 Mutex를 바라 봐야하고
// 이것은 Rust의 소유권 원칙에 위배되는 것이기 때문이다. 이를 해소하기 위해서 Arc로 뮤텍스를 감싸줘야 하는 것이다.

pub fn run() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for i in 0..5 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            for _ in 0..10 {
                // lock()을 호출한 스레드만 내부 값을 수정할 수 있다.
                let mut num = counter.lock().unwrap();
                *num += 1;

                println!("thread {i} incremented counter to {}", *num);
            }
        });

        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());

    println!("final counter = {}", *counter.lock().unwrap());
}
