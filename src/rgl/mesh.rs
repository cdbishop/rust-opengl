extern crate glfw;

use gl::types::*;

use std::ptr;
use std::mem;
use std::os::raw::c_void;

extern crate image;

use rgl::texture::RglTexture;
use rgl::vertexbuffer::RglVertexBuffer;

///////////////////////////////////////////////////////
/// RglMesh
///////////////////////////////////////////////////////

pub struct RglMesh {
  pub index_buffer: Option<u32>,
  pub texture: Option<RglTexture>,
  pub vertex_buffer: RglVertexBuffer,

  wireframe: bool,
}

impl RglMesh {

  pub fn create_cube() ->RglMesh {
         let vertices: [f32; 288] = [
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 0.0,
          0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 0.0,
          0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 1.0,
          0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 1.0,
        -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 0.0,

        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0, 0.0,
          0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  1.0, 0.0,
          0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0, 1.0,
          0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0, 1.0,
        -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  0.0, 1.0,
        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0, 0.0,

        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0, 0.0,
        -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  1.0, 1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0, 1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0, 1.0,
        -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0, 0.0,
        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0, 0.0,

          0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0, 0.0,
          0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0, 1.0,
          0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0, 1.0,
          0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0, 1.0,
          0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  0.0, 0.0,
          0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0, 0.0,

        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0, 1.0,
          0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0, 1.0,
          0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0, 0.0,
          0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0, 0.0,
        -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  0.0, 0.0,
        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0, 1.0,

        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0, 1.0,
          0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  1.0, 1.0,
          0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0, 0.0,
          0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0, 0.0,
        -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0, 0.0,
        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0, 1.0
      ];

    return RglMesh::from_data(&vertices, &[3, 3, 2])
  }

  pub fn from_pos_col_tex(positions: &[f32], colors: &[f32], texcoords: &[f32], num_vertex: u32) -> RglMesh {
    let mut vertices = Vec::new();
    for x in 0..num_vertex {
      let idx_pos = 3 * x as usize;
      if positions.len() > 0 {
        vertices.extend_from_slice(&positions[idx_pos..idx_pos+3]);
      }
      let idx_col = 3 * x as usize;
      if colors.len() > 0 {
        vertices.extend_from_slice(&colors[idx_col..idx_col+3]);
      }
      let idx_tex = 2 * x as usize;
      if texcoords.len() > 0 {
        vertices.extend_from_slice(&texcoords[idx_tex..idx_tex+2]);
      }
    }

    let pos_elems = 3;
    let col_elems = if colors.len() > 0 {
      3
    } else {
      0
    };

    let tex_elems = if texcoords.len() > 0 {
      2
    } else {
      0
    };


    return RglMesh {index_buffer: None, texture: None,
      vertex_buffer: RglVertexBuffer::from_data_vec(&vertices, &[pos_elems, col_elems, tex_elems]), wireframe: false };
  }

  pub fn from_pos_col_tex_index(positions: &[f32], colors: &[f32], texcoords: &[f32], indices: &[i32], num_vertex: u32) -> RglMesh {
    let mut vertices = Vec::new();
    for x in 0..num_vertex {
      let idx_pos = 3 * x as usize;
      if positions.len() > 0 {
        vertices.extend_from_slice(&positions[idx_pos..idx_pos+3]);
      }
      let idx_col = 3 * x as usize;
      if colors.len() > 0 {
        vertices.extend_from_slice(&colors[idx_col..idx_col+3]);
      }
      let idx_tex = 2 * x as usize;
      if texcoords.len() > 0 {
        vertices.extend_from_slice(&texcoords[idx_tex..idx_tex+2]);
      }
    }

    let mut index_buffer = 0;
    unsafe {
      gl::GenBuffers(1, &mut index_buffer);
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer);
      gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                      (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                      &indices[0] as *const i32 as *const c_void,
                      gl::STATIC_DRAW);
    }

    return RglMesh {index_buffer: Some(index_buffer), texture: None,
      vertex_buffer: RglVertexBuffer::from_data_vec(&vertices, &[3, 3, 2]), wireframe: false };
  }

  pub fn from_data(data: &[f32], parts: &[i32]) -> RglMesh {
    return RglMesh {index_buffer: None, texture: None, vertex_buffer: RglVertexBuffer::from_data(data, parts), wireframe: false };
  }

  pub fn from_data_indexed(data: &[f32], parts: &[i32], indices: &[i32]) -> RglMesh {
    let mut index_buffer = 0;
    unsafe {
      gl::GenBuffers(1, &mut index_buffer);
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer);
      gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                      (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                      &indices[0] as *const i32 as *const c_void,
                      gl::STATIC_DRAW);
    }

    return RglMesh {index_buffer: Some(index_buffer), texture: None, vertex_buffer: RglVertexBuffer::from_data(data, parts), wireframe: false };
  }

  pub fn set_texture(&mut self, texture: RglTexture) {
    self.texture = Some(texture);
  }

  pub fn set_wireframe(&mut self, enable: bool) {
    self.wireframe = enable;
  }

  pub fn bind(&mut self) {
    self.vertex_buffer.bind_array();

    unsafe {
      match &self.index_buffer {
        Some(id) => gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, *id),
        None => {}
      }
    }
  }

  pub fn draw(&self) {
    unsafe {

      gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

      match &self.texture {
        Some(t) => t.bind(),
        None => {}
      }

      match &self.index_buffer {
        Some(_) => gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null()),
        None => gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_buffer.count as i32)
      }

      gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
    }
  }
}