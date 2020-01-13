#[cfg(feature = "emulator")]
pub mod emulator;

use crate::address::{Address, Length};
use crate::error::Result;
use crate::ida_pattern::*;
use crate::mem::*;

// TODO: add more?
pub trait ProcessTrait {
    fn pid(&self) -> i32;
    fn name(&self) -> String;
    fn dtb(&self) -> Address;
}

// TODO: ProcessIterTrait

// TODO: generic Iterator
/*
pub trait ModuleIterTrait {
    fn module_iter(&self) -> Result<ModuleIterator<Self>>
    where
        Self: Sized + ArchitectureTrait + VirtualReadHelper + VirtualReadHelperFuncs;
}
*/

// TODO: Range impl for base to size?
// TODO: add more?
// TODO: maybe remove mut and fetch when module is loaded?
pub trait ModuleTrait {
    fn base(&mut self) -> Result<Address>;
    fn size(&mut self) -> Result<Length>;
    fn name(&mut self) -> Result<String>;
}

pub trait ExportTrait {
    fn name(&self) -> &str;
    fn offset(&self) -> Length;
}

pub trait SectionTrait {
    fn name(&self) -> &str;
    fn virt_addr(&self) -> Address;
    fn virt_size(&self) -> Length;
}

pub trait FindSignatureTrait {
    fn signature(&mut self, pattern: &str) -> Result<Length>;
}

impl<T> FindSignatureTrait for T
where
    T: ModuleTrait + VirtualReadHelper,
{
    fn signature(&mut self, pattern: &str) -> Result<Length> {
        let base = self.base()?;
        let size = self.size()?;

        let buf = self.virt_read(base, size)?;
        let m = pattern.try_match_ida_regex(&buf[..])?;

        Ok(len!(m.0))
    }
}
