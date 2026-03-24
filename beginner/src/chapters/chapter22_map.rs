/*
* for_each: 루프를 돌려서 값들을 변경하고 그냥 종결해도 될 때
* map: 루프를 돌려서 값들을 변경하고, 변경된 값을 이용해서 다시 무언가를 해야할 때

* iter(): iterator를 쓰고 난 후, 해당 컬렉션으로의 접근을 해야하는 경우 사용
* into_iter(): 해당 컬렉션을 다시 사용하지 않을거라고 생각되는 경우

만약 어떤 벡터에 대해서 그 벡터의 값을 바꾸고자 한다면 v.iter_mut().for_each(...);를 사용해서 바꾸는 게 좋고,
어떤 벡터의 값을 읽어서 가공 후 다른 벡터로 저장하길 원하면 v.iter().map(...).collect();와 같이 하는게 낫다.
* `v.iter_mut().for_each(...);`를 하게되면 `for_each`에 의한 연산에 의해 `v`의 원소 내용이 바뀌고 종료된다.
* `v.iter().map(...).collect();`을 하게되면, `map`에 의해 연산된 결과가 다시 반복자 형태로 나오고, 이 반복자에 대해 `collect()`를 하는 것이기에 새로운 벡터가 생성되는 것이다.

* Method
collect() : iterator의 내용을 collection으로 만든다. 어떤 컬렉션으로 만들지는 지정해줘야한다.
sum() : iterator의 내용을 합한 결과를 리턴. 결과가 어떤 타입(u32 등)일 지는 지정해줘야 한다.
max() : 최댓값을 리턴. 타입 지정 필요
min() : 최솟값을 리턴. 타입 지정 필요
count() : iterator의 원소 개수 리턴
product() : iterator의 각 원소를 곱한 결과를 리턴. 타입 지정 필요
*/

pub fn run() {
    map_test();
    map_test2();
    map_test3();
}

fn map_test() {
    let v = vec![1, 2, 3, 4, 5];
    // * iter()를 사용했기에 클로저에 전달되는 것은 &i32 타입 원소가 전달된다.
    // * 이것을 &x로 받은 것이기에 역참조(de-referencing)가 된다.
    // &x=&i32인 셈이기에 x=i32가 되어, 클로저 x에는 벡터의 값이 복사되어 전달된다. 따라서, 뒤에 오는 x+1에서의 x는 복사된 x이다.
    // 여기서는 |x|라고 전달해도 문제없이 동작한다. |x|라고 사용하면 x=&i32가 되어 x는 참조자다. Rust에서는 기본형 타입 변수에 대한 연산의 경우 참조자가 가리키는 값을 가지고 이루어지기에 결과는 똑같다.
    // i32, f32, f64 등이 기본형(primitive) 변수는 연산을 여러 형태로 (impl로)구현해 뒀다.
    let v1: Vec<_> = v.iter().map(|&x| x + 1).collect();

    println!("v={:?}", v); // v=[1, 2, 3, 4, 5]
    println!("v1={:?}", v1); // v1=[2, 3, 4, 5, 6]
}

fn map_test2() {
    // (1..=100).map이라고 한 것에 유의. (1..=100) 자체가 일종의 iterator이다.
    let ans: Vec<_> = (1..=100).map(|x| x * x).collect();
    println!("{:?}", ans);

    // <_>는 내부 데이터 타입에 의해서 타입 결정
    let ans = (1..=100).map(|x| x * x).collect::<Vec<_>>();
    println!("{:?}", ans);
}

fn map_test3() {
    // 1. 각 원소에 대해 2을 곱해서 다른 벡터로 만듦
    let v = vec![1, 2, 3, 4, 5];
    let v1: Vec<_> = v.iter().map(|x| (*x) * 2).collect();
    println!("{:?}", v1); //[2, 4, 6, 8, 10]
    // map(|x| x*2): 이렇게 해도 된다.
    let v2: Vec<_> = v.iter().map(|x| x * 2).collect();
    println!("{:?}", v2); //[2, 4, 6, 8, 10]
    // into_iter()을 써도 됨
    let v = vec![1, 2, 3, 4, 5];
    let v1: Vec<_> = v.into_iter().map(|x| x * 2).collect();
    println!("{:?}", v1); //[2, 4, 6, 8, 10]
    // collect::<Vec<u32>>()와 같이 해도 됨
    let v = vec![1, 2, 3, 4, 5];
    let v1 = v.iter().map(|x| (*x) * 2).collect::<Vec<u32>>();
    println!("{:?}", v1); //[2, 4, 6, 8, 10]

    //2. 문자열을 모두 소문자로 변환
    let words: Vec<&str> = vec!["Hello", "Good Morning", "Hi"];
    let low_words: Vec<String> = words.iter().map(|w| w.to_lowercase()).collect();
    println!("{:?}", low_words); //["hello", "good morning", "hi"]        

    //3. 문자열에서, 각 문자에 대해 문자열에서 해당 문자의 개수를 HashMap으로 저장. (ch: cnt)
    use std::collections::HashMap;
    let s = "abc aaa bb c";
    let map: HashMap<char, usize> = "abc"
        .chars()
        .map(|c| (c, s.matches(c).count()))
        .collect::<HashMap<char, usize>>();
    println!("{:?}", map); //{'a': 4, 'c': 2, 'b': 3}

    // (참조)어떤 문자열에서 알파벳만을 집합으로 뽑아낼 때
    use std::collections::HashSet;
    let s = "abc aaa bb c";
    let set = s
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<HashSet<_>>();
    println!("{:?}", set); //{'c', 'a', 'b'}
}
