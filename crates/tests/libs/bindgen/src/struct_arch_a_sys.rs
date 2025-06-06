#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub type DI_FUNCTION = u32;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_CLASSINSTALL_HEADER {
    pub cbSize: u32,
    pub InstallFunction: DI_FUNCTION,
}
#[repr(C)]
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy, Default)]
pub struct SP_CLASSINSTALL_HEADER {
    pub cbSize: u32,
    pub InstallFunction: DI_FUNCTION,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SP_POWERMESSAGEWAKE_PARAMS_A {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub PowerMessageWake: [i8; 512],
}
impl Default for SP_POWERMESSAGEWAKE_PARAMS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
