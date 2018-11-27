
use rgl::core::RglWindow;

pub trait RglApplication {
  fn init() -> Self;
  fn update(&mut self);
  fn draw(&mut self);

  fn get_window(&mut self) -> &mut RglWindow;

  fn should_close(&mut self) -> bool {
    return self.get_window().should_close();
  }

  fn process_events(&mut self) {
    self.get_window().process_events();
  }

  fn clear(&mut self, r: f32, g: f32, b: f32, a: f32, buffer_type: gl::types::GLenum) {
    self.get_window().clear(r, g, b, a, buffer_type);
  }

  fn swap_buffers(&mut self) {
    self.get_window().swap_buffers();
  }

  fn poll_events(&mut self) {
    self.get_window().poll_events();
  }
}