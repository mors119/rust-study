use reqwest;
use scraper::Html;

/*
1. ApiClient
   - async reqwest::Client 사용
   - tokio runtime 위에서 동작

2. BlockingApiClient
   - reqwest::blocking::Client 사용
   - 동기 방식으로 동작

둘 다 같은 역할:
- base_url 저장
- GET 요청 보내기
- HTML 문자열 가져오기
- 필요하면 scraper::Html로 파싱하기
 */

pub fn run() {
    if let Err(e) = run_api() {
        eprintln!("{e}");
    };
    run_blocking();
}

// - async 클라이언트는 tokio runtime 안에서 실행해야 함
// - 필요할 때 호출해서 사용
#[tokio::main]
async fn run_api() -> Result<(), Box<dyn std::error::Error>> {
    let client = ApiClient::new("http://httpbin.org");

    let html = client.fetch_text("/").await?;
    println!("[async] body length = {}", html.len());

    let fragment = client.fetch_fragment("/").await?;
    println!("[async] parsed fragment = {:#?}", fragment);

    Ok(())
}

// [1. Async API Client]
// - reqwest::Client는 재사용하는 것이 좋다
// - base_url과 함께 struct에 보관
struct ApiClient {
    client: reqwest::Client,
    base_url: String,
}

impl ApiClient {
    fn new(base_url: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            // 뒤쪽 슬래시를 제거해서 URL 조합 시 // 방지
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    // [URL 조합용 내부 헬퍼]
    fn build_url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path.trim_start_matches('/'))
    }

    // [HTML 텍스트 가져오기 - async]
    async fn fetch_text(&self, path: &str) -> Result<String, reqwest::Error> {
        let url = self.build_url(path);

        let response = self.client.get(url).send().await?;

        // 상태 코드가 4xx / 5xx면 에러로 바꿔줌
        let response = response.error_for_status()?;

        let body = response.text().await?;
        Ok(body)
    }

    // [HTML fragment 파싱 - async]
    async fn fetch_fragment(&self, path: &str) -> Result<Html, Box<dyn std::error::Error>> {
        let html = self.fetch_text(path).await?;
        let fragment = Html::parse_fragment(&html);
        Ok(fragment)
    }
}

// [2. Blocking API Client]

// - reqwest::blocking::Client 사용
// - 동기 방식
// - 간단한 CLI, 스크립트에 적합
struct BlockingApiClient {
    client: reqwest::blocking::Client,
    base_url: String,
}

impl BlockingApiClient {
    fn new(base_url: &str) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    fn build_url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path.trim_start_matches('/'))
    }

    // [HTML 텍스트 가져오기 - blocking]
    fn fetch_text(&self, path: &str) -> Result<String, reqwest::Error> {
        let url = self.build_url(path);

        let response = self.client.get(url).send()?;
        let response = response.error_for_status()?;
        let body = response.text()?;

        Ok(body)
    }

    // [HTML fragment 파싱 - blocking]
    fn fetch_fragment(&self, path: &str) -> Result<Html, Box<dyn std::error::Error>> {
        let html = self.fetch_text(path)?;
        let fragment = Html::parse_fragment(&html);
        Ok(fragment)
    }
}

fn run_blocking() {
    // blocking은 일반 함수 안에서도 바로 실행 가능
    let blocking_client = BlockingApiClient::new("http://httpbin.org");

    match blocking_client.fetch_text("/") {
        Ok(html) => {
            println!("[blocking] body length = {}", html.len());

            let fragment = blocking_client.fetch_fragment("/").unwrap();
            println!("[blocking] parsed fragment = {:#?}", fragment);
        }
        Err(err) => {
            eprintln!("[blocking] error: {}", err);
        }
    }
}
