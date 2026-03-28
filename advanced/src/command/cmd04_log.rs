pub fn run(args: &[String]) {
    // 전부다 콘솔에 출력하지만 에러와 경고는 log 파일에 저장
    log::trace!("cmd04_log entered, args={:?}", args); // 상세 로그 (보통 개발과 디버그에만 사용)
    log::debug!("debug example"); // 디버그용 (보통 개발과 디버그에만 사용)
    log::info!("info example"); // 일반 정보
    log::warn!("warn example"); // 경고
    log::error!("error example"); // 에러

    println!("cmd04_log 실행");
}
