# Rust Crate, Package, Module 치트시트

---

# 1. Package, Crate, Module

## Package

- `Cargo.toml`이 있는 프로젝트 단위
- 하나 이상의 Crate를 포함할 수 있다.

## Crate

- Rust 컴파일 단위
- Binary Crate: 실행 파일 (`src/main.rs`)
- Library Crate: 라이브러리 (`src/lib.rs`)

## Module

- 코드를 논리적으로 분리하는 단위
- 네임스페이스(namespace)를 제공한다.

---

# 2. Crate Root

크레이트의 시작 파일이다.

- Binary Crate → `src/main.rs`
- Library Crate → `src/lib.rs`

모든 모듈 경로는 여기서 시작한다.

```rust
crate::garden::vegetables::Apple
```

---

# 3. 모듈 선언

## src/main.rs

```rust
mod garden;
```

Rust는 다음 순서로 모듈을 찾는다.

1. 인라인 모듈

```rust
mod garden {
    // 코드
}
```

2. `src/garden.rs`
3. `src/garden/mod.rs`

---

# 4. 서브 모듈 선언

## src/garden.rs

```rust
pub mod vegetables;
```

Rust는 다음 순서로 모듈을 찾는다.

1. 인라인 모듈

```rust
pub mod vegetables {
    // 코드
}
```

2. `src/garden/vegetables.rs`
3. `src/garden/vegetables/mod.rs`

---

# 5. 실제 파일 구조

```text
src/
├── main.rs
├── garden.rs
└── garden/
    └── vegetables.rs
```

---

# 6. 실제 코드 예시

## src/main.rs

```rust
mod garden;

fn main() {
    garden::print_garden();

    let _apple = garden::vegetables::Apple;
}
```

## src/garden.rs

```rust
pub mod vegetables;

pub fn print_garden() {
    println!("정원 모듈입니다.");
}
```

## src/garden/vegetables.rs

```rust
pub struct Apple;
```

---

# 7. 모듈 경로

```rust
crate::garden::vegetables::Apple
```

- `crate` → 현재 크레이트 루트
- `garden` → 상위 모듈
- `vegetables` → 하위 모듈
- `Apple` → 구조체(struct)

---

# 8. 공개(pub)와 비공개

기본적으로 모든 모듈과 아이템은 부모 모듈에 대해 비공개(private)이다.

## 공개하려면

```rust
pub mod vegetables;
pub struct Apple;
pub fn hello() {}
```

---

# 9. use 키워드

긴 경로를 짧게 사용할 수 있게 한다.

## 기본 사용

```rust
use crate::garden::vegetables::Apple;

fn main() {
    let _apple = Apple;
}
```

---

# 10. as 별칭(alias)

```rust
use crate::garden::vegetables::Apple as Fruit;

fn main() {
    let _fruit = Fruit;
}
```

---

# 11. pub use (재공개)

다른 모듈의 아이템을 현재 모듈에서 다시 공개한다.

## src/garden.rs

```rust
pub mod vegetables;
pub use vegetables::Apple;
```

## src/main.rs

```rust
mod garden;

fn main() {
    let _apple = garden::Apple;
}
```

원래 경로:

```rust
garden::vegetables::Apple
```

재공개 후:

```rust
garden::Apple
```

---

# 12. 중첩 경로 가져오기

```rust
use std::{cmp::Ordering, io};
```

---

# 13. self 키워드

```rust
use std::io::{self, Write};
```

동일한 의미:

```rust
use std::io;
use std::io::Write;
```

---

# 14. 글롭(glob) 연산자

모든 공개 아이템을 가져온다.

```rust
use std::collections::*;
```

- `*` : 모든 public 아이템
- `_` : 값 무시용 패턴

---

# 15. 인라인 모듈

파일을 따로 만들지 않고 직접 작성할 수 있다.

```rust
mod garden {
    pub fn hello() {
        println!("Hello");
    }
}
```

---

# 16. 현대적 파일 구조 (권장)

```text
src/
├── main.rs
├── garden.rs
└── garden/
    └── vegetables.rs
```

---

# 17. 과거 방식 (`mod.rs`)

```text
src/
├── main.rs
└── garden/
    ├── mod.rs
    └── vegetables.rs
```

현재도 완전히 지원되지만, `garden.rs` 방식이 더 많이 사용된다.

---

# 18. Rust가 모듈을 찾는 규칙

```rust
mod garden;
```

찾는 위치:

- `garden.rs`
- `garden/mod.rs`

```rust
pub mod vegetables;
```

찾는 위치:

- `garden/vegetables.rs`
- `garden/vegetables/mod.rs`

---

# 19. 경로 종류

## 절대 경로

```rust
crate::garden::vegetables::Apple
```

## 상대 경로

```rust
vegetables::Apple
```

---

# 20. super 키워드

부모 모듈을 가리킨다.

```rust
super::print_garden()
```

---

# 21. self 키워드 (경로)

현재 모듈을 가리킨다.

```rust
self::vegetables::Apple
```

---

# 22. 전체 흐름 요약

1. `main.rs`는 크레이트 루트
2. `mod garden;` → `garden.rs` 또는 `garden/mod.rs` 탐색
3. `pub mod vegetables;` → `garden/vegetables.rs` 탐색
4. `pub struct Apple;` 공개
5. `crate::garden::vegetables::Apple`로 접근
6. `use`로 경로를 단축 가능
7. `pub use`로 재공개 가능

---

# 23. 한눈에 보는 구조

```text
crate
└── garden
    ├── print_garden()
    └── vegetables
        └── Apple
```

---

# 24. 요약

- Package = Cargo 프로젝트
- Crate = 컴파일 단위
- Module = 코드 그룹
- `mod` = 모듈 선언
- `pub` = 외부 공개
- `use` = 경로 단축
- `pub use` = 재공개
- `crate` = 크레이트 루트
- `self` = 현재 모듈
- `super` = 부모 모듈

```

```
