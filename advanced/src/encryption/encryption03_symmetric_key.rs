use ring::aead::{self, Aad, BoundKey, LessSafeKey, Nonce, NonceSequence};
use ring::rand::SecureRandom;
use ring::{error::Unspecified, rand};
use std::fs;
use std::path::Path;

/*
===========================================================
[AES-GCM 개념]

- AES: Advanced Encryption Standard (대칭키 암호화)
- GCM: Galois/Counter Mode (인증 + 암호화 동시 제공)

→ AEAD (Authenticated Encryption with Associated Data, 권장)

즉:
- 데이터 암호화 + 위조 방지(tag) 같이 제공

1. Counter 방식
   - nonce를 1, 2, 3 ... 순서대로 증가시키는 방식
   - 송신/수신 양쪽이 같은 규칙을 알고 있으면 nonce를 "계산으로 재현" 가능
   - 대신 상태(state)를 안전하게 관리해야 함
   - 실무에서는 counter 저장/복구를 잘못하면 nonce 재사용 사고가 날 수 있음

2. Random 방식
   - 매번 난수로 nonce를 새로 생성
   - 상태 관리가 단순함
   - 대신 복호화하려면 nonce를 메시지와 함께 저장/전송해야 함
   - 보통 [nonce | ciphertext+tag] 형태로 저장

===========================================================
[핵심 구성 요소]

1. key
   - 비밀 키 (32 bytes → AES-256)

2. nonce
   - 매 암호화마다 반드시 달라야 하는 값 (절대 재사용 금지)

3. plaintext
   - 암호화할 원본 데이터

4. AAD (Additional Authenticated Data)
   - 암호화되진 않지만 무결성 검증에 포함됨

5. tag
   - 위조 방지용 인증값

===========================================================
[주의 - 매우 중요]

❗ 같은 key + 같은 nonce 조합은 절대 재사용하면 안 된다

→ 재사용하면 암호가 깨질 수 있다 (실전에서 치명적)

===========================================================
*/

const KEY_FILE_PATH: &str = "secret.key";
const AES_KEY_LEN: usize = 32; // AES-256 = 32 bytes
const NONCE_LEN: usize = 12; // AES-GCM nonce length = 12 bytes

/// Counter nonce 생성기
/// - advance()가 호출될 때마다 nonce 값이 하나씩 증가한다.
/// - 여기서는 뒤 4바이트만 counter로 사용하고, 앞 8바이트는 0으로 둔다.
/// - 학습용 예제로는 충분하지만, 실서비스에서는 더 체계적인 nonce 설계가 필요하다.
struct CounterNonce(u32);

impl NonceSequence for CounterNonce {
    fn advance(&mut self) -> Result<Nonce, Unspecified> {
        let mut nonce_bytes = [0u8; NONCE_LEN];

        // nonce 12바이트 중 마지막 4바이트에 counter 삽입
        // big-endian: 사람이 큰 자리부터 읽는 일반적인 순서
        nonce_bytes[8..].copy_from_slice(&self.0.to_be_bytes());

        // 다음 호출을 위해 counter 증가
        self.0 = self.0.checked_add(1).ok_or(Unspecified)?;

        Nonce::try_assume_unique_for_key(&nonce_bytes)
    }
}

/// 프로젝트 루트에 secret.key 파일이 있으면 읽고,
/// 없으면 새로 생성해서 저장한 뒤 반환한다.
///
/// 반환 타입:
/// - [u8; 32]
///   - AES-256 키 길이에 정확히 맞는 고정 길이 배열
fn load_or_create_secret_key() -> [u8; AES_KEY_LEN] {
    let path = Path::new(KEY_FILE_PATH);

    if path.exists() {
        let bytes = fs::read(path).expect("secret.key 파일을 읽지 못했습니다.");

        if bytes.len() != AES_KEY_LEN {
            panic!(
                "secret.key 길이가 올바르지 않습니다. 기대 길이: {} bytes, 실제 길이: {} bytes",
                AES_KEY_LEN,
                bytes.len()
            );
        }

        let mut key = [0u8; AES_KEY_LEN];
        key.copy_from_slice(&bytes);

        println!("[key] 기존 secret.key 파일을 읽었습니다.");
        return key;
    }

    let rng = rand::SystemRandom::new();
    let mut key = [0u8; AES_KEY_LEN];
    rng.fill(&mut key).expect("랜덤 키 생성에 실패했습니다.");

    fs::write(path, key).expect("secret.key 파일 저장에 실패했습니다.");

    println!("[key] 새 secret.key 파일을 생성했습니다.");

    key
}

