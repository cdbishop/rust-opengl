extern crate glfw;

use std::ptr;
use std::mem;
use std::os::raw::c_void;

pub struct RglVertexBuffer {
  array: gl::types::GLuint,
  buffer: gl::types::GLuint,
  pub count: u32,
}

impl RglVertexBuffer {  
  pub fn bind_array(&mut self) {
    unsafe {
      gl::BindVertexArray(self.array);
    }
  }

  pub fn from_data(data: &[f32], part_count: &[u8]) -> RglVertexBuffer {
    let (mut vertex_buffer, mut vertex_array) = (0, 0);
    let stride = part_count.iter().fold(0, |a, &b| a+ b) as i32;
    let num_vertices = data.len() as u32 / stride as u32;

    unsafe {
      gl::GenVertexArrays(1, &mut vertex_array);
      gl::GenBuffers(1, &mut vertex_buffer);      

      let mut attribute_count = 0;
      let mut offset :u8 = 0; 
      for x in part_count {
        gl::VertexAttribPointer(attribute_count, *x as i32, gl::FLOAT, gl::FALSE, stride,
          (offset as usize * mem::size_of::<gl::types::GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(attribute_count);
        attribute_count += 1;
        offset += *x;
      }

      gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
      gl::BufferData(gl::ARRAY_BUFFER,
                      (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                      &data[0] as *const f32 as *const c_void,
                      gl::STATIC_DRAW);

      gl::BindBuffer(gl::ARRAY_BUFFER, 0);
      gl::BindVertexArray(0);
    }

    RglVertexBuffer { array: vertex_array, buffer: vertex_buffer, count: num_vertices }
  }

  pub fn from_data_vec(data: &Vec<f32>, part_count: &[u8]) -> RglVertexBuffer {
    let (mut vertex_buffer, mut vertex_array) = (0, 0);
    let stride = part_count.iter().fold(0, |a, &b| a+ b) as usize * mem::size_of::<gl::types::GLfloat>();
    let num_vertices = data.len() as u32 / stride as u32;

    unsafe {
      gl::GenVertexArrays(1, &mut vertex_array);
      gl::GenBuffers(1, &mut vertex_buffer);      

      let mut attribute_count = 0;
      let mut offset :u8 = 0; 
      for x in part_count {
        if *x == 0 {
          continue;
        }
        gl::VertexAttribPointer(attribute_count, *x as i32, gl::FLOAT, gl::FALSE, stride as i32,
          (offset as usize * mem::size_of::<gl::types::GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(attribute_count);
        attribute_count += 1;
        offset += *x;
      }

      gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
      gl::BufferData(gl::ARRAY_BUFFER,
                      (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                      &data[0] as *const f32 as *const c_void,
                      gl::STATIC_DRAW);

      gl::BindBuffer(gl::ARRAY_BUFFER, 0);
      gl::BindVertexArray(0);
    }

    RglVertexBuffer { array: vertex_array, buffer: vertex_buffer, count: num_vertices }
  }
}