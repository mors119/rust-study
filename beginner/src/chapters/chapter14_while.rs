// while 뒤의 조건식이 참일 경우에만 반복을 계속하는 루프
// while let 조건이 맞는 동안 계속 꺼내면서 반복

pub fn run() {
    let mut sum = 0;
    let mut i = 1;

    // 조건이 true/false
    while i <= 100 {
        sum += i;
        i += i;
    }
    println!("sum: {}", sum);

    // while let <타입>=<값>
    // 값이 계속 나오면 반복, 안 나오면 종료
    let mut v = vec![1, 2, 3];

    while let Some(x) = v.pop() {
        println!("{}", x);
    }
}
