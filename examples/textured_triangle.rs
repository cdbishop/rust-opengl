extern crate rgl;
use rgl::{
  RglContext,
  RglWindow,
  RglMesh,
  RglTexture,
  RglShader,
  RglShaderKind,
  RglShaderProgram,
  RglApplication,
  run_app
};


pub struct TexturedTriangle {
  pub window: RglWindow,
  pub shader_program: RglShaderProgram,
  pub triangle: RglMesh,
}

impl RglApplication for TexturedTriangle {
  fn init() -> TexturedTriangle {
    let ctx = RglContext::new();
    let mut window = RglWindow::new(ctx, 800, 600, "TexturedTriangle");
    window.load_gl_extensions();

    let shader_program = {      
      let vertex_shader = RglShader::from_file("shaders/textured.vert", RglShaderKind::Vertex).unwrap();
      let fragment_shader = RglShader::from_file("shaders/textured.frag", RglShaderKind::Fragment).unwrap();    
      let shader_program = RglShaderProgram::link(vertex_shader, fragment_shader).unwrap();       

      shader_program
    };

    let triangle = {
      let texture = RglTexture::from_file("textures/brickwall.jpg");

      let pos: [f32; 9] = [
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0,  0.5, 0.0,
      ];

      let col: [f32; 9] = [
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0
      ];

      let tex: [f32; 6] = [
        0.0, 1.0, 
        1.0, 1.0,
        0.5, 0.0,
      ];

      let mut triangle = RglMesh::from_pos_col_tex(&pos, &col, &tex, 3);    
      triangle.set_texture(texture);
      triangle
    };

    TexturedTriangle { window, shader_program, triangle }
  }

  fn update(&mut self) {
    
  }

  fn draw(&mut self) {
    self.shader_program.apply();
    self.shader_program.set_uniform_1i("texture1", 0);
    self.triangle.bind();
    self.triangle.draw();  
  }

  fn get_window(&mut self) -> &mut RglWindow {
    return &mut self.window;
  }
}

pub fn main() {
  let app = TexturedTriangle::init();
  run_app(app);
}