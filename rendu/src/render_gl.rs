use gl;
use std;
use std::ffi::{CString,CStr}; // for zero terminated string

//----------- Program struct for link ----------//
pub struct Program{
    id: gl::types::GLuint,
}

impl Program{

    //get program from shader load
    pub fn from_shader(shaders: &[Shader]) -> Result<Program, String>{
        let program_id = unsafe{gl::CreateProgram()};

        for shader in shaders{
            unsafe{ gl::AttachShader(program_id,shader.id());}
        }

        unsafe{ gl::LinkProgram(program_id);}

        let mut success: gl::types::GLint = 1;
        unsafe{
            gl::GetProgramiv(program_id,gl::LINK_STATUS, &mut success);
        }

        if success == 0{
            let mut len: gl::types::GLint = 0;
            unsafe{
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH,&mut len);
            }

            let error = whitespace_cstring(len as usize);

            unsafe{
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                )
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(Program{id: program_id})
    }

    pub fn id(&self) -> gl::types::GLuint{
        self.id
    }

    pub fn set_used(&self){
        unsafe{
            gl::UseProgram(self.id);
        }
    }

}

impl Drop for Program{
    fn drop(&mut self){
        unsafe{
            gl::DeleteProgram(self.id);
        }
    }
}

//----------- Shader struct for compile ----------//
pub struct Shader{
    id: gl::types::GLuint,
}

impl Shader{
    pub fn from_source(source: &CStr, shader_type: gl::types::GLuint)
                       -> Result<Shader,String>{

        // ? unwrap Result -> OK(id) to just id
        let id = shader_from_source(source,shader_type)?;
        Ok(Shader{id})
    }

    pub fn from_frag_source(source: &CStr)
                            -> Result<Shader,String>{

        Shader::from_source(source,gl::FRAGMENT_SHADER)
    }

    pub fn from_vert_source(source: &CStr)
                            -> Result<Shader,String>{

        Shader::from_source(source,gl::VERTEX_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint{
        self.id
    }
}


// shader resources clean up
impl Drop for Shader{
    fn drop(&mut self){
        unsafe{
            gl::DeleteShader(self.id);
        }
    }
}

// get shader from sources
fn shader_from_source(source: &CStr, shader_type: gl::types::GLuint)
                      -> Result<gl::types::GLuint, String>{

    let shader_id = unsafe{gl::CreateShader(shader_type)};
    //set shader sources
    unsafe{
        gl::ShaderSource(shader_id,1,&source.as_ptr(),std::ptr::null());
        gl::CompileShader(shader_id);
    }

    // shader compilation status
    let mut success: gl::types::GLint =1;
    unsafe{
        gl::GetShaderiv(shader_id,gl::COMPILE_STATUS, &mut success);
    }

    // success fail send error
    if success == 0{

        // get len
        let mut len:gl::types::GLint = 0;
        unsafe{
            gl::GetShaderiv(shader_id,gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = whitespace_cstring(len as usize);

        // ask OpenGL for shader info log 
        unsafe{
            gl::GetShaderInfoLog(
                shader_id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );
        }

        return Err(error.to_string_lossy().into_owned()); //conver error CString to rust String
    }

    //shader load correctly
    Ok(shader_id)

}

// convert String into CString for OpenGL error
fn whitespace_cstring(len: usize) -> CString{

    // allocate buffer 
    let mut buffer: Vec<u8>= Vec::with_capacity(len as usize+1);
    // fill space iter() -> iter, cycle() -> fill cycle iter
    buffer.extend([b' '].iter().cycle().take(len as usize));
    // convert into CString
    unsafe{ CString::from_vec_unchecked(buffer)}
}
