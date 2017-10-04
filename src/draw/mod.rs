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
    pub cell_font_size  : f64,
    pub cell_font_color : Color,
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
            bg_color        : Color::new(1., 1., 1.),   // White
            line_color      : Color::new(0., 0., 0.),   // Black
            line_width      : 1.,
            cell_size       : 48.,
            cell_font_size  : 10.,
            cell_font_color : Color::new(0., 0., 0.),   // Black
        }
    }
}

impl Drawer {

    /// Create new drawer with surface size 1x1.
    pub fn new() -> Self {
        Self::new_with_size(1, 1)
    }

    /// Create new drawer with specified size of surface.
    pub fn new_with_size(width: usize, height: usize) -> Self {
        use cairo::Format::Rgb24;
        let surface = ImageSurface::create(Rgb24, 1, 1).unwrap();
        let context = Context::new(&surface);

        Drawer {
            surface     : surface,
            context     : context,
            style       : Box::new(Default::default()),
        }
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

    /// Paint letter at specified cell.
    fn paint_letter(&self, row: u16, col: u16, letter: char) {
        use cairo::{FontFace, FontSlant, FontWeight};

        let row = row as f64;
        let col = col as f64;

        // Cell center point relative to cell start coords.
        let cell_center = self.style.cell_size / 2.;

        // Find cell absolute coords on the grid.
        let cell_x = col * (self.style.cell_size + self.style.line_width);
        let cell_y = row * (self.style.cell_size + self.style.line_width);

        let cr = self.cr();

        // Setup font parameters.
        cr.set_font_size(self.style.cell_font_size);
        self.style.cell_font_color.set_source_in(cr);
        cr.set_font_face(FontFace::toy_create(
                "monospace",
                FontSlant::Normal,
                FontWeight::Normal,
        ));

        // Wrap a letter in the string.
        let mut s = String::with_capacity(8);
        s.push(letter);

        // Calculate letter position offsets to center it.
        let txtext = cr.text_extents(s.as_str());
        let letter_offset_x = -txtext.width  / 2.;
        let letter_offset_y = -txtext.height / 2.;
        let letter_x = cell_center + letter_offset_x;
        let letter_y = cell_center + letter_offset_y;

        // Find exact absolute coords of a letter.
        let x = cell_x + letter_x;
        let y = cell_y + letter_y;

        cr.move_to(x, y);
        cr.show_text(s.as_str());
    }
}

pub fn draw_fn(area: &DrawingArea, cr: &Context) -> Inhibit {
        unimplemented!()
}
