use reqwest::{Client, StatusCode};
use std::time::Duration;

/*

[전체 구조]

- 하나의 Client를 재사용 (connection pooling)
- 모든 함수는 Result 반환 (unwrap 제거)
- retry는 "일시적 에러"만 재시도
- 상태코드 기반 처리

*/

#[tokio::main]
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://wikidocs.net/book/16747";

    // 공용 HTTP Client (실무에서는 반드시 재사용)
    let client = Client::builder().timeout(Duration::from_secs(5)).build()?;

    // 1. 상태 코드 + body 출력
    fetch_with_status(&client, url).await?;

    // 2. 헤더 포함 GET 요청
    client_get(&client, url).await?;

    // 3. retry 테스트 (잘못된 URL)
    let bad_url = format!("https://test.{}", &url[8..]);

    match retry_send(&client, &bad_url, 5).await {
        Ok(txt) => println!("Retry Success: {}", txt),
        Err(err) => eprintln!("Retry Failed: {}", err),
    }

    Ok(())
}

/*

[1. 상태 코드 + 응답 처리]

- HTTP 상태코드에 따라 분기 처리
- 200 OK → body 출력
- 기타 → 에러 처리

*/
async fn fetch_with_status(client: &Client, url: &str) -> Result<(), reqwest::Error> {
    let res = client.get(url).send().await?;

    match res.status() {
        StatusCode::OK => {
            let body = res.text().await?;
            println!("[OK] body length = {}", body.len());
        }
        StatusCode::UNAUTHORIZED => {
            println!("[401] Unauthorized");
        }
        StatusCode::NOT_FOUND => {
            println!("[404] Not Found");
        }
        other => {
            println!("[ERROR] status = {}", other);
        }
    }

    Ok(())
}

/*

[2. Client 기반 GET 요청]

- reqwest::get 대신 Client 사용 (실무 표준)
- 헤더 설정 예시 포함

*/
async fn client_get(client: &Client, url: &str) -> Result<(), reqwest::Error> {
    let res = client
        .get(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await?;

    println!("[client_get] status = {}", res.status());

    Ok(())
}

/*

[3. Retry 로직]

- 네트워크 오류 / timeout만 재시도
- 최대 retry 횟수 제한
- exponential backoff (점점 대기시간 증가)

*/
async fn retry_send(
    client: &Client,
    url: &str,
    max_retries: usize,
) -> Result<String, reqwest::Error> {
    let mut attempt = 0;

    loop {
        let res = client.get(url).send().await;

        match res {
            Ok(resp) => {
                // 정상 응답
                return resp.text().await;
            }
            Err(err) => {
                attempt += 1;

                // retry 가능한 에러만 재시도
                if attempt >= max_retries || !err.is_timeout() {
                    return Err(err);
                }

                println!("[retry] attempt {}...", attempt);

                // exponential backoff
                let delay = 2_u64.pow(attempt as u32);
                tokio::time::sleep(Duration::from_secs(delay)).await;
            }
        }
    }
}
