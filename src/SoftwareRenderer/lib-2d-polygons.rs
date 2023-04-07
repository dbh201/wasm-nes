use wasm_bindgen::prelude::*;

use web_sys::{WebGlRenderingContext, WebGlProgram, WebGlShader};


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f32;
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


#[wasm_bindgen]
pub struct WebGl2DSoftwareRenderer {
    program: WebGlProgram,
    context: WebGlRenderingContext,
    buffer: Vec<f32>,
    width: usize,
    height: usize,
}

#[wasm_bindgen]
impl WebGl2DSoftwareRenderer {
    pub fn new(s: &str,width: usize, height: usize) -> Result<WebGl2DSoftwareRenderer,JsValue> {
        let buffer: Vec<f32> = Vec::new();
        // Get the root DOM document
        let document = web_sys::window().unwrap().document().unwrap();
        // Get the canvas by ID
        let canvas = document.get_element_by_id(s).unwrap();
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
            attribute vec4 position;
            attribute vec4 color;
            varying vec4 inColor;
            void main() {
                gl_Position = position;
                inColor = color;
            }"##,
        )?;

        let frag_shader = Self::compile_shader(
            &context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            r##"#version 100
            precision highp float;

            varying vec4 inColor;
            void main() {
                gl_FragColor = inColor;
            }"##,
        )?;

        let program = Self::link_program( &context, &vertex_shader, &frag_shader)?;
        context.use_program(Some(&program));

        let framebuffer = context.create_buffer().ok_or("Failed to create buffer")?;
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&framebuffer));

        let mut ret = WebGl2DSoftwareRenderer {
            program,
            context,
            buffer,
            width,
            height
        };
        ret.buffer_init();
        Ok(ret)
    }

    // initialize buffer with width x height equally sized cells
    fn buffer_init(&mut self) {

        console_log!("Rebuilding buffers {} x {}",self.width,self.height);
        self.buffer.clear();
        for row in 0..self.height {
            for col in 0..self.width {
                // Make two triangles
                let r = row as f32;
                let c = col as f32;
                let w = self.width as f32;
                let h = self.height as f32;
                let top = ((r+0.00) * 2.0/h) - 1.0;
                let bot = ((r+1.00) * 2.0/h) - 1.0;
                let left = ((c+0.00) * 2.0/w) - 1.0;
                let right = ((c+1.00) * 2.0/w) - 1.0;

                self.buffer.push(left);
                self.buffer.push(top);
                self.buffer.push(0.0);

                self.buffer.push(0.0);
                self.buffer.push(0.0);
                self.buffer.push(0.0);
                self.buffer.push(1.0);

                self.buffer.push(left);
                self.buffer.push(bot);
                self.buffer.push(0.0);

                self.buffer.push(0.0);
                self.buffer.push(0.0);
                self.buffer.push(0.0);
                self.buffer.push(1.0);

                self.buffer.push(right);
                self.buffer.push(top);
                self.buffer.push(0.0);

                self.buffer.push(0.0);
                self.buffer.push(0.0);
                self.buffer.push(0.0);
                self.buffer.push(1.0);

                self.buffer.push(right);
                self.buffer.push(top);
                self.buffer.push(0.0);

                self.buffer.push(0.0);
                self.buffer.push(0.0);
                self.buffer.push(0.0);
                self.buffer.push(1.0);

                self.buffer.push(left);
                self.buffer.push(bot);
                self.buffer.push(0.0);

                self.buffer.push(0.0);
                self.buffer.push(0.0);
                self.buffer.push(0.0);
                self.buffer.push(1.0);

                self.buffer.push(right);
                self.buffer.push(bot);
                self.buffer.push(0.0);

                self.buffer.push(0.0);
                self.buffer.push(0.0);
                self.buffer.push(0.0);
                self.buffer.push(1.0);
            }
        }

        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(self.buffer.as_slice());

            self.context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGlRenderingContext::DYNAMIC_DRAW,
                );
        }

        let position_attribute_location = self.context.get_attrib_location(&self.program,"position");
        self.context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            7*4,
            0,
        );
        self.context.enable_vertex_attrib_array(position_attribute_location as u32);

        let color_attribute_location = self.context.get_attrib_location(&self.program,"color");
        self.context.vertex_attrib_pointer_with_i32(
            color_attribute_location as u32,
            4,
            WebGlRenderingContext::FLOAT,
            false,
            7*4,
            3*4,
        );
        self.context.enable_vertex_attrib_array(color_attribute_location as u32);
        console_log!("Buffer size is {}*{}*7 = {} ",self.width,self.height,self.buffer.len());
        console_log!("Position attribute location: {}\nColor attribute location: {}",
                     position_attribute_location,
                     color_attribute_location
                     );
    }
    fn get_buffer_index(&self, row: usize,col: usize) -> usize {
        (row*self.width + col)*42 as usize
    }

    pub fn color_cell(&mut self, col: usize, row: usize, color: &[f32]) {
        let index = self.get_buffer_index(row, col) as usize;

        // 6 vertices of 7 floats each per cell
        // for each vertex, float 4-7 are colors.

        for x in (3..42).step_by(7) {
            self.buffer[index + x]      = color[0];
            self.buffer[index + x + 1]  = color[1];
            self.buffer[index + x + 2]  = color[2];
            self.buffer[index + x + 3]  = color[3];
        }
    }
    pub fn fill_static(&mut self) {
        let h = self.height as f32;
        let w = self.width as f32;

        for y in 0..self.height {
            for x in 0..self.width {
                self.color_cell(x,y,&[(x as f32)*1.0/w,(y as f32)*1.0/h,((x+y) as f32)/(w+h),1.0]);
            }
        }
    }
    pub fn draw(&mut self) {
        self.context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
        // TODO: Can this buffer be 100% GPU-sided?
        // At the moment, it seems that we're filling the buffer from RAM every frame.
        //
        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(self.buffer.as_slice());

            self.context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGlRenderingContext::DYNAMIC_DRAW,
                );
        }
        self.context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, (self.height * self.width * 6).try_into().unwrap());
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
/*
 * THIS DOES NOT WORK: wasm_bindgen does not support explicit lifetimes,
 * so the critical closure in enable_refresh causes gol to escape function
 * lifetime.
 *
thread_local! {
    static refresh_enabled: RefCell<bool> = RefCell::new(false);
    static refresh_webgl: RefCell<Option<&'static mut WebGlGameOfLife>> = RefCell::new(None);
}
#[wasm_bindgen]
pub fn enable_refresh(gol: &mut WebGlGameOfLife) {
    refresh_enabled.with(|tl_ena| {
    refresh_webgl.with(|tl_gol| {
        *tl_gol.borrow_mut() = Some(gol);
        let step_init = Rc::new(RefCell::new(None));
        let step = step_init.clone();

        *step_init.borrow_mut() = Some(Closure::new(move || {
            if *tl_ena.borrow() == false {
                let _ = step.borrow_mut().take();
                return;
            };
            tl_gol.borrow_mut().as_ref().unwrap().draw();
            request_animation_frame(step.borrow().as_ref().unwrap());
        }));
        request_animation_frame(step_init.borrow().as_ref().unwrap());
    })});
}
#[wasm_bindgen]
pub fn disable_refresh() {
    refresh_enabled.with(|tl_ena| { *tl_ena.borrow_mut() = false } );
}
fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}
fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
*/
