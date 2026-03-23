// Vec

// Vec를 쓰는 경우
// stack에는 포인터, 길이, 최대 저장 개수만 있고 heap에 실제 데이터를 저장하므로 array보다는 느림.
//
// Vec
//  ├─ ptr → 실제 데이터 (heap)
//  ├─ len → 현재 길이
//  └─ capacity → 최대 저장 가능 개수
//
// 크기가 바뀔 가능 있음, 사용자 입력, 파일 데이터 등
// array는 크기가 고정되고, 성능이 중요한 경우 사용

pub fn run() {
    vec_new();
    vec_macro();
    vec_method();
}

// Vec::new()로 선언
fn vec_new() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1); // push: 추가
    v.push(2);

    println!("{:?}", v);
}

// Vec! 매크로 이용
fn vec_macro() {
    let mut v1 = vec![5, 6, 7];

    let arr1 = [3, 4];
    let v2 = arr1.to_vec(); // 배열을 벡터로

    v1.extend([1, 2]); // extend: 배열 값을 추가
    v1.extend(v2.clone());

    let arr2 = [8, 9];
    // Iterator Adapter를 이용해서 직접 벡터에 push
    // * IntoIterator 실제 구현은 아래와 같다. 아래와 같이 대체 사용이 가능하다.
    // * for a in v.into_iter()  == for a in v
    // * for a in v.iter() == for a in &v
    arr2.into_iter().for_each(|x| v1.push(x));

    println!("{:?}", v1);

    println!("v[0]={}", v1[0]); // 첫 원소 출력
    println!("last element={}", v1[v1.len() - 1]); // 마지막 원소 출력

    // println!("{}",v[3]);  // 값이 없는 경우 panic 발생

    // println!("{:?}", v.get(3)); // Option 타입으로 리턴하므로 이렇게 쓰면 None 출력
    if let Some(n) = v2.get(1) {
        println!("v2.get(1)={}", n);
    }

    println!("");
    print!("불변 참조: ");
    // 불변 참조
    // 내부적으로 (&v).into_iter()
    // 타입: &i32
    // 실제로는 v.iter()와 동일, 값이 아니라 “참조 &i32”를 순회
    for a in &v1 {
        print!("{} ", a);
    }

    println!("");
    print!("가변 참조: ");
    // 가변 참조
    // 타입: &mut i32
    // 내부 값을 수정 가능
    for a in &mut v1 {
        *a += 1;
        print!("{} ", a); // 원래는 print("{}" ,*a); 해야하지만,
        // print나 println은 자동으로 deref(*) 해줌.
    }

    println!("");
    print!("소유권 이동: ");
    // 소유권 이동
    // 내부적으로 v.into_iter()
    // Vec<T> → T를 하나씩 꺼내면서 소유권 이동
    // v는 비어버린 게 아니라 “소유권을 잃어서 사용 불가”
    for a in v1 {
        print!("{} ", a);
    }
    // println!("{}", v1[0]); // “소유권을 잃어서 사용 불가” ❌

    /* 업데이트 하기 */
    println!("");
    println!("---------업데이트 하기----------");
    let mut v3 = vec![1, 2, 3];

    // 인덱스로 접근해서 값 업데이트
    for i in 0..v3.len() {
        v3[i] *= 2;
    }

    // mutable 반복자 이용: &mut v
    for a in &mut v3 {
        *a *= 2;
    }

    // mutable 반복자 이용 : v.iter_mut()
    for a in v3.iter_mut() {
        *a *= 2;
    }

    // Iterator Adapter 이용
    v3.iter_mut().for_each(|a| *a *= 2);

    // Vec를 스택(후입선출, LIFO)으로 사용하기
    let mut st: Vec<i32> = Vec::new(); // 스택 선언: 그냥 선언하면 됨.
    st.push(0);
    st.push(1);

    while st.len() > 0 {
        let i = st.pop().unwrap();
        println!("pop: {}", i);
    }
}

