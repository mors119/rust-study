use crate::command;
use crate::files;
use crate::network;

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

            _ => {
                println!("files 카테고리의 해당 번호가 없습니다.");
                log::warn!("unknown command number: {}", args[2]);
            }
        },
        "network" => match args[2].as_str() {
            "1" => network::tcp::run(),

            _ => {
                println!("network 카테고리의 해당 번호가 없습니다.");
                log::warn!("unknown command number: {}", args[2]);
            }
        },

        _ => {
            println!("알 수 없는 category 입니다.");
            log::warn!("unknown category: {}", args[1]);
        }
    }
}
