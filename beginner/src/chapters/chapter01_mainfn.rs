pub fn run(args: &[String]) {
    // let args: Vec<String> = env::args().collect();

    // $ cargo run 1 src.txt target.txt
    /*
        if args.len() < 3 {
            println!("사용법: cargo run 1 <src> <target>");
            return;
        }

        let src_file = &args[2];
        // 안쓰는 파일이나 변수는 _로 시작하거나 이름을 _로 지정하는 관례
        let _tat_file = &args[3];
    */
    // 위 코드의 다른 표현 (Option<&String>으로 받음)
    let src_file = args.get(2);
    let tat_file = args.get(3);

    if src_file.is_none() || tat_file.is_none() {
        println!("사용법: cargo run <src> <target>");
        return;
    }

    // 줄을 바꾸는 것은 println!, 줄 바꿈 없는 것은 print!
    match args.get(2) {
        Some(src_file) => println!("Source File: {}", src_file),
        None => println!("src_file 없음"),
    }

    let v = vec![1, 2, 3];

    print!("{:?}", v); // 벡터와 같은 복합형 변수는 {:?}
}
