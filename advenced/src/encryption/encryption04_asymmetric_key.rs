use ring::rand::SystemRandom;
use ring::signature::{self, EcdsaKeyPair, Ed25519KeyPair, KeyPair, RsaKeyPair, UnparsedPublicKey};
use std::fs;
use std::path::Path;

pub fn run() {
    println!("================ ECDSA ================");
    run_ecdsa();

    println!();
    println!("================ Ed25519 ==============");
    run_ed25519();

    println!();
    println!("================ RSA ==================");
    run_rsa_if_exists();
}

/*
===========================================================
[ECDSA 개념]

- ECDSA = Elliptic Curve Digital Signature Algorithm
- 타원 곡선 기반 전자서명 알고리즘
- 여기서는 P-256 곡선 + SHA-256 조합을 사용

SIGNING 상수:
- 개인키로 서명할 때 사용

VERIFY 상수:
- 공개키로 검증할 때 사용
===========================================================
*/

fn run_ecdsa() {
    let rng = SystemRandom::new();

    // 1) PKCS#8 형식의 개인키 문서 생성
    let pkcs8_doc = EcdsaKeyPair::generate_pkcs8(&signature::ECDSA_P256_SHA256_ASN1_SIGNING, &rng)
        .expect("ECDSA PKCS#8 생성 실패");

    // 2) PKCS#8 바이트로부터 실제 키페어 객체 생성
    let key_pair = EcdsaKeyPair::from_pkcs8(
        &signature::ECDSA_P256_SHA256_ASN1_SIGNING,
        pkcs8_doc.as_ref(),
        &rng,
    )
    .expect("ECDSA 키페어 파싱 실패");

    let message = b"This is a message";
    // 위조/변조 테스트용
    let tampered = b"This is a tampered message";

    // 3) 서명 생성
    let sig = key_pair.sign(&rng, message).expect("ECDSA 서명 실패");

    // 4) 공개키 추출
    let public_key_bytes = key_pair.public_key().as_ref();

    // 5) 공개키 검증 객체 생성
    let public_key = UnparsedPublicKey::new(&signature::ECDSA_P256_SHA256_ASN1, public_key_bytes);

    // 6) 검증
    match public_key.verify(message, sig.as_ref()) {
        Ok(()) => println!("[ECDSA] verification: OK"),
        Err(_) => println!("[ECDSA] verification: FAILED"),
    }

    // 7) 위조/변조 테스트
    match public_key.verify(tampered, sig.as_ref()) {
        Ok(()) => println!("[ECDSA] tampered verification: unexpectedly OK"),
        Err(_) => println!("[ECDSA] tampered verification: FAILED as expected"),
    }
}

/*
===========================================================
[Ed25519 개념]

- Ed25519 = EdDSA 계열
- 현대적으로 많이 쓰이는 서명 알고리즘 중 하나
- API도 비교적 단순한 편
===========================================================
*/

fn run_ed25519() {
    let rng = SystemRandom::new();

    // 1) PKCS#8 형식 키 문서 생성
    let pkcs8_doc = Ed25519KeyPair::generate_pkcs8(&rng).expect("Ed25519 PKCS#8 생성 실패");

    // 2) 키페어 파싱
    let key_pair =
        Ed25519KeyPair::from_pkcs8(pkcs8_doc.as_ref()).expect("Ed25519 키페어 파싱 실패");

    let message = b"This is a message";
    // 위조/변조 테스트용
    let tampered = b"This is a tampered message";

    // 3) 서명 생성
    let sig = key_pair.sign(message);

    // 4) 공개키 추출
    let public_key_bytes = key_pair.public_key().as_ref();

    // 5) 검증
    let public_key = UnparsedPublicKey::new(&signature::ED25519, public_key_bytes);

    match public_key.verify(message, sig.as_ref()) {
        Ok(()) => println!("[Ed25519] verification: OK"),
        Err(_) => println!("[Ed25519] verification: FAILED"),
    }

    // 6) 변조 테스트
    match public_key.verify(tampered, sig.as_ref()) {
        Ok(()) => println!("[Ed25519] tampered verification: unexpectedly OK"),
        Err(_) => println!("[Ed25519] tampered verification: FAILED as expected"),
    }
}

/*
===========================================================
[RSA 서명/검증]

- ring에서는 RSA 서명용 키를 PKCS#8로 읽는 흐름이 일반적
- 검증 알고리즘은 해시/키크기 범위가 고정된 상수로 제공됨

[키 생성 예제]

```bash
openssl genpkey -algorithm RSA \
  -pkeyopt rsa_keygen_bits:2048 \
  -pkeyopt rsa_keygen_pubexp:65537 | \
openssl pkcs8 -topk8 -nocrypt -outform der > rsa-2048-private-key.pk8
```
===========================================================
*/

fn run_rsa_if_exists() {
    let path = "rsa-2048-private-key.pk8";

    if !Path::new(path).exists() {
        println!("RSA 개인키 파일이 없어서 건너뜁니다: {path}");
        println!("OpenSSL로 먼저 생성하세요.");
        return;
    }

    let rng = SystemRandom::new();

    // 1) PKCS#8 DER 개인키 읽기
    let private_key_der = fs::read(path).expect("rsa-2048-private-key.pk8 파일을 읽지 못했습니다.");

    let key_pair = RsaKeyPair::from_pkcs8(&private_key_der).expect("RSA 키 파싱 실패");

    let message = b"This is a message";
    // 변조 테스트용
    let tampered = b"This is a tampered message";

    // 2) 서명 버퍼 준비
    let mut signature_bytes = vec![0u8; key_pair.public().modulus_len()];

    // 3) RSA PKCS#1 v1.5 + SHA-256 서명
    key_pair
        .sign(
            &signature::RSA_PKCS1_SHA256,
            &rng,
            message,
            &mut signature_bytes,
        )
        .expect("RSA 서명 실패");

    // 4) 공개키 추출
    let public_key_bytes = key_pair.public().as_ref();

    // 5) 검증
    let public_key =
        UnparsedPublicKey::new(&signature::RSA_PKCS1_2048_8192_SHA256, public_key_bytes);

    match public_key.verify(message, &signature_bytes) {
        Ok(()) => println!("[RSA] verification: OK"),
        Err(_) => println!("[RSA] verification: FAILED"),
    }

    // 6) 변조 테스트
    match public_key.verify(tampered, &signature_bytes) {
        Ok(()) => println!("[RSA] tampered verification: unexpectedly OK"),
        Err(_) => println!("[RSA] tampered verification: FAILED as expected"),
    }
}
