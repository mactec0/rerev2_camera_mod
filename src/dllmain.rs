#![cfg(all(windows, feature = "nightly"))]
#[cfg(feature = "nightly")]
mod cfg;
mod hooks;
mod init;
mod utils;
mod vec2;

pub use cfg::CONFIG;
use init::initialize;

#[macro_use]
extern crate lazy_static;


use std::char;
use winapi::shared::minwindef::MAX_PATH;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};
use winapi::um::libloaderapi::GetModuleFileNameA;
use winapi::um::winnt::DLL_PROCESS_ATTACH;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DllMain(
  module: HINSTANCE,
  fdwReason: DWORD,
  _reserved: LPVOID,
) -> BOOL {
  if fdwReason == DLL_PROCESS_ATTACH {
    let mut str_path: String = "".to_string();
    let path: [i8; MAX_PATH] = [0; MAX_PATH];

    if GetModuleFileNameA(module, path.as_ptr() as *mut i8, MAX_PATH as u32) != 0 {
      for &x in path.iter() {
        if x == 0 {
          break;
        } else {
          str_path.push(x as u8 as char);
        }
      }

      if str_path.len() > 0 {
        match str_path.rsplitn(2, '\\').last() {
          Some(x) => {
            let cfg_path: String = x.to_string() + "\\sensitivity.txt";
            CONFIG.lock().unwrap().set_path(cfg_path);
            CONFIG.lock().unwrap().update();
          },
          None => (),
        }
      }
    }

    std::thread::spawn(initialize);
  }
  TRUE
}
