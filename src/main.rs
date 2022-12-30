use clap::ArgGroup;
use clap::Parser;
use exe::Buffer;
use exe::VecPE;
use rand::Rng;

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
    #[arg(long, requires = "output-file")]
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

fn main_set_timestamp(image_ro: &VecPE, ts: u32, out_file: String) {
    let mut image_rw = image_ro.clone(); // clone the original image and make it mutable
    println!("[+] Setting timestamp to {} (0x{:x})", ts, ts);
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

    let image_ro = VecPE::from_disk_file(&cli.file).unwrap();

    println!("[+] Input fie: {}", &cli.file);

    match &cli.out_file {
        Some(out_filename) => println!("[+] Output file: {}", out_filename),
        None => {}
    }

    match &cli.get_timestamp {
        true => {
            let ts = timestamp_module::get_timestamp(&image_ro);
            println!("[+] Timestamp: {} (0x{:x})", ts, ts);
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

    println!("wont print");
    // let mut image = image_ro.clone();

    // let a = arch_module::get_arch(&image);

    // timestamp_module::set_timestamp(&mut image, a, 42);
    // timestamp_module::get_timestamp(&image, a);

    // image.save("test/start_changed.exe").unwrap();

    // let nt_header = image.get_nt_headers_64().unwrap();
    // println!("0x{:x}", nt_header.signature);
}
