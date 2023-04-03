//! FRUG is intended to provide a similar abstraction layer over graphics programming as to how SDL does for C++, meaning that it should provide developers enough control and flexibility to implement their own architectures & design patterns, yet simplifying the process of working with graphics so developers won't have to worry about implementing all the repetitive tasks related to getting things to the screen.
//! 
//! FRUG aims to include the following features (unchecked items are the ones still under development):
//! - [x] Window management
//! - [ ]  Loading & rendering textures
//! - [ ]  Rotating textures
//! - [ ]  Scaling textures
//! - [ ]  Alpha blending for textures
//! - [ ]  Choosing a specific backend (aka. Direct X, Metal, Vulkan, etc.)
//! - [ ]  Writing and using custom shaders
//! - [ ]  Handle window state events
//! - [ ]  Handle Mouse input
//! - [ ]  Handle Keyboard input
//! - [ ]  Playing audio
//! - [ ]  Configure audio

use wgpu::{
    IndexFormat,
    PrimitiveTopology,
    ShaderSource
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow},
    window::Window
};

pub struct Inputs<'a> {
    pub source: ShaderSource<'a>,
    pub topology: PrimitiveTopology,
    pub strip_index_format: Option<IndexFormat>
}

/// Starts running your project.
/// 
/// Should receive a string which will be the title for the window created. It should also receive a loop which will be the main loop for your game/app.
/// * `window_title (&str)`         - The title for your window.
/// * `window_loop (static Fn())`   - The loop you want to execute with each frame.
/// 
/// # Example:
/// 
/// ```
/// let my_loop = || {
///     // your code
/// };
/// frug::run("My Game", my_loop);
/// ```
pub fn run<F: 'static + Fn()>(window_title: &str, window_loop: F) {
    // enable wgpu logging
    env_logger::init();

    // setup
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    window.set_title(window_title);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Act on events
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } 
            if window_id == window.id() => match event {
                // Window events
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => ()
            }
            _ => (),
        }

        window_loop();
    });
}