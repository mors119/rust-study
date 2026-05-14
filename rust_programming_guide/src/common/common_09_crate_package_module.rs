pub fn run() {
    // include_str!: 컴파일 시점에 파일 내용을 문자열(&'static str)로 포함
    let content = include_str!("module_cheat_sheet.md");

    // 파일 내용 전체 출력
    println!("{content}");
}
