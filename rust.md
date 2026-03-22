# Rust 학습 목차 (실무 연결형)

## 0. Rust는 어떤 언어인가 (오리엔테이션)

**목표**: 왜 Rust가 이런 문법을 가졌는지 이해

- Rust의 설계 철학
  - GC 없음, 메모리 안전
  - 컴파일 타임 검증

- Rust가 잘하는 것 / 안 하는 것
- Rust 컴파일 모델 개요
  - `rustc`
  - `cargo`

---

## 1. 개발 환경 & 기본 도구

**목표**: Rust 프로젝트를 혼자 만들 수 있는 상태

- `rustup`, `cargo` 개념
- 프로젝트 생성

  ```bash
  cargo new hello_rust
  cargo run
  ```

- `Cargo.toml` 구조
- 크레이트(crate)와 모듈(module) 개념

---

## 2. 기본 문법 핵심

**목표**: “Rust처럼” 코드를 쓰기 시작

### 2.1 변수와 불변성

- `let`, `let mut`
- 불변이 기본인 이유
- shadowing

### 2.2 기본 타입

- 정수/실수
- `bool`, `char`
- 튜플, 배열

### 2.3 함수

- 함수 선언
- 반환값과 표현식
- 세미콜론의 의미

---

## 3. 제어 흐름

**목표**: 분기/반복을 Rust스럽게 쓰기

- `if`는 표현식
- `match` (Rust의 핵심)
- `loop`, `while`, `for`
- `break`에 값 반환

---

## 4. 소유권(Ownership) — Rust의 핵심

**목표**: Rust가 왜 안전한지 “이해”

> ⚠️ 이 챕터가 Rust 난이도의 70%

### 4.1 소유권 규칙

- 값의 주인은 하나
- 스코프 종료 시 drop

### 4.2 Move vs Copy

- `Copy` 트레잇
- String vs &str 차이

### 4.3 함수와 소유권 이동

---

## 5. 참조(Borrowing)

**목표**: 복사 없이 안전하게 데이터 다루기

- `&T`, `&mut T`
- 참조 규칙 (불변 여러 개 / 가변 하나)
- dangling reference가 왜 불가능한지

---

## 6. 구조체와 열거형

**목표**: 데이터 모델링 감각 만들기

### 6.1 `struct`

- 필드
- impl 블록
- 메서드

### 6.2 `enum`

- 상태 표현
- Rust에서 enum이 중요한 이유

---

## 7. Option과 Result (에러 처리)

**목표**: 예외 없는 언어에 익숙해지기

### 7.1 `Option<T>`

- `Some`, `None`
- null이 없는 이유

### 7.2 `Result<T, E>`

- 성공/실패 모델링
- `?` 연산자
- 에러 전파 패턴

> (약어) Result = 결과 타입, Option = 값 존재 여부 표현 타입

---

## 8. 컬렉션

**목표**: 실무 데이터 처리 준비

- `Vec<T>`
- `HashMap<K, V>`
- 반복자(`iter`, `iter_mut`, `into_iter`)
- 소유권과 컬렉션의 관계

---

## 9. 모듈 시스템

**목표**: 프로젝트 구조를 스스로 설계

- `mod`
- `pub`
- 파일/폴더 구조
- 라이브러리 크레이트 vs 바이너리 크레이트

---

## 10. 트레잇(Trait)

**목표**: Rust의 추상화 이해

- 트레잇 개념
- 트레잇 구현
- 제네릭과 트레잇 바운드
- Rust식 다형성

---

## 11. 라이프타임(Lifetime)

**목표**: 컴파일러의 사고를 이해

- 라이프타임이 필요한 이유
- 명시 vs 추론
- 언제 직접 써야 하는가

👉 이 챕터를 이해하면 Rust가 “두렵지 않음”

---

## 12. 표준 라이브러리 활용

**목표**: 실무 코드로 연결

- 문자열 처리
- 파일 I/O
- 경로(`Path`, `PathBuf`)
- 환경 변수

---

## 13. 외부 크레이트 사용

**목표**: 생태계 활용

- `serde`, `serde_json`
- `clap`
- `anyhow`, `thiserror`
- 문서 읽는 법(`docs.rs`)

---

## 14. 비동기 Rust (기초)

**목표**: 서버/워크플로우 준비

- 동기 vs 비동기
- `async` / `await`
- `tokio` 개념
- 간단한 async 함수

---

## 15. 테스트 & 디버깅

**목표**: 신뢰 가능한 코드

- 단위 테스트
- `#[test]`
- 에러 재현
- 로그(`tracing`)

---

## 16. 다음 단계 미리보기 (선택)

**목표**: 어디로 확장할지 감 잡기

- CLI 자동화
- 파이프라인 워커
- axum API
- WASM
- Tauri 백엔드

---

# 추천 학습 순서 (요약)

**필수 코어**

1. 2 ~ 7 (문법 + 소유권 + 에러)
2. 8 ~ 10 (컬렉션 + 구조화)
3. 11 (라이프타임)

**실무 연결** 4. 12 ~ 14 5. 작은 프로젝트 1개

---

---

---

---

