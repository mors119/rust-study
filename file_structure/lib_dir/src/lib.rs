use crate::{note::save_note, notes::add};

mod note; // pub이 아니므로 내부에서만 사용됨.
pub mod notes; // 외부 crate에서 사용 가능하도록 public 모듈 (lib_dir -> note 공개)

// 가장 단순한 공개 (진입점에서 pub)
pub fn hello() {
    println!("hello");
    save_note();
    let _ = add::AddNote;
}
