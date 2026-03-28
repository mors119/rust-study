use reqwest::StatusCode;
use serde::Deserialize;

/*

[목표]

- 인증 없이 바로 호출 가능한 음악 검색 API 사용
- HTTP GET → JSON 파싱 → 결과 출력 흐름 학습
- Spotify 예제와 비슷한 목적을 더 단순하게 연습

이번 예제는 iTunes Search API를 사용한다.

*/

#[derive(Debug, Deserialize)]
struct TrackResponse {
    results: Vec<Track>,
}

#[derive(Debug, Deserialize)]
struct Track {
    // 곡 이름
    #[serde(rename = "trackName")]
    track_name: String,

    // 앨범 이름
    #[serde(rename = "collectionName")]
    collection_name: Option<String>,

    // 아티스트 이름
    #[serde(rename = "artistName")]
    artist_name: String,

    // 미리듣기 URL (없을 수도 있음)
    #[serde(rename = "previewUrl")]
    preview_url: Option<String>,

    // 곡 상세 URL
    #[serde(rename = "trackViewUrl")]
    track_view_url: Option<String>,
}

#[tokio::main]
pub async fn run() {
    if let Err(err) = search_tracks("Jimin").await {
        eprintln!("에러: {}", err);
    }
}

/*

[검색 함수]

- query를 받아 공개 음악 검색 API 호출
- 상태 코드 확인
- JSON을 TrackResponse로 파싱

*/
async fn search_tracks(query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // query parameter는 직접 문자열 조립보다 query() 사용이 더 안전하다.
    let response = client
        .get("https://itunes.apple.com/search")
        .query(&[
            ("term", query),
            ("media", "music"),
            ("entity", "song"),
            ("limit", "10"),
        ])
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => {
            let parsed = response.json::<TrackResponse>().await?;
            print_tracks(&parsed.results);
        }
        other => {
            eprintln!("요청 실패. status={}", other);
        }
    }

    Ok(())
}

/*

[출력 함수]

- Track 목록을 보기 좋게 출력
- Option 필드는 없을 수 있으므로 안전하게 처리

*/
fn print_tracks(tracks: &[Track]) {
    if tracks.is_empty() {
        println!("검색 결과가 없습니다.");
        return;
    }

    for track in tracks {
        println!("[트랙] {}", track.track_name);
        println!(
            "[앨범] {}",
            track.collection_name.as_deref().unwrap_or("앨범 정보 없음")
        );
        println!("[아티스트] {}", track.artist_name);
        println!(
            "[미리듣기] {}",
            track.preview_url.as_deref().unwrap_or("미리듣기 URL 없음")
        );
        println!(
            "[상세 URL] {}",
            track.track_view_url.as_deref().unwrap_or("상세 URL 없음")
        );
        println!("---------");
    }
}
