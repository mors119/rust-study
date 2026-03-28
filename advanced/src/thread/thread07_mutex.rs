use std::sync::Mutex;

// Mutex = Mutual Exclusion
// 여러 실행 흐름이 같은 데이터에 접근할 때,
// 한 번에 하나만 접근하도록 보장하는 동기화 도구이다.
pub fn run() {
    let m = Mutex::new(5); // Mutex가 i32 값을 감싼다.

    {
        // lock()은 잠금을 획득하고 MutexGuard를 반환한다.
        // 다른 누군가 이미 잠그고 있다면 여기서 기다린다.
        let mut n = m.lock().unwrap();

        // MutexGuard를 통해 내부 데이터에 접근
        *n = 6;
    } // 여기서 n이 drop되면서 자동으로 unlock

    println!("m = {:?}", m);
}