그렇다. C를 안다면 Rust를 배울 때 이미 큰 무기를 하나 가지고 있는 셈이다.
메모리, 포인터, 스택/힙, 구조체, 함수 호출, 컴파일, 링크 같은 감각이 이미 있기 때문이다.

하지만 Rust는 **“C와 비슷하게 생겼지만, 사고방식이 꽤 다른 언어”**다.
그래서 C를 안다고 해서 바로 Rust 코드를 읽히는 것은 아니고, 특히 아래 영역은 Rust에서 따로 깊게 공부해야 한다.

핵심만 먼저 말하면 Rust에서 특수하게 공부해야 하는 건 이거다.

1. 소유권(ownership)
2. 빌림(borrowing)과 참조(reference)
3. 수명(lifetime)
4. 패턴 매칭과 대수적 자료형(enum 중심 사고)
5. trait 기반 추상화
6. Result / Option 중심 오류 처리
7. 모듈 / crate / package 구조
8. iterator, closure, 함수형 스타일
9. 스마트 포인터와 내부 가변성
10. unsafe Rust와 안전한 Rust의 경계
11. async / concurrency
12. 매크로
13. 표준 라이브러리와 대표 관용구(idioms)

네가 원하는 건 “대충 배우는 로드맵”이 아니라
**“Rust 코드를 하나도 빠짐없이 읽어낼 수 있을 정도의 세세한 공부 목록”**이니까,
그 기준으로 **읽기 능력 중심**의 정밀 커리큘럼으로 짜줄게.

다만 먼저 한 가지는 분명히 할게.
“세상 모든 Rust 코드”를 하나도 빠짐없이 읽는 건 현실적으로 끝이 없다.
왜냐하면 Rust는 언어 자체뿐 아니라 다음도 같이 읽어야 하기 때문이다.

- 표준 라이브러리
- 비동기 생태계(tokio, futures)
- 웹 생태계(axum, actix-web)
- 직렬화(serde)
- 매크로 생태계
- unsafe / FFI / 임베디드 / 운영체제 코드

그래서 목표를 이렇게 잡는 게 현실적이다.

- **1단계:** 일반 Rust 코드 80~90% 읽기
- **2단계:** 실무 crate 코드 읽기
- **3단계:** 고급 generic / macro / async / unsafe 코드 읽기
- **4단계:** std / compiler 주변 스타일 일부 읽기

이제 정말 세세하게 가보자.

---

# 0. 전체 목표

최종 목표는 단순히 문법 암기가 아니다.
아래 4가지를 할 수 있어야 한다.

1. 코드를 보고 **소유권 이동과 빌림 흐름**을 머릿속으로 추적할 수 있다.
2. 타입 시그니처를 보고 **데이터 구조와 제약 조건**을 해석할 수 있다.
3. trait bound와 generic을 보고 **어떤 종류의 값이 허용되는지** 읽을 수 있다.
4. async, macro, unsafe가 섞여도 **대략 무엇이 일어나는지** 추론할 수 있다.

---

# 1. C를 아는 사람이 Rust에서 가장 먼저 교정해야 하는 사고방식

C 감각이 도움 되는 부분도 많지만, 몇 가지는 반드시 교정해야 한다.

## 1-1. “포인터를 잘 쓰면 된다”가 아니라 “소유권을 먼저 본다”

C에서는 보통 이렇게 본다.

- 이 값은 어디 메모리에 있나
- 포인터가 어디를 가리키나
- 누가 free 하나

Rust에서는 먼저 이렇게 본다.

- 이 값의 **owner**가 누구인가
- 여기서 **move**가 일어났나
- 지금은 **borrow** 중인가
- mutable borrow와 immutable borrow가 충돌하나
- lifetime이 유효한가

즉, Rust는 포인터 감각보다 먼저 **소유권 흐름 추적 능력**이 필요하다.

## 1-2. enum이 C enum이 아니다

C의 enum은 거의 정수 상수 집합에 가깝다.
Rust의 enum은 **데이터를 품을 수 있는 합 타입(sum type)** 이다.

예:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
```

이건 C의 enum과 완전히 다르다.
Rust 코드 읽기에서 enum과 match는 매우 중요하다.

## 1-3. NULL 대신 Option

C에서는 NULL 체크가 흔하다.
Rust는 `null`을 일반 값으로 거의 쓰지 않고, 대신 `Option<T>`를 쓴다.

```rust
let name: Option<String> = Some(String::from("Heeseong"));
```

즉 Rust 코드는 “포인터가 null인지”보다 “값이 있을 수도 없을 수도 있는 상태를 타입으로 표현했는지”를 읽어야 한다.

## 1-4. 오류 코드를 반환하는 대신 Result

C에서는 보통 정수 에러 코드나 errno 계열을 많이 본다.
Rust는 오류를 `Result<T, E>`로 표현한다.

```rust
fn read_file() -> Result<String, std::io::Error> {
    // ...
}
```

그래서 Rust 코드를 읽을 때는 **정상 흐름 + 실패 흐름이 타입에 드러난다**는 점을 익혀야 한다.

---

# 2. 가장 먼저 공부해야 할 “Rust 특수 핵심” 1순위

이건 정말 제일 중요하다.

## 2-1. Ownership

반드시 알아야 하는 세부 항목:

- stack value와 heap value
- `String` 과 `&str` 차이
- move semantics
- copy semantics
- `Copy` 와 `Clone`
- 함수 인자로 넘길 때 move 되는지
- 대입 시 move 되는지
- return 시 ownership 이전

예:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // move
    // println!("{}", s1); // 사용 불가
    println!("{}", s2);
}
```

