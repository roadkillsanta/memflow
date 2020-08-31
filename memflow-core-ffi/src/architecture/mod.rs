use memflow_core::architecture::{ArchitectureObj, Endianess};

pub mod x86;

#[no_mangle]
pub extern "C" fn arch_bits(arch: &ArchitectureObj) -> u8 {
    arch.bits()
}

#[no_mangle]
pub extern "C" fn arch_endianess(arch: &ArchitectureObj) -> Endianess {
    arch.endianess()
}

#[no_mangle]
pub extern "C" fn page_size(arch: &ArchitectureObj) -> usize {
    arch.page_size()
}

#[no_mangle]
pub extern "C" fn arch_size_addr(arch: &ArchitectureObj) -> usize {
    arch.size_addr()
}

#[no_mangle]
pub extern "C" fn arch_address_space_bits(arch: &ArchitectureObj) -> u8 {
    arch.address_space_bits()
}