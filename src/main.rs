pub mod state;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use crate::state::State;
use futures::executor::block_on;

fn main() {
    env_logger::init();

    // create the event loop for the window
    let event_loop = EventLoop::new();

    // create the window
    let window_result = WindowBuilder::new()
        .with_title("")
        .build(&event_loop);
    if window_result.is_err() {
        eprintln!("Failed to create window: {}", window_result.unwrap_err());
        return;
    }
    let window = window_result.unwrap();

    // initialize the state
    let mut state = block_on(State::new(&window));

    // start the event loop
    event_loop.run(move |event, _, control_flow|
        match event {
            Event::WindowEvent {
                event: ref window_event,
                window_id
            } if window_id == window.id() => if !state.input(window_event) {
                    match window_event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput {
                            input,
                            ..
                        } => {
                            match input {
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                } => *control_flow = ControlFlow::Exit,
                                _ => {}
                            }
                        }
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            state.resize(**new_inner_size);
                        }
                        _ => {} // window_event
                    }
                },
            Event::RedrawRequested(_) => {
                state.update();
                match state.render() {
                    Ok(_) => { }
                    // recreate the swap chain if it is lost
                    Err(wgpu::SwapChainError::Lost) => state.resize(state.size),
                    // If the system runs out of memory, exit
                    Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless it is manually requested.
                window.request_redraw();
            }
            _ => {} // event
    });
}
