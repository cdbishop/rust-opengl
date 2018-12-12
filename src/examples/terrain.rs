extern crate glfw;

use super::super::rgl::{
  RglContext,
  RglWindow,
  RglMesh,
  RglTexture,
  RglShader,
  RglShaderKind,
  RglShaderProgram,
  RglApplication,
  RglCamera,
  RglTerrain,
};

use self::glfw::{Key};

use cgmath::{Matrix4, vec3, Rad, perspective, Deg, Point3};
use cgmath::prelude::*;

use std::ops::Mul;

pub struct Terrain {
  pub window: RglWindow,
  pub shader_program: RglShaderProgram,
  pub terrain: RglTerrain,
  pub cam: RglCamera,
}

impl RglApplication for Terrain {
  fn init() -> Terrain {
    let ctx = RglContext::new();
    let mut window = RglWindow::new(ctx, 800, 600, "Terrain");
    window.load_gl_extensions();

    let shader_program = {
      let vertex_shader = RglShader::from_file("shaders/lighting.vert", RglShaderKind::Vertex).unwrap();
      let fragment_shader = RglShader::from_file("shaders/lighting.frag", RglShaderKind::Fragment).unwrap();
      let shader_program = RglShaderProgram::link(vertex_shader, fragment_shader).unwrap();

      shader_program
    };

    let terrain = RglTerrain::new(1, 1);

    Terrain { window, shader_program, terrain, cam: RglCamera::new(Point3::new(0.0, 0.0, 3.0)) }
  }

  fn update(&mut self) {
    let dt = self.get_window().dt();
    if self.key_pressed(Key::W) {
      self.cam.move_forward(2.5 * dt);
    } else if self.key_pressed(Key::S) {
      self.cam.move_forward(-2.5 * dt);
    }

    if self.key_pressed(Key::A) {
      self.cam.strafe(-2.5 * dt);
    } else if self.key_pressed(Key::D) {
      self.cam.strafe(2.5 * dt);
    }

    self.cam.rotate_yaw(self.window.mouse.delta_x() * 0.1);
    self.cam.rotate_pitch(-self.window.mouse.delta_y() * 0.1);
  }

  fn draw(&mut self) {
    self.shader_program.apply();
    self.shader_program.set_uniform_1i("texture", 0);

    let model: Matrix4<f32> = Matrix4::identity();
    // let model: Matrix4<f32> = Matrix4::from_axis_angle(vec3(0.0, 1.0, 0.0).normalize(),
    //                                                          Rad(self.window.get_time() as f32));

    let view = self.cam.get_view();
    let projection: Matrix4<f32> = perspective(Deg(45.0), 800 as f32 / 600 as f32, 0.1, 100.0);
    let transformation = projection.mul(view.mul(model));

    self.shader_program.set_uniform_1f("ambientStrength", 0.1);
    self.shader_program.set_uniform_1f("specularStrength", 0.5);
    self.shader_program.set_uniform_4fv("transform", &transformation);
    self.shader_program.set_uniform_4fv("model", &model);
    self.shader_program.set_uniform_3f("lightColor", &[1.0, 1.0, 1.0]);
    self.shader_program.set_uniform_3f("lightPos", &[0.0, 0.0, 3.0]);
    self.shader_program.set_uniform_3f("viewPos", self.cam.pos.as_ref());

    self.terrain.draw();
  }

  fn get_window(&mut self) -> &mut RglWindow {
    return &mut self.window;
  }
}