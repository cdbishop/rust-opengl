
use rgl::mesh::RglMesh;
use rgl::texture::RglTexture;
use rgl::heightmap::RglHeightmap;

// use cgmath::{vec3};
// use cgmath::prelude::*;

///////////////////////////////////////////////////////
/// RglTerrain
///////////////////////////////////////////////////////
pub struct RglTerrain {
  mesh: RglMesh,
}

impl RglTerrain {

  pub fn new(_width: u32, _depth: u32) -> RglTerrain {

    // TODO: move this to diff function?
    let heightmap = RglHeightmap::from_file("textures/terrain_small.png");
    let width = heightmap.get_image_width() / 2;
    let depth = heightmap.get_image_height() / 2;

    let num_vert_rows = width * 2;
    let num_vert_cols = depth * 2;
    let dx = 1.0;
    let dz = 1.0;
    let num_vertices = (num_vert_rows * num_vert_cols) as usize;
    let num_cell_rows = num_vert_rows - 1;
    let num_cell_cols = num_vert_cols - 1;

    let width = num_cell_cols as f32 * dx;
    let depth = num_cell_rows as f32 * dz;

    let num_tris = (num_cell_rows * num_cell_cols * 2) as usize;

    let mut vertices:Vec<f32> = Vec::with_capacity(num_vertices * 8);
    vertices.resize(num_vertices * 8, 0.0);

    let x_offset = width as f32 * -0.5;
    let z_offset = depth as f32 * 0.5;    

    let mut k = 0;    
    for i in 0..num_vert_rows {
      for j in 0..num_vert_cols {
        vertices[k] = j as f32 * dx as f32 + x_offset;
        let mut h:f32 = heightmap.get_height(i, j).into();     
        h = h / 10.0;   
        vertices[k + 1] = h;
        vertices[k + 2] = (i as f32 * -1.0) * dz as f32 + z_offset;

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

    let mut indices:Vec<i32> = Vec::with_capacity(num_tris * 8);
    indices.resize(num_tris * 6, 0);

    k = 0;
    for i in 0..num_cell_rows {
      for j in 0..num_cell_cols {
        indices[k] = (i * num_vert_cols + j) as i32;
        indices[k + 1] = (i * num_vert_cols + j + 1) as i32;
        indices[k + 2] = ((i + 1) * num_vert_cols + j) as i32;

        indices[k + 3] = ((i + 1) * num_vert_cols + j) as i32;
        indices[k + 4] = (i * num_vert_cols + j + 1) as i32;
        indices[k + 5] = ((i + 1) * num_vert_cols + j + 1) as i32;

        k += 6;
      }
    } 

    let mut mesh = RglMesh::from_data_indexed(&vertices, &[3, 3, 2], &indices);
    let texture = RglTexture::from_file("textures/grass.jpg");
    mesh.set_texture(texture);
    RglTerrain { mesh: mesh }
  }

  pub fn draw(&mut self) {
    self.mesh.bind();
    self.mesh.draw();
  }
}

