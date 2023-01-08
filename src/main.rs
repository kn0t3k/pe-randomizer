use exe::VecPE;
use inquire::{Select, Text};
use std::fs::File;

mod timestamp;
mod utils;

const SET_CMD: &str = "set";
const GET_CMD: &str = "get";
const HELP_CMD: &str = "help";
const EXIT_CMD: &str = "exit";
const TIMESTAMP_PROP: &str = "timestamp";
const PROP_CMDS: [&str; 1] = [TIMESTAMP_PROP];
const CMDS: [&str; 4] = [SET_CMD, GET_CMD, HELP_CMD, EXIT_CMD];

fn handle_get(image_ro: &VecPE) {
    let property = Select::new("What do you want to get?", PROP_CMDS.to_vec())
        .prompt()
        .unwrap();

    match property {
        TIMESTAMP_PROP => {
            let ts = timestamp::get_timestamps(&image_ro);
            println!("[+] Timestamps: {}", ts);
        }
        _ => {}
    }
}

fn handle_set(_image_ro: &VecPE) {
    // match &cli.out_file {
    //     Some(out_filename) => println!("[+] Output file: {}", out_filename),
    //     None => {}
    // }

    // match &cli.set_timestamp {
    //     Some(ts) => {
    //         timestamp::set_timestamp_save_file(&image_ro, *ts, cli.out_file.unwrap());
    //         return;
    //     }
    //     _ => {}
    // }

    // match &cli.set_random_timestamp {
    //     true => {
    //         timestamp::set_random_timestamp(&image_ro, cli.out_file.unwrap());
    //         return;
    //     }
    //     _ => {}
    // }

    let property = Select::new("What do you want to set?", PROP_CMDS.to_vec())
        .prompt()
        .unwrap();

    match property {
        TIMESTAMP_PROP => {
            // let ts = timestamp::get_timestamps(&image_ro);
            // println!("[+] Timestamps: {}", ts);
            todo!()
        }
        _ => {}
    }
}

fn handle_help() {
    todo!()
}

fn main() {
    let current_dir = std::env::current_dir().unwrap();
    let help_message = format!("Current directory: {}", current_dir.to_string_lossy());

    let ans = Text::new("Input file:")
        .with_help_message(&help_message)
        .prompt();

    let path = match ans {
        Ok(p) => p,
        Err(error) => {
            println!("Error: {:?}", error);
            String::from("")
        }
    };

    println!("[+] Input file: {}", path);

    match File::open(&path) {
        Err(_) => {
            println!("[!] Input file does not exist");
            return;
        }
        _ => {}
    }

    let image_ro = VecPE::from_disk_file(&path).unwrap();

    loop {
        let c = Select::new("Enter command: ", CMDS.to_vec())
            .prompt()
            .unwrap();

        match c {
            SET_CMD => {
                handle_set(&image_ro);
            }
            GET_CMD => {
                handle_get(&image_ro);
            }
            HELP_CMD => {
                handle_help();
            }
            EXIT_CMD => {
                break;
            }
            _ => {}
        };
    }
}
