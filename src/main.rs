extern crate gl;

extern crate glfw;

mod rgl;

use rgl::{
  RglContext,
  RglWindow,
  RglMesh,
  RglTexture,
  RglShader,
  RglShaderKind,
  RglShaderProgram,
};


#[allow(non_snake_case)]
pub fn main() {    
  let ctx = RglContext::new();
  let mut window = RglWindow::new(ctx, 800, 600, "Rgl");

  window.load_gl_extensions();

  let shaderProgram = {      
    //let vertexShader = RglShader::from_file("shaders/color.vert", RglShaderKind::Vertex).unwrap();
    //let fragmentShader = RglShader::from_file("shaders/color.frag", RglShaderKind::Fragment).unwrap();
    let vertexShader = RglShader::from_file("shaders/textured.vert", RglShaderKind::Vertex).unwrap();
    let fragmentShader = RglShader::from_file("shaders/textured.frag", RglShaderKind::Fragment).unwrap();
    let shaderProgram = RglShaderProgram::link(vertexShader, fragmentShader).unwrap();       

    shaderProgram
  };

  // let triangle = {
  //   let vertices: [f32; 18] = [
  //     -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
  //     0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
  //     0.0,  0.5, 0.0, 0.0, 0.0, 1.0
  //   ];
  //   let triangle = RglMesh::from_vertex_data(&vertices);
  //   triangle
  // };

  let triangle = {
    let texture = RglTexture::from_file("textures/brickwall.jpg");

    let pos: [f32; 9] = [
      -0.5, -0.5, 0.0,
      0.5, -0.5, 0.0,
      0.0,  0.5, 0.0,
    ];

    let col: [f32; 9] = [
      1.0, 0.0, 0.0,
      0.0, 1.0, 0.0,
      0.0, 0.0, 1.0
    ];

    let tex: [f32; 6] = [
      0.0, 1.0, 
      1.0, 1.0,
      0.5, 0.0,
    ];

    let mut triangle = RglMesh::from_pos_col_tex(&pos, &col, &tex, 3);    
    triangle.set_texture(texture);
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
    //shaderProgram.set_uniform_4f("inColor", &[0.0, window.get_time().sin() / 2.0 + 0.5, 0.0, 1.0]);
    shaderProgram.set_uniform_1i("texture1", 0);
    triangle.bind();
    triangle.draw();            
    // glBindVertexArray(0); // no need to unbind it every time

    // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
    // -------------------------------------------------------------------------------
    window.swap_buffers();        
    window.poll_events();
  }
}
