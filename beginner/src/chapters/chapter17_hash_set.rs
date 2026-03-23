use std::collections::HashSet;

pub fn run() {
    let mut set1 = HashSet::new();
    set1.insert(1);
    set1.insert(2);
    println!("{:?}", set1);

    // 벡터나 배열로 해시셋 만들기
    let mut set2 = HashSet::from([1, 2, 3]);
    set2.extend([3, 4, 5].iter());
    println!("{:?}", set2); // {5, 1, 4, 2, 3}
    // 출력하면 데이터의 순서가 마음대로 나온다. HashMap과 마찬기지로 HashSet도 순서 보장을 하지 않는다.

    // 정렬을 하려면 벡터로 바꿔서 정렬을 한다.
    let mut vec1: Vec<_> = set2.into_iter().collect();
    vec1.sort();
    println!("{:?}", vec1);

    // 일부만 선택해서 벡터에 담기
    let set3 = HashSet::from([1, 2, 3]);
    let mut vec2 = Vec::new();
    for x in set3.iter() {
        if *x % 2 == 0 {
            vec2.push(*x);
        }
    }
    // 또는
    let vec3: Vec<_> = set3.iter().filter(|x| *x % 2 == 0).collect();
    println!("{:?}", vec2);
    println!("{:?}", vec3);

    // 집합 연산
    let a = HashSet::from([1, 2, 3, 4]);
    let b = HashSet::from([3, 4, 5]);

    let u: Vec<_> = a.union(&b).collect(); // 합집합
    let i: Vec<_> = a.intersection(&b).collect(); // 교집합
    let d: Vec<_> = a.difference(&b).collect(); // 차집합

    println!("union={:?}", u); //union=[3, 4, 1, 2, 5]
    println!("intersection={:?}", i); //intersection=[3, 4]
    println!("difference={:?}", d); //difference=[1, 2]
}
