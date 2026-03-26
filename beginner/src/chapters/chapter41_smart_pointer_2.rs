/*
핵심 기준
---------
1. 소유자 1명인가?
2. 여러 곳에서 공유해야 하는가?
3. 수정이 필요한가?
4. 수정 시점이 단일 스레드인가, 멀티스레드인가?

큰 그림
-------
- Box<T>
  : 소유자 1명, 힙에 저장
- Rc<T>
  : 단일 스레드 공유 소유권
- RefCell<T>
  : 단일 스레드 내부 가변성 (borrow(대여, 대여 -> 참조) 검사 = 런타임)
- Arc<T>
  : 멀티스레드 공유 소유권
- Mutex<T>
  : 멀티스레드에서 한 번에 하나만 수정 가능하게 잠금
- 실전 자주 조합:
  - Rc<RefCell<T>>   : 단일 스레드 공유 + 수정
  - Arc<Mutex<T>>   : 멀티스레드 공유 + 수정

꼭 이해해야 할 기준
-------------------
1. Box는 "공유"가 아니라 "단일 소유"
2. Rc/Arc는 "공유 소유"이지만, 그 자체로 내부 수정은 쉽지 않음
3. RefCell/Mutex는 "수정 가능성"을 열어줌
4. Rc는 single-thread, Arc는 atomic reference count로 multi-thread
5. RefCell은 borrow 규칙을 컴파일 타임이 아니라 런타임에 검사
6. Mutex는 lock을 통해 동시 접근을 제어

짧은 선택 가이드
----------------
- 그냥 값 하나 힙에 두고 싶다                -> Box<T>
- 단일 스레드에서 여러 곳이 같이 소유        -> Rc<T>
- 단일 스레드에서 공유하면서 수정도 필요      -> Rc<RefCell<T>>
- 멀티스레드에서 여러 곳이 같이 소유          -> Arc<T>
- 멀티스레드에서 공유하면서 수정도 필요        -> Arc<Mutex<T>>
*/

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

/*
Box<T>
------

언제 쓰나?
- 소유자가 1명일 때
- 힙(heap)에 데이터를 두고 싶을 때
- 재귀 타입(recursive type)을 만들 때

뭘 이해해야 하나?
- Box는 "힙에 저장된 값의 단일 소유자"
- *로 역참조 가능 (Deref)
- 스코프 종료 시 자동 해제 (Drop)

어떻게 쓰나?
- Box::new(value)
*/
fn box_example() {
    println!("\n[Box<T>]");

    let n3 = Node {
        value: 3,
        next: None,
    };

    let n2 = Node {
        value: 2,
        next: Some(Box::new(n3)),
    };

    let n1 = Node {
        value: 1,
        next: Some(Box::new(n2)),
    };

    println!("linked list head = {:?}", n1);

    /*
    왜 Box가 필요하나?
    - Node 안에 Node를 직접 넣으면 크기가 무한히 커지는 문제가 생김
    - Box<Node>는 포인터 크기로 고정되므로 재귀 타입 가능
    */
}

/*
Rc<T>
-----

언제 쓰나?
- 단일 스레드에서 여러 곳이 같은 데이터를 "소유"해야 할 때

뭘 이해해야 하나?
- Rc = reference counting
- clone()은 깊은 복사가 아니라 참조 카운트 증가
- immutable 공유에 적합
- 멀티스레드에서는 사용 불가 (Send/Sync 아님)

어떻게 쓰나?
- Rc::new(value)
- Rc::clone(&x) 또는 x.clone()
*/
fn rc_example() {
    println!("\n[Rc<T>]");

    let shared = Rc::new(String::from("shared text"));

    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);

    println!("shared = {}", shared);
    println!("a = {}", a);
    println!("b = {}", b);

    println!("strong_count = {}", Rc::strong_count(&shared));

    /*
    핵심
    - a, b, shared가 모두 같은 String을 "공유 소유"
    - 실제 데이터는 하나
    - clone은 데이터 복사가 아니라 소유권 공유 증가
    */
}

/*
RefCell<T>
----------

언제 쓰나?
- 단일 스레드에서
- 겉으로는 불변 구조인데 내부 값을 수정해야 할 때
- 컴파일 타임 borrow 검사로 표현이 어려운 경우

뭘 이해해야 하나?
- interior mutability (내부 가변성)
- borrow()/borrow_mut()로 접근
- borrow 규칙 위반 시 런타임 panic 가능

어떻게 쓰나?
- RefCell::new(value)
- *cell.borrow_mut() = ...
*/
fn refcell_example() {
    println!("\n[RefCell<T>]");

    let cell = RefCell::new(10);

    println!("before = {}", cell.borrow());

    *cell.borrow_mut() += 5;

    println!("after = {}", cell.borrow());

    /*
    핵심
    - 변수 자체는 mut가 아니어도 내부 값 수정 가능
    - 대신 borrow 규칙을 런타임에 검사
    */

    /*
    아래 코드는 panic 예시라서 주석으로만 둠

    let m1 = cell.borrow_mut();
    let m2 = cell.borrow_mut(); // 런타임 panic
    println!("{:?} {:?}", m1, m2);
    */
}

