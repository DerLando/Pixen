# Pixen

A minimal **Pix**el **En**gine

## Motivation and Design

Oftentimes I find myself looking at intereseting graphics related programming content for low-level-ish languages like *C* or *C++* and want to do them in a language I am more comfortable with, so *rust*. I could not find a *truly minimal* pixel engine, which allows me push pixels to a window with only a few lines of code, so I created my own on top of the [Pixels](https://crates.io/crates/pixels) crate.
With only the default features enabled, this library should take care of all the low-level details of setting up a window, handling input/resizing, or reading the pixel buffer. The code a user has to write should be only focused on the pixels themselves.

## Example

A minimal example drawing a single white pixel

```no_run
use pixen::run_stateless;

// Open a window 200 by 200 pixels wide and run the draw closure
run_stateless(200, 200, |window| {
  // draws a white pixel at 50,50
  window.set_pixel(50, 50, [255, 255, 255, 255]);
});
```

## Roadmap

I will probably only work on this from time to time, when a feature feels missing for a tutorial I'm working on. A few fetures that would be nice to have already come to mind though, so here is an unordered roadmap

- [ ] Different windowing libraries insteaf of just using winit as the default. Probably it would make sense to have those behind a feature flag and having the default implementation just take and write to a pixel buffer.
- [ ] Support for input handling. Right now only the escape key is tracked to close the window
- [ ] Builder API that allows to initialize an engine in multiple steps, which in turn will make the app code relying on this library more readable
- [ ] Implementing scripting languages behind feature flags. This would allow hot-reloading of the pixel content generating code.
- [ ] Compile to WASM, as a feature, which would allow using the engine on the web in an easy way. For this an example would be great, too.
- [ ] Feature gated primitive drawing functions. By default only the `set_pixel` function should be available, so users can build their own drawing algorithms how they like. But it would also be nice to offer a few common drawing routines behind a feature flag, like f.e. *line*, *circle*, *rectangle*, etc.
- [ ] Publish to [crates.io](https://crates.io). The name is still free and I should do it once I'm comfortable with the initial public API.
- [ ] Built-in support for multiple color spaces, instead of everything assuming rgba under the hood.
