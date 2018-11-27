
use super::super::rgl::{
  RglContext,
  RglWindow,
  RglMesh,
  RglShader,
  RglShaderKind,
  RglShaderProgram,
  RglApplication,
};


pub struct ColouredTriangle {
  pub window: RglWindow,
  pub shader_program: RglShaderProgram,
  pub triangle: RglMesh,
}

impl RglApplication for ColouredTriangle {
  fn init() -> ColouredTriangle {
    let ctx = RglContext::new();
    let mut window = RglWindow::new(ctx, 800, 600, "TexturedTriangle");
    window.load_gl_extensions();

    let shader_program = {      
      let vertex_shader = RglShader::from_file("shaders/color.vert", RglShaderKind::Vertex).unwrap();
      let fragment_shader = RglShader::from_file("shaders/color.frag", RglShaderKind::Fragment).unwrap();    
      let shader_program = RglShaderProgram::link(vertex_shader, fragment_shader).unwrap();       

      shader_program
    };

    let triangle = {
      let vertices: [f32; 18] = [
        -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
        0.0,  0.5, 0.0, 0.0, 0.0, 1.0
      ];
      let triangle = RglMesh::from_vertex_data(&vertices);
      triangle
    };

    ColouredTriangle { window, shader_program, triangle }
  }

  fn update(&mut self) {
    
  }

  fn draw(&mut self) {
    self.shader_program.apply();
    self.shader_program.set_uniform_4f("inColor", &[0.0, self.window.get_time().sin() / 2.0 + 0.5, 0.0, 1.0]);
    self.triangle.bind();
    self.triangle.draw();  
  }

  fn get_window(&mut self) -> &mut RglWindow {
    return &mut self.window;
  }
}