use wasm_bindgen::prelude::*;

use web_sys::{WebGlRenderingContext, WebGlBuffer, WebGlTexture, WebGlProgram, WebGlShader};
use getrandom::getrandom;
use crate::real_console_log as console_log;

#[wasm_bindgen]
pub struct WebGl2DSoftwareRenderer {
    program: WebGlProgram,
    context: WebGlRenderingContext,
    attrib_buffer: WebGlBuffer,
    texture_buffer: WebGlBuffer,
    texture: WebGlTexture,
    pixel_buffer: Vec<u8>,
    width: usize,
    height: usize
}


#[wasm_bindgen]
impl WebGl2DSoftwareRenderer { 
    pub fn new(s: &str,width: usize, height: usize) -> Result<WebGl2DSoftwareRenderer,JsValue> {
        // Get the root DOM document
        let document = web_sys::window().ok_or("Couldn't get window")?.document().ok_or("Couldn't get document")?;
        // Get the canvas by ID
        let canvas = document.get_element_by_id(s).ok_or(format!("Couldn't get canvas: {}",s))?;
        // Convert canvas object to HtmlCanvasElement
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
        // Get a WebGL context and convert object to WebGlRenderingContext
        let context = canvas.get_context("webgl")?.unwrap().dyn_into::<WebGlRenderingContext>()?;
        
        // Compile some simple shaders in the webGL context.

        // This one is a simple point vertex shader:
        // position vector goes in, gl_Position comes out.
        // also sets inColor from the color attribute
        // Which will directly set the fragment color.
        //
        // This can be turned into a more reasonable shader later.

        let vertex_shader = Self::compile_shader(
            &context,
            WebGlRenderingContext::VERTEX_SHADER,
            r##"#version 100
            attribute vec4 aPosition;
            attribute vec2 aTextureCoord;
            varying vec2 vTextureCoord;
            void main() {
                gl_Position = aPosition;
                vTextureCoord = aTextureCoord;
            }"##,
        )?;

        let frag_shader = Self::compile_shader(
            &context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            r##"#version 100
            precision highp float;

            varying vec2 vTextureCoord;
            uniform sampler2D uSampler;
            void main() {
                gl_FragColor = texture2D(uSampler, vTextureCoord);
                //gl_FragColor = vec4(1.0,1.0,1.0,1.0);
            }"##,
        )?;

        let program = Self::link_program( &context, &vertex_shader, &frag_shader)?;
        context.use_program(Some(&program));

        let attrib_buffer = context.create_buffer().ok_or("Failed to create buffer")?;
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&attrib_buffer));

        let full_rect = [
            -1.0, -1.0, 0.0,
            -1.0,  1.0, 0.0,
             1.0,  1.0, 0.0,

             1.0,  1.0, 0.0,
             1.0, -1.0, 0.0,
            -1.0, -1.0, 0.0,
        ];
        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(full_rect.as_slice());

            context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGlRenderingContext::STATIC_DRAW,
                );
        }


        let texture_buffer = context.create_buffer().ok_or("Failed to create buffer")?;
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&texture_buffer));
        let full_texture = [
            0.0, 0.0, 0.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 0.0, 0.0, 0.0,
        ];
        unsafe {
            let coords_array_buf_view = js_sys::Float32Array::view(full_texture.as_slice());

            context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &coords_array_buf_view,
                WebGlRenderingContext::STATIC_DRAW,
                );
        }

        let texture = context.create_texture().ok_or("Failed to create texture")?;
        context.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&texture));
        let pixel_buffer: Vec<u8> = Vec::new();
        let mut ret = WebGl2DSoftwareRenderer {
            program,
            context,
            texture,
            attrib_buffer,
            texture_buffer,
            pixel_buffer,
            width,
            height
        };
        ret.buffer_init();

        Ok(ret)
    }

    // initialize buffer with width x height equally sized cells
    fn buffer_init(&mut self) {
        console_log!("Rebuilding buffers {} x {}",self.width,self.height);

        self.pixel_buffer.clear();
        for row in 0..self.height {
            for col in 0..self.width {
                // pixel data
                self.pixel_buffer.push((row*255/self.height) as u8);
                self.pixel_buffer.push((col*255/self.width) as u8);
                self.pixel_buffer.push(((row+col)*255/(self.height + self.width)) as u8);
                self.pixel_buffer.push(255);
            }
        }
        // Texture
        self.context.active_texture(WebGlRenderingContext::TEXTURE0);
        self.context.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&self.texture));
        self.context.tex_parameteri(WebGlRenderingContext::TEXTURE_2D,WebGlRenderingContext::TEXTURE_WRAP_S,WebGlRenderingContext::CLAMP_TO_EDGE.try_into().unwrap());
        self.context.tex_parameteri(WebGlRenderingContext::TEXTURE_2D,WebGlRenderingContext::TEXTURE_WRAP_T,WebGlRenderingContext::CLAMP_TO_EDGE.try_into().unwrap());
        self.context.tex_parameteri(WebGlRenderingContext::TEXTURE_2D,WebGlRenderingContext::TEXTURE_MIN_FILTER,WebGlRenderingContext::LINEAR.try_into().unwrap());
        self.context.uniform1i(self.context.get_uniform_location(&self.program, "uSampler").as_ref(),0);

        self.context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.texture_buffer));
        let a_texture_coord = self.context.get_attrib_location(&self.program,"aTextureCoord") as u32;
        self.context.vertex_attrib_pointer_with_i32(
            a_texture_coord,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0
        );
        self.context.enable_vertex_attrib_array(a_texture_coord);

        self.context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.attrib_buffer));
        let a_position = self.context.get_attrib_location(&self.program,"aPosition") as u32;
        self.context.vertex_attrib_pointer_with_i32(
            a_position,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0
        );
        self.context.enable_vertex_attrib_array(a_position);



        console_log!("Buffer size is {}*{}*7 = {} ",self.width,self.height,self.pixel_buffer.len());
    }
    fn get_buffer_index(&self, row: usize,col: usize) -> usize {
        (row*self.width + col)*4 as usize
    }

    // INEFFICIENT! Is there a way to simply replace pixel_buffer with the new one?
    pub fn replace_pixel_buffer(&mut self, buffer: &[u8]) -> Result<(),String> {
        if buffer.len() != self.width * self.height * 4 {
            Err(format!("new pixel buffer length was {}, should be {}",buffer.len(), self.width * self.height * 4))
        } else {
            self.pixel_buffer = Vec::from(buffer);
            Ok(())
        }
    }
    pub fn color_cell(&mut self, col: usize, row: usize, color: &[u8]) {
        let index = self.get_buffer_index(row, col) as usize;

        self.pixel_buffer[index + 0]  = color[0];
        self.pixel_buffer[index + 1]  = color[1];
        self.pixel_buffer[index + 2]  = color[2];
        self.pixel_buffer[index + 3]  = color[3];
    }

    pub fn fill_static(&mut self) {
        getrandom(self.pixel_buffer.as_mut_slice()).expect("Noise generation failed for some reason");
        for y in (3..self.pixel_buffer.len()).step_by(4) {
            self.pixel_buffer[y] = 255;
        }
    }

    pub fn draw(&mut self) {
        self.context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        //self.context.active_texture(WebGlRenderingContext::TEXTURE0);
        //self.context.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&self.texture));
        self.context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGlRenderingContext::TEXTURE_2D,
            0,
            WebGlRenderingContext::RGBA.try_into().unwrap(),
            self.width.try_into().unwrap(),
            self.height.try_into().unwrap(),
            0,
            WebGlRenderingContext::RGBA,
            WebGlRenderingContext::UNSIGNED_BYTE,
            Some(&self.pixel_buffer.as_slice())
            ).expect("Failed to generate texture image");
        //self.context.uniform1i(self.context.get_uniform_location(&self.program, "uSampler").as_ref(),0);
        self.context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);
    }

    pub fn compile_shader(
        context: &WebGlRenderingContext,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, String> {
        let shader = context
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        if context
            .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }

    pub fn link_program(
        context: &WebGlRenderingContext,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = context
            .create_program()
            .ok_or_else(|| String::from("Unable to create program object"))?;

        context.attach_shader(&program, vert_shader);
        context.attach_shader(&program, frag_shader);
        context.link_program(&program);

        if context
            .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }

}
