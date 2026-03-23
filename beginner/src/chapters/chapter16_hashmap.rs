// HashMap: Key-Value로 구성
use std::collections::HashMap;

pub fn run() {
    // 메크로는 따로 없으므로 아래처럼 선언만 가능하다
    let mut map1: HashMap<&str, i32> = HashMap::new();
    map1.insert("Jeff", 100); // insert: 값을 넣는다.

    let map2: HashMap<&str, i32> = HashMap::from([("Jeff", 100), ("Tom", 90)]);
    println!("{:?}", map2);

    // vec 값을 넣기
    let name = vec!["Jeff", "Tom", "Josh"];
    let score = vec![100, 90, 80];
    //  HashMap<_,_> 타입을 지정하면 자동으로 타입을 추론한다.
    // zip 메서드는 반복자 2개에 대해서 1:1로 데이터를 짝짓기 해준다
    // 3개의 튜플이 생기고, 이것이 collect에 의해서 HashMap으로 생성
    let mut map3: HashMap<_, _> = name.into_iter().zip(score.into_iter()).collect();
    println!("{:?}", map3);

    /* 값 가져오기 / 쓰기 */
    // Key를 지정해서 get
    println!("Jeff's score:{}", map3.get("Jeff").unwrap()); //Jeff's score:100

    // HashMap에 있는 모든 Key-Value
    for (k, val) in &map3 {
        println!("{}: {}", k, val);
    }

    // Key를 얻어낸 후 Value 액세스
    for k in map3.keys() {
        if k.starts_with("J") {
            print!("{} ", map3.get(k).unwrap());
        }
    }

    //덮어쓰기
    map3.insert("Jeff", 50);

    // 없는 데이터만 추가하기
    let new_data = [("Jeff", 50), ("Alice", 10)];
    for (k, v) in &new_data {
        // .entry(k).or_insert(*v): Key가 이미 있는지 조사해서 있으면 안쓰고 없으면
        // "Jeff"는 이미 값이 있기에 갱신하지 않고 "Alice" 데이터만 추가
        map3.entry(k).or_insert(*v);
    }

    // 나온 문자의 수 세기
    let text = "stay foolish stay hungry";
    let mut map4 = HashMap::new();

    // text.chars(): text에서 문자를 뽑는다.
    for c in text.chars() {
        // map에 해당 문자가 없으면 해당 문자에 대한 value를 0
        // let cnt = map.entry(c).or_insert(0);을 했기에 cnt는 &val을 가리키게 된다.
        // 해당 문자의 val값인 카운트를 가리킨다. 여기에 *cnt += 1을 했기에 해당 문자에 대한 카운트가 증가
        let cnt = map4.entry(c).or_insert(0);
        *cnt += 1;
    }
    println!("{:?}", map4);

    // 단어 세어보기
    let mut map5 = HashMap::new();

    // text.split_whitespace()를 통해 공백으로 분리
    for w in text.split_whitespace() {
        let cnt = map5.entry(w).or_insert(0);
        *cnt += 1;
    }
    println!("{:?}", map5);

    hashmap_method(map5);
}

fn hashmap_method(mut map1: HashMap<&str, i32>) {
    println!("{}", map1.len());

    // clear: pub fn clear(&mut self)
    // key-value 페어를 모두 삭제한다. 할당된 메모리는 유지
    map1.clear();
    println!("{}", map1.len());

    // contains_key : pub fn contains_key<Q>(&self, k: &Q) -> bool
    // 해당 key를 가지고 있다면 true를 리턴
    map1.insert("kim", 1);
    assert_eq!(map1.contains_key("kim"), true);

    let mut map2 = HashMap::from([(1, "a")]);
    // get : pub fn get<Q>(&self, k: &Q) -> Option<&V>
    // Key 데이터에서 `k`와 일치하는 것을 찾고, 해당 Value를 `Some`에 담아서 리턴한다. 해당 Key 값이 없으면 `None`을 리턴한다.
    assert_eq!(map2.get(&1), Some(&"a"));
    assert_eq!(map2.get(&2), None);

    // insert : pub fn insert(&mut self, k: K, v: V) -> Option<V>
    // HashMap의 기존 Key에 집어 넣으려는 key가 없다면, key-value를 추가하고 `None`을 리턴한다.
    // 만약 기존 Key에 집어 넣으려는 key가 이미 존재한다면, 해당 value를 신규 값으로 갱신하고, 예전 값을 `Some`에 담아서 리턴한다.
    map2.insert(2, "b");
    assert_eq!(map2[&2], "b");

    // keys : pub fn keys(&self) -> Keys<'_, K, V>
    // 해시맵이 가지고 있는 모든 Key에 대한 반복자를 리턴
    // 순서는 정렬되어 있지도 않고 입력된 순서를 보장하지 않는다.
    for key in map2.keys() {
        print!("{key} ");
    }
    println!("");

    // remove : pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
    // 주어진 `k`에 해당하는 키를 삭제한다. 만약 해당 키가 존재하면, 키를 삭제하고 해당 키에 매칭되는 value를 `Some`타입으로 리턴하고,
    // 만약 해당 키가 존재하지 않는다면 아무 작업도 하지 않고 `None`을 리턴
    assert_eq!(map2.remove(&2), Some("b"));
    assert_eq!(map2.remove(&2), None);

    // values : pub fn values(&self) -> Values<'_, K, V>
    // 해시맵에 있는 모든 Value에 대한 반복자를 리턴한다. 반복자의 타입은 `&'a V`이다. 즉, Value의 참조자에 대한 반복자다.
    for val in map2.values() {
        println!("{val}");
    }
}
