use exe::pe::VecPE;
use exe::types::Arch;
use exe::PE;

use crate::arch_module;

pub fn get_timestamp(image: &VecPE) -> u32 {
    match arch_module::get_arch(image) {
        Arch::X86 => {
            image
                .get_nt_headers_32()
                .unwrap()
                .file_header
                .time_date_stamp
        }
        Arch::X64 => {
            image
                .get_nt_headers_64()
                .unwrap()
                .file_header
                .time_date_stamp
        }
    }
}

pub fn set_timestamp(image: &mut VecPE, new_ts: u32) {
    match arch_module::get_arch(image) {
        Arch::X86 => {
            image
                .get_mut_nt_headers_32()
                .unwrap()
                .file_header
                .time_date_stamp = new_ts;
        }
        Arch::X64 => {
            image
                .get_mut_nt_headers_64()
                .unwrap()
                .file_header
                .time_date_stamp = new_ts;
        }
    };
}
