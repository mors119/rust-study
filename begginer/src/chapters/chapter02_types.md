⭐ 많이 쓰는 타입

# 1. 정수 타입 (Integer Types)

### 1) 부호 있는 정수 (signed) → i

음수 / 양수 모두 가능
i8 → 8bit (-128 ~ 127)
i16 → 16bit
i32 → 32bit ⭐ (정수 기본값)
i64 → 64bit
i128 → 128bit
isize → 플랫폼 크기 (32bit or 64bit)

### 2) 부호 없는 정수 (unsigned) → u

양수만 가능 (0 이상)
u8
u16
u32 ⭐
u64
u128
usize → 포인터 크기 (배열 인덱스에 많이 사용) ⭐

```rust
let x: i32 = 10;
let y: u32 = x; // ❌ 에러
let y: u32 = x as u32; // ✅ 명시적 캐스팅 필요
```

# 2. 실수 타입 (Floating Point)

f32 → 32bit
f64 → 64bit ⭐ (기본값)

# 3. 문자 타입

char → 유니코드 문자 ⭐ (4byte라 🔥같은 유니코드 표현가능, C의 char는 1byte)

# 4. 불리언

bool ⭐ → true / false

# 5. 튜플 (Tuple)

여러 타입을 한 번에 묶음

```rust
let t: (i32, f64, char) = (10, 3.14, 'a');
// 접근
println!("{}", t.0);
```

# 6. 배열 (Array)

고정 길이

```rust
let arr: [i32; 3] = [1, 2, 3];
```

# 7. 슬라이스 (Slice)

배열의 참조

```rust
let s: &[i32] = &arr;
```

# 8. 문자열

Rust는 문자열이 2개다

### 1) &str (string slice) ⭐

불변, 빠름, 문자열 기본값

```rust
let s: &str = "hello";
```

### 2) String ⭐

가변, 힙에 저장

```rust
let s: String = String::from("hello");
```