C 관점에서는 “포인터 복사”처럼 보일 수 있지만 Rust에서는 `s1`의 소유권이 `s2`로 이동했다고 봐야 한다.

공부 목표:

- 모든 변수 대입에서 move/copy를 구분할 수 있어야 함
- 함수 호출 시 인자 전달이 move인지 borrow인지 바로 읽을 수 있어야 함

## 2-2. Borrowing

세부 항목:

- shared borrow: `&T`
- mutable borrow: `&mut T`
- 같은 시점에 여러 `&T` 가능
- 같은 시점에 하나의 `&mut T`만 가능
- `&T`와 `&mut T` 동시 공존 제약
- non-lexical lifetimes(NLL) 감각

예:

```rust
fn print_len(s: &String) {
    println!("{}", s.len());
}

fn append_world(s: &mut String) {
    s.push_str(" world");
}
```

공부 목표:

- 함수 시그니처만 보고 읽기 전용인지 수정 가능한지 파악
- borrow checker 에러가 왜 나는지 설명 가능

## 2-3. Lifetimes

초반에는 무섭게 보이지만, 코드를 읽으려면 꼭 필요하다.

세부 항목:

- reference는 referent보다 오래 살 수 없음
- dangling reference가 왜 금지되는지
- 함수에서 lifetime elision 규칙
- 명시적 lifetime parameter: `'a`
- 구조체 안에 reference를 넣을 때 lifetime
- `'static`
- lifetime은 “얼마나 오래 메모리가 살아있다”보다 “참조 관계 제약을 타입으로 표현한다”는 쪽으로 이해

예:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

공부 목표:

- lifetime annotation이 왜 필요한지 읽을 수 있어야 함
- struct에 reference가 들어가면 왜 lifetime이 붙는지 이해해야 함

---

# 3. 문법이 아니라 “Rust식 데이터 표현”을 공부해야 한다

## 3-1. struct

세부 항목:

- named field struct
- tuple struct
- unit-like struct
- method와 associated function
- `impl`

```rust
struct User {
    name: String,
    age: u32,
}

impl User {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    fn greet(&self) {
        println!("Hello, {}", self.name);
    }
}
```

## 3-2. enum

정말 중요하다.

세부 항목:

- enum variant가 데이터를 가질 수 있음
- `Option<T>`
- `Result<T, E>`
- 직접 enum 설계 읽기
- state machine 코드 읽기

예:

```rust
enum State {
    Idle,
    Running { progress: u32 },
    Failed(String),
}
```

## 3-3. pattern matching

Rust 읽기의 핵심 중 핵심이다.

세부 항목:

- `match`
- `if let`
- `while let`
- destructuring
- match guard
- `_`, `..`, `ref`, `ref mut`
- nested pattern

```rust
match state {
    State::Idle => println!("idle"),
    State::Running { progress } if progress < 100 => println!("running"),
    State::Running { progress: 100 } => println!("done"),
    State::Failed(msg) => println!("error: {}", msg),
}
```

공부 목표:

- enum + match 조합을 보고 제어 흐름을 빠르게 읽기
- destructuring 패턴을 보고 값 꺼내는 방식을 이해하기

---

# 4. Rust 타입 시스템을 읽는 법

C보다 훨씬 중요하다.

## 4-1. 기본 타입과 참조 타입

- 정수형: `i32`, `u64`, `usize`
- 부동소수점: `f32`, `f64`
- bool, char
- slice: `&[T]`
- string slice: `&str`
- array: `[T; N]`
- tuple: `(T, U)`
- raw pointer: `*const T`, `*mut T`

특히 꼭 공부:

- `String` vs `&str`
- `Vec<T>` vs `&[T]`
- 소유한 데이터 vs 빌린 데이터

## 4-2. 제네릭(generics)

세부 항목:

- `Vec<T>`
- `Option<T>`
- `Result<T, E>`
- generic function
- generic struct / enum
- monomorphization
- turbofish: `::<T>`

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

## 4-3. trait

Rust에서 interface 비슷한 개념이지만 더 중요하다.

세부 항목:

- trait 정의
- trait 구현
- trait bound
- default method
- supertrait
- blanket implementation
- auto trait (`Send`, `Sync`)
- marker trait 개념

```rust
trait Summary {
    fn summarize(&self) -> String;
}
```

## 4-4. where clause

복잡한 generic 코드 읽기에 필수다.

```rust
fn process<T, U>(x: T, y: U)
where
    T: Clone + std::fmt::Debug,
    U: Into<String>,
{
    // ...
}
```

## 4-5. associated types

Iterator 읽을 때 매우 중요하다.

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

## 4-6. trait object

동적 디스패치 이해에 필요하다.

