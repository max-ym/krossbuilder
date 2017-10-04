use super::page::Page;

use cairo::{Context, ImageSurface, Surface};
use gtk::{DrawingArea, Inhibit};

#[derive(Default)]
pub struct Color {
    pub r   : f64,
    pub g   : f64,
    pub b   : f64,
}

pub struct PageStyle {
    pub bg_color        : Color,
    pub line_color      : Color,
    pub line_width      : f64,
    pub cell_size       : f64,
}

/// Drawing functions to render page elements to surface and show it when
/// needed on the screen.
pub struct Drawer {
    /// Surface with prerendered stuff.
    surface     : ImageSurface,
    style       : Box<PageStyle>,
}

impl Color {

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r:r, g:g, b:b }
    }

    /// Set source in Cairo context.
    pub fn set_source_in(&self, cr: &Context) {
        cr.set_source_rgb(self.r, self.g, self.b);
    }
}

impl Default for PageStyle {

    fn default() -> Self {
        PageStyle {
            bg_color    : Color::new(1., 1., 1.),   // White
            line_color  : Color::new(0., 0., 0.),   // Black
            line_width  : 1.,
            cell_size   : 48.,
        }
    }
}

impl Drawer {

    /// Create new drawer.
    pub fn new(cr: &Context) -> Self {
        use cairo::Format::Rgb24;
        let surface = ImageSurface::create(Rgb24, 1, 1).unwrap();

        Drawer {
            surface     : surface,
            style       : Box::new(Default::default()),
        }
    }

    fn fill_background(&self, color: &Color) {
        let cr = Context::new(&self.surface);

        color.set_source_in(&cr);
        cr.paint();
    }
}

pub fn draw_fn(area: &DrawingArea, cr: &Context) -> Inhibit {
        unimplemented!()
}
