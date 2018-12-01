pub struct RglMouse {
  pub abs_x: f32,
  pub abs_y: f32,
  last_x: f32,
  last_y: f32,
  
  first_update: bool,
}

impl RglMouse {
  pub fn new() -> RglMouse {
    return RglMouse { abs_x: 0.0, abs_y: 0.0, last_x: 0.0, last_y: 0.0, first_update: true };
  }

  pub fn delta_x(&self) -> f32 {
    return self.abs_x - self.last_x;
  }

  pub fn delta_y(&self) -> f32 {
    return self.abs_y - self.last_y;
  }

  pub fn update(&mut self, x : f32, y : f32) {    
    self.abs_x = x;
    self.abs_y = y;

    // to prevent large mouse delta on first frame, first event is treated as prev
    if self.first_update {
      self.post_update();
      self.first_update = false
    }
  }

  pub fn post_update(&mut self) {
    self.last_x = self.abs_x;
    self.last_y = self.abs_y;
  }
}