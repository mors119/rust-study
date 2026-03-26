pub fn run(args: &[String]) {
    // let args: Vec<String> = env::args().collect(); // 가능은 하지만 main파일에서 물려받기
    // file_backup "usr/src" "usr/target" 라고 입력했다면
    // &args[0]은 프로그램 이름 file_backup이다.
    // let src_dir = &args[1];
    // let tgt_dir = &args[2];

    // 구조체 사용
    let args_config = ArgsConfig::build(&args).unwrap_or_else(|err| {
        println!("err = {}", err);
        std::process::exit(1);
    });
    let source = args_config.src_dir;
    let target = args_config.tgt_dir;

    println!("src = {} / tgt = {}", source, target);
}

// 위의 내용을 구조체로도 받아본다.
struct ArgsConfig {
    src_dir: String,
    tgt_dir: String,
}
impl ArgsConfig {
    // Self가 없으므로 메서드가 아니라 함수임.
    fn build(args: &[String]) -> Result<ArgsConfig, &'static str> {
        if args.len() < 3 {
            return Err("인자가 충분하지 않습니다. file_backup \"<백업 파일>\" \"<백업될 파일>\"");
        }
        let src_dir = args[1].replace("\"", ""); // 더블 따옴표 지우기
        let tgt_dir = args[2].replace("\"", "");

        Ok(ArgsConfig { src_dir, tgt_dir }) // Result이므로 Ok에 담아서 리턴
    }
}
