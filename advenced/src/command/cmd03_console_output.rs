use std::io::{self, BufWriter, Write};
// println!  → 출력 대상이 고정 (stdout)
// writeln!  → 출력 대상이 유연 (파일, 버퍼, stdout 등)

pub fn run() {
    // 락 핸들러는 해당 변수의 스코프를 벗어날 때 해제, run 함수를 벗어나면 해제
    let mut lock = std::io::stdout().lock(); //  for 루프 수행 전에 stdout에 대한 lock을 수행
    for i in 1..=1000 {
        writeln!(lock, "{}", i).unwrap(); // writeln! 매크로는 Result 타입을 리턴
    }
}

// 여러 줄 한번에 출력 BufWriter
fn _lines_output() {
    let stdout = io::stdout().lock(); // 락으로 다른 스레드 인터럽트 방지
    let mut writer = BufWriter::new(stdout); // BufWriter는 stdout뿐 아니라 파일, TCP Stream에도 사용

    for i in 1..=1000 {
        writeln!(writer, "{}", i).unwrap(); // 버퍼에 쓰기 작업
    }
    writer.flush().unwrap(); //  flush() 메서드에 의해서 실제 출력
}
