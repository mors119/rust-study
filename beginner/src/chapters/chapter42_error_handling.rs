/*
Rust Error Handling Cheat Sheet
===============================

핵심 기준
---------
- 값이 "없을 수 있음"      -> Option<T>
- "왜 실패했는지"가 중요함 -> Result<T, E>
- 처리 못 하겠으면 상위로 보내기 -> ?
- 절대 복구 불가능 / 버그 성격   -> panic! 또는 assert!
- overflow는 상황에 맞는 명시적 API 사용

선택 기준
----------------
- 검색 결과가 없을 수 있다          -> Option
- 파일 읽기/파싱/네트워크 실패 등   -> Result
- 현재 함수가 직접 처리 못 한다      -> ?
- 테스트/프로토타입에서 빠르게 실패 -> unwrap / expect
- 운영 코드에서는 가능하면 명시 처리  -> match / ? / map / ok_or_else

참고
----
- ? = "에러를 자동으로 상위로 반환"
- Option에서 ? = None이면 즉시 None 반환
- Result에서 ? = Err면 즉시 Err 반환
*/

use std::num::ParseIntError;

/*
사용할 사용자 정의 에러 타입
--------------------------
실무에서는 보통 thiserror, anyhow 같은 크레이트를 자주 쓰지만,
여기서는 표준 라이브러리만으로 흐름을 설명합니다.
*/
#[derive(Debug)]
enum AppError {
    EmptyInput,
    Parse(ParseIntError),
    NotFound,
    Overflow,
}

/*
ParseIntError -> AppError 자동 변환
----------------------------------
이걸 구현해두면 parse()? 사용 시
ParseIntError가 AppError로 자동 변환됩니다.

즉:
"하위 에러 타입" -> "상위 에러 타입" 변환 규칙
*/
impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::Parse(err)
    }
}

/*
1. panic! 과 recoverable error
-----------------------------
panic!:
- 프로그램 진행이 의미 없을 때
- 절대 일어나면 안 되는 버그 상황
- 보통 라이브러리 사용 실수, invariant 깨짐 등

Option / Result:
- 복구 가능한 실패
- 호출자에게 처리 기회를 줌
*/
fn panic_example() {
    println!("\n[panic! vs recoverable error]");

    println!("panic!은 프로그램을 즉시 중단시킬 수 있으므로");
    println!("운영 코드에서는 recoverable error를 우선 고려하는 편이 좋습니다.");

    /*
    실제 panic 예시는 프로그램을 멈추므로 주석 처리

    panic!("something went terribly wrong");
    */
}

