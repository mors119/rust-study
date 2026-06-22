use lib_dir::hello;
use lib_dir::notes::entity::NoteEntity;
use lib_dir::notes::save;

fn main() {
    hello();
    // save_note(); // lib.rs의 mod note에 pub이 아니므로 불가능
    save();
    let _ = NoteEntity;
}
