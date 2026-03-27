use seahash::hash;
use std::io::Read;
use std::path::{self, Path};
use std::time::{Instant, UNIX_EPOCH};
use std::{env, fs, io, process};

// 사용자 입력값(src, dst)을 담는 구조체
struct ArgsConfig {
    src_dir: String, // 원본 디렉토리
    dst_dir: String, // 대상 디렉토리
}

impl ArgsConfig {
    // CLI 인자 파싱 함수
    fn build(args: &[String]) -> Result<ArgsConfig, &'static str> {
        // 인자 부족 시 에러
        if args.len() < 3 {
            return Err("not enough arguments...");
        }

        // 따옴표 제거 (Windows 경로 대응)
        let src_dir = args[1].clone().replace("\"", "");
        let dst_dir = args[2].clone().replace("\"", "");

        Ok(ArgsConfig { src_dir, dst_dir })
    }
}

// 복사 진행 상태 추적용 구조체
struct Counter {
    total: u32,   // 전체 처리 대상 수
    success: u32, // 복사 성공
    skip: u32,    // 건너뜀
    dir: u32,     // 디렉토리 수
}

// 핵심: 디렉토리 재귀 복사
fn copy_dir_all(src: &path::Path, dst: &path::Path, cnt: &mut Counter) -> io::Result<()> {
    // 대상 디렉토리 생성 (없으면)
    fs::create_dir_all(&dst)?;

    // src 디렉토리 순회
    for entry in fs::read_dir(src)? {
        let e = entry?;
        let ty = e.file_type()?; // 파일인지 디렉토리인지 확인

        // 전체 처리 개수 증가
        cnt.total += 1;

        // 디렉토리면 → 재귀 호출
        if ty.is_dir() {
            cnt.dir += 1;

            let src_path = e.path();
            let dst_path = dst.join(e.file_name());

            // 재귀 복사
            copy_dir_all(&src_path, &dst_path, cnt)?;
        } else {
            // 파일 복사 처리
            let src_file = e.path();
            let dst_file = dst.join(e.file_name());

            // 이미 존재하고 복사할 필요 없으면 skip
            if dst_file.exists() && !is_worth_to_copy(&src_file, &dst_file) {
                cnt.skip += 1;
            } else {
                // 실제 파일 복사
                fs::copy(&src_file, &dst_file)?;
                cnt.success += 1;
            }
        }
    }

    Ok(())
}

// src 디렉토리 내부 전체 파일 개수 카운트
fn cnt_all_entries(src: &path::Path, cnt: &mut u32) -> io::Result<()> {
    for entry in fs::read_dir(src)? {
        let e = entry?;
        let ty = e.file_type()?;

        if ty.is_dir() {
            // 디렉토리면 재귀
            cnt_all_entries(&e.path(), cnt)?;
        } else {
            *cnt += 1; // 파일 카운트 증가
        }
    }
    Ok(())
}

// 복사 여부 판단 로직
fn is_worth_to_copy(src_file: &path::PathBuf, dst_file: &path::PathBuf) -> bool {
    // 1. 수정 시간 비교 (src가 더 최신이어야 함)
    let src_time = file_modified_time_in_seconds(src_file.to_str().unwrap());
    let dst_time = file_modified_time_in_seconds(dst_file.to_str().unwrap());

    if src_time <= dst_time {
        return false;
    }

    // 2. 해시 비교 (내용이 같으면 복사 안 함)
    let src_hash = file_hash(src_file);
    let dst_hash = file_hash(dst_file);

    if src_hash == dst_hash {
        return false;
    }

    true
}

// 파일 수정 시간 확인 (최신 여부 판단)
fn file_modified_time_in_seconds(path: &str) -> u64 {
    fs::metadata(path)
        .unwrap()
        .modified()
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

// 파일 내용 동일 여부 확인
fn file_hash(file_path: &path::PathBuf) -> u64 {
    let mut file = std::fs::File::open(file_path).unwrap();
    let mut buffer = Vec::new();

    // 파일 전체 읽기
    let _ = file.read_to_end(&mut buffer);

    // 해시 계산
    hash(&buffer)
}

pub fn run() {
    // 1. CLI 인자 파싱
    let args: Vec<String> = env::args().collect();
    let args_config = ArgsConfig::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // 2. 로그 설정
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    // 3. 카운터 초기화
    let mut cnt = Counter {
        total: 0,
        success: 0,
        skip: 0,
        dir: 0,
    };

    let src_path = Path::new(&args_config.src_dir);
    let dst_path = Path::new(&args_config.dst_dir);

    // 4. 파일 개수 계산
    let mut file_nums = 0;
    cnt_all_entries(src_path, &mut file_nums).unwrap();

    println!("[Copy Started] try to copy {} files", file_nums);

    // 5. 복사 실행
    let start_time = Instant::now();
    copy_dir_all(src_path, dst_path, &mut cnt).expect("Error while copying");

    // 6. 결과 출력
    println!("[Copy Finished]");
    println!("Copied files: {}", cnt.success);

    let elapsed = start_time.elapsed();
    println!("Elapsed: {:?}", elapsed);
}
