use base64::{Engine as _, engine::general_purpose};
use ring::digest::{self, Context, Digest};

/*
===========================================================
[해시(Hash)란?]

- 임의 길이의 입력 데이터를
- 고정 길이의 출력으로 바꾸는 함수
- 복호화를 생각하고 만드는 것이 아님 (암호화와 차이)

예:
"hello" -> 32바이트 또는 64바이트 같은 고정 길이 결과

중요한 성질:
1. 같은 입력 -> 항상 같은 출력
2. 입력이 조금만 바뀌어도 출력이 크게 바뀜
3. 출력만 보고 원래 입력을 되돌리기 어려움
4. 충돌(collision): 서로 다른 입력이 같은 해시를 만드는 것은 매우 어려워야 함

===========================================================
[ring::digest 사용법]

1. digest()
   - 입력 데이터가 한 번에 준비되어 있을 때
   - 가장 간단한 방식
   - 예: 메모리에 이미 올라와 있는 작은 문자열/바이트

2. Context
   - 데이터를 여러 번 나눠서 넣어야 할 때
   - 예: 큰 파일, 네트워크 스트림, chunk 단위 처리

===========================================================
[알고리즘 선택 감각]

- SHA-1
  -> 레거시 전용. 새 코드에서는 사용하지 않음.

- SHA-256
  -> 가장 널리 쓰이는 범용 해시
  -> 블록체인, 파일 무결성 확인, 일반적인 서명/토큰 관련 전처리 등에 자주 등장

- SHA-384 / SHA-512
  -> 더 긴 출력이 필요하거나 SHA-512 계열이 필요한 경우 사용
  -> 64비트 환경에서 SHA-512 계열이 유리한 경우도 있음

- SHA-512/256
  -> 이름은 SHA-512지만 출력은 256비트(32바이트)
  -> "SHA-512를 그냥 잘라낸 것"이 아니라 별도의 초기 상태를 사용하는 정식 알고리즘

===========================================================
[중요]
해시는 "비밀번호 저장" 용도로 그대로 쓰면 안 됨.
비밀번호 저장은 Argon2id / scrypt / bcrypt / PBKDF2 같은
"password hashing" 전용 알고리즘을 사용해야 함.
===========================================================
*/

pub fn run() {
    let input = b"hello, world";

    println!("== one-shot digest ==");
    print_hash(
        "SHA-1 (legacy only)",
        &digest::SHA1_FOR_LEGACY_USE_ONLY,
        input,
    );
    print_hash("SHA-256", &digest::SHA256, input);
    print_hash("SHA-384", &digest::SHA384, input);
    print_hash("SHA-512", &digest::SHA512, input);
    print_hash("SHA-512/256", &digest::SHA512_256, input);

    println!();
    println!("== incremental hashing with Context ==");
    hash_with_context_example();

    // 사용자가 입력했다고 가정 (같으면 일치 출력)
    let original = b"hello, world";

    let saved_hash = digest::digest(&digest::SHA256, original);
    let input_hash = digest::digest(&digest::SHA256, input);

    if saved_hash.as_ref() == input_hash.as_ref() {
        println!("일치");
    } else {
        println!("불일치");
    }
}

/// one-shot 해시 계산
/// - 입력 데이터가 한 번에 있을 때 가장 간단한 방식
fn print_hash(label: &str, algorithm: &'static digest::Algorithm, input: &[u8]) {
    let result: Digest = digest::digest(algorithm, input);
    let base64 = general_purpose::STANDARD.encode(&result);

    println!("{label}");
    println!("  output bytes length: {}", result.as_ref().len());
    println!("  raw bytes: {:?}", result.as_ref());
    println!("  hex: {}", to_hex(result.as_ref()));
    println!("  base64 encode: {}", base64);

    println!();
}

/// Context 사용 예시
/// - 입력 데이터를 여러 조각으로 나누어 넣을 수 있음
/// - 큰 파일, 스트리밍 데이터에 적합
fn hash_with_context_example() {
    let mut ctx = Context::new(&digest::SHA256);

    // 실제로는 파일 chunk, 네트워크 packet 등을 순서대로 update 한다고 생각하면 됨.
    ctx.update(b"hello, ");
    ctx.update(b"world");

    let result = ctx.finish();

    println!("SHA-256 with Context");
    println!("  output bytes length: {}", result.as_ref().len());
    println!("  hex: {}", to_hex(result.as_ref()));
    println!();
}

/// 바이트 배열을 사람이 보기 쉬운 hex 문자열로 변환
/// - 해시 결과는 보통 바이너리이므로 그대로보다 hex가 훨씬 읽기 쉬움
fn to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);

    for byte in bytes {
        use core::fmt::Write;
        write!(&mut out, "{:02x}", byte).unwrap();
    }

    out
}
