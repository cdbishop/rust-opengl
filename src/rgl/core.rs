extern crate glfw;
use self::glfw::{Context, Key, Action};
use std::sync::mpsc::Receiver;

use std::str;

extern crate image;

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