// * filter와 유사하지만 filter는 조건을 모든 것을 원소 끝까지, take_while은 조건을 벗어나는 원소를 만나면 더 이상 찾지 않는다.
pub fn run() {
    let v = vec![1, 3, 5, 7, 9, 10, 11, 13, 15];
    let v1: Vec<_> = v.iter().filter(|&&x| x % 2 == 1).collect(); //모든 홀수를 찾는다.
    let v2: Vec<_> = v.iter().take_while(|&&x| x % 2 == 1).collect(); // 홀수가 아닐 때까지의 홀수만

    println!("v1={:?}", v1); //v1=[1, 3, 5, 7, 9, 11, 13, 15]
    println!("v2={:?}", v2); //v2=[1, 3, 5, 7, 9]  
}
