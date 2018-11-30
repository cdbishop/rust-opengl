
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

use cgmath::{Matrix4, vec3, Rad, perspective, Deg, Point3};
use cgmath::prelude::*;

use std::ops::Mul;

pub struct Camera {
  pub window: RglWindow,
  pub shader_program: RglShaderProgram,
  pub cube: RglMesh,
}

impl RglApplication for Camera {
  fn init() -> Camera {
    let ctx = RglContext::new();
    let mut window = RglWindow::new(ctx, 800, 600, "Transformation");
    window.load_gl_extensions();

    let shader_program = {      
      let vertex_shader = RglShader::from_file("shaders/transformation.vert", RglShaderKind::Vertex).unwrap();
      let fragment_shader = RglShader::from_file("shaders/transformation.frag", RglShaderKind::Fragment).unwrap();    
      let shader_program = RglShaderProgram::link(vertex_shader, fragment_shader).unwrap();       

      shader_program
    };

    let cube = {
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

      let mut cube = RglMesh::from_pos_col_tex(&pos, &[], &tex, 36);
      cube.set_texture(texture);
      cube
    };

    Camera { window, shader_program, cube }
  }

  fn update(&mut self) {
    
  }

  fn draw(&mut self) {
    self.shader_program.apply();
    self.shader_program.set_uniform_1i("texture1", 0);

    //let model: Matrix4<f32> = Matrix4::from_axis_angle(vec3(0.5, 1.0, 0.0).normalize(),
    //                                                         Rad(self.window.get_time() as f32));
    
    let model: Matrix4<f32> = Matrix4::from_axis_angle(vec3(0.5, 1.0, 0.0).normalize(), Rad(0.0));

    // camera code
    let radius: f32 = 10.0;
    let camX = self.window.get_time().sin() as f32 * radius;
    let camZ = self.window.get_time().cos() as f32 * radius;
    let view: Matrix4<f32> =
      Matrix4::look_at(Point3::new(camX, 0.0, camZ), Point3::new(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0));

    let projection: Matrix4<f32> = perspective(Deg(45.0), 800 as f32 / 600 as f32, 0.1, 100.0);
    let transformation = projection.mul(view.mul(model));

    self.shader_program.set_uniform_4fv("transform", &transformation);

    self.cube.bind();
    self.cube.draw(); 

  }

  fn get_window(&mut self) -> &mut RglWindow {
    return &mut self.window;
  }
}