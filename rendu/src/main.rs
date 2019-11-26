extern crate sdl2;
extern crate gl;

//module reference
pub mod render_gl;

use std::ffi::CString;

fn main() {

    let sdl = sdl2::init().unwrap(); //init SDL2 no quit auto
    let video_subsystem = sdl.video().unwrap(); // new ref of sdl

    // define OpenGL Version to use
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4,5);

    let window = video_subsystem
        .window("Test",800,600) //window builder
        .opengl() // opengl flag
        .resizable()
        .build() //create window
        .unwrap(); // handle error

    // gl_context
    let _gl_context = window.gl_create_context().unwrap();

    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe{
        gl::Viewport(0,0,800,600);
        gl::ClearColor(0.3,0.3,0.5,1.0);
    }

    // compile shaders
    let vertex_shader = render_gl::Shader::from_vert_source(
        &CString::new(include_str!("vertex.shd")).unwrap()
    ).unwrap();


    let fragment_shader = render_gl::Shader::from_frag_source(
        &CString::new(include_str!("fragment.shd")).unwrap()
    ).unwrap();

    // link shades to program
    let shader_program = render_gl::Program::from_shader(
        &[vertex_shader,fragment_shader]
    ).unwrap();

    shader_program.set_used();

    let mut event_pump = sdl.event_pump().unwrap(); //receive events.
    'main: loop{
        for event in event_pump.poll_iter(){
            //handle event here
            match event{
                sdl2::event::Event::Quit{..} => break 'main,
                _ => {},
            }
        }

        // render window here
        unsafe{
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.gl_swap_window();
    }
}