/// 바이트를 16진수(hex) 문자열로 보기 좋게 변환
/// - 암호화/해시 결과는 사람이 읽기 어려운 binary 이므로
///   디버깅할 때 hex 출력이 매우 유용하다.
fn to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);

    for b in bytes {
        use std::fmt::Write;
        write!(&mut out, "{:02x}", b).unwrap();
    }

    out
}

/*
===========================================================
[1. Counter nonce 방식]
===========================================================

메시지 포맷:
[counter_start: 4 bytes | ciphertext | tag]

왜 nonce 자체를 저장하지 않고 counter_start만 저장하나?
- Counter 방식에서는 nonce가 "counter_start로부터 계산 가능한 값"이기 때문
- 단, 송신/수신 양쪽이 같은 규칙으로 nonce를 만들어야 한다

주의:
- 여러 메시지를 암호화할 때는 같은 counter_start를 재사용하면 안 된다
- 실제 서비스에서는 다음 counter 값을 안전하게 저장해야 한다
*/

/// Counter 방식 암호화
///
/// 매개변수:
/// - key_bytes: AES-256 키 (32 bytes)
/// - plaintext: 암호화할 원문
/// - aad_bytes: AAD (암호화는 안 되지만 인증 대상에 포함)
/// - counter_start: nonce sequence 시작값
///
/// 반환값:
/// - [counter_start(4바이트) | ciphertext | tag]
fn encrypt_with_counter(
    key_bytes: &[u8; AES_KEY_LEN],
    plaintext: &[u8],
    aad_bytes: &[u8],
    counter_start: u32,
) -> Result<Vec<u8>, Unspecified> {
    let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, key_bytes)?;
    let nonce_sequence = CounterNonce(counter_start);

    // SealingKey:
    // - NonceSequence를 이용해 내부적으로 nonce를 하나 꺼내며 암호화
    let mut sealing_key = aead::SealingKey::new(unbound_key, nonce_sequence);

    // in-place 암호화:
    // - 입력 버퍼가 암호문 버퍼로 덮어써진다.
    // - 따라서 원문을 남기고 싶다면 복사본(Vec)을 만들어 써야 한다.
    let mut in_out = plaintext.to_vec();

    let tag = sealing_key.seal_in_place_separate_tag(Aad::from(aad_bytes), &mut in_out)?;

    let mut packaged = Vec::with_capacity(4 + in_out.len() + tag.as_ref().len());

    // 복호화 측에서 같은 CounterNonce(counter_start)를 재현할 수 있도록 저장
    packaged.extend_from_slice(&counter_start.to_be_bytes());

    // 암호문
    packaged.extend_from_slice(&in_out);

    // 인증 태그
    packaged.extend_from_slice(tag.as_ref());

    Ok(packaged)
}

/// Counter 방식 복호화
///
/// 입력 포맷:
/// - [counter_start(4바이트) | ciphertext | tag]
fn decrypt_with_counter(
    key_bytes: &[u8; AES_KEY_LEN],
    packaged: &[u8],
    aad_bytes: &[u8],
) -> Result<Vec<u8>, Unspecified> {
    if packaged.len() < 4 + aead::AES_256_GCM.tag_len() {
        return Err(Unspecified);
    }

    let counter_start = u32::from_be_bytes([packaged[0], packaged[1], packaged[2], packaged[3]]);

    let mut cipher_and_tag = packaged[4..].to_vec();

    let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, key_bytes)?;
    let nonce_sequence = CounterNonce(counter_start);

    let mut opening_key = aead::OpeningKey::new(unbound_key, nonce_sequence);

    let plaintext = opening_key.open_in_place(Aad::from(aad_bytes), &mut cipher_and_tag)?;

    Ok(plaintext.to_vec())
}

/*
===========================================================
[2. Random nonce 방식]
===========================================================

메시지 포맷:
[nonce: 12 bytes | ciphertext | tag]

왜 nonce를 메시지에 같이 저장하나?
- Random 방식에서는 nonce를 "다시 계산"할 수 없기 때문
- 복호화 시 암호화 때 쓴 nonce와 완전히 같은 값을 알아야 한다
- 그래서 보통 nonce를 ciphertext 앞에 붙여 저장/전송한다

핵심 차이:
- Counter 방식: nonce를 재현(reproduce)할 수 있음
- Random 방식: nonce를 저장(store)해야 함
*/

