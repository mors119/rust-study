## HTTP 메시지는 [헤더 + 바디]

## 헤더(Header)는 4가지

- 일반 헤더 : 요청/응답 헤더에 공통으로 사용되는 헤더. 메시지 송신 시간 등
- 요청 헤더 : 대상이 되는 URL, 요청되는 메서드 종류 등
- 응답 헤더 : 서버 정보, 응답 상태 정보 등
- 엔티티 헤더: 바디에 대한 정보

## HTTP Method:

GET → 조회 (읽기)
POST → 생성 (쓰기)
PUT → 전체 수정 (덮어쓰기)
PATCH → 부분 수정
DELETE → 삭제
HEAD → 헤더만 조회 (GET에서 body 제외)
OPTIONS → 지원 메서드 확인
TRACE → 요청 경로 추적 (디버깅용, 거의 안 씀)
CONNECT → 터널 연결 (HTTPS 프록시 등)

## GET header

```text
GET /api/users?id=123 HTTP/1.1 → 요청라인 / key=value
Host: example.com → 요청할 서버 주소 / 필수
User-Agent: Mozilla/5.0 → 요청을 보내는 클라이언트 정보 / 브라우저, 앱, 봇 구분
Accept: application/json → 서버 응답 형식을 선택할 때 사용 ("json 형식으로 주세요.")
Authorization: Bearer abc123token → 인증 토큰
Accept-Language: ko-KR → 언어 설정 (다국어 설정 시 사용)
Cache-Control: no-cache → 캐시 정책 (최신 데이터 강제 요청)
Connection: keep-alive → 연결 유지 여부 (성능 최적화와 관련)
GET은 URL에 데이터를 담는다.
POST는 body에 데이터를 담는다.
```

## reqwest 추가

```bash
cargo add reqwest --features json
```
