
use super::super::rgl::{
  RglContext,
  RglWindow,
  RglMesh,
  RglTexture,
  RglShader,
  RglShaderKind,
  RglShaderProgram,
  RglApplication,
};

use cgmath::{Matrix4, vec3, Rad, perspective, Deg};
use cgmath::prelude::*;

use std::ops::Mul;

pub struct Cube {
  pub window: RglWindow,
  pub shader_program: RglShaderProgram,
  pub quad: RglMesh,
}

impl RglApplication for Cube {
  fn init() -> Cube {
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

      let pos: [f32; 108] = [
        -0.5, -0.5, -0.5,
        0.5, -0.5, -0.5,
        0.5,  0.5, -0.5,
        0.5,  0.5, -0.5,
        -0.5,  0.5, -0.5,
        -0.5, -0.5, -0.5,

        -0.5, -0.5,  0.5,
        0.5, -0.5,  0.5,
        0.5,  0.5,  0.5,
        0.5,  0.5,  0.5,
        -0.5,  0.5,  0.5,
        -0.5, -0.5,  0.5,

        -0.5,  0.5,  0.5,
        -0.5,  0.5, -0.5,
        -0.5, -0.5, -0.5,
        -0.5, -0.5, -0.5,
        -0.5, -0.5,  0.5,
        -0.5,  0.5,  0.5,

        0.5,  0.5,  0.5,
        0.5,  0.5, -0.5,
        0.5, -0.5, -0.5,
        0.5, -0.5, -0.5,
        0.5, -0.5,  0.5,
        0.5,  0.5,  0.5,

        -0.5, -0.5, -0.5,
        0.5, -0.5, -0.5,
        0.5, -0.5,  0.5,
        0.5, -0.5,  0.5,
        -0.5, -0.5,  0.5,
        -0.5, -0.5, -0.5,

        -0.5,  0.5, -0.5,
        0.5,  0.5, -0.5,
        0.5,  0.5,  0.5,
        0.5,  0.5,  0.5,
        -0.5,  0.5,  0.5,
        -0.5,  0.5, -0.5
      ];

      let tex: [f32; 72] = [
        0.0, 0.0,
        1.0, 0.0,
        1.0, 1.0,
        1.0, 1.0,
        0.0, 1.0,
        0.0, 0.0,

        0.0, 0.0,
        1.0, 0.0,
        1.0, 1.0,
        1.0, 1.0,
        0.0, 1.0,
        0.0, 0.0,

        1.0, 0.0,
        1.0, 1.0,
        0.0, 1.0,
        0.0, 1.0,
        0.0, 0.0,
        1.0, 0.0,

        1.0, 0.0,
        1.0, 1.0,
        0.0, 1.0,
        0.0, 1.0,
        0.0, 0.0,
        1.0, 0.0,

        0.0, 1.0,
        1.0, 1.0,
        1.0, 0.0,
        1.0, 0.0,
        0.0, 0.0,
        0.0, 1.0,

        0.0, 1.0,
        1.0, 1.0,
        1.0, 0.0,
        1.0, 0.0,
        0.0, 0.0,
        0.0, 1.0
      ];

      let mut quad = RglMesh::from_pos_col_tex(&pos, &[], &tex, 36);
      quad.set_texture(texture);
      quad
    };

    Cube { window, shader_program, quad }
  }

  fn update(&mut self) {
    
  }

  fn draw(&mut self) {
    self.shader_program.apply();
    self.shader_program.set_uniform_1i("texture1", 0);

     let model: Matrix4<f32> = Matrix4::from_axis_angle(vec3(0.5, 1.0, 0.0).normalize(),
                                                                Rad(self.window.get_time() as f32));
    let view: Matrix4<f32> = Matrix4::from_translation(vec3(0., 0., -3.));
    let projection: Matrix4<f32> = perspective(Deg(45.0), 800 as f32 / 600 as f32, 0.1, 100.0);
    let transformation = projection.mul(view.mul(model));

    self.shader_program.set_uniform_4fv("transform", &transformation);

    self.quad.bind();
    self.quad.draw(); 

  }

  fn get_window(&mut self) -> &mut RglWindow {
    return &mut self.window;
  }
}