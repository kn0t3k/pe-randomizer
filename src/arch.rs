use exe::pe::{VecPE, PE};
use exe::types::Arch;

pub fn get_arch(image: &VecPE) -> Arch {
    let arch = image.get_arch().unwrap();
    arch
}