- `dyn Trait`
- `&dyn Trait`
- `Box<dyn Trait>`
- vtable 개념
- 정적 디스패치 vs 동적 디스패치

---

# 5. 반드시 알아야 할 표준 타입들

이걸 모르면 실무 코드 읽기가 안 된다.

## 5-1. Option<T>

반드시 익힐 메서드:

- `is_some`
- `is_none`
- `unwrap`
- `expect`
- `map`
- `and_then`
- `unwrap_or`
- `as_ref`
- `as_mut`
- `take`

## 5-2. Result<T, E>

반드시 익힐 것:

- `Ok`, `Err`
- `?` 연산자
- `map`
- `map_err`
- `and_then`
- `unwrap`, `expect`의 위험성
- 사용자 정의 에러 타입

## 5-3. String / str

세부 항목:

- `String`은 소유형, 가변 버퍼
- `&str`은 문자열 슬라이스
- UTF-8
- 인덱싱이 금지되는 이유
- bytes / chars / grapheme 구분

이건 C 사용자에게 꽤 충격일 수 있다.
C의 char 배열 감각으로 보면 자꾸 헷갈린다.

## 5-4. Vec<T>

세부 항목:

- push/pop
- indexing vs get
- iter/iter_mut/into_iter
- capacity
- ownership과 iteration 관계

## 5-5. HashMap<K, V>

- entry API
- ownership 이슈
- borrow 문제 자주 발생

## 5-6. Box, Rc, Arc, RefCell, Mutex

이건 뒤에서 다시 크게 다룰 거지만 표준 타입으로 꼭 익혀야 한다.

---

# 6. 함수, 메서드, 클로저 읽기

## 6-1. 함수 시그니처 해석

Rust 코드 읽기는 함수 몸체보다 먼저 시그니처를 읽는다.

예:

```rust
fn find_user<'a>(users: &'a [User], id: u64) -> Option<&'a User>
```

이걸 보고 바로 읽어야 한다.

- `users`는 User slice를 빌려받는다
- id는 값 복사로 받는다
- 결과는 `Option<&User>`
- 반환 참조의 lifetime은 users와 연결됨
- 즉 반환된 참조는 users보다 오래 살 수 없음

이런 해석이 자동으로 되어야 한다.

## 6-2. 클로저(closure)

세부 항목:

- `|x| x + 1`
- 환경 캡처
- immutable borrow capture
- mutable borrow capture
- move closure
- `Fn`, `FnMut`, `FnOnce`

```rust
let s = String::from("hello");
let f = move || println!("{}", s);
```

클로저는 특히 iterator, async, thread 코드에서 계속 나온다.

---

# 7. Iterator는 반드시 따로 깊게 공부해야 한다

이건 Rust의 핵심 관용구다.

C 배경이면 loop만 봐도 된다고 생각하기 쉬운데, Rust 실무 코드는 iterator 체인이 많다.

## 7-1. iterator 기본

- `iter()`
- `iter_mut()`
- `into_iter()`
- 차이점
- borrow vs move 차이

## 7-2. 자주 나오는 adapter / consumer

반드시 익힐 것:

- `map`
- `filter`
- `filter_map`
- `find`
- `any`
- `all`
- `fold`
- `collect`
- `enumerate`
- `zip`
- `take`
- `skip`
- `chain`
- `flat_map`

예:

```rust
let names: Vec<String> = users
    .iter()
    .filter(|u| u.active)
    .map(|u| u.name.clone())
    .collect();
```

이걸 바로 읽을 수 있어야 한다.

- users를 빌려 순회
- active인 것만 필터
- name 복제
- Vec<String>으로 수집

## 7-3. iterator와 ownership

가장 많이 막히는 포인트다.

- 왜 `into_iter()` 후 원본을 못 쓰는지
- 왜 `iter()`면 `&T`가 오는지
- 왜 `cloned()`가 필요한지

---

# 8. 스마트 포인터와 내부 가변성

Rust 중급 이상 코드를 읽으려면 반드시 필요하다.

## 8-1. Box<T>

- heap allocation
- recursive type
- trait object 보관

## 8-2. Rc<T>

- single-thread reference counting
- shared ownership
- 불변 공유

## 8-3. Arc<T>

- multi-thread reference counting
- thread-safe shared ownership

## 8-4. RefCell<T>

- runtime borrow checking
- interior mutability(내부 가변성)
- compile-time 대신 runtime에 검사

## 8-5. Cell<T>

- Copy 타입 내부 변경에 적합

## 8-6. Rc<RefCell<T>> 조합

GUI, 트리, 그래프 같은 구조에서 자주 나온다.
읽을 수 있어야 한다.

## 8-7. Arc<Mutex<T>> 조합

멀티스레드 공유 상태에서 자주 나온다.

공부 목표:

- 왜 일반 `&mut T`로 안 되는 상황에서 interior mutability가 필요한지 이해
- `Rc<RefCell<T>>` 와 `Arc<Mutex<T>>`의 사용 이유 구분

---

# 9. 모듈 시스템과 프로젝트 구조

Rust 코드를 읽으려면 파일 구조도 읽어야 한다.

## 9-1. package / crate / module 차이

반드시 정확히 구분:

- package: Cargo 단위
- crate: 컴파일 단위
- module: 코드 구성 단위

## 9-2. `mod`, `pub`, `use`

- 모듈 선언
- 공개 범위
- 경로 지정
- `crate::`, `super::`, `self::`

## 9-3. lib crate와 binary crate

- `src/lib.rs`
- `src/main.rs`

## 9-4. workspace

실무 프로젝트에서 자주 본다.

---

# 10. 오류 처리 전체를 공부해야 한다

C에서는 에러 처리 전략이 비교적 단순한 경우가 많지만, Rust는 코드 전반에 퍼져 있다.

## 10-1. panic vs recoverable error

- `panic!`
- `Result`
- 언제 panic이 적절한지

## 10-2. `?` 연산자

아주 중요하다.

```rust
fn read_name() -> Result<String, std::io::Error> {
    let text = std::fs::read_to_string("name.txt")?;
    Ok(text)
}
```

이걸 보고 “에러면 즉시 반환, 성공이면 안쪽 값을 꺼냄”을 바로 읽어야 한다.

## 10-3. 사용자 정의 에러

- enum 기반 에러
- `thiserror` 자주 사용
- `anyhow`는 앱 레벨에서 자주 사용

## 10-4. 에러 전파와 변환

- `From`
- `map_err`
- `ok_or`
- `ok_or_else`

---

# 11. Rust에서 자주 쓰는 trait들을 반드시 정리해서 외워야 한다

이건 읽기 속도를 크게 올려준다.

## 필수 trait 목록

- `Debug`
- `Display`
- `Clone`
- `Copy`
- `Default`
- `PartialEq`
- `Eq`
- `PartialOrd`
- `Ord`
- `Hash`
- `From`
- `Into`
- `AsRef`
- `Deref`
- `Drop`
- `Iterator`
- `IntoIterator`
- `FromIterator`
- `Send`
- `Sync`

각 trait에 대해 최소한 이 4가지는 알아야 한다.

1. 역할
2. 언제 자동 derive 가능한지
3. 코드에서 보이면 어떤 능력을 기대해야 하는지
4. generic bound에서 왜 붙는지

예:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
struct User {
    name: String,
}
```

---

# 12. derive와 attribute 문법

Rust 코드를 읽다가 초반에 많이 막히는 부분이다.

## 12-1. derive

- `#[derive(Debug)]`
- `#[derive(Clone)]`
- `#[derive(Serialize, Deserialize)]`

## 12-2. attribute

- `#[allow(dead_code)]`
- `#[cfg(test)]`
- `#[cfg(feature = "x")]`
- `#[inline]`
- `#[repr(C)]`

특히 `serde`, `tokio`, `clap` 같은 crate는 attribute 사용이 많다.

---

# 13. unsafe Rust는 별도 과목처럼 공부해야 한다

“Rust를 읽는다”의 최종 보스 중 하나다.

## 13-1. unsafe가 필요한 이유

Rust는 안전한 언어이지만, 시스템 프로그래밍에서는 unsafe가 필요할 수 있다.

## 13-2. unsafe로 할 수 있는 것

- raw pointer dereference
- unsafe function 호출
- mutable static 접근
- unsafe trait 구현
- union field 접근

## 13-3. raw pointer

- `*const T`
- `*mut T`
- reference와 차이
- null 가능
- dangling 가능
- aliasing 문제

## 13-4. FFI

- `extern "C"`
- C 라이브러리 연동
- `#[repr(C)]`
- ABI

## 13-5. unsafe code 읽을 때 봐야 하는 것

- 안전 불변식(invariant)
- 누가 그 불변식을 보장하는가
- 이 포인터가 유효한가
- aliasing이 없는가
- lifetime 가정이 맞는가

이 부분은 C를 알면 도움이 크다.
하지만 Rust에서는 “unsafe 블록 내부가 왜 외부에서 안전하게 감싸졌는가”까지 읽어야 한다.

---

# 14. concurrency와 parallelism

Rust는 여기서도 특성이 강하다.

## 14-1. thread

- `std::thread::spawn`
- move closure
- join

## 14-2. message passing

- `std::sync::mpsc`
- 채널 기반 통신

## 14-3. shared state

- `Mutex<T>`
- `RwLock<T>`
- `Arc<T>`

## 14-4. Send / Sync

이건 정말 중요하다.

- `Send`: 다른 스레드로 소유권 이동 가능
- `Sync`: 여러 스레드가 참조 가능

공부 목표:

- 왜 `Rc<T>`는 멀티스레드에 안 되고 `Arc<T>`는 되는지 설명 가능
- 왜 `RefCell<T>`는 스레드 안전이 아닌지 설명 가능

---

# 15. async/await는 별도 트랙으로 공부해야 한다

실무 Rust 코드 읽기에서 거의 필수다.

## 15-1. async fn

```rust
async fn fetch_data() -> Result<String, Error> {
    Ok("done".to_string())
}
```

## 15-2. Future 개념

- async fn은 즉시 실행 결과를 주는 게 아니라 Future를 반환
- poll 기반
- lazy execution 감각

## 15-3. `.await`

- await 지점에서 일시 중단 가능
- state machine으로 변환된다고 이해

