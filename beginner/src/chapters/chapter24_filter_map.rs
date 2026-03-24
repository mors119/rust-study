// * 클로저에 대한 결과가 Option 형태이면, 이 Option 형태에서 Some(value)의 value 부분만을 뽑아내는 것이 map_filter 메서드
// * filter_map의 괄호 안에 있게되는 클로저의 리턴 타입은 Option 타입
pub fn run() {
    filter_map_test1();
}

fn filter_map_test1() {
    let a = ["1", "two", "NaN", "four", "5"];
    let v1: Vec<_> = a
        .iter()
        .map(|s| s.parse::<i32>()) // 문자열에 대해 parsing 시도, Result 리턴
        .filter(|s| s.is_ok()) // 결과 중 ok된 것만(err 아닌 것만)을 filtering해서
        .map(|s| s.unwrap())
        .collect();
    // 동일, map - filter - map 한 것을 하나의 메서드에서 수행
    let v2: Vec<_> = a.iter().filter_map(|s| s.parse::<i32>().ok()).collect();

    println!("v1={:?}, v2={:?}", v1, v2); // v = [1, 5]
}
