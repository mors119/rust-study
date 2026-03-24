pub fn run() {
    filter_test1();
    filter_test2();
}

// 원소 중 짝수인 것만을 추려서 벡터로
fn filter_test1() {
    let v = vec![1, 2, 3, 4, 5];
    // * v.iter()에 의해 리턴되는 것은 `&i32`타입이다.
    // * filter()는 &T 타입으로 인자를 받는다.
    // * 그래서, 결국 filter(|x|) 에서의 x에 대한 타입은 `&&i32`라고 할 수 있고, 따라서 `&&x`의 형태로 받았다. (가장 추천)
    let v1: Vec<_> = v.iter().filter(|&&x| x % 2 == 0).collect();
    // * `&&i32` 타입이어도 rust가 한번 deref 해주기 때문에 `*`를 하나만 붙여서 역참조도 가능하다.
    // 또는 let v1: Vec<_> = v.iter().filter(|x| *x % 2 == 0).collect(); 정석은 아님.
    // 또는 let v1: Vec<_> = v.iter().filter(|x| **x % 2 == 0).collect();

    println!("v={:?}", v); // v = [1, 2, 3, 4, 5]
    println!("v1={:?}", v1); // v1 = [2, 4]
}

// iter()은 컬렉션의 레퍼런스가 넘겨진다고 했다. 즉, x가 아니라 &x인 셈이다. 여기에 더해서 filter 또한 레퍼런스를 넘긴다. 즉, filter에 넘겨지는 것은 &&x가 되어 버린다. 따라서, x값을 dereferencing하려면**x를 해야하는 것이다
fn filter_test2() {
    //1. 모든 원소 중 짝수인 원소만 추려서 벡터로 만듦
    let v = vec![1, 2, 3, 4, 5];
    let v1: Vec<_> = v.iter().filter(|x| *x % 2 == 0).collect();
    println!("{:?}", v1); //[2, 4]

    //into_iter()를 써도 됨
    let v = vec![1, 2, 3, 4, 5];
    let v1: Vec<_> = v.into_iter().filter(|x| x % 2 == 0).collect();
    println!("{:?}", v1); //[2, 4]

    //2. 10이상이면서 홀수인 원소만 추려서 벡터로 만듦
    let v = vec![1, 2, 3, 10, 11, 12, 13];
    let v1: Vec<_> = v
        .iter()
        .filter(|x| (*x) >= (&10) && (*x) % (&2) == 1)
        .collect();
    println!("{:?}", v1); //[11, 13]

    //"**x>=10 && **x%2==1"와 같이 해도 됨. 더블 * 사용
    let v = vec![1, 2, 3, 10, 11, 12, 13];
    let v1: Vec<_> = v.iter().filter(|x| **x >= 10 && **x % 2 == 1).collect();
    println!("{:?}", v1); //[11, 13]

    //into_iter()를 써도 됨
    let v = vec![1, 2, 3, 10, 11, 12, 13];
    let v1: Vec<_> = v.into_iter().filter(|x| x >= &10 && x % 2 == 1).collect();
    println!("{:?}", v1); //[11, 13]
}
