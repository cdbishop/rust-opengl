use std::ptr;
use std::mem;
use std::os::raw::c_void;

use rgl::error::gl_check_error_;

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

  pub fn from_data(data: &[f32], part_count: &[i32]) -> RglVertexBuffer {
    let (mut vertex_buffer, mut vertex_array) = (0, 0);
    let num_floats = part_count.iter().fold(0, |a, &b| a + b);
    let stride = (num_floats as usize * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizei;
    let num_vertices = data.len() as u32 / num_floats as u32;

    unsafe {
      gl::GenVertexArrays(1, &mut vertex_array);
      gl::GenBuffers(1, &mut vertex_buffer);        
      // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
      gl::BindVertexArray(vertex_array);
      gl_check_error_(file!(), line!());
      
      gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
      gl::BufferData(gl::ARRAY_BUFFER,
                      (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                      &data[0] as *const f32 as *const c_void,
                      gl::STATIC_DRAW);

      gl_check_error_(file!(), line!());

      let mut vertex_attrib = 0;
      let mut offset = 0usize;
      let mut offset_ptr = ptr::null();
      for x in part_count {
        if *x == 0 {
          continue;
        }

        gl::VertexAttribPointer(vertex_attrib, *x, gl::FLOAT, gl::FALSE, stride, offset_ptr);
        gl::EnableVertexAttribArray(vertex_attrib);

        vertex_attrib += 1;
        offset = offset + (*x as usize);
        offset_ptr = (offset * mem::size_of::<gl::types::GLfloat>()) as *const c_void
      }

      gl_check_error_(file!(), line!());

      gl::BindBuffer(gl::ARRAY_BUFFER, 0);
      gl_check_error_(file!(), line!());

      gl::BindVertexArray(0);
      gl_check_error_(file!(), line!());

    }

    RglVertexBuffer { array: vertex_array, buffer: vertex_buffer, count: num_vertices }
  }

  pub fn from_data_vec(data: &Vec<f32>, part_count: &[i32]) -> RglVertexBuffer {
    let (mut vertex_buffer, mut vertex_array) = (0, 0);
    let num_floats = part_count.iter().fold(0, |a, &b| a + b);
    let stride = (num_floats as usize * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizei;
    let num_vertices = data.len() as u32 / num_floats as u32;

    unsafe {
      gl::GenVertexArrays(1, &mut vertex_array);
      gl::GenBuffers(1, &mut vertex_buffer);        
      // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
      gl::BindVertexArray(vertex_array);
      gl_check_error_(file!(), line!());
      
      gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
      gl::BufferData(gl::ARRAY_BUFFER,
                      (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                      &data[0] as *const f32 as *const c_void,
                      gl::STATIC_DRAW);

      gl_check_error_(file!(), line!());

      let mut vertex_attrib = 0;
      let mut offset = 0usize;
      let mut offset_ptr = ptr::null();
      for x in part_count {
        if *x == 0 {
          continue;
        }

        gl::VertexAttribPointer(vertex_attrib, *x, gl::FLOAT, gl::FALSE, stride, offset_ptr);
        gl::EnableVertexAttribArray(vertex_attrib);

        vertex_attrib += 1;
        offset = offset + (*x as usize);
        offset_ptr = (offset * mem::size_of::<gl::types::GLfloat>()) as *const c_void
      }

      gl_check_error_(file!(), line!());

      gl::BindBuffer(gl::ARRAY_BUFFER, 0);
      gl_check_error_(file!(), line!());

      gl::BindVertexArray(0);
      gl_check_error_(file!(), line!());

    }

    RglVertexBuffer { array: vertex_array, buffer: vertex_buffer, count: num_vertices }
  }
}