fn vec_method() {
    /* 메서드 */
    println!("");
    println!("---------벡터의 메서드----------");

    // append : pub fn append(&mut self, other: &mut Vec<T, A>)
    // other 벡터의 모든 원소를 self로 이동시킨다. 이동되고 난 후 other 벡터 안의 내용은 사라진다.
    let mut vec = vec![1, 2, 3];
    let mut vec2 = vec![4, 5, 6];
    vec.append(&mut vec2);

    // clear: pub fn clear(&mut self)
    // 벡터안의 데이터를 지운다. vec 크기가 줄어 들지는 않는다.
    vec2.clear();
    assert!(vec2.is_empty());

    // len: pub fn len(&self) -> usize
    // 벡터안의 원소의 개수를 리턴한다. 벡터의 크기를 리턴하는 셈
    assert_eq!(vec.len(), 6);

    // is_empty: pub fn is_empty(&self) -> bool
    // 벡터안의 원소가 하나도 없으면 true를 리턴
    assert!(!vec.is_empty());

    // insert: pub fn insert(&mut self, index: usize, element: T)
    // 벡터의 index 위치에 값을 집어 넣는다. 삽입된 위치 오른쪽에 있는 데이터들은 오른쪽으로 shift된다. 만약 `index > len`이면 panic 발생
    vec.insert(6, 7);
    println!("{:?}", vec);

    // pop: pub fn pop(&mut self) -> Option<T>
    // 벡터를 스택으로 사용할 때 벡터의 제일 마지막 원소를 리턴하고 해당 원소를 벡터에서 제거한다. 만약 원소가 하나도 없으면 `None`을 리턴한다.
    assert_eq!(vec.pop(), Some(7));

    // push: pub fn push(&mut self, value: T)
    // 벡터의 제일 뒤 편에 값을 집어 넣는다.
    vec.push(7);

    // remove: pub fn remove(&mut self, index: usize) -> T
    // 벡터의 index 위치에 있는 원소를 삭제한다. 삭제되는 원소 다음에 있는 데이터들은 왼쪽으로 shift 된다.
    vec.remove(1);
    println!("remove vec[1] => {:?}", vec);

    // resize: pub fn resize(&mut self, new_len: usize, value: T)
    // 벡터의 크기를 new_len가 되게 조정한다. 만약 new_len가 기존 크기보다 크면, 기존 벡터를 new_len가 되게 확장하고 확장된 부분은 value로 채운다.
    // 만약 new_len가 기존 크기보다 작으면, 기존 벡터의 크기를 new_len가 되게 축소해버리고, 뒤 쪽에 있는 원소들은 짤려 없어지게 된다.
    // 단순하게 축소하는 거면 `truncate`를 사용하는게 낫다.
    vec.resize(8, 8);
    println!("resize => {:?}", vec);
    vec.resize(7, 0);
    println!("resize => {:?}", vec);

    // truncate: pub fn truncate(&mut self, len: usize)
    // 벡터를 주어진 len 크기가 되도록 줄인다. len 보다 큰 위치에 있던 데어터들은 사라지게 된다. 만약 len가 기존 벡터 크기보다 크다면, 아무 영향도 안 미친다.
    vec.truncate(6);
    println!("truncate len = 6 => {:?}", vec);

    // fill: pub fn fill(&mut self, value: T)
    // value 값으로 벡터의 모든 원소를 채운다.
    vec.fill(0);
    println!("fill => {:?}", vec);

    let mut v = vec![4, -5, 1, -3, 2, 3];
    // reverse: pub fn reverse(&mut self)
    // 모든 원소의 순서를 바꿔서 재배열한다.

    v.reverse();
    println!("vec![4, -5, 1, -3, 2, 1] reverse => {:?}", v);

    // sort: pub fn sort(&mut self)
    // 모든 원소를 정렬한다.
    v.sort();
    // v.reverse();
    println!("sort => {:?}", v);
}