/*
Rc<RefCell<T>>
--------------

언제 쓰나?
- 단일 스레드에서
- 여러 곳이 공유하면서
- 내부 값을 수정해야 할 때

뭘 이해해야 하나?
- Rc = 공유 소유권
- RefCell = 내부 가변성
- 둘을 합치면 "공유 + 수정" 가능

어떻게 쓰나?
- Rc<RefCell<T>>
*/
fn rc_refcell_example() {
    println!("\n[Rc<RefCell<T>>]");

    let shared_num = Rc::new(RefCell::new(100));

    let a = Rc::clone(&shared_num);
    let b = Rc::clone(&shared_num);

    *a.borrow_mut() += 1;
    *b.borrow_mut() += 10;

    println!("result = {}", shared_num.borrow());
    println!("strong_count = {}", Rc::strong_count(&shared_num));

    /*
    핵심
    - 같은 값 하나를 여러 곳이 들고 있음
    - 각자 borrow_mut()로 수정 가능
    - UI state tree, graph 구조, 단일 스레드 shared mutable state에서 자주 등장
    */
}

/*
Arc<T>
------

언제 쓰나?
- 멀티스레드에서 여러 곳이 같은 데이터를 소유해야 할 때

뭘 이해해야 하나?
- Arc = atomically reference counted
- Rc와 목적은 비슷하지만 스레드 안전
- 그 자체로는 내부 수정 문제를 해결하지 않음

어떻게 쓰나?
- Arc::new(value)
- Arc::clone(&x)
*/
fn arc_example() {
    println!("\n[Arc<T>]");

    let shared = Arc::new(String::from("hello from arc"));

    let mut handles = vec![];

    for i in 0..3 {
        let s = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            println!("thread {i}: {s}");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("all threads done");

    /*
    핵심
    - 여러 스레드가 같은 데이터를 읽기 공유 가능
    - immutable shared data에 적합
    */
}

/*
Mutex<T>
--------

언제 쓰나?
- 멀티스레드에서 공유 데이터 수정이 필요할 때

뭘 이해해야 하나?
- 한 번에 하나의 스레드만 lock 획득 가능
- lock()은 LockResult<MutexGuard<T>> 반환
- guard가 살아있는 동안 lock 유지
- guard drop 시 자동 unlock

어떻게 쓰나?
- Mutex::new(value)
- let mut guard = mutex.lock().unwrap();
*/
fn mutex_example() {
    println!("\n[Mutex<T>]");

    let m = Mutex::new(0);

    {
        let mut guard = m.lock().unwrap();
        *guard += 1;
        println!("inside lock = {}", *guard);
    } // 여기서 자동 unlock

    println!("after lock = {}", *m.lock().unwrap());

    /*
    핵심
    - 동시에 여러 스레드가 수정할 때 데이터 경쟁(data race)을 막음
    - 락 범위를 짧게 유지하는 습관 중요
    */
}

/*
Arc<Mutex<T>>
-------------

언제 쓰나?
- 멀티스레드에서
- 여러 스레드가 같은 값을 공유하면서
- 수정도 해야 할 때

뭘 이해해야 하나?
- Arc = 공유 소유권
- Mutex = 동시 수정 제어
- Rust 멀티스레드 shared mutable state의 대표 패턴

어떻게 쓰나?
- Arc<Mutex<T>>
*/
fn arc_mutex_example() {
    println!("\n[Arc<Mutex<T>>]");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_cloned = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut guard = counter_cloned.lock().unwrap();
            *guard += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("final counter = {}", *counter.lock().unwrap());

    /*
    핵심
    - 10개 스레드가 같은 counter를 공유
    - lock으로 안전하게 수정
    */
}

/*
실전 선택 요약 함수
------------------
*/
fn decision_guide() {
    println!("\n[선택 가이드]");
    println!("1) 소유자 1명, 힙 저장 필요            -> Box<T>");
    println!("2) 단일 스레드 공유 읽기              -> Rc<T>");
    println!("3) 단일 스레드 공유 + 수정            -> Rc<RefCell<T>>");
    println!("4) 멀티스레드 공유 읽기               -> Arc<T>");
    println!("5) 멀티스레드 공유 + 수정             -> Arc<Mutex<T>>");
}

/*
추가로 꼭 알아둘 오해
--------------------
1. Box는 "주소를 직접 다루기 위한 도구"가 아님
   -> 주된 목적은 힙 할당과 소유권 모델링

2. Rc/Arc의 clone은 깊은 복사가 아님
   -> 참조 카운트 증가

3. RefCell은 편하지만 남용하면 런타임 panic 위험
   -> 가능하면 먼저 일반 borrow 규칙으로 풀어보기

4. Mutex는 lock 비용과 교착상태(deadlock) 위험이 있음
   -> lock 범위를 짧게, 중첩 lock 조심

5. 실무 핵심은 포인터 문법보다 "소유권 구조 설계"
   -> 누가 소유하고, 누가 공유하고, 누가 수정하는지 먼저 결정
*/

pub fn run() {
    box_example();
    rc_example();
    refcell_example();
    rc_refcell_example();
    arc_example();
    mutex_example();
    arc_mutex_example();
    decision_guide();
}
