extern crate gl;
extern crate libc;

use gl::types::*;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;

fn create_shader(shader_source: &str, shader_type: GLenum) -> u32 {
    let c_str = CString::new(shader_source.as_bytes())
        .expect("Could not create c string from shader source.");

    unsafe {
        let shader = gl::CreateShader(shader_type);
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        if status != (gl::TRUE as GLint) {
            let mut length = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut length);
            let mut message = Vec::with_capacity(length as usize);
            message.set_len((length as usize) - 1);
            gl::GetShaderInfoLog(shader, length, ptr::null_mut(), message.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(&message).ok().expect("ShaderInfoLog not valid utf8."));
        }
        shader
    }
}

pub fn create_program(vs_src: &str, fs_src: &str) -> u32 {
    unsafe {
        let program = gl::CreateProgram();
        let vs = create_shader(vs_src, gl::VERTEX_SHADER);
        let fs = create_shader(fs_src, gl::FRAGMENT_SHADER);
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        let mut success = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == gl::FALSE as i32 {
            let mut length = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut length);
            let mut log = Vec::with_capacity(length as usize);
            log.set_len(length as usize - 1);
            gl::GetProgramInfoLog(program, length, ptr::null_mut(), log.as_mut_ptr() as *mut gl::types::GLchar);
            panic!("{}", str::from_utf8(&log).ok().expect("ProgramInfoLog not valid utf8."));
        }

        gl::DetachShader(program, vs);
        gl::DetachShader(program, fs);

        program
    }
}

pub fn create_object(location: u32, vertices: &[f32]) -> u32 {
    unsafe {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       mem::transmute(&vertices[0]),
                       gl::STATIC_DRAW);
        gl::VertexAttribPointer(location, 4, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::EnableVertexAttribArray(location);

        vao
    }
}
