extern crate glfw;

use rgl::mesh::RglMesh;

///////////////////////////////////////////////////////
/// RglTerrain
///////////////////////////////////////////////////////
pub struct RglTerrain {
  mesh: RglMesh,
}

impl RglTerrain {

  pub fn new(width: u32, height: u32) -> RglTerrain {
    let vertices: [f32; 36] = [
        -0.5, -1.0, -0.5,  0.0,  1.0, 0.0,
         0.5, -1.0, -0.5,  0.0,  1.0, 0.0,
         0.5,  -1.0, -0.5,  0.0,  1.0, 0.0,
         0.5,  -1.0, -0.5,  0.0,  1.0, 0.0,
        -0.5,  -1.0, -0.5,  0.0,  1.0, 0.0,
        -0.5, -1.0, -0.5,  0.0,  1.0, 0.0,
      ];

    let mut mesh = RglMesh::from_data(&vertices, &[3, 3]);
    mesh.set_wireframe(true);
    RglTerrain { mesh: mesh }
  }

  pub fn draw(&mut self) {
    self.mesh.bind();
    self.mesh.draw();
  }
}
