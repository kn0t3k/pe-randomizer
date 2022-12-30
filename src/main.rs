use chrono::NaiveDateTime;
use chrono::{DateTime, Utc};
use clap::ArgGroup;
use clap::Parser;
use exe::Buffer;
use exe::VecPE;
use rand::Rng;
use std::fs::File;

mod arch_module;
mod timestamp_module;

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

fn convert_timestamp_to_utc(ts: u32) -> String {
    let naive = NaiveDateTime::from_timestamp_opt(ts.into(), 0).unwrap();
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn main_set_timestamp(image_ro: &VecPE, ts: u32, out_file: String) {
    let mut image_rw = image_ro.clone(); // clone the original image and make it mutable
    println!(
        "[+] Setting timestamp to {} (0x{:x} ({}))",
        ts,
        ts,
        convert_timestamp_to_utc(ts)
    );
    timestamp_module::set_timestamp(&mut image_rw, ts); // change the ts
    image_rw.save(out_file).unwrap(); // save the changed file with the new timestamp
}

fn main_set_random_timestamp(image_ro: &VecPE, out_file: String) {
    let mut rng = rand::thread_rng();
    let ts: u32 = rng.gen::<u32>();
    main_set_timestamp(image_ro, ts, out_file);
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
            println!(
                "[+] Timestamp: {} (0x{:x}) ({})",
                ts,
                ts,
                convert_timestamp_to_utc(ts)
            );
            return;
        }
        _ => {}
    }

    match &cli.set_timestamp {
        Some(ts) => {
            main_set_timestamp(&image_ro, *ts, cli.out_file.unwrap());
            return;
        }
        _ => {}
    }

    match &cli.set_random_timestamp {
        true => {
            main_set_random_timestamp(&image_ro, cli.out_file.unwrap());
            return;
        }
        _ => {}
    }
}
