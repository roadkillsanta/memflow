use crate::error::{Error, Result};

use flow_core::address::{Address, Length};
use flow_core::arch::{self, InstructionSet};
use flow_core::mem::*;

use byteorder::{BigEndian, ByteOrder, LittleEndian};
use widestring::U16CString;

pub trait VirtualReadUnicodeString {
    fn virt_read_unicode_string(&mut self, addr: Address) -> Result<String>;
}

// TODO: split up cpu and proc arch in read_helper.rs
impl<'a, T: VirtualMemoryTrait> VirtualReadUnicodeString for VirtualMemory<'a, T> {
    fn virt_read_unicode_string(&mut self, addr: Address) -> Result<String> {
        /*
        typedef struct _windows_unicode_string32 {
            uint16_t length;
            uint16_t maximum_length;
            uint32_t pBuffer; // pointer to string contents
        } __attribute__((packed)) win32_unicode_string_t;

        typedef struct _windows_unicode_string64 {
            uint16_t length;
            uint16_t maximum_length;
            uint32_t padding; // align pBuffer
            uint64_t pBuffer; // pointer to string contents
        } __attribute__((packed)) win64_unicode_string_t;
        */

        // length is always the first entry
        let mut length = 0u16;
        self.virt_read(addr + Length::zero(), &mut length)?;
        if length == 0 {
            return Err(Error::new("unable to read unicode string length"));
        }

        // TODO: chek if length exceeds limit
        // buffer is either aligned at 4 or 8
        let buffer = match self.proc_arch().instruction_set {
            InstructionSet::X64 => self.virt_read_addr64(addr + Length::from(8))?,
            InstructionSet::X86 => self.virt_read_addr32(addr + Length::from(4))?,
            _ => {
                return Err(Error::new("invalid proc_arch parameter"));
            }
        };
        if buffer.is_null() {
            return Err(Error::new("unable to read unicode string length"));
        }

        // check if buffer length is mod 2 (utf-16)
        if length % 2 != 0 {
            return Err(Error::new("unicode string length is not a multiple of two"));
        }

        // read buffer
        let mut content = vec![0; Length::from(length + 2).as_usize()];
        self.virt_read_raw(buffer, &mut content)?;
        content[length as usize] = 0;
        content[length as usize + 1] = 0;

        // TODO: check length % 2 == 0

        let content16 = content
            .chunks_exact(2)
            .map(|b| match self.proc_arch().instruction_set.byte_order() {
                arch::ByteOrder::LittleEndian => LittleEndian::read_u16(b),
                arch::ByteOrder::BigEndian => BigEndian::read_u16(b),
            })
            .collect::<Vec<u16>>();
        Ok(U16CString::from_vec_with_nul(content16)?.to_string_lossy())
    }
}
