use chrono::{DateTime, NaiveDateTime, Utc};
use exe::{Arch, ImageFileHeader, VecPE, PE};

pub fn get_arch(image: &VecPE) -> Arch {
    let arch = image.get_arch().unwrap();
    arch
}

pub fn convert_timestamp_to_utc(ts: u32) -> String {
    let naive = NaiveDateTime::from_timestamp_opt(ts.into(), 0).unwrap();
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn read_file_header(image: &VecPE) -> ImageFileHeader {
    match get_arch(image) {
        Arch::X86 => image.get_nt_headers_32().unwrap().file_header,
        Arch::X64 => image.get_nt_headers_64().unwrap().file_header,
    }
}

pub fn read_mut_file_header(image: &mut VecPE) -> &mut ImageFileHeader {
    match get_arch(image) {
        Arch::X86 => &mut image.get_mut_nt_headers_32().unwrap().file_header,
        Arch::X64 => &mut image.get_mut_nt_headers_64().unwrap().file_header,
    }
}
