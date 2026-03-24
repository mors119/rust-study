pub fn run() {
    simple1();
    simple2();
    middle();
    string_test1();
    string_test2();
    string_test3();
    complicated();
}

fn simple1() {
    // 1~100까지 합
    let sum: usize = (1..=100).sum();
    println!("sum={}", sum); //5050

    //1에서10까지의 곱
    let p: usize = (1..=10).product();
    println!("porduct={}", p); //3628800

    // 1~100에서 짝수의 합
    let sum: usize = (1..=100).filter(|x| *x % 2 == 0).sum();
    println!("sum={}", sum); //2550

    // 1~100에서 3혹은 5의 배수의 합
    let sum: usize = (1..=100).filter(|x| *x % 3 == 0 || *x % 5 == 0).sum();
    println!("sum={}", sum); //2418

    // 1~100에서 3혹은 5의 배수의 개수
    let cnt: usize = (1..=100).filter(|x| *x % 3 == 0 || *x % 5 == 0).count();
    println!("cnt={}", cnt); //47
}

fn simple2() {
    let v: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    //count(): 짝수 개수
    let cnt = v.iter().filter(|x| *x % 2 == 0).count();
    println!("count={}", cnt); //count=5

    //sum(): 벡터의 모든 원소 합
    let sum: usize = v.iter().sum();
    println!("sum={}", sum); //sum=55

    //product(): 벡터의 모든 원소 곱
    let product1: usize = v.iter().product();
    println!("product={}", product1); //product=3628800

    //filter().product: 원소 중 4보다 작은 값에 대한 곱 : |&x|
    let product: usize = v.iter().filter(|&x| x < &4).product();
    println!("product={}", product); //product=6

    //filter().product(): 원소 중 4보다 작은 값에 대한 곱 : |x|
    let product: usize = v.iter().filter(|x| *x < &4).product();
    println!("product={}", product); //product=6

    //take(n).product(): 원소중 앞으로부터 3개를 취해서 곱: take(n)
    let product: usize = v.iter().take(3).product();
    println!("product={}", product); //product=6

    //take_while: filter와 달리, 조건식이 false이면 실행중단
    let sum: usize = v.iter().take_while(|x| *x < &5).sum(); //1,2,3,4
    println!("sum={}", sum); //10

    //step_by(n).sum(): 4씩 건너뛰면서 읽어서 합
    let sum: usize = v.iter().step_by(4).sum(); //1,5,9
    println!("sum={}", sum); //sum=15

    //filter().max(): 3의 배수 중 최댓값
    let max = v.iter().filter(|x| *x % 3 == 0).max().unwrap();
    println!("max={}", max); //max=9
}

fn middle() {
    //[1..100]에서 10의 배수값들에 대해서 10으로 나눈 후 합
    let sum: u32 = (1..=100)
        .filter(|x| x % 10 == 0) //10,20,...,90,100
        .map(|x| x / 10) //1,2,...,9,10
        .sum();
    println!("sum={}", sum); //55

    //[1..100]에서, 10보다 작은 수에서, 3의 배수 합
    let sum: u32 = (1..=100)
        .take_while(|x| *x < 10)
        .filter(|x| x % 3 == 0)
        .sum(); //3,6,9
    println!("sum={}", sum); //18  
}

fn string_test1() {
    //map().filter().map(): 문자열 중 숫자로된 문자열만 골라내서, 해당 문자열을 숫자로 변환해서 벡터로 만듦
    let a = ["1", "two", "NaN", "four", "5"];
    let v: Vec<_> = a
        .iter()
        .map(|s| s.parse::<i32>())
        .filter(|s| s.is_ok())
        .map(|s| s.unwrap())
        .collect();
    println!("v={:?}", v); //v=[1, 5]

    //filter_map(): 위 예제와 동일
    let a = ["1", "two", "NaN", "four", "5"];
    let v: Vec<_> = a.iter().filter_map(|s| s.parse::<i32>().ok()).collect();
    println!("v={:?}", v); //v=[1, 5]

    //문자 배열을 십진수 숫자 배열로 변환
    let a = ['1', '2', '3', 'f', '5'];
    let v: Vec<_> = a.iter().filter_map(|c| c.to_digit(10)).collect();
    println!("v={:?}", v); //v=[1, 2, 3, 5]
}

fn string_test2() {
    //앞 5개 문자만 짤라내서, 숫자로 변환
    let input: &str = "73167176531330624919225119674426574742355349194934";
    let v: Vec<u32> = input
        .chars()
        .take(5)
        .filter_map(|c| c.to_digit(10))
        .collect();
    println!("{:?}", v); //[7, 3, 1, 6, 7]

    //5번째 숫자부터 5개 문자를 짤라내서, 숫자로 변환
    let input: &str = "73167176531330624919225119674426574742355349194934";
    let v: Vec<u32> = input[4..9].chars().filter_map(|c| c.to_digit(10)).collect();
    println!("{:?}", v); //[7, 1, 7, 6, 5]

    //숫자로 모두 변환 후 짝수만 더하기
    let input: &str = "73167176531330624919225119674426574742355349194934";
    let sum: u32 = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .filter(|x| *x % 2 == 0)
        .sum();
    println!("sum={}", sum); //72
}

fn string_test3() {
    //3칸씩 이동 후 3자리씩 짤라내면서, 숫자 정수로 변환하기
    let input: &str = "73167176531330624919225119674426574742355349194934";
    let sum: u32 = input
        .chars()
        .collect::<Vec<char>>()
        .chunks(3) //"731", "671", ...
        .map(|c| c.iter().collect::<String>())
        .filter_map(|s| s.parse::<u32>().ok())
        .sum();
    println!("sum={}", sum); //sum=7880

    //1칸씩 이동 후 3자리씩 짤라내면서, 숫자 정수로 변환하기
    let input: &str = "73167176531330624919225119674426574742355349194934";
    let sum: u32 = input
        .chars()
        .collect::<Vec<char>>()
        .windows(3) //"731", "316", "167", ...
        .map(|c| c.iter().collect::<String>())
        .filter_map(|s| s.parse::<u32>().ok())
        .sum();
    println!("sum={}", sum); //sum=23711
}

// #[test]
fn complicated() {
    // 체(sieve) 이용해서 소수 구하기
    let sieve = [
        0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0,
    ]; //prime number sieve
    let v: Vec<u32> = sieve
        .iter()
        .enumerate()
        .filter(|(_, val)| **val == 1)
        .map(|(i, _)| i as u32)
        .collect();
    println!("prime number: {:?}", v); //[2, 3, 5, 7, 11, 13, 17, 19]

    //10000에서 20000사이의 소수 구한 후, 합 구하기
    let sum: u32 = (10000..=20000).filter(|i| is_prime(*i)).sum();
    println!("sum={}", sum); //15434795
}

// #[cfg(test)]
//소수인지 판별
fn is_prime(t: u32) -> bool {
    if t % 2 == 0 {
        return false;
    }

    let sqrt_n = (t as f64).sqrt() as u32;
    let mut i: u32 = 3;
    while i <= sqrt_n {
        if t % i == 0 {
            return false;
        }
        i += 2;
    }
    return true;
}
