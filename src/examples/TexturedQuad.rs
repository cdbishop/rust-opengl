
use super::super::rgl::{
  RglContext,
  RglWindow,
  RglMesh,
  RglTexture,
  RglShader,
  RglShaderKind,
  RglShaderProgram,
  RglApplication,
};


pub struct TexturedQuad {
  pub window: RglWindow,
  pub shader_program: RglShaderProgram,
  pub Quad: RglMesh,
}

impl RglApplication for TexturedQuad {
  fn init() -> TexturedQuad {
    let ctx = RglContext::new();
    let mut window = RglWindow::new(ctx, 800, 600, "TexturedQuad");
    window.load_gl_extensions();

    let shader_program = {      
      let vertex_shader = RglShader::from_file("shaders/textured.vert", RglShaderKind::Vertex).unwrap();
      let fragment_shader = RglShader::from_file("shaders/textured.frag", RglShaderKind::Fragment).unwrap();    
      let shader_program = RglShaderProgram::link(vertex_shader, fragment_shader).unwrap();       

      shader_program
    };

    let Quad = {
      let texture = RglTexture::from_file("textures/brickwall.jpg");

      let pos: [f32; 12] = [
        0.5, 0.5, 0.0,    // top-right
        0.5, -0.5, 0.0,   // bottom-right
        -0.5,  -0.5, 0.0, // bottom-left
        -0.5,  0.5, 0.0,  // top-left
      ];

      let col: [f32; 12] = [
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
        1.0, 1.0, 1.0
      ];

      let tex: [f32; 8] = [
        1.0, 0.0, 
        1.0, 1.0,
        0.0, 1.0,
        0.0, 0.0,
      ];

      let indices = [
        0, 1, 3,
        1, 2, 3
      ];

      let mut Quad = RglMesh::from_pos_col_tex_index(&pos, &col, &tex, &indices, 4);    
      Quad.set_texture(texture);
      Quad
    };

    TexturedQuad { window, shader_program, Quad }
  }

  fn update(&mut self) {
    
  }

  fn draw(&mut self) {
    self.shader_program.apply();
    self.shader_program.set_uniform_1i("texture1", 0);
    self.Quad.bind();
    self.Quad.draw();  
  }

  fn get_window(&mut self) -> &mut RglWindow {
    return &mut self.window;
  }
}