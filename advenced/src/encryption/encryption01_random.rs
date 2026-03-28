use base64::{Engine as _, engine::general_purpose};
use ring::rand::{self, SecureRandom, SystemRandom};

/*
===========================================================
[RNG 개념 정리]

- RNG: Random Number Generator (난수 생성기)

- SystemRandom:
  OS (운영체제)의 보안 난수원(/dev/urandom 등)을 사용하는
  "CSPRNG" (Cryptographically Secure RNG)

- SecureRandom::fill():
  → 이미 만들어진 버퍼(Vec, 배열)에 난수를 채운다
  → 가장 기본적이고 유연한 방식

- rand::generate():
  → 고정 크기 타입([u8; N])을 바로 생성
  → 간단하지만 유연성은 적음

===========================================================
[언제 무엇을 쓰는가]

1. fill()
   - 길이가 동적인 경우 (Vec)
   - 다양한 길이를 재사용해야 할 때
   - 일반적으로 가장 많이 사용

2. generate()
   - 고정 길이 타입이 필요할 때
   - 간단하게 "딱 하나의 값" 생성할 때

===========================================================
[암호화에서의 대표적인 길이]

- nonce: 12 bytes (AES-GCM에서 표준)
- key: 32 bytes (AES-256)
- salt: 16 bytes (일반적인 권장값)

===========================================================
*/

/// 공용 RNG 생성 (실무에서는 재사용하는 것이 좋음)
fn get_rng() -> SystemRandom {
    SystemRandom::new()
}

/// 가변 길이 난수 생성
/// - Vec<u8> 형태
/// - 다양한 길이에 대응 가능
fn random_bytes(len: usize) -> Result<Vec<u8>, ring::error::Unspecified> {
    let rng = get_rng();

    let mut buf = vec![0u8; len];

    // SecureRandom::fill()
    // → buffer를 직접 채우는 가장 기본적인 방식
    rng.fill(&mut buf)?;

    Ok(buf)
}

/// 고정 길이 난수 생성 (32 bytes)
/// - AES key, hash seed 등에 사용
fn random_array_32() -> Result<[u8; 32], ring::error::Unspecified> {
    let rng = get_rng();

    let mut buf = [0u8; 32];

    rng.fill(&mut buf)?;

    Ok(buf)
}

/// rand::generate() 사용 예시
/// - 고정 타입을 바로 생성
/// - 내부적으로 안전하게 감싸진 Random<T> 반환
fn random_array_4_generate() -> Result<[u8; 4], ring::error::Unspecified> {
    let rng = get_rng();

    let value: rand::Random<[u8; 4]> = rand::generate(&rng)?;

    /*
    expose():
    - 내부 값을 꺼내는 함수
    - Random<T>는 안전한 wrapper라서 바로 접근 못하게 막아둠
    - "실제로 사용할 때만 꺼내라"는 의도
    */
    Ok(value.expose())
}

/// 실제 사용 예제
pub fn run() {
    let rng = get_rng();

    // 1. Vec 기반 난수 (동적 길이)
    let nonce = random_bytes(12).unwrap(); // AES-GCM nonce
    println!("nonce (12 bytes): {:?}", nonce);

    let salt = random_bytes(16).unwrap(); // password salt
    let password = general_purpose::STANDARD.encode(&salt);
    println!("salt (16 bytes) password: {:?}", password);

    // 2. 배열 기반 난수 (고정 길이)
    let key = random_array_32().unwrap(); // AES-256 key
    println!("key (32 bytes): {:?}", key);

    // 3. generate() 방식
    let small = random_array_4_generate().unwrap();
    println!("small random ([u8; 4]): {:?}", small);

    // 4. 직접 fill 사용 (가장 low-level)
    let mut raw = [0u8; 8];
    rng.fill(&mut raw).unwrap();

    println!("raw fill ([u8; 8]): {:?}", raw);
}
