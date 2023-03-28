//! This is the FRUG crate

use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow},
    window::Window
};

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