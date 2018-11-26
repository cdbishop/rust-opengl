extern crate glfw;
use self::glfw::{Context, Key, Action};
use std::sync::mpsc::Receiver;

use std::ffi::CString;
use gl::types::*;

use std::ptr;
use std::str;
use std::fs;
use std::path::Path;
use std::mem;
use std::os::raw::c_void;
use std::vec;

extern crate image;
use self::image::GenericImage;

///////////////////////////////////////////////////////
/// RglContext 
///////////////////////////////////////////////////////

pub struct RglContext {
  pub ctx: glfw::Glfw,  
}

impl RglContext {
  pub fn new() -> RglContext {

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let out = RglContext {
      ctx: glfw,
    };

    out
  }

  pub fn poll_events(&mut self) {
    self.ctx.poll_events();
  }
}

///////////////////////////////////////////////////////
/// RglWindow 
///////////////////////////////////////////////////////

pub struct RglWindow {
  pub window: glfw::Window,
  pub events: Receiver<(f64, glfw::WindowEvent)>,
  pub ctx: RglContext
}

impl RglWindow {
  pub fn new(ctx: RglContext, width: u32, height: u32, title: &str) -> RglWindow {
    let (mut window, events) = ctx.ctx.create_window(width, height, title, glfw::WindowMode::Windowed)
      .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    RglWindow {
      window: window,
      events: events,
      ctx: ctx,
    }
  }

  pub fn load_gl_extensions(&mut self) {
    gl::load_with(|symbol| self.window.get_proc_address(symbol) as *const _);

  }

  pub fn should_close(&self) -> bool {
    return self.window.should_close();
  }

  pub fn process_events(&mut self) {
    for (_, event) in glfw::flush_messages(&self.events) {
      match event {
        glfw::WindowEvent::FramebufferSize(width, height) => {
            // make sure the viewport matches the new window dimensions; note that width and
            // height will be significantly larger than specified on retina displays.
            unsafe { gl::Viewport(0, 0, width, height) }
        }
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => self.window.set_should_close(true),
        _ => {}
      }
    }
  }

  pub fn clear(&mut self, r: f32, g: f32, b: f32, a: f32, buffer_type: gl::types::GLenum) {
    unsafe {
      gl::ClearColor(r, g, b, a);
      gl::Clear(buffer_type);
    }
  }

  pub fn swap_buffers(&mut self) {
    self.window.swap_buffers();
  }

  pub fn poll_events(&mut self) {
    self.ctx.poll_events();
  }

  pub fn get_time(&mut self) -> f32 {
    return self.ctx.ctx.get_time() as f32;
  }
}

///////////////////////////////////////////////////////
/// RglShader
///////////////////////////////////////////////////////

pub enum RglShaderKind {
  Vertex,
  Fragment
}

pub struct RglShader {
  id: gl::types::GLuint,
  kind: RglShaderKind,
}

impl RglShader {
  pub fn from_source(source: &str, kind: RglShaderKind) -> Result<RglShader, String> {
    let glkind = match kind {
      RglShaderKind::Vertex => gl::VERTEX_SHADER,
      RglShaderKind::Fragment => gl::FRAGMENT_SHADER,
    };

    let shader = unsafe {
      let id = gl::CreateShader(glkind);
      let c_str_frag = CString::new(source.as_bytes()).unwrap();
      gl::ShaderSource(id, 1, &c_str_frag.as_ptr(), std::ptr::null());
      gl::CompileShader(id);
      id
    };

    let (success, compile_err) = unsafe {
      let mut success = gl::FALSE as GLint;
      let mut info_log = Vec::with_capacity(512);
      let mut err = String::new();
      info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
      gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
      if success != gl::TRUE as GLint {
        gl::GetShaderInfoLog(shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);        
        err = format!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap()).to_string();
      }

      (success, err)
    };

    if success != gl::TRUE as GLint {
      return Err(compile_err);
    }
    
    return Ok(RglShader { id: shader, kind: kind })
  }

  pub fn from_file(file: &str, kind: RglShaderKind) -> Result<RglShader, String> {
    let source = fs::read_to_string(file).unwrap();
    return RglShader::from_source(&source, kind);
  }
}

///////////////////////////////////////////////////////
/// RglShaderProgram
///////////////////////////////////////////////////////

pub struct RglShaderProgram {
  pub id: gl::types::GLuint,
}

impl RglShaderProgram {
  pub fn link(vertex: RglShader, fragment : RglShader) -> Result<RglShaderProgram, String> {    

    let program = unsafe {
      let shader_program = gl::CreateProgram();
      gl::AttachShader(shader_program, vertex.id);
      gl::AttachShader(shader_program, fragment.id);
      gl::LinkProgram(shader_program);
      shader_program
    };

    let (success, link_err) = unsafe {
      let mut success = gl::FALSE as GLint;
      let mut info_log = Vec::with_capacity(512);
      let mut err = String::new();
      info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character      
      gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
      if success != gl::TRUE as GLint {
        gl::GetShaderInfoLog(program, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);        
        err = format!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap()).to_string();
      }

      (success, err)
    };

