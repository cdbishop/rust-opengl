
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

  fn key_pressed(&mut self, key: glfw::Key) -> bool {
    return self.get_window().key_pressed(key);
  }

  fn pre_update(&mut self) {
    self.get_window().pre_update();
  }

  fn post_update(&mut self) {
    self.get_window().post_update();
  }
}

pub fn run_app<T: RglApplication>(mut app: T) {
  // render loop
  // -----------
  while !app.should_close() {
    // events
    // -----
    app.process_events();

    app.pre_update();
    app.update();
    app.post_update();

    app.clear(0.2, 0.3, 0.3, 1.0, gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    app.draw();

    // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
    // -------------------------------------------------------------------------------
    app.swap_buffers();
    app.poll_events();
  }
}