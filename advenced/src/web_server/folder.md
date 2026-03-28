```text
axum-app/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── app.rs              # Router 구성
    ├── config.rs           # 환경설정
    ├── state.rs            # AppState
    │
    ├── handlers/           # HTTP layer
    │   ├── mod.rs
    │   └── user.rs
    │
    ├── services/           # 비즈니스 로직
    │   ├── mod.rs
    │   └── user_service.rs
    │
    ├── models/             # DB / Entity
    │   ├── mod.rs
    │   └── user.rs
    │
    └── dto/                # 요청/응답 구조
        ├── mod.rs
        └── user_dto.rs
```
