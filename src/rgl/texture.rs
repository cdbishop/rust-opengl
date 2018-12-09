extern crate glfw;

use gl::types::*;

use std::str;
use std::path::Path;
use std::os::raw::c_void;

extern crate image;
use self::image::GenericImage;
use self::image::DynamicImage::*;

///////////////////////////////////////////////////////
/// RglTexture
///////////////////////////////////////////////////////

pub struct RglTexture {
  id: GLuint,
}

impl RglTexture {

  pub fn from_file(path: &str) -> RglTexture {
    let id = unsafe {
      let mut texture_id = 0;
      gl::GenTextures(1, &mut texture_id);
      gl::BindTexture(gl::TEXTURE_2D, texture_id);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

      let img = image::open(&Path::new(path)).expect("Failed to load texture");
      let format = match img {
        ImageLuma8(_) => gl::RED,
        ImageLumaA8(_) => gl::RG,
        ImageRgb8(_) => gl::RGB,
        ImageRgba8(_) => gl::RGBA,
      };
      let data = img.raw_pixels();
      gl::TexImage2D(gl::TEXTURE_2D, 0,
        format as i32, img.width() as i32, img.height() as i32, 0,
        format, gl::UNSIGNED_BYTE, &data[0] as *const u8 as *const c_void);

      gl::GenerateMipmap(gl::TEXTURE_2D);

      texture_id
    };

    RglTexture { id: id }
  }

  pub fn bind(&self) {
    unsafe {
      gl::ActiveTexture(gl::TEXTURE0);
      gl::BindTexture(gl::TEXTURE_2D, self.id);      
    }
  }
}