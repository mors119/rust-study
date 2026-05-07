pub fn run() {
    // 튜플 (tuple)
    let _t: () = (); // 단위 타입 (unit type) - 값이 없는 타입
    let t: (i32, f64, char) = (42, 3.14, 'A');
    t.0; // 42
    t.1; // 3.14
    t.2; // 'A'

    // 배열 (array)
    let _arr: [i32; 5] = [1, 2, 3, 4, 5];
    _arr[0]; // 1
    _arr[1]; // 2

    // 슬라이스 (slice)
    let _s: &[i32] = &_arr[1..4]; // 슬라이스는 배열의 일부분을 참조하는 타입입니다.
    let _s: &[i32] = &_arr; // 전체 배열을 슬라이스로 참조할 수도 있습니다.
    _s[0]; // 1
    _s[1]; // 2

    println!("All types have been defined successfully.");
}
