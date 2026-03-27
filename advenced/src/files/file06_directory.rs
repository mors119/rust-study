use std::path::{Path, PathBuf};
use std::{env, fs, io};

/*
fs::create_dir 비어 있는 단일 디렉토리 생성
fs::create_dir_all 중간 경로까지 한 번에 생성
fs::read_dir 디렉토리 내부 읽기
fs::remove_dir 빈 디렉토리 삭제
fs::remove_dir_all 내부까지 전부 삭제
fs::rename 이동 또는 이름 변경
디렉토리 복사 fs::copy 하나로 안 되고, 재귀 구현 필요
*/

pub fn run() {
    if let Err(e) = create_dir() {
        eprintln!("{}", e);
    }
    if let Err(e) = create_all_dir() {
        eprintln!("{}", e);
    }
    if let Err(e) = read_directory() {
        eprintln!("{}", e);
    }
    if let Err(e) = read_all_directory() {
        eprintln!("{}", e);
    }
    if let Err(e) = copy_directory() {
        eprintln!("{}", e);
    }
    if let Err(e) = delete_empty_directory() {
        eprintln!("{}", e);
    }
    if let Err(e) = delete_all_directory() {
        eprintln!("{}", e);
    }
    if let Err(e) = move_or_rename_directory() {
        eprintln!("{}", e);
    }
}

// 하나의 디렉토리 생성 (현재 파일 루트에)
// create_dir는 dir/1을 생성할 때 경로 상 dir 디렉토리가 없으면 오류가 생김
fn create_dir() -> std::io::Result<()> {
    let cur_dir = env::current_dir()?; // 현재 실행 경로 확인
    fs::create_dir(cur_dir.join("dir"))?; // 디렉토리 생성
    // 또는 (아래의 경우 os 별 문제가 생길 수 있음. window는 \\도 사용되기 때문에)
    // let mut path = cur_dir.clone();
    // path.push("dir");
    // fs::create_dir(path)?;

    Ok(())
}

// 부모 디렉토리가 없는 경우 경로상 모든 디렉토리 생성
fn create_all_dir() -> io::Result<()> {
    let cur_dir = env::current_dir()?;
    fs::create_dir_all(cur_dir.join("dir/some/foo/crazy"))?;
    Ok(())
}

// 현재 디렉토리 내부 읽기
fn read_directory() -> std::io::Result<()> {
    // read_dir의 path는 path, &str, String 모두 가능하다.
    for entry in fs::read_dir(".")? {
        let e = entry?; // Result를 벗기기 위해 ?를 사용
        println!("{:?}", e.path());
    }
    Ok(())
}

// 디렉토리 내부 전체 읽기
fn read_all_directory() -> io::Result<()> {
    let v = collect_all_files(Path::new("."))?;
    v.into_iter().for_each(|p| println!("{:?}", p));
    Ok(())
}

fn collect_all_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut v: Vec<PathBuf> = vec![];
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let e = entry?;
            let path = e.path();
            // 재귀로 모든 파일 수집
            if path.is_dir() {
                v.extend(collect_all_files(&path)?);
            } else {
                v.push(path);
            }
        }
    }
    Ok(v)
}

// 디렉토리 복사
fn copy_directory() -> io::Result<()> {
    copy_dir_recursive(Path::new("dir"), Path::new("dir_copy"))?;
    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    if !src.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "source path is not a directory",
        ));
    }

    // 대상 디렉토리가 없으면 생성
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        // 디렉토리 재귀 복사 함수
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

// 비어 있는 디렉토리만 삭제
fn delete_empty_directory() -> io::Result<()> {
    // 비어 있어야만 삭제 가능
    fs::remove_dir("empty_dir")?;
    Ok(())
}

// 디렉토리 전체 삭제
fn delete_all_directory() -> io::Result<()> {
    // 내부 파일/하위 디렉토리까지 모두 삭제
    fs::remove_dir_all("dir_copy")?;
    Ok(())
}

// 디렉토리 이동 / 이름 변경
fn move_or_rename_directory() -> io::Result<()> {
    // dir/ → dir_renamed/
    let src = Path::new("dir");
    let dst = Path::new("dir_renamed");

    fs::rename(src, dst)?;

    println!("디렉토리 이동/이름 변경 완료: {:?} -> {:?}", src, dst);

    Ok(())
}