## 15-4. runtime

- tokio
- async-std
- executor 개념

## 15-5. pin / Unpin

고급 async 코드 읽기에서 중요하다.

## 15-6. select, join, spawn

- `tokio::spawn`
- `join!`
- `select!`

초반에는 깊게 구현 원리까지 안 가도 되지만, **실무 코드를 읽으려면 최소한 Future / executor / await / spawn은 알아야 한다**.

---

# 16. macro는 반드시 공부해야 한다

Rust 코드를 막는 큰 원인 중 하나다.

## 16-1. declarative macro (`macro_rules!`)

예:

```rust
macro_rules! say_hello {
    () => {
        println!("hello");
    };
}
```

## 16-2. procedural macro

- derive macro
- attribute macro
- function-like macro

실무에서 많이 보는 예:

- `#[derive(Serialize)]`
- `#[tokio::main]`
- `#[async_trait]`
- `clap` derive

## 16-3. 매크로 읽기 전략

처음부터 다 구현할 필요는 없다.
하지만 최소한:

- 이게 함수가 아니라 매크로인지
- 컴파일 타임에 코드 생성하는지
- 어떤 문법을 숨기고 있는지

정도는 읽을 수 있어야 한다.

---

# 17. 테스트 코드도 읽을 줄 알아야 한다

오픈소스 읽을 때 매우 중요하다.

## 17-1. 단위 테스트

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(2 + 2, 4);
    }
}
```

## 17-2. 자주 보는 assertion

- `assert!`
- `assert_eq!`
- `assert_ne!`
- `matches!`

테스트는 오히려 코드 의도를 파악하기 쉬운 부분이라 적극적으로 읽는 습관이 좋다.

---

# 18. 꼭 공부해야 할 표준 라이브러리 영역

전부 구현까지 파지 않아도 되지만, 최소한 이름과 역할은 알아야 한다.

## std에서 필수 범위

- `std::option`
- `std::result`
- `std::vec`
- `std::string`
- `std::collections`
- `std::fs`
- `std::io`
- `std::path`
- `std::sync`
- `std::thread`
- `std::time`
- `std::iter`
- `std::cmp`
- `std::hash`

---

# 19. 코드를 “읽기 위해” 반드시 알아야 하는 유명 crate

언어 자체만 알아서는 부족하다.
실무 Rust 코드를 많이 읽으려면 다음 crate 스타일은 알아야 한다.

## 19-1. serde

- `Serialize`, `Deserialize`
- attribute
- JSON 변환

## 19-2. tokio

- async runtime
- spawn
- TCP/IO
- time/sleep

## 19-3. anyhow / thiserror

- 에러 처리 관용구

## 19-4. clap

- CLI parsing
- derive 기반 설정

## 19-5. tracing / log

- 로깅 코드 읽기

## 19-6. axum / hyper / tower

네가 웹/서버도 관심 있으니까 중요하다.

- handler가 왜 그런 시그니처인지
- extractor가 뭔지
- middleware가 trait 기반으로 어떻게 엮이는지

---

# 20. “Rust 코드를 다 읽을 수 있게 해주는” 공부 순서

이제 실제 순서를 줄게.

---

# 1단계. 기초 문법이 아니라 소유권 모델 정복

이 단계의 목표는 “컴파일 에러를 읽고 이유를 설명할 수 있는 것”이다.

공부 목록:

1. 변수, mut, shadowing
2. 기본 타입
3. 함수
4. if / loop / while / for
5. ownership
6. borrowing
7. references
8. slices
9. String vs &str
10. struct
11. enum
12. match
13. Option / Result 기초

이 단계에서 반드시 직접 써봐야 할 예제:

- 문자열을 함수에 넘기고 다시 반환하기
- `&String` 과 `&str` 비교
- mutable borrow 충돌 예제
- enum 상태 분기 예제
- `Option<String>` 처리 예제

---

# 2단계. 타입과 추상화 읽기

목표: 함수 시그니처와 타입 선언만 보고 대략 동작을 추론

공부 목록:

1. impl block
2. method syntax
3. generic
4. trait
5. trait bound
6. where
7. associated types
8. derive
9. modules
10. crate 구조

반드시 연습할 것:

- 제네릭 함수 작성
- trait 구현
- `impl<T>` 읽기
- `where T: ...` 해석
- `pub`, `mod`, `use`가 섞인 프로젝트 읽기

---

# 3단계. 관용구(idiom) 읽기

목표: 실무 스타일 Rust를 무리 없이 읽기

공부 목록:

1. iterator
2. closure
3. collect
4. map/filter/fold
5. error propagation with `?`
6. pattern matching 심화
7. if let / while let
8. HashMap entry API
9. Result combinator
10. Option combinator

연습 추천:

- 반복문 코드를 iterator 체인으로 바꾸기
- 파일 읽기 + 파싱 + 에러 전파 코드 작성
- HashMap 카운팅 예제 작성

---

# 4단계. 중급 소유권과 스마트 포인터

목표: 자료구조/GUI/공유상태 코드 읽기

공부 목록:

1. Box
2. Deref
3. Drop
4. Rc
5. Arc
6. RefCell
7. Cell
8. Mutex
9. RwLock
10. interior mutability 패턴

연습 추천:

- 트리 구조 만들기
- 공유 카운터 만들기
- `Rc<RefCell<T>>` 예제 읽기
- `Arc<Mutex<T>>` 멀티스레드 예제 읽기

---

# 5단계. 실무 비동기 읽기

목표: tokio/axum 류 코드 읽기

공부 목록:

1. async fn
2. Future 개념
3. await
4. tokio runtime
5. spawn
6. channel
7. select/join
8. Send/Sync
9. Pin 기초
10. stream 기초

연습 추천:

- async 파일/네트워크 예제
- 간단한 tokio TCP 서버
- axum hello world 읽기
- handler extractor 구조 파악

---

# 6단계. unsafe / FFI / 고급 읽기

목표: 저수준 crate도 겁먹지 않고 접근

공부 목록:

1. raw pointer
2. unsafe block
3. unsafe fn
4. extern "C"
5. repr(C)
6. aliasing / UB 감각
7. MaybeUninit
8. NonNull
9. slice / str 저수준 생성 원리
10. unsafe abstraction pattern

연습 추천:

- C 라이브러리 하나 바인딩 읽기
- 안전한 wrapper가 unsafe를 감싸는 구조 분석
- Vec/String 내부 개념 자료 읽기

UB는 Undefined Behavior의 약자다.

---

# 21. 정말 세세한 “체크리스트형 공부 목록”

아래는 빠짐없이 체크하라고 만든 목록이다.

## A. 기본 문법

- 변수 선언
- mut
- shadowing
- 상수
- 스칼라 타입
- 튜플
- 배열
- 제어문
- 함수와 반환식
- statement vs expression

## B. 소유권

- move
- copy
- clone
- drop 시점
- 함수 인자 ownership
- return ownership
- partial move
- destructuring과 ownership

## C. 참조와 빌림

- `&T`
- `&mut T`
- reborrow
- deref coercion
- aliasing 규칙
- NLL

## D. lifetime

- elision rules
- explicit annotation
- struct lifetime
- `'static`
- lifetime bound

