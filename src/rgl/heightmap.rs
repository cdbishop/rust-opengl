use std::str;
use std::path::Path;

use image::DynamicImage;
use image::GenericImage;

///////////////////////////////////////////////////////
/// RglHeightmap
///////////////////////////////////////////////////////

pub struct RglHeightmap {
  data: DynamicImage
}

impl RglHeightmap {

  pub fn from_file(path: &str) -> RglHeightmap {
    let img = {      
      let img = image::open(&Path::new(path)).expect("Failed to load texture");
      // let format = match img {
      //   ImageLuma8(_) => gl::RED,
      //   ImageLumaA8(_) => gl::RG,
      //   ImageRgb8(_) => gl::RGB,
      //   ImageRgba8(_) => gl::RGBA,
      // };

      img
    };

    RglHeightmap {data: img}
  }

  pub fn get_height(&self, x: u32, y: u32) -> u8 {
    return self.data.get_pixel(x, y)[3];
  }

  pub fn get_image_width(&self) -> u32 {
    self.data.width()
  }

  pub fn get_image_height(&self) -> u32 {
    self.data.height()
  }
}