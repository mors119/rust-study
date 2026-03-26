use crate::chapters; // 현재 크레이트(crate)의 루트에 있는 chapters 모듈을 가져옴

// args를 받아서 어떤 챕터를 실행할지 결정
// &Vec<String> == &[String]
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
        // 반복자
        "21" => chapters::chapter21_iter::run(),
        "22" => chapters::chapter22_map::run(),
        "23" => chapters::chapter23_filter::run(),
        "24" => chapters::chapter24_filter_map::run(),
        "25" => chapters::chapter25_for_each::run(),
        "26" => chapters::chapter26_take_while::run(),
        "27" => chapters::chapter27_example::run(),
        // 소유권
        "28" => chapters::chapter28_owner_ship::run(),
        "29" => chapters::chapter29_copy::run(),
        "30" => chapters::chapter30_struct::run(),
        // 열거형
        "31" => chapters::chapter31_enum::run(),
        "32" => chapters::chapter32_option::run(),
        "33" => chapters::chapter33_result::run(),
        // 트레잇
        "34" => chapters::chapter34_trait_1::run(),
        "35" => chapters::chapter35_trait_2::run(),
        "36" => chapters::chapter36_polymorphism::run(),
        // 제네릭
        "37" => chapters::chapter37_generic_1::run(),
        "38" => chapters::chapter38_generic_2::run(),
        "39" => chapters::chapter39_lite_time::run(),
        // 스마트 포인터
        "40" => chapters::chapter40_smart_pointer_1::run(),
        "41" => chapters::chapter41_smart_pointer_2::run(),
        // 에러 핸들링
        "42" => chapters::chapter42_error_handling::run(),
        // 모듈화
        "43" => chapters::chapter43_module::run(),
        // etc
        "44" => chapters::chapter44_etc_1::run(),
        "45" => chapters::chapter45_etc_2::run(),

        // _
        _ => println!("없는 챕터입니다."),
    }
}
