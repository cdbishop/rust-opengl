extern crate glfw;

use std::ffi::CString;
use gl::types::*;

use std::ptr;
use std::str;
use std::fs;

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