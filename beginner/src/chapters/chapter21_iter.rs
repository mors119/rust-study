/*
* iter
반복자에 의해 접근되는 원소들의 레퍼런스가 넘어온다. 소유권이 이동되는 것이 아니다.
따라서, 원소들을 사용하고 난 후에도, 해당 컬렉션에 대한 사용이 가능하다. 컬렉션의 소유권이 이동하지 않았기 때문이다.
* into_iter
컬렉션 자체가 넘겨져서, 소유권이 넘어가 버리기에, into_iter()를 수행하고 난 후에는, 해당 컬렉션 변수로의 접근이 안된다.
* iter_mut
컬렉션의 값을 수정해야할 때 사용한다. 레퍼런스로 받은 다음에 수정하는 것이다. 소유권이 넘어가지는 않는다.
*/

pub fn run() {
    iter();
    into_iter();
    iter_mut();
}

fn iter() {
    let v = vec![1, 2, 3, 4, 5];

    //v.iter()를 사용했다.
    for val in v.iter() {
        print!("{} ", val); // 1 2 3 4 5 
    }
    println!("");

    //for 루프에서는 벡터의 iterator가 자동으로 나온다.
    for val in &v {
        print!("{} ", val); // 1 2 3 4 5 
    }
}

// iterator 사용 후에 다시 해당 컬렉션을 접근해야하는 경우는 iter()를 사용하는 게 편리하고, 그렇지 않은 경우는 into_iter()를 사용
fn into_iter() {
    let v = vec![1, 2, 3, 4, 5];

    for val in v.into_iter() {
        //into_iter()를 사용했다. 소유권이 넘어간다.
        print!("{} ", val); // 1 2 3 4 5 
    }
    println!("");

    // 위 v.into_iter()에 의해 소유권이 넘어갔기 때문에 에러가 발생한다..
    // for val in &v {
    //     print!("{} ", val); // 1 2 3 4 5
    // }
}

fn iter_mut() {
    let mut v = vec![1, 2, 3, 4, 5];
    for x in v.iter_mut() {
        // x는 &mut i32 (참조)이므로, 실제 값을 수정하려면 역참조(*)가 필요하다.
        *x += 1;
    }
    println!("{:?}", v); // [2,3,4,5,6]

    // iter_mut()에 의해 소유권이 넘어가지 않기 때문 동작이 가능하다.
    for val in &v {
        print!("{} ", val); // 2,3,4,5,6 
    }
}

/*
다루는 것 외의 다양한 메소드
https://doc.rust-lang.org/std/iter/trait.Iterator.html

take(n) : iterator에서 n개의 원소를 취해서 iterator를 만든다.
windows(n) : iterator에서 n개씩의 원소를 취해서 iterator를 만든다.
step_by(n) : iterator에서 n개씩 건너 뛰면서 원소를 취해서 iterator를 만든다.
등
*/
