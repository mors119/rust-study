use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;

// [요청 DTO (Data Transfer Object)]

// - 서버로 보낼 JSON 구조 정의
// - 실제 API 스펙과 1:1 매핑됨

#[derive(Deserialize, Serialize, Debug)]
struct CreateRecordsRequest {
    title: String,
    completed: bool,
}

// [응답 DTO]

// - 서버에서 내려주는 JSON 구조
// - 필요한 필드만 정의해도 됨 (serde가 알아서 매핑)

#[derive(Deserialize, Debug)]
struct CreateRecordResponse {
    json: Option<CreateRecordsRequest>, // httpbin 특성
    url: String,
}

// [API Client]

// - reqwest Client를 감싼 구조
// - 실무에서는 이걸 서비스 레이어처럼 사용

struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    // [Client 생성]

    // - timeout 설정
    // - connection pooling 자동 적용됨

    fn new(base_url: &str) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .expect("Client 생성 실패");

        Self {
            client,
            base_url: base_url.to_string(),
        }
    }

    // [POST 요청 - 사용자 생성]

    // - JSON body 전송
    // - 상태 코드 체크
    // - JSON 응답 파싱

    async fn create_user(
        &self,
        req: &CreateRecordsRequest,
    ) -> Result<CreateRecordResponse, reqwest::Error> {
        let url = format!("{}/post", self.base_url);

        let res = self
            .client
            .post(&url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::ACCEPT, "application/json")
            // 실무에서는 JWT 같은 인증 토큰
            // .bearer_auth("your_token_here")
            .json(req)
            .send()
            .await?;

        // [상태 코드 기반 처리]

        match res.status() {
            StatusCode::OK | StatusCode::CREATED => {
                let parsed = res.json::<CreateRecordResponse>().await?;
                Ok(parsed)
            }
            StatusCode::BAD_REQUEST => {
                panic!("잘못된 요청 (400)");
            }
            StatusCode::UNAUTHORIZED => {
                panic!("인증 실패 (401)");
            }
            other => {
                panic!("서버 에러: {}", other);
            }
        }
    }

    // [Retry POST 요청]

    // - 네트워크 오류 시 재시도
    // - exponential backoff 적용

    async fn create_user_with_retry(
        &self,
        req: &CreateRecordsRequest,
        max_retries: usize,
    ) -> Result<CreateRecordResponse, reqwest::Error> {
        let mut attempt = 0;

        loop {
            let result = self.create_user(req).await;

            match result {
                Ok(res) => return Ok(res),
                Err(err) => {
                    attempt += 1;

                    if attempt >= max_retries || !err.is_timeout() {
                        return Err(err);
                    }

                    println!("[retry] attempt {}", attempt);

                    let delay = 2_u64.pow(attempt as u32);
                    tokio::time::sleep(Duration::from_secs(delay)).await;
                }
            }
        }
    }
}

// [메인 실행]

// - 실제 요청 흐름

#[tokio::main]
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let api = ApiClient::new("https://httpbin.org/");

    let request = CreateRecordsRequest {
        title: "Ship the frontend".to_string(),
        completed: false,
    };

    // [기본 POST 요청]

    let response = api.create_user(&request).await?;
    println!("Response = {:?}", response);

    // [Retry 포함 POST 요청]

    let response_retry = api.create_user_with_retry(&request, 3).await?;
    println!("Response (retry) = {:?}", response_retry);

    Ok(())
}
