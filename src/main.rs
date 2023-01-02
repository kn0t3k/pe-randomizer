use clap::ArgGroup;
use clap::Parser;
use exe::VecPE;

use std::fs::File;

mod arch_module;
mod timestamp_module;
mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(group(
    ArgGroup::new("timestamp-args")
        .required(false)
        .args(["set_timestamp", "get_timestamp", "set_random_timestamp"]),
))]
struct Cli {
    /// Name of modified file.
    #[arg(long, short, group = "output-file")]
    out_file: Option<String>,

    /// Print the timestamp and exit.
    #[arg(long)]
    get_timestamp: bool,

    /// Set timestamp and exit. Specify the timestamp as an unsigned 32bit integer epoch time.
    #[arg(long, requires = "output-file", value_name = "TIMESTAMP")]
    set_timestamp: Option<u32>,

    /// Set the timestamp to a random value.
    #[arg(long, requires = "output-file")]
    set_random_timestamp: bool,

    /// Input file.
    #[arg(long, short, group = "input-file")]
    file: String,
}

fn main() {
    let cli = Cli::parse();

    println!("[+] Input fie: {}", &cli.file);

    match File::open(&cli.file) {
        Err(_) => {
            println!("[!] Input file does not exist");
            return;
        }
        _ => {}
    }

    let image_ro = VecPE::from_disk_file(&cli.file).unwrap();

    match &cli.out_file {
        Some(out_filename) => println!("[+] Output file: {}", out_filename),
        None => {}
    }

    match &cli.get_timestamp {
        true => {
            let ts = timestamp_module::get_timestamp(&image_ro);
            println!("[+] Timestamps: {}", ts);
            // println!(
            //     "[+] Timestamp: {} (0x{:x}) ({})",
            //     ts,
            //     ts,
            //     utils::convert_timestamp_to_utc(ts)
            // );
            return;
        }
        _ => {}
    }

    match &cli.set_timestamp {
        Some(ts) => {
            timestamp_module::set_timestamp_save_file(&image_ro, *ts, cli.out_file.unwrap());
            return;
        }
        _ => {}
    }

    match &cli.set_random_timestamp {
        true => {
            timestamp_module::set_random_timestamp(&image_ro, cli.out_file.unwrap());
            return;
        }
        _ => {}
    }
}
