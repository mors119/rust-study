use crate::asynchronous;
use crate::command;
use crate::files;
use crate::network;
use crate::thread;

pub fn run(args: &[String]) {
    match args[1].as_str() {
        "command" => match args[2].as_str() {
            "1" => command::cmd01_command_line::run(&args[3..]),
            "2" => command::cmd02_console_input::run(),
            "3" => command::cmd03_console_output::run(),
            "4" => command::cmd04_log::run(&args[3..]),

            _ => {
                println!("command 카테고리의 해당 번호가 없습니다.");
                log::warn!("unknown command number: {}", args[2]);
            }
        },
        "files" => match args[2].as_str() {
            "1" => files::file01_create::run(),
            "2" => files::file02_write::run(),
            "3" => files::file03_open::run(),
            "4" => files::file04_read::run(),
            "5" => files::file05_del_copy_move::run(),
            "6" => files::file06_directory::run(),
            "7" => files::file07_file_backup::run(),

            _ => {
                println!("files 카테고리의 해당 번호가 없습니다.");
                log::warn!("unknown files number: {}", args[2]);
            }
        },
        "network" => match args[2].as_str() {
            "1" => network::net01_tcp::run(),
            "2" => network::net02_udp::run(),
            "3" => network::net03_udp_like_tcp::run(),

            _ => {
                println!("network 카테고리의 해당 번호가 없습니다.");
                log::warn!("unknown network number: {}", args[2]);
            }
        },

        "thread" => match args[2].as_str() {
            "1" => thread::thread01_basic::run(),
            "2" => thread::thread02_move::run(),
            "3" => thread::thread03_mpsc::run(),
            "4" => thread::thread04_mpmc::run(),
            "5" => thread::thread05_rc::run(),
            "6" => thread::thread06_arc::run(),
            "7" => thread::thread07_mutex::run(),
            "8" => thread::thread08_arc_mutex::run(),

            _ => {
                println!("thread 카테고리의 해당 번호가 없습니다.");
                log::warn!("unknown thread number: {}", args[2]);
            }
        },

        "asynchronous" => match args[2].as_str() {
            "1" => asynchronous::async01_async_await::run(),
            "2" => asynchronous::async02_tokio_file::run(),
            "3" => asynchronous::async03_tokio_byte::run(),
            "4" => asynchronous::async04_tokio_buf::run(),
            "5" => asynchronous::async05_tokio_tcp_server::run().expect("REASON"),
            "6" => asynchronous::async06_tokio_tcp_client::run(),
            "7" => asynchronous::async07_tokio_web_socket::run(),
            "8" => asynchronous::async08_tokio_sample::run(),
            "9" => asynchronous::async09_tokio_web_socket_test::run(),

            _ => {
                println!("asynchronous 카테고리의 해당 번호가 없습니다.");
                log::warn!("unknown asynchronous number: {}", args[2]);
            }
        },

        _ => {
            println!("알 수 없는 category 입니다.");
            log::warn!("unknown category: {}", args[1]);
        }
    }
}
