use exe::pe::VecPE;
use exe::types::Arch;
use exe::{ImageFileHeader, PE};

use crate::arch_module;

fn read_file_header(image: &VecPE) -> ImageFileHeader {
    match arch_module::get_arch(image) {
        Arch::X86 => image.get_nt_headers_32().unwrap().file_header,
        Arch::X64 => image.get_nt_headers_64().unwrap().file_header,
    }
}

fn read_mut_file_header(image: &mut VecPE) -> &mut ImageFileHeader {
    match arch_module::get_arch(image) {
        Arch::X86 => &mut image.get_mut_nt_headers_32().unwrap().file_header,
        Arch::X64 => &mut image.get_mut_nt_headers_64().unwrap().file_header,
    }
}

pub fn get_timestamp(image: &VecPE) -> u32 {
    let foo = read_file_header(image);
    foo.time_date_stamp
}

pub fn set_timestamp(image: &mut VecPE, new_ts: u32) {
    let mut foo = read_mut_file_header(image);
    foo.time_date_stamp = new_ts
}
