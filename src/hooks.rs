use crate::cfg::CONFIG;
use crate::utils::update_camera_angles;
use crate::vec2::Vec2;
use detour::static_detour;
use std::sync::Mutex;
use std::time::Instant;
use winapi::um::utilapiset::Beep;
use winapi::shared::windef::{LPRECT, POINT};
use winapi::um::winuser::{GetAsyncKeyState, GetCursorPos, SetCursorPos};

static_detour! {
    pub static ClipCursorHook: unsafe extern "system" fn(LPRECT ) -> i8;
    pub static WriteAnglesHook: unsafe extern "fastcall" fn(u32, u32, u32, u32, u32, u32) -> i8;
}

lazy_static! {
  pub static ref HK_MTX: Mutex<u8> = (Mutex::new(0));
}

struct GlobalVars {
  last_cur_pos: POINT,
  screen_center: POINT,
  view_angles_ptr: u32,
  timer: Option<Instant>,
}

static mut GLOBALS: GlobalVars = GlobalVars {
  last_cur_pos: POINT { x: 0, y: 0 },
  screen_center: POINT { x: 0, y: 0 },
  view_angles_ptr: 0,
  timer: None,
};

pub fn hk_write_angles(ecx: u32, edx: u32, a1: u32, a2: u32, view_angles_ptr: u32, a4: u32) -> i8 {
  let _mtx = HK_MTX.lock();
  static mut FIRST_RUN: bool = true;
  let can_set_ang: bool;

  unsafe {
    if FIRST_RUN == true {
      let mut start_point = POINT { x: 0, y: 0 };
      GetCursorPos(&mut start_point as *mut POINT);
      GLOBALS.timer = Some(Instant::now());
      GLOBALS.last_cur_pos = start_point;
      FIRST_RUN = false;
    }

    let try_set: bool = *((ecx + 0x2200) as *const u8) & 3 != 0;
    let fx = f32::from_bits(*((ecx + 0x21F0) as *const u32) & 0x7FFFFFF);
    let fy = f32::from_bits(*((ecx + 0x21F4) as *const u32) & 0x7FFFFFF);
    if try_set && (fy.is_normal() && fy <= 0.0001) && (fx.is_normal() && fx <= 0.0001) {
      GLOBALS.view_angles_ptr = view_angles_ptr;
    }

    can_set_ang = try_set && GLOBALS.view_angles_ptr != 0;
  }

  if can_set_ang && unsafe { GLOBALS.timer.unwrap().elapsed().as_secs_f32() } >= 0.005 {
    let mut m_pos = POINT { x: 0, y: 0 };
    let delta_time: f32;
    let delta_pos: Vec2;
    unsafe {
      GetCursorPos(&mut m_pos as *mut POINT);

      delta_time = GLOBALS.timer.unwrap().elapsed().as_secs_f32();
      GLOBALS.timer = Some(Instant::now());

      delta_pos = Vec2 {
        x: (m_pos.x - GLOBALS.last_cur_pos.x) as f32,
        y: (m_pos.y - GLOBALS.last_cur_pos.y) as f32,
      };

      if GLOBALS.screen_center.x != 0 || GLOBALS.screen_center.y != 0 {
        SetCursorPos(
          GLOBALS.screen_center.x as i32,
          GLOBALS.screen_center.y as i32,
        );
        GLOBALS.last_cur_pos = GLOBALS.screen_center;
      } else {
        GLOBALS.last_cur_pos = m_pos;
      }
    }

    if delta_time < 0.1 {
      update_camera_angles(&delta_pos, unsafe { GLOBALS.view_angles_ptr }, delta_time);
    }
  }

  unsafe { WriteAnglesHook.call(ecx, edx, a1, a2, view_angles_ptr, a4) }
}

pub fn hk_clip_cursor(lp_rect: LPRECT) -> i8 {
  unsafe {
    if lp_rect != std::ptr::null_mut() {
      GLOBALS.screen_center = POINT {
        x: ((*lp_rect).left + (*lp_rect).right) / 2,
        y: ((*lp_rect).bottom + (*lp_rect).top) / 2,
      };
    }

    if GetAsyncKeyState(0x74) != 0 {
      CONFIG.lock().unwrap().update();
      Beep(512, 400);
      Beep(600, 350);
      Beep(750, 300);
    }

    ClipCursorHook.call(lp_rect)
  }
}
