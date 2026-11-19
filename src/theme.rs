//! Colours and metrics of the mid-90s corporate look.

/// An RGB colour, stored as `0x00RRGGBB`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Color(pub u32);

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color(((r as u32) << 16) | ((g as u32) << 8) | b as u32)
    }
    pub fn r(self) -> u8 {
        (self.0 >> 16) as u8
    }
    pub fn g(self) -> u8 {
        (self.0 >> 8) as u8
    }
    pub fn b(self) -> u8 {
        self.0 as u8
    }
}

/// The corporate theme: silver, navy, and teal, exactly as the
/// 1995 style guide (a three-ring binder) prescribed.
#[derive(Clone, Copy, Debug)]
pub struct Theme {
    /// Window and control face — the famous silver.
    pub face: Color,
    /// Desktop behind the windows — teal, of course.
    pub desktop: Color,
    /// Outermost light edge of a raised bevel.
    pub highlight: Color,
    /// Inner light edge of a raised bevel.
    pub light: Color,
    /// Inner dark edge of a raised bevel.
    pub shadow: Color,
    /// Outermost dark edge of a raised bevel.
    pub dark_shadow: Color,
    /// Text on the face colour.
    pub text: Color,
    /// Greyed-out text.
    pub disabled_text: Color,
    /// Editable field background.
    pub field: Color,
    /// Title bar of the active window (left colour of the gradient).
    pub title_active: Color,
    /// Title bar gradient, right-hand colour.
    pub title_active_end: Color,
    /// Title bar of an inactive window.
    pub title_inactive: Color,
    /// Text on a title bar.
    pub title_text: Color,
    /// Selection highlight (list boxes, menus).
    pub selection: Color,
    /// Text on the selection colour.
    pub selection_text: Color,
    /// The navy used for progress-bar blocks and the J mark.
    pub accent: Color,
}

impl Theme {
    pub fn corporate() -> Theme {
        Theme {
            face: Color::rgb(0xC0, 0xC0, 0xC0),
            desktop: Color::rgb(0x00, 0x80, 0x80),
            highlight: Color::rgb(0xFF, 0xFF, 0xFF),
            light: Color::rgb(0xDF, 0xDF, 0xDF),
            shadow: Color::rgb(0x80, 0x80, 0x80),
            dark_shadow: Color::rgb(0x00, 0x00, 0x00),
            text: Color::rgb(0x00, 0x00, 0x00),
            disabled_text: Color::rgb(0x80, 0x80, 0x80),
            field: Color::rgb(0xFF, 0xFF, 0xFF),
            title_active: Color::rgb(0x00, 0x00, 0x80),
            title_active_end: Color::rgb(0x10, 0x84, 0xD0),
            title_inactive: Color::rgb(0x80, 0x80, 0x80),
            title_text: Color::rgb(0xFF, 0xFF, 0xFF),
            selection: Color::rgb(0x00, 0x00, 0x80),
            selection_text: Color::rgb(0xFF, 0xFF, 0xFF),
            accent: Color::rgb(0x00, 0x00, 0x80),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::corporate()
    }
}