## E. 자료형

- struct
- tuple struct
- enum
- pattern match
- method
- associated fn
- trait object
- generic struct

## F. 문자열

- `String`
- `str`
- `&str`
- UTF-8
- bytes/chars
- format!
- `to_string`
- `as_str`

## G. 컬렉션

- Vec
- slice
- HashMap
- BTreeMap
- HashSet
- VecDeque

## H. trait / generic

- generic fn
- generic impl
- bound
- where
- associated type
- supertrait
- object safety
- blanket impl

## I. 표준 trait

- Debug
- Display
- Clone
- Copy
- Default
- PartialEq/Eq
- PartialOrd/Ord
- Hash
- From/Into
- AsRef
- Deref
- Drop
- Iterator
- Send/Sync

## J. 오류 처리

- panic
- Result
- Option
- `?`
- map_err
- custom error
- anyhow
- thiserror

## K. iterator / closure

- iter/iter_mut/into_iter
- map/filter/fold
- collect
- enumerate
- zip
- flat_map
- closure capture
- Fn/FnMut/FnOnce

## L. 모듈 시스템

- mod
- use
- pub
- crate
- self/super
- lib/main
- workspace

## M. 스마트 포인터

- Box
- Rc
- Arc
- RefCell
- Cell
- Mutex
- RwLock
- Weak

## N. concurrency

- thread
- channel
- Arc<Mutex<T>>
- Send
- Sync
- atomic type 기초

## O. async

- async fn
- Future
- await
- runtime
- spawn
- join
- select
- Pin/Unpin 기초

## P. 매크로

- macro_rules!
- derive macro
- attribute macro
- function-like macro
- 흔한 crate macro 읽기

## Q. unsafe

- raw pointer
- dereference
- extern C
- repr(C)
- union
- MaybeUninit
- NonNull
- unsafe trait
- unsafe abstraction

## R. 툴링

- cargo
- rustfmt
- clippy
- rust-analyzer
- cargo doc
- cargo test
- cargo bench 기초
- feature flag

## S. 읽기 실전

- std 문서 읽기
- docs.rs 읽기
- 라이브러리 예제 먼저 읽기
- 테스트 코드 먼저 읽기
- public API에서 private 구현으로 내려가기

---

# 22. “코드를 읽기 위한” 실전 독해 순서

Rust 코드를 보면 아래 순서로 읽어라.

## 1. crate 목적 확인

- CLI인지
- 웹 서버인지
- 라이브러리인지
- async인지
- unsafe가 많은지

## 2. Cargo.toml 확인

- 의존성
- feature
- edition
- crate 타입

## 3. public API 먼저 보기

- `lib.rs`
- `main.rs`
- 주요 `pub struct`, `pub enum`, `pub trait`, `pub fn`

## 4. 타입부터 보기

- 어떤 데이터 구조가 중심인지
- enum 상태가 무엇인지
- 참조인지 소유형인지

## 5. 함수 시그니처만 훑기

- 입력
- 출력
- borrow/move
- Result/Option 여부

## 6. impl / trait 관계 보기

