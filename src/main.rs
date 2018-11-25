extern crate gl;

extern crate glfw;

mod lib;

use lib::{
  RglContext,
  RglWindow,
  RglShader,
  RglShaderKind,
  RglShaderProgram,
  RglMesh
};

#[allow(non_snake_case)]
pub fn main() {    
  let ctx = RglContext::new();
  let mut window = RglWindow::new(ctx, 800, 600, "Rgl");

  window.load_gl_extensions();

  let shaderProgram = {      
    let vertexShader = RglShader::from_file("shaders/color.vert", RglShaderKind::Vertex).unwrap();
    let fragmentShader = RglShader::from_file("shaders/color.frag", RglShaderKind::Fragment).unwrap();
    let shaderProgram = RglShaderProgram::link(vertexShader, fragmentShader).unwrap();       

    shaderProgram
  };

  let triangle = {
    let vertices: [f32; 18] = [
      -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
      0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
      0.0,  0.5, 0.0, 0.0, 0.0, 1.0
    ];
    let triangle = RglMesh::from_vertex_data(&vertices);
    triangle
  };

  // render loop
  // -----------
  while !window.should_close() {
    // events
    // -----
    window.process_events();

    
    window.clear(0.2, 0.3, 0.3, 1.0, gl::COLOR_BUFFER_BIT);

    // draw our first triangle            
    shaderProgram.apply();
    shaderProgram.set_uniform("inColor", &[0.0, window.get_time().sin() / 2.0 + 0.5, 0.0, 1.0]);
    triangle.bind();
    triangle.draw();            
    // glBindVertexArray(0); // no need to unbind it every time

    // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
    // -------------------------------------------------------------------------------
    window.swap_buffers();        
    window.poll_events();
  }
}
