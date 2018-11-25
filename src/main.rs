#![allow(non_upper_case_globals)]

extern crate gl;

extern crate glfw;

use std::str;

mod lib;

use lib::{RglContext, RglWindow, RglShader, RglShaderKind, RglShaderProgram, RglMesh};

const vertex_shader_source: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const fragment_shader_source: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
       FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

#[allow(non_snake_case)]
pub fn main() {    
    let ctx = RglContext::new();
    let mut window = RglWindow::new(ctx, 800, 600, "Rgl");

    window.load_gl_extensions();

    let shaderProgram = {
      let vertexShader = RglShader::from_source(vertex_shader_source, RglShaderKind::Vertex).unwrap();
      let fragmentShader = RglShader::from_source(fragment_shader_source, RglShaderKind::Fragment).unwrap();
      let shaderProgram = RglShaderProgram::link(vertexShader, fragmentShader).unwrap();       

      shaderProgram
    };

    let triangle = {
      let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0, // left
        0.5, -0.5, 0.0, // right
        0.0,  0.5, 0.0  // top
      ];
      let triangle = RglMesh::from_vertex_data(&vertices);
      triangle
    };

    // render loop
    // -----------
    while !window.should_close() {
        // events
        // -----
        window.process_events();

        // render
        // ------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // draw our first triangle            
            shaderProgram.apply();
            triangle.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            // glBindVertexArray(0); // no need to unbind it every time
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();        
        window.poll_events();
    }
}
