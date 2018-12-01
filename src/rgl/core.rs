extern crate glfw;
use self::glfw::{Context, Key, Action};
use std::sync::mpsc::Receiver;

use std::str;

extern crate image;

use rgl::RglMouse;
use rgl::texture::RglTexture;

///////////////////////////////////////////////////////
/// RglContext 
///////////////////////////////////////////////////////

pub struct RglContext {
  pub ctx: glfw::Glfw,  
  last_frame: f32,
  delta: f32
}

impl RglContext {
  pub fn new() -> RglContext {

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let out = RglContext {
      ctx: glfw,
      last_frame: 0.0,
      delta: 0.0
    };

    out
  }

  pub fn poll_events(&mut self) {
    self.ctx.poll_events();
  }

  pub fn update_delta(&mut self) {
    let current_frame = self.ctx.get_time() as f32;
    self.delta = current_frame - self.last_frame;
    self.last_frame = current_frame;
  }
}

///////////////////////////////////////////////////////
/// RglWindow 
///////////////////////////////////////////////////////

pub struct RglWindow {
  pub window: glfw::Window,
  pub events: Receiver<(f64, glfw::WindowEvent)>,
  pub ctx: RglContext,
  pub mouse: RglMouse
}

impl RglWindow {
  pub fn new(ctx: RglContext, width: u32, height: u32, title: &str) -> RglWindow {
    let (mut window, events) = ctx.ctx.create_window(width, height, title, glfw::WindowMode::Windowed)
      .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    RglWindow {
      window: window,
      events: events,
      ctx: ctx,
      mouse: RglMouse::new(),
    }
  }

  pub fn load_gl_extensions(&mut self) {
    gl::load_with(|symbol| self.window.get_proc_address(symbol) as *const _);
    
    unsafe {
      gl::Enable(gl::DEPTH_TEST);
    }
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
        glfw::WindowEvent::CursorPos(xpos, ypos) => {          
          self.mouse.update(xpos as f32, ypos as f32);
        }
        glfw::WindowEvent::Scroll(_xoffset, _yoffset) => {
          //self.mouse.updateMouseWheel(yoffset as f32);
        }
        _ => {}
      }
    }
  }

  pub fn pre_update(&mut self) {
    self.ctx.update_delta();
  }

  pub fn post_update(&mut self) {
    self.mouse.post_update();
  }

  pub fn dt(&mut self) -> f32 {
    return self.ctx.delta;
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

  pub fn key_pressed(&mut self, key: glfw::Key) -> bool {
    return self.window.get_key(key) == Action::Press;
  }
}