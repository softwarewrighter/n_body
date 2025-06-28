use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext as GL,
    WebGlShader, WebGlUniformLocation,
};
use n_body_shared::Particle;

pub struct Renderer {
    gl: GL,
    program: WebGlProgram,
    position_buffer: WebGlBuffer,
    color_buffer: WebGlBuffer,
    u_projection: WebGlUniformLocation,
    u_view: WebGlUniformLocation,
    width: f32,
    height: f32,
}

impl Renderer {
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Self, JsValue> {
        let gl = canvas
            .get_context("webgl")?
            .unwrap()
            .dyn_into::<GL>()?;
        
        // Enable blending for particle effects
        gl.enable(GL::BLEND);
        gl.blend_func(GL::SRC_ALPHA, GL::ONE);
        
        // Create shaders
        let vertex_shader = Self::compile_shader(
            &gl,
            GL::VERTEX_SHADER,
            include_str!("shaders/vertex.glsl"),
        )?;
        
        let fragment_shader = Self::compile_shader(
            &gl,
            GL::FRAGMENT_SHADER,
            include_str!("shaders/fragment.glsl"),
        )?;
        
        // Create program
        let program = Self::link_program(&gl, &vertex_shader, &fragment_shader)?;
        gl.use_program(Some(&program));
        
        // Create buffers
        let position_buffer = gl.create_buffer().ok_or("Failed to create position buffer")?;
        let color_buffer = gl.create_buffer().ok_or("Failed to create color buffer")?;
        
        // Get uniform locations
        let u_projection = gl
            .get_uniform_location(&program, "u_projection")
            .ok_or("Failed to get u_projection")?;
        let u_view = gl
            .get_uniform_location(&program, "u_view")
            .ok_or("Failed to get u_view")?;
        
        Ok(Renderer {
            gl,
            program,
            position_buffer,
            color_buffer,
            u_projection,
            u_view,
            width: canvas.width() as f32,
            height: canvas.height() as f32,
        })
    }
    
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width as f32;
        self.height = height as f32;
        self.gl.viewport(0, 0, width as i32, height as i32);
    }
    
    pub fn render(&self, particles: &[Particle]) {
        // Clear
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(GL::COLOR_BUFFER_BIT);
        
        // Prepare particle data
        let mut positions = Vec::with_capacity(particles.len() * 3);
        let mut colors = Vec::with_capacity(particles.len() * 4);
        
        for particle in particles {
            positions.push(particle.position.x);
            positions.push(particle.position.y);
            positions.push(particle.position.z);
            
            colors.extend_from_slice(&particle.color);
        }
        
        // Update position buffer
        self.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.position_buffer));
        unsafe {
            let positions_array = js_sys::Float32Array::view(&positions);
            self.gl.buffer_data_with_array_buffer_view(
                GL::ARRAY_BUFFER,
                &positions_array,
                GL::DYNAMIC_DRAW,
            );
        }
        
        // Update color buffer
        self.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.color_buffer));
        unsafe {
            let colors_array = js_sys::Float32Array::view(&colors);
            self.gl.buffer_data_with_array_buffer_view(
                GL::ARRAY_BUFFER,
                &colors_array,
                GL::DYNAMIC_DRAW,
            );
        }
        
        // Set up attributes
        let position_attrib = self.gl.get_attrib_location(&self.program, "a_position") as u32;
        self.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.position_buffer));
        self.gl.vertex_attrib_pointer_with_i32(position_attrib, 3, GL::FLOAT, false, 0, 0);
        self.gl.enable_vertex_attrib_array(position_attrib);
        
        let color_attrib = self.gl.get_attrib_location(&self.program, "a_color") as u32;
        self.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.color_buffer));
        self.gl.vertex_attrib_pointer_with_i32(color_attrib, 4, GL::FLOAT, false, 0, 0);
        self.gl.enable_vertex_attrib_array(color_attrib);
        
        // Set uniforms
        let aspect = self.width / self.height;
        let fov = 45.0_f32.to_radians();
        let near = 0.1;
        let far = 100.0;
        
        let projection = self.perspective_matrix(fov, aspect, near, far);
        self.gl.uniform_matrix4fv_with_f32_array(Some(&self.u_projection), false, &projection);
        
        let view = self.look_at_matrix(
            [0.0, 0.0, 20.0],  // eye
            [0.0, 0.0, 0.0],   // center
            [0.0, 1.0, 0.0],   // up
        );
        self.gl.uniform_matrix4fv_with_f32_array(Some(&self.u_view), false, &view);
        
        // Draw particles as points
        self.gl.draw_arrays(GL::POINTS, 0, particles.len() as i32);
    }
    
    fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
        let shader = gl
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);
        
        if gl
            .get_shader_parameter(&shader, GL::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(gl
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }
    
    fn link_program(
        gl: &GL,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = gl
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        
        gl.attach_shader(&program, vert_shader);
        gl.attach_shader(&program, frag_shader);
        gl.link_program(&program);
        
        if gl
            .get_program_parameter(&program, GL::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(gl
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }
    
    fn perspective_matrix(&self, fov: f32, aspect: f32, near: f32, far: f32) -> [f32; 16] {
        let f = 1.0 / (fov / 2.0).tan();
        [
            f / aspect, 0.0, 0.0, 0.0,
            0.0, f, 0.0, 0.0,
            0.0, 0.0, (far + near) / (near - far), -1.0,
            0.0, 0.0, (2.0 * far * near) / (near - far), 0.0,
        ]
    }
    
    fn look_at_matrix(&self, eye: [f32; 3], center: [f32; 3], up: [f32; 3]) -> [f32; 16] {
        let f = normalize([
            center[0] - eye[0],
            center[1] - eye[1],
            center[2] - eye[2],
        ]);
        let s = normalize(cross(f, up));
        let u = cross(s, f);
        
        [
            s[0], u[0], -f[0], 0.0,
            s[1], u[1], -f[1], 0.0,
            s[2], u[2], -f[2], 0.0,
            -dot(s, eye), -dot(u, eye), dot(f, eye), 1.0,
        ]
    }
}

fn normalize(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / len, v[1] / len, v[2] / len]
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}