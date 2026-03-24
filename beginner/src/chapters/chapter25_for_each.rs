// map은 액션을 한 후 그 결과를 또 다른 Iterator로 만들어서 리턴한다
// * for_each는 액션을 그냥 수행해서 종결한다

pub fn run() {
    //1. 각 원소에 대해 +1을 해서 업데이트
    let mut v = vec![1, 2, 3, 4, 5];
    v.iter_mut().for_each(|x| *x += 1);
    println!("{:?}", v); //[2, 3, 4, 5, 6]

    //2. 홀수 인덱스에는 1, 짝수 인덱스에는 0을 가지는 배열 만들기
    let mut v = vec![1; 10];
    v.iter_mut()
        .enumerate()
        .filter(|(i, _)| *i % 2 == 0)
        .for_each(|(_, val)| *val = 0);
    println!("{:?}", v); //[0, 1, 0, 1, 0, 1, 0, 1, 0, 1]
}
