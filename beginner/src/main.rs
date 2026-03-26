use std::env;

// 이 파일(모듈)을 컴파일 트리에 포함시켜라
// 모든 모듈은 "부모 모듈에서 선언되어야 존재한다"
mod chapters;
mod match_chapter;

fn main() {
    // args: 커맨드라인 인자 목록
    // 예: cargo run 1
    // => ["target/debug/프로그램이름", "1"]
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("사용법: cargo run <chapter>");
        return;
    }

    // args를 match_chapter 모듈의 run 함수로 전달
    match_chapter::run(&args);
}
