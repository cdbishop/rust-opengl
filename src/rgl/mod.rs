#![feature(macro_rules)]

#[macro_use]
pub mod error {
  pub use glCheckError_;
}

pub mod core;
pub mod mesh;
pub mod shader;
pub mod texture;
pub mod application;
pub mod camera;
pub mod mouse;
pub mod vertexbuffer;

pub use self::core::{
  RglContext,
  RglWindow,
};

pub use self::mesh::{
  RglMesh,
};

pub use self::texture::{
  RglTexture,
};

pub use self::shader::{
  RglShader,
  RglShaderKind,
  RglShaderProgram,
};

pub use self::application::{
  RglApplication,
};

pub use self::camera::{
  RglCamera,
};

pub use self::mouse::{
  RglMouse,
};

pub use self::vertexbuffer::{
  RglVertexBuffer,
};