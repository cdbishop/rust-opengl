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

  pub vertex_buffer: u32,
  pub num_poly: u32,
}

impl RglMesh {
  pub fn from_pos_col_tex(positions: &[f32], colors: &[f32], texcoords: &[f32], num_vertex: u32) -> RglMesh {
    let (mut vertex_buffer, mut vertex_array) = (0, 0);

    unsafe {
      gl::GenVertexArrays(1, &mut vertex_array);
      gl::GenBuffers(1, &mut vertex_buffer);
      // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
      gl::BindVertexArray(vertex_array);

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

      gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
      gl::BufferData(gl::ARRAY_BUFFER,
                      (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                      &vertices[0] as *const f32 as *const c_void,
                      gl::STATIC_DRAW);

      // TODO(cb): store vertex elements separately then build up at runtime the vertex attrib
      // based on which values exist (e.g. texCoords, normals etc)
      let mut stride = 3 * mem::size_of::<GLfloat>() as GLsizei;
      
      if colors.len() > 0 {
        stride = stride + 3 * mem::size_of::<GLfloat>() as GLsizei;
      }

      if texcoords.len() > 0 {
        stride = stride + 2 * mem::size_of::<GLfloat>() as GLsizei;
      }

      // x, y, z position data
      gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
      gl::EnableVertexAttribArray(0);

      let mut offset = 3;

      if colors.len() > 0 {
        // r, g, b, color data
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (offset * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1);
        offset = offset + 3;
      }

      if texcoords.len() > 0 {
        // u, v
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (offset * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(2);
      }

      // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
      gl::BindBuffer(gl::ARRAY_BUFFER, 0);

      // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
      // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
      gl::BindVertexArray(0);

      // uncomment this call to draw in wireframe polygons.
      // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }   

    return RglMesh {vertex_buffer: vertex_array, index_buffer: None, texture: None, num_poly: num_vertex};
  }

  pub fn from_pos_col_tex_index(positions: &[f32], colors: &[f32], texcoords: &[f32], indices: &[i32], num_vertex: u32) -> RglMesh {
    let (mut vertex_buffer, mut vertex_array) = (0, 0);
    let mut index_buffer = 0;

    unsafe {
      gl::GenVertexArrays(1, &mut vertex_array);
      gl::GenBuffers(1, &mut vertex_buffer);
      // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
      gl::BindVertexArray(vertex_array);

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

      gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
      gl::BufferData(gl::ARRAY_BUFFER,
                      (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                      &vertices[0] as *const f32 as *const c_void,
                      gl::STATIC_DRAW);

      gl::GenBuffers(1, &mut index_buffer);
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer);
      gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                      (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                      &indices[0] as *const i32 as *const c_void,
                      gl::STATIC_DRAW);

      // TODO(cb): store vertex elements separately then build up at runtime the vertex attrib
      // based on which values exist (e.g. texCoords, normals etc)
      let mut stride = 3 * mem::size_of::<GLfloat>() as GLsizei;
      
      if colors.len() > 0 {
        stride = stride + 3 * mem::size_of::<GLfloat>() as GLsizei;
      }

      if texcoords.len() > 0 {
        stride = stride + 2 * mem::size_of::<GLfloat>() as GLsizei;
      }

      // x, y, z position data
      gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
      gl::EnableVertexAttribArray(0);

      let mut offset = 3;

      if colors.len() > 0 {
        // r, g, b, color data
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (offset * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1);
        offset = offset + 3;
      }

      if texcoords.len() > 0 {
        // u, v
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (offset * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(2);
      }

      // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
      gl::BindBuffer(gl::ARRAY_BUFFER, 0);

      // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
      // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
      gl::BindVertexArray(0);

      // uncomment this call to draw in wireframe polygons.
      // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }   

    return RglMesh {vertex_buffer: vertex_array, index_buffer: Some(index_buffer), texture: None, num_poly: num_vertex};
  }  

  pub fn set_texture(&mut self, texture: RglTexture) {
    self.texture = Some(texture);
  }

  pub fn bind(&mut self) {
    unsafe {
      gl::BindVertexArray(self.vertex_buffer);
    }
  }

  pub fn draw(&self) {
    unsafe {

      match &self.texture {
        Some(t) => t.bind(),
        None => {}
      }

      match &self.index_buffer {
        Some(_) => gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null()),
        None => gl::DrawArrays(gl::TRIANGLES, 0, self.num_poly as i32)
      }
    }
  }
}