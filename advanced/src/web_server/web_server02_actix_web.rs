// cargo add actix-web
// 아래를 모두 충족하는 경우 actix web 사용
// ✔ 기존 프로젝트 유지보수
// ✔ 극한 성능 요구
// ✔ actix 경험 있음
// ✔ actor 모델 필요
// 현재 만들어지는 대부분은 axum 사용

use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Rust!")
}

#[actix_web::main]
pub async fn run() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
