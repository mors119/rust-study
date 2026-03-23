use crate::chapters; // 현재 크레이트(crate)의 루트 기준으로 chapters 모듈을 가져옴

// args를 받아서 어떤 챕터를 실행할지 결정
pub fn run(args: &[String]) {
    match args[1].as_str() {
        "1" => chapters::chapter01_mainfn::run(args),
        "2" => chapters::chapter02_var_types::run(),
        // 데이터 타입
        "3" => chapters::chapter03_scalar_types::run(),
        "4" => chapters::chapter04_sequence_types::run(),
        "5" => chapters::chapter05_types_default::run(),
        // 함수
        "6" => chapters::chapter06_function::run(),
        "7" => chapters::chapter07_method::run(),
        "8" => chapters::chapter08_macro::run(),
        "9" => chapters::chapter09_closure::run(),
        // 제어문 반복문
        "10" => chapters::chapter10_if::run(),
        "11" => chapters::chapter11_match::run(),
        "12" => chapters::chapter12_for::run(),
        "13" => chapters::chapter13_loop::run(),
        "14" => chapters::chapter14_while::run(),
        // 컬렉션
        "15" => chapters::chapter15_vec::run(),
        "16" => chapters::chapter16_hashmap::run(),
        "17" => chapters::chapter17_hash_set::run(),
        // 문자열
        "18" => chapters::chapter18_str::run(),
        "19" => chapters::chapter19_string::run(),
        "20" => chapters::chapter20_str_and_string::run(),
        "21" => chapters::chapter21_str_example::run(),
        "22" => chapters::chapter22_str::run(),
        _ => println!("없는 챕터입니다."),
    }
}