/// Random 방식 암호화
///
/// 반환 포맷:
/// - [nonce(12바이트) | ciphertext | tag]
fn encrypt_with_random(
    key_bytes: &[u8; AES_KEY_LEN],
    plaintext: &[u8],
    aad_bytes: &[u8],
) -> Result<Vec<u8>, Unspecified> {
    let rng = rand::SystemRandom::new();

    let mut nonce_bytes = [0u8; NONCE_LEN];
    rng.fill(&mut nonce_bytes)?;

    let nonce = Nonce::try_assume_unique_for_key(&nonce_bytes)?;

    let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, key_bytes)?;

    // LessSafeKey:
    // - nonce를 직접 전달하는 방식
    // - Random nonce처럼 "이번에 생성한 nonce를 내가 직접 관리"할 때 이해하기 좋다
    let sealing_key = LessSafeKey::new(unbound_key);

    let mut in_out = plaintext.to_vec();

    // append_tag:
    // - ciphertext 뒤에 tag를 자동으로 붙여준다.
    // - 결과적으로 in_out = [ciphertext | tag]
    sealing_key.seal_in_place_append_tag(nonce, Aad::from(aad_bytes), &mut in_out)?;

    let mut packaged = Vec::with_capacity(NONCE_LEN + in_out.len());

    // 복호화를 위해 nonce를 메시지 앞에 저장
    packaged.extend_from_slice(&nonce_bytes);

    // [ciphertext | tag]
    packaged.extend_from_slice(&in_out);

    Ok(packaged)
}

/// Random 방식 복호화
///
/// 입력 포맷:
/// - [nonce(12바이트) | ciphertext | tag]
fn decrypt_with_random(
    key_bytes: &[u8; AES_KEY_LEN],
    packaged: &[u8],
    aad_bytes: &[u8],
) -> Result<Vec<u8>, Unspecified> {
    if packaged.len() < NONCE_LEN + aead::AES_256_GCM.tag_len() {
        return Err(Unspecified);
    }

    let nonce_bytes = &packaged[..NONCE_LEN];
    let nonce = Nonce::try_assume_unique_for_key(nonce_bytes)?;

    let mut cipher_and_tag = packaged[NONCE_LEN..].to_vec();

    let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, key_bytes)?;
    let opening_key = LessSafeKey::new(unbound_key);

    let plaintext = opening_key.open_in_place(nonce, Aad::from(aad_bytes), &mut cipher_and_tag)?;

    Ok(plaintext.to_vec())
}

pub fn run() {
    let key_bytes = load_or_create_secret_key();

    println!();
    println!("==================== key info ====================");
    println!("key file: {}", KEY_FILE_PATH);
    println!("key hex : {}", to_hex(&key_bytes));

    let aad = b"Authentication Data";
    let plaintext = b"This is a plain text.";

    println!();
    println!("================ counter nonce ==================");
    println!("explanation:");
    println!("- nonce를 counter 값으로부터 계산한다.");
    println!("- 따라서 메시지에 nonce 12바이트 전체를 넣지 않고,");
    println!("  여기서는 시작 counter 값 4바이트만 저장했다.");
    println!(
        "- 하지만 여러 메시지를 보낼 때 counter 상태를 잘못 관리하면 nonce 재사용 사고가 난다."
    );

    let counter_start = 1;

    let counter_encrypted = encrypt_with_counter(&key_bytes, plaintext, aad, counter_start)
        .expect("counter 암호화 실패");

    let counter_decrypted =
        decrypt_with_counter(&key_bytes, &counter_encrypted, aad).expect("counter 복호화 실패");

    println!("counter encrypted(hex): {}", to_hex(&counter_encrypted));
    println!(
        "counter decrypted     : {}",
        String::from_utf8(counter_decrypted).expect("UTF-8 변환 실패")
    );

    println!();
    println!("================ random nonce ===================");
    println!("explanation:");
    println!("- nonce를 난수로 매번 새로 만든다.");
    println!("- nonce를 다시 계산할 수 없으므로,");
    println!("  메시지 앞에 nonce를 직접 붙여서 저장/전송해야 한다.");
    println!("- 보통 [nonce | ciphertext | tag] 형태를 많이 쓴다.");

    let random_encrypted =
        encrypt_with_random(&key_bytes, plaintext, aad).expect("random 암호화 실패");

    let random_decrypted =
        decrypt_with_random(&key_bytes, &random_encrypted, aad).expect("random 복호화 실패");

    println!("random encrypted(hex): {}", to_hex(&random_encrypted));
    println!(
        "random decrypted     : {}",
        String::from_utf8(random_decrypted).expect("UTF-8 변환 실패")
    );
}
