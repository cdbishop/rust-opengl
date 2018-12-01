extern crate gl;

extern crate glfw;
extern crate cgmath;

mod rgl;
mod examples;

use rgl::{
  RglApplication,
};

// use examples::ColouredTriangle::ColouredTriangle;
// use examples::TexturedTriangle::TexturedTriangle;
// use examples::TexturedQuad::TexturedQuad;
// use examples::Transformations::Transformations;
use examples::cube::Cube;
use examples::camera::Camera;

pub fn main() {
  //colored_triangle_app();
  //textured_triangle_app();

  //let app = ColouredTriangle::init();
  //let app = TexturedTriangle::init();
  //let app = TexturedQuad::init();
  //let app = Transformations::init();
  let app = Camera::init();
  run_app(app);
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