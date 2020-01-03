use std::{error::Error, mem};
use winapi::shared::windef::LPRECT;
use winapi::um::utilapiset::Beep;

use crate::hooks::{hk_clip_cursor, hk_write_angles, ClipCursorHook, WriteAnglesHook};
use crate::utils::{get_module_symbol_address, patch_game};

type FnClipCursorHook = unsafe extern "system" fn(LPRECT) -> i8;
type FnWriteAngles = unsafe extern "fastcall" fn(u32, u32, u32, u32, u32, u32) -> i8;

pub fn initialize() {
  unsafe {
    match unsafe_initialize() {
      Ok(v) => v,
      Err(_e) => (),
    }
  }
}

unsafe fn unsafe_initialize() -> Result<(), Box<dyn Error>> {
  let address = get_module_symbol_address("user32.dll", "ClipCursor")
    .expect("could not find 'ClipCursor' address");
  let fn_clip_cursor_addr: FnClipCursorHook = mem::transmute(address);

  // TODO: pattern scanning:
  // 83 EC 1C 56 57 FF 74 24 28
  let address = 0x049E130;
  let fn_write_ang_addr: FnWriteAngles = mem::transmute(address);

  patch_game();

  ClipCursorHook
    .initialize(fn_clip_cursor_addr, hk_clip_cursor)?
    .enable()?;

  WriteAnglesHook
    .initialize(fn_write_ang_addr, hk_write_angles)?
    .enable()?;
  
  Beep(500, 180);
  Beep(500, 180);

  Ok(())
}