    if success != gl::TRUE as GLint {
      return Err(link_err);
    }

    return Ok( RglShaderProgram { id: program });
  }

  pub fn apply(&self) {
    unsafe {
      gl::UseProgram(self.id);
    }
  }

  pub fn find_uniform(&self, uniform: &str) -> i32 {
    let c_str_uniform = CString::new(uniform.as_bytes()).unwrap();

    let uniform_location = unsafe {
      let id = gl::GetUniformLocation(self.id, c_str_uniform.as_ptr());
      id
    };

    return uniform_location;
  }

  pub fn set_uniform_4f(&self, uniform: &str, value: &[f32; 4]) {
    let location = self.find_uniform(uniform);
    unsafe {
      gl::ProgramUniform4f(self.id, location, value[0], value[1], value[2], value[3]);
    }
  }

  pub fn set_uniform_1i(&self, uniform: &str, value: i32) {
    let location = self.find_uniform(uniform);
    unsafe {
      gl::ProgramUniform1i(self.id, location, value);
    }
  }
}

impl Drop for RglShaderProgram {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteProgram(self.id);
    }
  }
}

///////////////////////////////////////////////////////
/// RglMesh
///////////////////////////////////////////////////////

pub struct RglMesh {
  pub vertex_buffer: u32,
  pub texture: Option<RglTexture>,
}

impl RglMesh {
  pub fn from_vertex_data(vertices: &[f32]) -> RglMesh {
    let (mut vertex_buffer, mut vertex_array) = (0, 0);

    unsafe {
      gl::GenVertexArrays(1, &mut vertex_array);
      gl::GenBuffers(1, &mut vertex_buffer);
      // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
      gl::BindVertexArray(vertex_array);

      gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
      gl::BufferData(gl::ARRAY_BUFFER,
                      (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                      &vertices[0] as *const f32 as *const c_void,
                      gl::STATIC_DRAW);

      // TODO(cb): store vertex elements separately then build up at runtime the vertex attrib
      // based on which values exist (e.g. texCoords, normals etc)
      let stride = 6 * mem::size_of::<GLfloat>() as GLsizei;

      // x, y, z position data
      gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
      gl::EnableVertexAttribArray(0);

      // r, g, b, color data
      gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
      gl::EnableVertexAttribArray(1);

      // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
      gl::BindBuffer(gl::ARRAY_BUFFER, 0);

      // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
      // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
      gl::BindVertexArray(0);

      // uncomment this call to draw in wireframe polygons.
      // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }   

    return RglMesh {vertex_buffer: vertex_array, texture: None};
  }

  pub fn from_pos_col_tex(positions: &[f32], colors: &[f32], texcoords: &[f32]) -> RglMesh {
    let (mut vertex_buffer, mut vertex_array) = (0, 0);

    unsafe {
      gl::GenVertexArrays(1, &mut vertex_array);
      gl::GenBuffers(1, &mut vertex_buffer);
      // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
      gl::BindVertexArray(vertex_array);

      let mut vertices = Vec::new();
      vertices.extend_from_slice(positions);
      vertices.extend_from_slice(colors);
      vertices.extend_from_slice(texcoords);

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

      if colors.len() > 0 {
        // r, g, b, color data
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1);
      }

      if texcoords.len() > 0 {
        // u, v
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (2 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1);
      }

      // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
      gl::BindBuffer(gl::ARRAY_BUFFER, 0);

      // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
      // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
      gl::BindVertexArray(0);

      // uncomment this call to draw in wireframe polygons.
      // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }   

    return RglMesh {vertex_buffer: vertex_array, texture: None};
  }

  pub fn set_texture(&mut self, texture: RglTexture) {
    self.texture = Some(texture);
  }

  pub fn bind(&self) {
    unsafe {
      gl::BindVertexArray(self.vertex_buffer);
    }
  }

  pub fn draw(&self) {
    unsafe {
      gl::DrawArrays(gl::TRIANGLES, 0, 3);
      match &self.texture {
        Some(t) => t.bind(),
        None => {}
      }
    }
  }
}

///////////////////////////////////////////////////////
/// RglTexture
///////////////////////////////////////////////////////

pub struct RglTexture {
  id: GLuint,
}

impl RglTexture {

  pub fn from_file(path: &str) -> RglTexture {
    let id = unsafe {
      let mut texture_id = 0;
      gl::GenTextures(1, &mut texture_id);
      gl::BindTexture(gl::TEXTURE_2D, texture_id);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

      let img = image::open(&Path::new(path)).expect("Failed to load texture");
      let data = img.raw_pixels();
      gl::TexImage2D(gl::TEXTURE_2D, 0,
        gl::RGB as i32, img.width() as i32, img.height() as i32, 0,
        gl::RGB, gl::UNSIGNED_BYTE, &data[0] as *const u8 as *const c_void);

      gl::GenerateMipmap(gl::TEXTURE_2D);

      texture_id
    };

    RglTexture { id: id }
  }

  pub fn bind(&self) {
    unsafe {
      gl::ActiveTexture(gl::TEXTURE0);
      gl::BindTexture(gl::TEXTURE_2D, self.id);      
    }
  }
}