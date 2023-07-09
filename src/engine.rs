use crate::pixel::PixelWindow;
use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

enum BuilderOutput {
    Stateless,
    Statefull,
    StatefullUpdate,
}

pub struct EngineBuilder<D, S>
where
    S: Sized,
    D: Fn(&mut PixelWindow) + 'static + Clone,
{
    title: String,
    width: u32,
    height: u32,
    state: Option<S>,
    draw_fn: D,
}

impl<D, S> EngineBuilder<D, S>
where
    S: Sized + 'static,
    D: Fn(&mut PixelWindow) + 'static + Clone,
{
    pub fn new(draw_fn: D) -> Self {
        Self {
            title: "Pixen".to_string(),
            width: 160,
            height: 144,
            state: None,
            draw_fn,
        }
    }

    pub fn with_title(mut self, title: impl AsRef<str>) -> Self {
        self.title = title.as_ref().to_owned();
        self
    }
    pub fn with_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }
    pub fn with_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }
    pub fn with_state(mut self, state: S) -> Self {
        self.state = Some(state);
        self
    }
    fn output_type(&self) -> BuilderOutput {
        if self.state.is_some() {
            // TODO: Check for update fn
            BuilderOutput::Statefull
        } else {
            BuilderOutput::Stateless
        }
    }

    pub fn build(self) -> Box<dyn PixenEngine> {
        match self.output_type() {
            BuilderOutput::Stateless => Box::new(StatelessEngine {
                title: self.title,
                width: self.width,
                height: self.height,
                draw_fn: self.draw_fn,
            }),
            BuilderOutput::Statefull => Box::new(StatefullEngine {
                title: self.title,
                width: self.width,
                height: self.height,
                state: self.state.unwrap(),
                draw_fn: self.draw_fn,
            }),
            _ => unreachable!(),
        }
    }
}

pub trait PixenEngine {
    fn run(&mut self);
}

pub struct StatelessEngine<D>
where
    D: Fn(&mut PixelWindow) + 'static + Clone,
{
    title: String,
    width: u32,
    height: u32,
    draw_fn: D,
}

impl<D> PixenEngine for StatelessEngine<D>
where
    D: Fn(&mut PixelWindow) + 'static + Clone,
{
    fn run(&mut self) {
        run_stateless(self.width, self.height, self.draw_fn.clone());
    }
}

pub struct StatefullEngine<D, S>
where
    S: Sized,
    D: Fn(&mut PixelWindow) + 'static + Clone,
{
    title: String,
    width: u32,
    height: u32,
    state: S,
    draw_fn: D,
}

impl<D, S> PixenEngine for StatefullEngine<D, S>
where
    S: Sized,
    D: Fn(&mut PixelWindow) + 'static + Clone,
{
    fn run(&mut self) {
        todo!()
    }
}

/// Run a stateless pixel engine. This engine only needs width, height and
/// a draw function that will be called everytime the window refreshes.
pub fn run_stateless<D: Fn(&mut PixelWindow) + 'static>(
    width: u32,
    height: u32,
    draw_fn: D,
) -> Result<(), Error> {
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
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
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

            #[cfg(feature = "image")]
            if input.key_pressed(VirtualKeyCode::S) {
                let window = PixelWindow::new(width, height, pixels.frame_mut());
                let screenshot = window.capture_to_image();
                screenshot.save("screenshot.png");
                println!("Saved screenshot");
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

/// Run a statelfull pixel engine. This engine needs width, height and
/// a draw function that will be called everytime the window refreshes. Additionally there is a
/// state parameter, which can be used inside of the draw function and will be updated inside of the
/// update function after every draw call.
pub fn run_statefull<D, U, S: 'static>(
    width: u32,
    height: u32,
    mut state: S,
    draw_fn: D,
    update_state_fn: U,
) -> Result<(), Error>
where
    D: Fn(&mut PixelWindow, &S) + 'static,
    U: Fn(&mut S) + 'static,
{
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
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(width, height, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            draw_fn(
                &mut PixelWindow::new(width, height, pixels.frame_mut()),
                &state,
            );
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

            #[cfg(feature = "image")]
            if input.key_pressed(VirtualKeyCode::S) {
                let window = PixelWindow::new(width, height, pixels.frame_mut());
                let screenshot = window.capture_to_image();
                screenshot.save("screenshot.png");
                println!("Saved screenshot");
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
            update_state_fn(&mut state);
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
