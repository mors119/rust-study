// Loop
// for while과 다르게 조건없이 루프문에 진입한다.
// loop문은 break 키워드가 없으면 멈추지 않는다.

pub fn run() {
    let i: i32 = 0;
    loop_test(i);

    let p = max_factor(10);
    println!("max_factor = {}", p); //max_factor=5
}

fn loop_test(mut i: i32) {
    loop {
        if i > 10 {
            println!("");
            break;
        }
        print!("{}", i);
        i += 1;
    }
}

fn max_factor(mut n: u64) -> u64 {
    let mut p = 2;

    loop {
        let (q, r) = (n / p, n % p);

        // p == n 이면 루프 벗어남.
        if q == 1 {
            break;
        }

        // n % p == 0
        if r == 0 {
            n = q;
        } else {
            p += 1;
        }
    }
    return n;
}
