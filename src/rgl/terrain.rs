extern crate glfw;

use rgl::mesh::RglMesh;
use rgl::texture::RglTexture;
use rgl::heightmap::RglHeightmap;

///////////////////////////////////////////////////////
/// RglTerrain
///////////////////////////////////////////////////////
pub struct RglTerrain {
  mesh: RglMesh,
}

impl RglTerrain {

  pub fn new(width: u32, depth: u32) -> RglTerrain {

    let heightmap = RglHeightmap::from_file("textures/terrain_small.png");
    let width = heightmap.get_image_width() / 2;
    let depth = heightmap.get_image_height() / 2;

    let numVertRows = width * 2;
    let numVertCols = depth * 2;
    let dx = 1.0;
    let dz = 1.0;
    let numVertices = (numVertRows * numVertCols) as usize;
    let numCellRows = numVertRows - 1;
    let numCellCols = numVertCols - 1;

    let width = numCellCols as f32 * dx;
    let depth = numCellRows as f32 * dz;

    let numTris = (numCellRows * numCellCols * 2) as usize;

    let mut vertices:Vec<f32> = Vec::with_capacity(numVertices * 8);
    vertices.resize(numVertices * 8, 0.0);

    let xOffset = width as f32 * -0.5;
    let zOffset = depth as f32 * 0.5;    

    let mut k = 0;    
    for i in 0..numVertRows {
      for j in 0..numVertCols {
        vertices[k] = j as f32 * dx as f32 + xOffset;
        let mut h:f32 = heightmap.get_height(i, j).into();     
        h = h / 10.0;   
        vertices[k + 1] = h;
        vertices[k + 2] = (i as f32 * -1.0) * dz as f32 + zOffset;

        vertices[k + 3] = 0.0;
        vertices[k + 4] = 1.0;
        vertices[k + 5] = 0.0;

        let x = vertices[k];
        let z = vertices[k + 2];
        vertices[k + 6] = (x + (0.5 * width)) / width;
        vertices[k + 7] = (z - (0.5 * depth)) / (depth * -1.0);

        k += 8;
      } 
    }

    let mut indices:Vec<i32> = Vec::with_capacity(numTris * 8);
    indices.resize(numTris * 6, 0);

    k = 0;
    for i in 0..numCellRows {
      for j in 0..numCellCols {
        indices[k] = (i * numVertCols + j) as i32;
        indices[k + 1] = (i * numVertCols + j + 1) as i32;
        indices[k + 2] = ((i + 1) * numVertCols + j) as i32;

        indices[k + 3] = ((i + 1) * numVertCols + j) as i32;
        indices[k + 4] = (i * numVertCols + j + 1) as i32;
        indices[k + 5] = ((i + 1) * numVertCols + j + 1) as i32;

        k += 6;
      }
    } 

    let mut mesh = RglMesh::from_data_indexed(&vertices, &[3, 3, 2], &indices);
    //mesh.set_wireframe(true);
    let texture = RglTexture::from_file("textures/grass.jpg");
    mesh.set_texture(texture);
    RglTerrain { mesh: mesh }
  }

  pub fn draw(&mut self) {
    self.mesh.bind();
    self.mesh.draw();
  }
}

