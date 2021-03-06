extern crate rgl;
extern crate cgmath;
extern crate glfw;

use rgl::{
  RglContext,
  RglWindow,
  RglMesh,
  RglTexture,
  RglShader,
  RglShaderKind,
  RglShaderProgram,
  RglApplication,
  run_app
};

use cgmath::{Matrix4, vec3, Rad};
use cgmath::prelude::*;

pub struct Transformations {
  pub window: RglWindow,
  pub shader_program: RglShaderProgram,
  pub quad: RglMesh,
}

impl RglApplication for Transformations {
  fn init() -> Transformations {
    let ctx = RglContext::new();
    let mut window = RglWindow::new(ctx, 800, 600, "Transformation");
    window.load_gl_extensions();

    let shader_program = {      
      let vertex_shader = RglShader::from_file("shaders/transformation.vert", RglShaderKind::Vertex).unwrap();
      let fragment_shader = RglShader::from_file("shaders/transformation.frag", RglShaderKind::Fragment).unwrap();    
      let shader_program = RglShaderProgram::link(vertex_shader, fragment_shader).unwrap();       

      shader_program
    };

    let quad = {
      let texture = RglTexture::from_file("textures/brickwall.jpg");

      let pos: [f32; 12] = [
        0.5, 0.5, 0.0,    // top-right
        0.5, -0.5, 0.0,   // bottom-right
        -0.5,  -0.5, 0.0, // bottom-left
        -0.5,  0.5, 0.0,  // top-left
      ];

      let col: [f32; 12] = [
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
        1.0, 1.0, 1.0
      ];

      let tex: [f32; 8] = [
        1.0, 0.0, 
        1.0, 1.0,
        0.0, 1.0,
        0.0, 0.0,
      ];

      let indices = [
        0, 1, 3,
        1, 2, 3
      ];

      let mut quad = RglMesh::from_pos_col_tex_index(&pos, &col, &tex, &indices, 4);    
      quad.set_texture(texture);
      quad
    };

    Transformations { window, shader_program, quad }
  }

  fn update(&mut self) {
    
  }

  fn draw(&mut self) {
    self.shader_program.apply();
    self.shader_program.set_uniform_1i("texture1", 0);

    let mut transform: Matrix4<f32> = Matrix4::identity();
    transform = transform * Matrix4::<f32>::from_translation(vec3(0.5, -0.5, 0.0));
    transform = transform * Matrix4::<f32>::from_angle_z(Rad(self.window.get_time() as f32));

    self.shader_program.set_uniform_4fv("transform", &transform);

    self.quad.bind();
    self.quad.draw(); 

  }

  fn get_window(&mut self) -> &mut RglWindow {
    return &mut self.window;
  }
}

pub fn main() {
  let app = Transformations::init();
  run_app(app);
}