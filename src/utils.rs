use crate::cfg::CONFIG;
use crate::vec2::Vec2;
use std::ptr;
use std::{ffi::CString, iter};
use winapi::shared::minwindef::{DWORD, LPVOID};
use winapi::um::libloaderapi::{GetModuleHandleW, GetProcAddress};
use winapi::um::memoryapi::VirtualProtect;
use winapi::um::winnt::PAGE_EXECUTE_READWRITE;

pub unsafe fn patch_game() {
  let addr1: u32 = 0x49e1a0;
  let addr2: u32 = 0x49E1db;
  let mut old_protect: DWORD = 0;

  VirtualProtect(
    addr1 as LPVOID,
    0x3f,
    PAGE_EXECUTE_READWRITE,
    &mut old_protect as *mut u32,
  );

  ptr::write(addr1 as *mut u32, 0x90909090);
  ptr::write(addr2 as *mut u32, 0x90909090);

  VirtualProtect(addr1 as LPVOID, 0x3f, old_protect, std::ptr::null_mut());
}

pub fn get_module_symbol_address(module: &str, symbol: &str) -> Option<usize> {
  let module = module
    .encode_utf16()
    .chain(iter::once(0))
    .collect::<Vec<u16>>();
  let symbol = CString::new(symbol).unwrap();
  unsafe {
    let handle = GetModuleHandleW(module.as_ptr());
    match GetProcAddress(handle, symbol.as_ptr()) as usize {
      0 => None,
      n => Some(n),
    }
  }
}

pub fn update_camera_angles(delta: &Vec2, view_angles_ptr: u32, delta_time: f32) {
  let sens: f32 = CONFIG.lock().unwrap().get_sens();
  unsafe {
    let mut view_angles = view_angles_ptr as *mut Vec2;
    (*view_angles).y -= delta.x * 0.1 * delta_time * sens;
    (*view_angles).x -= delta.y * 0.1 * delta_time * sens;
  }
}
