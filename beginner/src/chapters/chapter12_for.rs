pub fn run() {
    let mut sum = 0; //값 변경이 필요한 변수는 mut 키워드를 사용해야한다. 
    for i in 1..=100 {
        // 동일: for i in 1..101{
        sum += i;
    }
    println!("{}", sum);

    // 반복자 iterator 를 사용하여 루프를 사용할 수 있다.
    let v = vec![1, 2, 3, 4, 5];
    for val in v.iter() {
        //v.iter()를 사용했다.
        print!("{} ", val); // 1 2 3 4 5 
    }

    for val in &v {
        //for 루프에서는 벡터의 iterator가 자동으로 나온다.
        print!("{} ", val); // 1 2 3 4 5 
    }

    'outer: for i in 0..3 {
        for j in 0..3 {
            if j == 1 {
                continue 'outer; // 바깥 루프의 다음 반복으로 바로 이동
            }
            println!("i={}, j={}", i, j);
        }
    }
}
