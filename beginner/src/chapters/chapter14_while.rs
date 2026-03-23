// while 뒤의 조건식이 참일 경우에만 반복을 계속하는 루프

pub fn run() {
    let mut sum = 0;
    let mut i = 1;

    while i <= 100 {
        sum += i;
        i += i;
    }
    println!("sum: {}", sum);
}
