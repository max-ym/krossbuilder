use super::page::Page;

use cairo::Context;
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

/// Page style currently set as prefered.
fn page_style() -> PageStyle {
    // TODO default one is returned. Read actual settings in the future.
    Default::default()
}

fn page() -> Page {
    // TEST
    Page::new(10, 10)
}

pub fn draw_fn(area: &DrawingArea, cr: &Context) -> Inhibit {
    let paint_background = |style: &PageStyle| {
        style.bg_color.set_source_in(cr);
        cr.paint();
    };

    let paint_grid = |style: &PageStyle, rows: u16, cols: u16| {
        let cell_width = style.cell_size + style.line_width;

        // Length of vertical and horizontal line.
        let ver_length = cols as f64 * cell_width;
        let hor_length = rows as f64 * cell_width;

        // Set line color.
        style.line_color.set_source_in(cr);
        cr.set_line_width(style.line_width);

        for i in 0..(rows + 1) {
            let offset = i as f64 * cell_width;

            cr.move_to(0., offset);
            cr.line_to(hor_length, offset);
            cr.stroke();
        }

        for i in 0..(cols + 1) {
            let offset = i as f64 * cell_width;

            cr.move_to(offset, 0.);
            cr.line_to(offset, ver_length);
            cr.stroke();
        }
    };

    let style = page_style();
    let page = page();

    paint_background(&style);
    paint_grid(&style, page.cols(), page.rows());


    Inhibit(false)
}
