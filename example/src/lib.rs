mod state;

#[cfg(target_arch = "wasm32")]
mod runner {
    use wasm_bindgen::prelude::*;

    use log::Level;

    use winit::event::*;
    use winit::event_loop::EventLoop;
    use winit::window::WindowBuilder;

    use super::state::State;

    #[wasm_bindgen(start)]
    pub async fn run() {
        console_error_panic_hook::set_once();
        console_log::init_with_level(Level::Info).unwrap();

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::<u32>::new(1920, 1080));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.body()?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");

        let mut state = State::new(&window).await;

        event_loop.run(move |event, _, _control_flow| match event {
            Event::RedrawRequested(..) => {
                state.update(event, &window);
                state.render();
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        });
    }
}
