pub mod network;

use std::process;

use nix;

use super::errors::Result;

pub fn hostname() -> Result<String> {
    let mut buf = [0u8; 64];
    let it = nix::unistd::gethostname(&mut buf)?.to_str()?;
    Ok(it.to_string())
}

pub fn uts_name() -> nix::sys::utsname::UtsName {
    nix::sys::utsname::uname()
}

pub fn sys_info() -> Result<nix::sys::sysinfo::SysInfo> {
    let it = nix::sys::sysinfo::sysinfo()?;
    Ok(it)
}

pub fn pid() -> u32 {
    process::id()
}
