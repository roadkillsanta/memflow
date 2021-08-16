//! Describes modules

use crate::prelude::v1::*;

/// Module information structure
#[repr(C)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct ModuleInfo {
    /// Returns the address of the module header.
    ///
    /// # Remarks
    ///
    /// On Windows this will be the address where the [`PEB`](https://docs.microsoft.com/en-us/windows/win32/api/winternl/ns-winternl-peb) entry is stored.
    pub address: Address,
    /// The base address of the parent process.
    ///
    /// # Remarks
    ///
    /// This field is analog to the `ProcessInfo::address` field.
    pub parent_process: Address,
    /// The actual base address of this module.
    ///
    /// # Remarks
    ///
    /// The base address is contained in the virtual address range of the process
    /// this module belongs to.
    pub base: Address,
    /// Size of the module
    pub size: umem,
    /// Name of the module
    pub name: ReprCString,
    /// Path of the module
    pub path: ReprCString,
    /// Architecture of the module
    ///
    /// # Remarks
    ///
    /// Emulated processes often have 2 separate lists of modules, one visible to the emulated
    /// context (e.g. all 32-bit modules in a WoW64 process), and the other for all native modules
    /// needed to support the process emulation. This should be equal to either
    /// `ProcessInfo::proc_arch`, or `ProcessInfo::sys_arch` of the parent process.
    pub arch: ArchitectureIdent,
}

pub type ModuleInfoCallback<'a> = OpaqueCallback<'a, ModuleInfo>;

/// Pair of address and architecture used for callbacks
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ModuleAddressInfo {
    pub address: Address,
    pub arch: ArchitectureIdent,
}

pub type ModuleAddressCallback<'a> = OpaqueCallback<'a, ModuleAddressInfo>;

/// Import information structure
#[repr(C)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct ImportInfo {
    /// Name of the import
    pub name: ReprCString,
    /// Offset of this import from the containing modules base address
    pub offset: umem,
}

pub type ImportCallback<'a> = OpaqueCallback<'a, ImportInfo>;

/// Export information structure
#[repr(C)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct ExportInfo {
    /// Name of the export
    pub name: ReprCString,
    /// Offset of this export from the containing modules base address
    pub offset: umem,
}

pub type ExportCallback<'a> = OpaqueCallback<'a, ExportInfo>;

/// Section information structure
#[repr(C)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct SectionInfo {
    /// Name of the section
    pub name: ReprCString,
    /// Virtual address of this section (essentially module_info.base + virtual_address)
    pub base: Address,
    /// Size of this section
    pub size: umem,
}

pub type SectionCallback<'a> = OpaqueCallback<'a, SectionInfo>;
