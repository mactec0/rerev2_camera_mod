use std::fs;
use std::sync::Mutex;

pub struct Cfg {
  sens: f32,
  cfg_path: String,
}

impl Cfg {
  pub fn new() -> Cfg {
    Cfg {
      sens: 2.5,
      cfg_path: "".to_string(),
    }
  }

  pub fn set_path(&mut self, path: String) {
    self.cfg_path = path;
  }

  pub fn update(&mut self) {
    let result = match fs::read_to_string(&self.cfg_path) {
      Ok(x) => Some(x),
      Err(_e) => return,
    };

    let new_sens = match result.unwrap().trim().parse::<f32>() {
      Ok(x) => x,
      Err(_e) => -1.0,
    };

    if new_sens >= 0.001 && new_sens <= 20.0 {
      self.sens = new_sens;
    }
  }

  pub fn get_sens(&self) -> f32 {
    self.sens
  }
}

lazy_static! {
  pub static ref CONFIG: Mutex<Cfg> = Mutex::new(Cfg::new());
}
