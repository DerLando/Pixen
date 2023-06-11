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