/*
2. Option<T>
------------
언제 쓰나?
- 값이 "없을 수는 있는데", 그게 자연스러운 상황일 때

예:
- 검색 결과 없음
- 첫 번째 원소 없음
- 맵에서 키 못 찾음

뭘 이해해야 하나?
- Some(value) 또는 None
- 에러 메시지는 없음
- "왜 없는가?" 보다 "있나 없나?"가 중요할 때 적합
*/
fn find_even(nums: &[i32]) -> Option<i32> {
    for &n in nums {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

fn option_example() {
    println!("\n[Option<T>]");

    let nums = [1, 3, 5, 8, 9];

    let found = find_even(&nums);

    match found {
        Some(value) => println!("first even = {}", value),
        None => println!("no even number found"),
    }

    /*
    자주 쓰는 처리 방식
    */
    let first = nums.first(); // Option<&i32>
    println!("first element = {:?}", first);

    let fallback = find_even(&[1, 3, 5]).unwrap_or(-1);
    println!("fallback with unwrap_or = {}", fallback);

    /*
    map:
    Some 안의 값을 변환
    */
    let doubled = find_even(&nums).map(|x| x * 2);
    println!("mapped option = {:?}", doubled);

    /*
    if let:
    Some일 때만 빠르게 꺼내기
    */
    if let Some(v) = find_even(&nums) {
        println!("if let extracted = {}", v);
    }

    /*
    let else:
    실패 시 빠르게 반환/분기
    */
    let Some(v) = find_even(&nums) else {
        println!("let else: no value");
        return;
    };
    println!("let else extracted = {}", v);
}

/*
3. Result<T, E>
---------------
언제 쓰나?
- 실패 이유를 알려줘야 할 때
- 호출자가 실패 원인에 따라 다르게 처리해야 할 때

예:
- 파일 읽기 실패
- 숫자 파싱 실패
- 네트워크 실패
*/
fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>()
}

fn result_example() {
    println!("\n[Result<T, E>]");

    let good = parse_number("123");
    let bad = parse_number("abc");

    match good {
        Ok(v) => println!("parsed good = {}", v),
        Err(e) => println!("parse failed = {}", e),
    }

    match bad {
        Ok(v) => println!("parsed bad = {}", v),
        Err(e) => println!("parse failed = {}", e),
    }

    /*
    unwrap / expect:
    - 실패 시 panic
    - 테스트, 예제, 빠른 검증에는 유용
    - 운영 코드에서는 신중하게 사용
    */
    let v = parse_number("42").expect("42 should be a valid integer");
    println!("expect value = {}", v);
}

/*
4. ? 로 에러 위임하기 (Result 버전)
----------------------------------
현재 함수가 직접 처리하지 않고 상위로 넘기고 싶을 때 사용

? 동작:
- Ok(value)  -> value 꺼냄
- Err(err)   -> 현재 함수에서 즉시 return Err(err.into())

즉 "중간 함수"를 매우 깔끔하게 만듦
*/
fn parse_positive_number(s: &str) -> Result<i32, AppError> {
    if s.trim().is_empty() {
        return Err(AppError::EmptyInput);
    }

    let n = s.parse::<i32>()?; // ParseIntError -> AppError 로 From 변환됨

    if n <= 0 {
        return Err(AppError::NotFound); // 예시용. 실제 의미상 다른 에러명을 써도 됨
    }

    Ok(n)
}

fn question_mark_result_example() {
    println!("\n[? with Result<T, E>]");

    println!(
        "parse_positive_number(\"7\") = {:?}",
        parse_positive_number("7")
    );
    println!(
        "parse_positive_number(\"abc\") = {:?}",
        parse_positive_number("abc")
    );
    println!(
        "parse_positive_number(\"\") = {:?}",
        parse_positive_number("")
    );
}

/*
5. 상위 함수에서 ? 로 올라온 에러 처리
------------------------------------
하위 함수:
- parse_positive_number() 가 Err를 만들 수 있음

상위 함수:
- ? 로 다시 위임할 수도 있고
- match 로 최종 처리할 수도 있음
*/
fn compute_double_from_text(s: &str) -> Result<i32, AppError> {
    let n = parse_positive_number(s)?; // 하위 에러를 상위로 다시 위임
    Ok(n * 2)
}

fn handle_upper_result_example() {
    println!("\n[upper-level handling after ? propagation]");

    for input in ["10", "abc", "", "-5"] {
        match compute_double_from_text(input) {
            Ok(v) => println!("input={input:?}, doubled={v}"),
            Err(AppError::EmptyInput) => println!("input={input:?}, error=empty input"),
            Err(AppError::Parse(e)) => println!("input={input:?}, parse error={e}"),
            Err(AppError::NotFound) => println!("input={input:?}, error=not positive"),
            Err(AppError::Overflow) => println!("input={input:?}, error=overflow"),
        }
    }
}

/*
6. ? with Option<T>
-------------------
Option에서도 ? 사용 가능

동작:
- Some(value) -> value 꺼냄
- None        -> 현재 함수에서 즉시 return None

즉 "없으면 바로 끝" 흐름을 매우 간단히 표현 가능
*/
fn first_char_of_first_word(s: &str) -> Option<char> {
    let first_word = s.split_whitespace().next()?; // 없으면 None 반환
    let first_char = first_word.chars().next()?; // 없으면 None 반환
    Some(first_char)
}

fn question_mark_option_example() {
    println!("\n[? with Option<T>]");

    println!("{:?}", first_char_of_first_word("hello world"));
    println!("{:?}", first_char_of_first_word(""));
    println!("{:?}", first_char_of_first_word("   "));
}

/*
7. Option -> Result 변환
------------------------
Option은 "왜 실패했는지" 정보가 없음
그래서 상위에서 Result가 필요하면
ok_or / ok_or_else 로 에러를 붙여서 Result로 바꾼다
*/
fn get_user_age(name: &str) -> Option<u32> {
    match name {
        "alice" => Some(20),
        "bob" => Some(30),
        _ => None,
    }
}

fn require_user_age(name: &str) -> Result<u32, AppError> {
    get_user_age(name).ok_or(AppError::NotFound)
}

fn option_to_result_example() {
    println!("\n[Option -> Result]");

    println!("alice = {:?}", require_user_age("alice"));
    println!("nobody = {:?}", require_user_age("nobody"));
}

/*
8. Result -> Option 변환
------------------------
실패 이유는 버리고 "성공 값만 있으면 된다"면 ok() 사용
*/
fn result_to_option_example() {
    println!("\n[Result -> Option]");

    let a = parse_number("123").ok();
    let b = parse_number("abc").ok();

    println!("ok result -> option = {:?}", a);
    println!("err result -> option = {:?}", b);
}

/*
9. overflow(오버플로우) 처리
---------------------------
Rust의 정수 overflow는 상황에 따라 다르게 다룰 수 있다.

기본 감각
---------
- debug 빌드: overflow 시 panic 가능
- release 빌드: wrapping 동작이 될 수 있음

그래서 실무에서는 "의도를 드러내는 API"를 쓰는 편이 좋다.

대표 API
--------
- checked_add      -> Option 반환, overflow면 None
- overflowing_add  -> (결과, overflow 여부) 반환
- saturating_add   -> 최대/최소값에 고정
- wrapping_add     -> 경계를 넘어가면 순환
*/
fn overflow_example() {
    println!("\n[overflow handling]");

    let a: u8 = 250;
    let b: u8 = 10;

    let checked = a.checked_add(b);
    println!("checked_add = {:?}", checked);

    let (overflowing_value, did_overflow) = a.overflowing_add(b);
    println!(
        "overflowing_add = value={}, overflow={}",
        overflowing_value, did_overflow
    );

    let saturating = a.saturating_add(b);
    println!("saturating_add = {}", saturating);

    let wrapping = a.wrapping_add(b);
    println!("wrapping_add = {}", wrapping);

    /*
    의미
    ----
    checked_add:
      - 안전하게 실패를 감지하고 싶을 때
      - Option으로 처리 가능

    overflowing_add:
      - overflow 발생 사실까지 직접 받고 싶을 때

    saturating_add:
      - 최대값/최소값을 넘지 않게 clamp 하고 싶을 때
      - 예: 점수, 밝기, 제한값

    wrapping_add:
      - 비트 연산, 해시, low-level code 등에서 순환이 의도일 때
    */
}

/*
10. overflow를 Result로 바꾸기
-----------------------------
checked_add는 Option 반환이므로,
업무 로직에서는 Result로 바꾸는 경우가 많다.
*/
fn safe_add_u8(a: u8, b: u8) -> Result<u8, AppError> {
    a.checked_add(b).ok_or(AppError::Overflow)
}

fn overflow_result_example() {
    println!("\n[overflow -> Result]");

    println!("100 + 20 = {:?}", safe_add_u8(100, 20));
    println!("250 + 10 = {:?}", safe_add_u8(250, 10));
}

/*
11. 여러 단계에서 ? 사용하기
---------------------------
실무에서 가장 많이 보는 형태에 가깝다.

흐름
----
1. 입력 검증
2. Option -> Result 변환
3. parse
4. overflow 검사
5. 최종 결과 반환
*/
fn read_first_and_add_one(inputs: &[&str]) -> Result<u8, AppError> {
    let first = inputs.first().ok_or(AppError::NotFound)?; // Option -> Result
    let number: u8 = first.parse()?; // ParseIntError -> AppError
    let plus_one = number.checked_add(1).ok_or(AppError::Overflow)?; // Option -> Result
    Ok(plus_one)
}

fn multi_step_question_mark_example() {
    println!("\n[multi-step ? example]");

    for input in [vec!["41"], vec!["255"], vec!["abc"], vec![]] {
        match read_first_and_add_one(&input) {
            Ok(v) => println!("input={input:?}, result={v}"),
            Err(e) => println!("input={input:?}, error={e:?}"),
        }
    }
}

/*
12. unwrap / expect 는 언제 쓰나?
--------------------------------
가능한 사용처
- 테스트 코드
- 예제 코드
- "이건 절대 실패하면 안 된다"가 매우 명확한 초기화 코드

운영 코드에서는 남용 주의
- 실패 시 panic
- 사용자 입력/외부 I/O에서는 보통 명시 처리 선호
*/
fn unwrap_expect_example() {
    println!("\n[unwrap / expect]");

    let a = Some(10).unwrap();
    println!("unwrap option = {}", a);

    let b = parse_number("77").expect("77 must parse");
    println!("expect result = {}", b);

    /*
    아래는 panic 예시라서 주석 처리

    let x = None::<i32>.unwrap();
    let y = parse_number("abc").unwrap();
    */
}

/*
13. Option / Result 사용 감각 요약
---------------------------------
Option:
- 값이 없을 수 있음
- 실패 원인은 중요하지 않음

Result:
- 성공/실패 모두 중요
- 실패 이유를 호출자에게 전달해야 함

?:
- 현재 함수가 처리하지 않고 상위에 맡김
- 코드 중첩을 크게 줄여줌
*/

fn summary_guide() {
    println!("\n[selection guide]");
    println!("1) 값이 없을 수만 있으면 된다              -> Option<T>");
    println!("2) 실패 이유까지 전달해야 한다             -> Result<T, E>");
    println!("3) 현재 함수가 처리 못 하면               -> ?");
    println!("4) Option을 Result로 바꾸려면             -> ok_or / ok_or_else");
    println!("5) Result를 Option으로 바꾸려면           -> ok()");
    println!(
        "6) overflow를 명시적으로 처리하려면       -> checked_* / saturating_* / wrapping_* / overflowing_*"
    );
}

/*
14. 자주 하는 오해
-----------------
1. ? 는 에러를 "삼키는 것"이 아니다
   -> 현재 함수에서 즉시 반환하는 것

2. Option은 단순히 "가벼운 Result"가 아니다
   -> 실패 이유 자체가 필요 없는 상황용

3. unwrap은 나쁜 문법이 아니다
   -> 다만 운영 코드에서 남용하면 위험

4. overflow는 그냥 두면 된다?
   -> 빌드 모드/상황에 따라 다를 수 있으므로 의도를 드러내는 API를 쓰는 편이 안전
*/

pub fn run() {
    panic_example();
    option_example();
    result_example();
    question_mark_result_example();
    handle_upper_result_example();
    question_mark_option_example();
    option_to_result_example();
    result_to_option_example();
    overflow_example();
    overflow_result_example();
    multi_step_question_mark_example();
    unwrap_expect_example();
    summary_guide();
}
