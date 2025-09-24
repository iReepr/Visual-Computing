// Uncomment these following global attributes to silence most warnings of "low" interest:
/*
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_code)]
#![allow(unused_mut)]
#![allow(unused_unsafe)]
#![allow(unused_variables)]
*/
extern crate nalgebra_glm as glm;
use std::{ mem, ptr, os::raw::c_void };
use std::thread;
use std::sync::{Mutex, Arc, RwLock};

mod shader;
mod util;
mod mesh;
mod scene_graph;

use scene_graph::SceneNode;
use glutin::event::{Event, WindowEvent, DeviceEvent, KeyboardInput, ElementState::{Pressed, Released}, VirtualKeyCode::{self, *}};
use glutin::event_loop::ControlFlow;

// initial window size
const INITIAL_SCREEN_W: u32 = 800;
const INITIAL_SCREEN_H: u32 = 600;

// == // Helper functions to make interacting with OpenGL a little bit prettier. You *WILL* need these! // == //

// Get the size of an arbitrary array of numbers measured in bytes
// Example usage:  byte_size_of_array(my_array)
fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

// Get the OpenGL-compatible pointer to an arbitrary array of numbers
// Example usage:  pointer_to_array(my_array)
fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

// Get the size of the given type in bytes
// Example usage:  size_of::<u64>()
fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

// Get an offset in bytes for n units of type T, represented as a relative pointer
// Example usage:  offset::<u64>(4)
fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

// Get a null pointer (equivalent to an offset of 0)
// ptr::null()


// == // Generate your VAO here
unsafe fn create_vao(vertices: &Vec<f32>, indices: &Vec<u32>, colors: &Vec<f32>, normals: &Vec<f32>) -> u32 {
    // Implement me!

    // Also, feel free to delete comments :)

    // This should:
    // * Generate a VAO and bind it
    let mut vao_id = 0;
    gl::GenVertexArrays(1, &mut vao_id);
    gl::BindVertexArray(vao_id);

    // * Generate a VBO and bind it
    let mut vbo_id = 0;
    gl::GenBuffers(1, &mut vbo_id);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);

    // * Fill it with data
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(vertices),
        pointer_to_array(vertices),
        gl::STATIC_DRAW,
    );

    // * Configure a VAP for the data and enable it
    gl::VertexAttribPointer(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        0,
        std::ptr::null()
    );
    gl::EnableVertexAttribArray(0);

    // Create and fill a VBO for colors, configure a VAP for it and enable it
    let mut vbo_colors = 0;
    gl::GenBuffers(1, &mut vbo_colors);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo_colors);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(colors),
        pointer_to_array(colors),
        gl::STATIC_DRAW,
    );
    gl::VertexAttribPointer(
        1,
        4,
        gl::FLOAT,
        gl::FALSE,
        0,
        ptr::null(),
    );
    gl::EnableVertexAttribArray(1);

    // Generate a normals VBO, fill it with data, configure a VAP for it and enable it
    let mut vbo_normals = 0;
    gl::GenBuffers(1, &mut vbo_normals);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo_normals);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(normals),
        pointer_to_array(normals),
        gl::STATIC_DRAW,
    );
    gl::VertexAttribPointer(
        2,
        3,
        gl::FLOAT,
        gl::FALSE,
        0,
        ptr::null(),
    );
    gl::EnableVertexAttribArray(2);

    // * Generate a IBO and bind it
    let mut ibo_id = 0;
    gl::GenBuffers(1, &mut ibo_id);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo_id);

    // * Fill it with data
     gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        byte_size_of_array(indices),
        pointer_to_array(indices),
        gl::STATIC_DRAW,
    );

    gl::BindVertexArray(0);
    // * Return the ID of the VAO
    vao_id
}


