pub mod add;

pub mod entity; // main에서 use lib_dir::notes::entity::NoteEntity; 가능
// mod entity인 경우 main에서 접근 불가능 (note -> entity 공개)

// 실제로는 아래 구조를 많이 사용함.
mod save;
pub use save::*; // save 자체는 숨기고(== 쓰지 않고도) 내부 함수는 공개됨.
