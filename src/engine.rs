use crate::pixel::PixelWindow;
use std::{marker::PhantomData, ops::Deref};
use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

pub(crate) trait PixelEngine {
    fn draw(&self, window: &mut PixelWindow);
}

pub(crate) trait StatefullPixelEngine: PixelEngine {
    fn update(&mut self);
}

pub(crate) struct StatelessEngine<D: Fn(&mut PixelWindow)> {
    width: u32,
    height: u32,
    draw_fn: D,
}

impl<D> PixelEngine for StatelessEngine<D>
where
    D: Fn(&mut PixelWindow)
{
    fn draw(&self, window: &mut PixelWindow) {
        (self.draw_fn)(window)
    }
}

pub(crate) struct StatefullEngine<S> {
    width: u32,
    height: u32,
    marker: PhantomData<S>,
}

/// Run a stateless pixel engine. This engine only needs width, height and
/// a draw function that will be called everytime the window refreshes.
pub fn run_stateless<D: Fn(&mut PixelWindow) + 'static>(width: u32, height: u32, draw_fn: D) -> Result<(), Error> {
        env_logger::init();
        let event_loop = EventLoop::new();
        let mut input = WinitInputHelper::new();
        let window = {
            let size = LogicalSize::new(width as f64, height as f64);
            WindowBuilder::new()
                .with_title("Pixen")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(width, height, surface_texture)?
        };

        event_loop.run(move |event, _, control_flow| {
            // Draw the current frame
            if let Event::RedrawRequested(_) = event {
                draw_fn(&mut PixelWindow::new(width, height, pixels.frame_mut()));
                if let Err(err) = pixels.render() {
                    log_error("pixels.render", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Handle input events
            if input.update(&event) {
                // Close events
                if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                // Resize the window
                if let Some(size) = input.window_resized() {
                    if let Err(err) = pixels.resize_surface(size.width, size.height) {
                        log_error("pixels.resize_surface", err);
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }

                // Update internal state and request a redraw
                window.request_redraw();
            }
        })
    
}


fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

