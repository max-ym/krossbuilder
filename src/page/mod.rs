/// Object position in the page. Usually this is a cell position or a
/// top left corner of large objects like images.
pub struct Position {
    pub x   : u16,
    pub y   : u16,
}

/// Page.
pub struct Page {
    width   : u16,
    height  : u16,
}

/// Elements of the page.
mod elems;
pub use self::elems::*;

/// Cell of any type that is used on the page.
mod cells;
pub use self::cells::*;
