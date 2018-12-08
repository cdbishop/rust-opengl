
use cgmath::{Matrix4, Vector3, Point3};
use cgmath::prelude::*;

pub struct RglCamera {
  front: Vector3<f32>,
  up: Vector3<f32>,
  pub pos: Point3<f32>,
  yaw: f32,
  pitch: f32,
}

impl RglCamera {
  pub fn new(pos: Point3<f32>) -> RglCamera {
    return RglCamera { front: Vector3 { x: 0.0, y: 0.0, z: -1.0}, 
                up: Vector3 { x: 0.0, y: 1.0, z: 0.0},
                pos: pos,
                yaw: -90.0, pitch: 0.0 };
  }

  pub fn get_view(&self) -> Matrix4<f32> {
    return Matrix4::look_at(self.pos, self.pos + self.front, self.up);
  }

  pub fn move_forward(&mut self, amount: f32) {
    self.pos += amount * self.front;
  }

  pub fn strafe(&mut self, amount: f32) {
    self.pos += self.front.cross(self.up).normalize() * amount;
  }

  pub fn rotate_yaw(&mut self, amount: f32) {
    self.yaw += amount;
    self.front = Vector3 {
      x: self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
      y: self.pitch.to_radians().sin(),
      z: self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
    };
  }

  pub fn rotate_pitch(&mut self, amount: f32) {
    self.pitch += amount;

    if self.pitch > 89.0 {
      self.pitch = 89.0;
    }

    if self.pitch < -89.0 {
      self.pitch = -89.0;
    }

    self.front = Vector3 {
      x: self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
      y: self.pitch.to_radians().sin(),
      z: self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
    };
  }
}