/// Image on the page.
pub struct Image {
    width   : u16,
    height  : u16,
}

/// Definition of some word.
pub struct Definition {
    text    : String
}

/// Arrow from some definition.
pub struct Arrow {
    dir     : ArrowDirection,
}

/// A letter on the page in individual cell.
pub struct Letter {
    letter  : char,
}

/// Arrow direction.
pub enum ArrowDirection {

    TopUp,
    TopLeft,
    TopRight,

    LeftUp,
    LeftDown,
    Left,

    RightUp,
    RightDown,
    Right,

    BottomDown,
    BottomLeft,
    BottomRight,
}
