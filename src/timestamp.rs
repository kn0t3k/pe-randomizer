use core::fmt;

use exe::{pe::VecPE, ExportDirectory};
use exe::{Buffer, DebugDirectory, ImageDirectoryEntry, ResourceDirectory, PE};
use rand::Rng;

use crate::{timestamp, utils};

pub struct Timestamps {
    file_ts: u32,
    export_ts: u32,
    rsrc_ts: u32,
    dbg_ts: u32,
}
impl fmt::Display for Timestamps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "
        File:     (0x{:0<8x}) ({})
        Export:   (0x{:0<8x}) ({})
        Resource: (0x{:0<8x}) ({})
        Debug:    (0x{:0<8x}) ({})",
            self.file_ts,
            utils::convert_timestamp_to_utc(self.file_ts),
            self.export_ts,
            utils::convert_timestamp_to_utc(self.export_ts),
            self.rsrc_ts,
            utils::convert_timestamp_to_utc(self.rsrc_ts),
            self.dbg_ts,
            utils::convert_timestamp_to_utc(self.dbg_ts)
        )
    }
}

pub fn get_timestamps(image: &VecPE) -> Timestamps {
    let file_ts = get_file_timestamp(image);
    let export_ts = get_export_timestamp(&image);
    let rsrc_ts = get_resource_timestamp(&image);
    let dbg_ts = get_debug_timestamp(&image);
    Timestamps {
        file_ts,
        export_ts,
        rsrc_ts,
        dbg_ts,
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
    timestamp::set_timestamp_mut(&mut image_rw, ts); // change the ts
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

fn get_file_timestamp(image: &VecPE) -> u32 {
    let coff_header = utils::read_file_header(image);
    coff_header.time_date_stamp
}

fn get_resource_timestamp(image: &VecPE) -> u32 {
    if image.has_data_directory(ImageDirectoryEntry::Resource) {
        let rsrc_dir = ResourceDirectory::parse(image);
        rsrc_dir.unwrap().root_node.directory.time_date_stamp
    } else {
        0
    }
}

fn get_debug_timestamp(image: &VecPE) -> u32 {
    if image.has_data_directory(ImageDirectoryEntry::Debug) {
        let dbg_dir = DebugDirectory::parse(image);
        dbg_dir.unwrap().time_date_stamp
    } else {
        0
    }
}