- 어떤 trait를 구현하는지
- generic bound가 무엇인지

## 7. 그 다음 함수 몸체 보기

즉, Rust는 **몸체보다 타입과 시그니처를 먼저 읽어야 한다.**

---

# 23. 공부 우선순위 추천

네가 “하나도 빠짐없이” 읽고 싶다면, 중요도 순으로는 이렇다.

## 최우선

- ownership
- borrowing
- lifetime
- enum + match
- Option / Result
- String / &str
- iterator
- trait / generic

## 그다음

- smart pointer
- module system
- error handling idiom
- async
- Send/Sync

## 고급

- macro
- unsafe
- FFI
- Pin/Unpin
- advanced trait system

---

# 24. C를 아는 사람에게 특히 추천하는 학습 방식

너는 C를 아니까 아래 방식이 제일 효율적이다.

## 방식 1. C와 비교하며 Rust 보기

예를 들어 이런 식으로 계속 비교해라.

- C pointer vs Rust reference
- malloc/free vs ownership/drop
- NULL vs Option
- error code vs Result
- struct + function pointer vs trait
- enum(int-like) vs enum(sum type)

## 방식 2. 작은 코드를 ownership 관점으로 주석 달기

예:

```rust
fn main() {
    let mut s = String::from("hi"); // s가 String의 owner
    let r = &s; // immutable borrow
    println!("{}", r); // borrow 사용 끝
    s.push('!'); // 이제 mutable access 가능
}
```

이런 식으로 모든 줄에 ownership 주석을 붙이면 빨리 는다.

## 방식 3. 오픈소스는 작은 crate부터

처음부터 tokio 내부 구현 같은 데 들어가면 너무 어렵다.
순서 추천:

1. 작은 CLI crate
2. serde 사용 예제
3. clap 기반 CLI
4. 간단한 axum app
5. tokio 예제
6. unsafe/ffi crate 일부

---

# 25. 네 수준 목표에 맞는 구체적 학습 플랜

## 1주차

- ownership
- borrowing
- String / &str
- struct / enum / match

## 2주차

- Option / Result
- generic
- trait
- module system

## 3주차

- iterator
- closure
- error propagation
- HashMap / Vec 실전

## 4주차

- Box / Rc / Arc / RefCell / Mutex
- thread / channel / Send / Sync

## 5주차

- async / tokio / Future / await

## 6주차

- macro 기초
- unsafe 기초
- FFI 기초

## 7주차 이후

- 작은 오픈소스 crate 독해
- public API → tests → implementation 순서로 분석

---

# 26. 마지막으로, “Rust에서 특별히 공부해야 하는 것”을 한 문장씩 정리

정말 압축해서 말하면 이렇다.

- **ownership**: 누가 값을 소유하는지 추적하는 규칙
- **borrowing**: 소유권 없이 값을 잠깐 참조하는 방식
- **lifetime**: 참조 관계의 유효 범위를 타입으로 표현하는 규칙
- **enum + match**: 상태와 분기를 타입 안전하게 표현하는 핵심 도구
- **trait**: 공통 동작을 정의하는 추상화 수단
- **generic**: 타입에 독립적인 코드 작성 방식
- **Option / Result**: 부재와 오류를 타입으로 다루는 방식
- **iterator**: 컬렉션 처리의 핵심 관용구
- **smart pointer**: ownership/shared ownership/interior mutability를 위한 포인터 래퍼
- **async**: 비동기 작업을 Future 기반으로 표현하는 시스템
- **macro**: 컴파일 타임 코드 생성 도구
- **unsafe**: 안전 보장을 일부 수동으로 맡는 저수준 영역

---

# 27. 내가 추천하는 현실적인 최종 목표

“Rust를 하나도 빠짐없이 읽는다”를 현실적으로 다시 정의하면:

1. Rust Book 내용 완전 이해
2. Rust By Example 주요 섹션 소화
3. std 핵심 타입 문서 반복 읽기
4. tokio / serde / clap / axum 예제 읽기
5. 작은 오픈소스 crate 10개 분석
6. unsafe/async/macro가 섞인 코드도 최소한 겁먹지 않고 구조 파악

여기까지 오면, 웬만한 Rust 코드는 “아예 못 읽는 상태”에서 완전히 벗어난다.

원하면 다음 답변에서 내가 이걸 이어서
**“C 개발자를 위한 Rust 완전 정복 체크리스트.md 형태”**로 다시 정리해줄게.
그때는 각 항목마다

- 왜 필요한지
- C와 뭐가 다른지
- 최소 예제
- 꼭 읽어야 할 표준 타입/문법
  까지 붙여서 더 실전형으로 만들어줄 수 있다.

##확인하기

1. Rust에서 C보다 먼저 추적해야 하는 핵심은 왜 “포인터”보다 “소유권”인가?
2. `String`과 `&str`의 차이를 소유권 관점에서 설명할 수 있나?
3. `Rc<RefCell<T>>`와 `Arc<Mutex<T>>`가 각각 어떤 상황에서 쓰이는지 말할 수 있나?
4. Rust 코드를 읽을 때 함수 몸체보다 시그니처와 타입을 먼저 봐야 하는 이유는 무엇인가?
