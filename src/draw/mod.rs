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

    /// Cairo context to draw on the surface.
    context     : Context,

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
        let context = Context::new(&surface);

        let drawer = Drawer {
            surface     : surface,
            context     : context,
            style       : Box::new(Default::default()),
        };

        drawer.fill_background();

        drawer
    }

    /// Cairo context reference.
    fn cr(&self) -> &Context {
        &self.context
    }

    /// Clear the surface and paing background with color specified in the
    /// style.
    fn fill_background(&self) {
        self.style.bg_color.set_source_in(self.cr());
        self.context.paint();
    }

    /// Paint grid with specified count of rows and columns.
    fn paint_grid(&self, cols: u16, rows: u16) {
        let style = &self.style;
        let cell_width = style.cell_size + style.line_width;

        // Length of vertical and horizontal line.
        let ver_length = cols as f64 * cell_width;
        let hor_length = rows as f64 * cell_width;

        // Set line color.
        style.line_color.set_source_in(self.cr());
        self.cr().set_line_width(style.line_width);

        // Draw rows.
        for i in 0..(rows + 1) {
            let offset = i as f64 * cell_width;
            println!("{0}", cell_width);

            self.cr().move_to(0.5, offset + 0.5);
            self.cr().line_to(hor_length + 0.5, offset + 0.5);
            self.cr().stroke();
        }

        // Draw cols.
        for i in 0..(cols + 1) {
            let offset = i as f64 * cell_width;

            self.cr().move_to(offset + 0.5, 0.5);
            self.cr().line_to(offset + 0.5, ver_length + 0.5);
            self.cr().stroke();
        }
    }
}

pub fn draw_fn(area: &DrawingArea, cr: &Context) -> Inhibit {
        unimplemented!()
}
