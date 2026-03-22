// 클로저: 내부함수에서 외부 변수의 상태를 기억하며 어떤 연산을 하는 목적
// let [변수명] = | [파라미터] | [표현식];
// 람다함수와 유사하다.

pub fn run() {
    // 함수와 달리 파라미터의 타입지정 안해도 됨
    // let add = |x:i32, y:i32| x+y;
    let add1 = |x, y| x + y;
    println!("{}", add1(2, 3)); //5

    let add2 = |x, y| {
        let a = x + y;
        let b = x + y + add1(5, 6);
        // return (a, b);
        (a, b)
    }; // 클로저 문장의 끝에 ;을 븉여야함.
    println!("{}, {}", add2(1, 5).1, add2(2, 5).1); //17, 18

    let v: Vec<i32> = (1..=10000)
        .into_iter()
        .filter(|x| x % 3 == 0 || x % 5 == 0)
        .collect();
    assert_eq!(4667, v.len());

    let v = vec![1, 2, 3];
    assert_eq!(1, get_val(&v, 3));
    assert_eq!(3, get_val(&v, 2));

    let v: Vec<i32> = Vec::new();
    assert_eq!(0, get_val(&v, 1));
}

// & → 참조(주소 가져오기)
// * → 역참조(값 꺼내기), 그 주소 실제 값
fn get_val(v: &Vec<i32>, idx: usize) -> i32 {
    let val = v
        .get(idx)
        .unwrap_or_else(|| if v.get(0).is_some() { &v[0] } else { &0 });
    return *val;
}
// 또는
// fn get_val(v: &Vec<i32>, idx: usize) -> i32 {
//     let val = match v.get(idx) {
//         Some(x) => x,
//         None => {
//             if v.get(0).is_some() { &v[0] }
//             else { &0 }
//         },
//     };
//     return *val;
// }
