/*
프로젝트
 cargo new [project_name]
패키지: 하나 이상의 크레이트로 이루줘진 최상위 개념
 Cargo.toml에 기술
크레이트:
 바이너리: main 함수 포함, 실행 가능한 프로그램으로 컴파일 가능
  cargo new <프로젝트명> [--bin 기본값] : 바이너리 크레이트 생성. src/main.rs가 생성 됨
 라이브러리: main 미포함, 다른 프로젝트에서 공용으로 사용할 수 있는 기능들을 포함
  cargo new <프로젝트명> --lib : 라이브러리 크레이트 생성 src/lib.rs가 생성됨
모듈: Rust에서는 각각의 파일이 하나의 모듈, 한 파일안에서도 mod [모듈명]으로 모듈을 만들 수도 있다.

*/

// 메인 모듈 밑에 'gargen'이라는 모듈이 생성
mod garden {
    // 'garden' 모듈 밑에 'vegetable'이라는 서브 모듈
    // 이 모듈을 메인 모듈에서 접근하려면 pub를 붙여서 선언
    pub mod vegetables {

        // vegetable 모듈 안에서 구조체 하나를 선언
        //  main에서 접근하려면 pub를 붙여줘야 한다.
        #[derive(Debug)]
        pub struct Asparagus {}
    }
}

pub fn run() {
    // garden::vegetables::Asparagus {} 처럼 계층구조를 명시해서 접근
    let plant = garden::vegetables::Asparagus {};
    println!("I'm growing {:?}", plant);
}