unsafe fn draw_scene(
    node: &scene_graph::SceneNode,
    view_projection_matrix: &glm::Mat4,
    transformation_so_far: &glm::Mat4,
    shader_program: u32
) {
    let to_origin = glm::translation(&-node.reference_point);

    let rot_x = glm::rotation(node.rotation.x.to_radians(), &glm::vec3(1.0, 0.0, 0.0));
    let rot_y = glm::rotation(node.rotation.y.to_radians(), &glm::vec3(0.0, 1.0, 0.0));
    let rot_z = glm::rotation(node.rotation.z.to_radians(), &glm::vec3(0.0, 0.0, 1.0));
    let rotation = rot_x * rot_y * rot_z;

    let from_origin = glm::translation(&node.reference_point);
    let translation = glm::translation(&node.position);

    let model_transform = transformation_so_far * translation * from_origin * rotation * to_origin;

    if node.vao_id != 0 && node.index_count > 0 {
      
        let mvp_matrix = view_projection_matrix * model_transform;

        let loc = gl::GetUniformLocation(shader_program, b"mvp\0".as_ptr() as *const i8);
        gl::UniformMatrix4fv(loc, 1, gl::FALSE, mvp_matrix.as_ptr());

     
        gl::BindVertexArray(node.vao_id);
        gl::DrawElements(
            gl::TRIANGLES,
            node.index_count,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );
    }

    
    for &child in &node.children {
        draw_scene(&*child, view_projection_matrix, &model_transform, shader_program);
    }
}



fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize::new(INITIAL_SCREEN_W, INITIAL_SCREEN_H));
    let cb = glutin::ContextBuilder::new()
        .with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();
    // Uncomment these if you want to use the mouse for controls, but want it to be confined to the screen and/or invisible.
    // windowed_context.window().set_cursor_grab(true).expect("failed to grab cursor");
    // windowed_context.window().set_cursor_visible(false);

    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Make a reference of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Set up shared tuple for tracking mouse movement between frames
    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    // Make a reference of this tuple to send to the render thread
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    // Set up shared tuple for tracking changes to the window size
    let arc_window_size = Arc::new(Mutex::new((INITIAL_SCREEN_W, INITIAL_SCREEN_H, false)));
    // Make a reference of this tuple to send to the render thread
    let window_size = Arc::clone(&arc_window_size);

    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let render_thread = thread::spawn(move || {
        // Acquire the OpenGL Context and load the function pointers.
        // This has to be done inside of the rendering thread, because
        // an active OpenGL context cannot safely traverse a thread boundary
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        let mut window_aspect_ratio = INITIAL_SCREEN_W as f32 / INITIAL_SCREEN_H as f32;

        // Set up openGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::CULL_FACE);
            gl::Disable(gl::MULTISAMPLE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());

            // Print some diagnostics
            println!("{}: {}", util::get_gl_string(gl::VENDOR), util::get_gl_string(gl::RENDERER));
            println!("OpenGL\t: {}", util::get_gl_string(gl::VERSION));
            println!("GLSL\t: {}", util::get_gl_string(gl::SHADING_LANGUAGE_VERSION));
        }

        // == // Set up your VAO around here
        let terrain = mesh::Terrain::load("resources/lunarsurface.obj");
    
        let terrain_vao = unsafe { create_vao(&terrain.vertices, &terrain.indices, &terrain.colors, &terrain.normals) };

       
        let helicopter = mesh::Helicopter::load("resources/helicopter.obj");
        let heli_body_vao      = unsafe { create_vao(&helicopter.body.vertices, &helicopter.body.indices, &helicopter.body.colors, &helicopter.body.normals) };
        let heli_door_vao      = unsafe { create_vao(&helicopter.door.vertices, &helicopter.door.indices, &helicopter.door.colors, &helicopter.door.normals) };
        let heli_main_rotor_vao = unsafe { create_vao(&helicopter.main_rotor.vertices, &helicopter.main_rotor.indices, &helicopter.main_rotor.colors, &helicopter.main_rotor.normals) };
        let heli_tail_rotor_vao = unsafe { create_vao(&helicopter.tail_rotor.vertices, &helicopter.tail_rotor.indices, &helicopter.tail_rotor.colors, &helicopter.tail_rotor.normals) };
        
        // Terrain
        let mut terrain_node = SceneNode::from_vao(terrain_vao, terrain.index_count);

        // Helicopter nodes
        let mut heli_root_node: mem::ManuallyDrop<std::pin::Pin<Box<SceneNode>>>  = SceneNode::new(); // root of the helicopter, empty node
        let mut body_node: mem::ManuallyDrop<std::pin::Pin<Box<SceneNode>>>      = SceneNode::from_vao(heli_body_vao, helicopter.body.index_count);
        let mut door_node: mem::ManuallyDrop<std::pin::Pin<Box<SceneNode>>>      = SceneNode::from_vao(heli_door_vao, helicopter.door.index_count);
        let mut main_rotor_node: mem::ManuallyDrop<std::pin::Pin<Box<SceneNode>>> = SceneNode::from_vao(heli_main_rotor_vao, helicopter.main_rotor.index_count);
        let mut tail_rotor_node: mem::ManuallyDrop<std::pin::Pin<Box<SceneNode>>> = SceneNode::from_vao(heli_tail_rotor_vao, helicopter.tail_rotor.index_count);

        let mut helicopter_root_node = SceneNode::new();

        helicopter_root_node.add_child(&*body_node);
        helicopter_root_node.add_child(&*door_node);
        helicopter_root_node.add_child(&*main_rotor_node);
        helicopter_root_node.add_child(&*tail_rotor_node);

        let mut scene_root = SceneNode::new();

        scene_root.add_child(&*terrain_node);
        terrain_node.add_child(&*helicopter_root_node);

        tail_rotor_node.reference_point = glm::vec3(0.35, 2.3, 10.4);
        main_rotor_node.reference_point = glm::vec3(0.0, 0.0, 0.0);
        body_node.reference_point = glm::vec3(0.0, 0.0, 0.0);
        door_node.reference_point = glm::vec3(0.0, 0.0, 0.0);

        body_node.position = glm::vec3(0.0, 2.0, 0.0); // monte le corps du helico
        main_rotor_node.rotation.y = 90.0; // tourne le rotor principal



        scene_root.print();
        helicopter_root_node.print();

        // == // Set up your shaders here

        // Basic usage of shader helper:
        // The example code below creates a 'shader' object.
        // It which contains the field `.program_id` and the method `.activate()`.
        // The `.` in the path is relative to `Cargo.toml`.
        // This snippet is not enough to do the exercise, and will need to be modified (outside
        // of just using the correct path), but it only needs to be called once

        /*
        let simple_shader = unsafe {
            shader::ShaderBuilder::new()
                .attach_file("./path/to/simple/shader.file")
                .link()
        };
        */

        let simple_shader = unsafe {
            shader::ShaderBuilder::new()
                .attach_file("./shaders/simple.vert")
                .attach_file("./shaders/simple.frag")
                .link()
        };

        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        let mut z: f32 = 5.0;

        let mut yaw: f32 = 0.0; 
        let mut pitch: f32 = 0.0;

        // Used to demonstrate keyboard handling for exercise 2.
        let mut _arbitrary_number = 0.0; // feel free to remove


        // The main rendering loop
        let first_frame_time = std::time::Instant::now();
        let mut previous_frame_time = first_frame_time;
        loop {
            // Compute time passed since the previous frame and since the start of the program
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(previous_frame_time).as_secs_f32();
            previous_frame_time = now;

            // Handle resize events
            if let Ok(mut new_size) = window_size.lock() {
                if new_size.2 {
                    context.resize(glutin::dpi::PhysicalSize::new(new_size.0, new_size.1));
                    window_aspect_ratio = new_size.0 as f32 / new_size.1 as f32;
                    (*new_size).2 = false;
                    println!("Window was resized to {}x{}", new_size.0, new_size.1);
                    unsafe { gl::Viewport(0, 0, new_size.0 as i32, new_size.1 as i32); }
                }
            }

            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    match key {
                        // The `VirtualKeyCode` enum is defined here:
                        //    https://docs.rs/winit/0.25.0/winit/event/enum.VirtualKeyCode.html
                        
                        // Movement (WASD + Space/LShift)
                        VirtualKeyCode::W => z -= 15.0 * delta_time,
                        VirtualKeyCode::S => z += 15.0 * delta_time,
                        VirtualKeyCode::A => x -= 15.0 * delta_time,
                        VirtualKeyCode::D => x += 15.0 * delta_time,
                        VirtualKeyCode::Space => y += 10.0 * delta_time,
                        VirtualKeyCode::LShift => y -= 10.0 * delta_time,

                        // Rotations
                        VirtualKeyCode::Up => pitch += 30.0 * delta_time,
                        VirtualKeyCode::Down => pitch -= 30.0 * delta_time,
                        VirtualKeyCode::Left => yaw -= 30.0 * delta_time,
                        VirtualKeyCode::Right => yaw += 30.0 * delta_time,

                        // default handler:
                        _ => { }
                    }
                }
            }
            // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
            if let Ok(mut delta) = mouse_delta.lock() {

                // == // Optionally access the accumulated mouse movement between
                // == // frames here with `delta.0` and `delta.1`

                *delta = (0.0, 0.0); // reset when done
            }

            // == // Please compute camera transforms here (exercise 2 & 3)
            /*
            let translation : glm::Mat4 = glm::translation(&glm::vec3(0.0, 0.0, -5.0));
            let transform: glm::Mat4 = projection * translation;
            */

            let projection: glm::Mat4 = glm::perspective(
                window_aspect_ratio,
                45.0_f32.to_radians(),
                1.0,
                1000.0,
            );

            let scale = glm::scaling(&glm::vec3(1.0, 1.0, 1.0));           

            let translation = glm::translation(&glm::vec3(-x, -y, -z));
           
            let rot_x = glm::rotation(pitch.to_radians(), &glm::vec3(1.0, 0.0, 0.0));
            let rot_y = glm::rotation(yaw.to_radians(), &glm::vec3(0.0, 1.0, 0.0));
            let rotation = rot_x * rot_y;
         
            let transform = projection * rotation * translation * scale;

         
            unsafe {
                // Clear the color and depth buffers
                gl::ClearColor(0.035, 0.046, 0.078, 1.0); // night sky
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                // == // Issue the necessary gl:: commands to draw your scene here
                let oscillation = elapsed.sin(); 
                simple_shader.activate();
                
                let identity = glm::identity();
                draw_scene(&scene_root, &transform, &identity, simple_shader.program_id);
                gl::BindVertexArray(0);

            }

            // Display the new color buffer on the display
            context.swap_buffers().unwrap(); // we use "double buffering" to avoid artifacts
        }
    });


    // == //
    // == // From here on down there are only internals.
    // == //


    // Keep track of the health of the rendering thread
    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if !render_thread.join().is_ok() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });

    // Start the event loop -- This is where window events are initially handled
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Terminate program if render thread panics
        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent { event: WindowEvent::Resized(physical_size), .. } => {
                println!("New window size received: {}x{}", physical_size.width, physical_size.height);
                if let Ok(mut new_size) = arc_window_size.lock() {
                    *new_size = (physical_size.width, physical_size.height, true);
                }
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent { event: WindowEvent::KeyboardInput {
                    input: KeyboardInput { state: key_state, virtual_keycode: Some(keycode), .. }, .. }, .. } => {

                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        },
                        Pressed => {
                            if !keys.contains(&keycode) {
                                keys.push(keycode);
                            }
                        }
                    }
                }

                // Handle Escape and Q keys separately
                match keycode {
                    Escape => { *control_flow = ControlFlow::Exit; }
                    Q      => { *control_flow = ControlFlow::Exit; }
                    _      => { }
                }
            }
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                // Accumulate mouse movement
                if let Ok(mut position) = arc_mouse_delta.lock() {
                    *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
                }
            }
            _ => { }
        }
    });
}
