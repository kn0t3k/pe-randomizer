use core::fmt;

use exe::{pe::VecPE, ExportDirectory};
use exe::{Buffer, ImageDirectoryEntry, PE};
use rand::Rng;

use crate::{timestamp_module, utils};

pub struct Timestamps {
    coff_ts: u32,
    export_ts: u32,
}
impl fmt::Display for Timestamps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[-] TODO {}, {}", self.coff_ts, self.export_ts)
    }
}

pub fn get_timestamp(image: &VecPE) -> Timestamps {
    let coff_ts = get_coff_timestamp(image);
    let export_ts = get_export_timestamp(&image);
    Timestamps {
        coff_ts: coff_ts,
        export_ts: export_ts,
    }
}

fn set_timestamp_mut(image: &mut VecPE, new_ts: u32) {
    let mut foo = utils::read_mut_file_header(image);
    foo.time_date_stamp = new_ts;
}

pub fn set_timestamp_save_file(image_ro: &VecPE, ts: u32, out_file: String) {
    let mut image_rw = image_ro.clone(); // clone the original image and make it mutable
    println!(
        "[+] Setting timestamp to {} (0x{:x} ({}))",
        ts,
        ts,
        utils::convert_timestamp_to_utc(ts)
    );
    timestamp_module::set_timestamp_mut(&mut image_rw, ts); // change the ts
    image_rw.save(out_file).unwrap(); // save the changed file with the new timestamp
}

pub fn set_random_timestamp(image_ro: &VecPE, out_file: String) {
    let mut rng = rand::thread_rng();
    let ts: u32 = rng.gen::<u32>();
    set_timestamp_save_file(image_ro, ts, out_file);
}

fn get_export_timestamp(image: &VecPE) -> u32 {
    if image.has_data_directory(ImageDirectoryEntry::Export) {
        let export_dir = ExportDirectory::parse(image);
        export_dir.unwrap().time_date_stamp
    } else {
        0
    }
}

fn get_coff_timestamp(image: &VecPE) -> u32 {
    let coff_header = utils::read_file_header(image);
    coff_header.time_date_stamp
}
