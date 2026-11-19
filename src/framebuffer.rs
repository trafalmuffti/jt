//! The software renderer: a pixel buffer plus the drawing primitives
//! every widget is built from — fills, lines, bevels, and bitmap text.

use crate::font;
use crate::theme::Color;

/// An axis-aligned rectangle (x, y = top-left corner).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub const fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect { x, y, w, h }
    }
    pub fn contains(&self, px: i32, py: i32) -> bool {
        px >= self.x && py >= self.y && px < self.x + self.w && py < self.y + self.h
    }
}

/// A `u32` pixel buffer (`0x00RRGGBB`), row-major, origin at top-left.
pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    pub pixels: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Framebuffer {
        Framebuffer {
            width,
            height,
            pixels: vec![0; (width.max(0) * height.max(0)) as usize],
        }
    }

    pub fn clear(&mut self, c: Color) {
        for p in self.pixels.iter_mut() {
            *p = c.0;
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, c: Color) {
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            self.pixels[(y * self.width + x) as usize] = c.0;
        }
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, c: Color) {
        let x0 = x.max(0);
        let y0 = y.max(0);
        let x1 = (x + w).min(self.width);
        let y1 = (y + h).min(self.height);
        for yy in y0..y1 {
            let row = (yy * self.width) as usize;
            for xx in x0..x1 {
                self.pixels[row + xx as usize] = c.0;
            }
        }
    }

    pub fn hline(&mut self, x: i32, y: i32, len: i32, c: Color) {
        self.fill_rect(x, y, len, 1, c);
    }

    pub fn vline(&mut self, x: i32, y: i32, len: i32, c: Color) {
        self.fill_rect(x, y, 1, len, c);
    }

    /// One-pixel outline of a rectangle.
    pub fn outline(&mut self, x: i32, y: i32, w: i32, h: i32, c: Color) {
        self.hline(x, y, w, c);
        self.hline(x, y + h - 1, w, c);
        self.vline(x, y, h, c);
        self.vline(x + w - 1, y, h, c);
    }

    /// The classic two-tone raised bevel: bright edges at top/left,
    /// dark edges at bottom/right.
    pub fn bevel_raised(&mut self, t: &crate::Theme, x: i32, y: i32, w: i32, h: i32) {
        // Outer edge
        self.hline(x, y, w - 1, t.highlight);
        self.vline(x, y, h - 1, t.highlight);
        self.hline(x, y + h - 1, w, t.dark_shadow);
        self.vline(x + w - 1, y, h, t.dark_shadow);
        // Inner edge
        self.hline(x + 1, y + 1, w - 3, t.light);
        self.vline(x + 1, y + 1, h - 3, t.light);
        self.hline(x + 1, y + h - 2, w - 2, t.shadow);
        self.vline(x + w - 2, y + 1, h - 2, t.shadow);
    }

    /// The inverse bevel, for fields, list boxes, and status panes.
    pub fn bevel_sunken(&mut self, t: &crate::Theme, x: i32, y: i32, w: i32, h: i32) {
        // Outer edge
        self.hline(x, y, w - 1, t.shadow);
        self.vline(x, y, h - 1, t.shadow);
        self.hline(x, y + h - 1, w, t.highlight);
        self.vline(x + w - 1, y, h, t.highlight);
        // Inner edge
        self.hline(x + 1, y + 1, w - 3, t.dark_shadow);
        self.vline(x + 1, y + 1, h - 3, t.dark_shadow);
        self.hline(x + 1, y + h - 2, w - 2, t.light);
        self.vline(x + w - 2, y + 1, h - 2, t.light);
    }

    /// A dotted rectangle — the era's keyboard-focus indicator.
    pub fn dotted_rect(&mut self, x: i32, y: i32, w: i32, h: i32, c: Color) {
        for xx in x..x + w {
            if (xx - x) % 2 == 0 {
                self.set_pixel(xx, y, c);
                self.set_pixel(xx, y + h - 1, c);
            }
        }
        for yy in y..y + h {
            if (yy - y) % 2 == 0 {
                self.set_pixel(x, yy, c);
                self.set_pixel(x + w - 1, yy, c);
            }
        }
    }

    /// Draw text with the built-in 5x7 font. `scale` is 1 or 2 in practice.
    /// Returns the x position just past the last glyph.
    pub fn text(&mut self, x: i32, y: i32, s: &str, c: Color, scale: i32) -> i32 {
        let scale = scale.max(1);
        let mut cx = x;
        for ch in s.chars() {
            let g = font::glyph(ch);
            for (row, line) in g.iter().enumerate() {
                for (col, b) in line.bytes().enumerate() {
                    if b == b'#' {
                        self.fill_rect(
                            cx + col as i32 * scale,
                            y + row as i32 * scale,
                            scale,
                            scale,
                            c,
                        );
                    }
                }
            }
            cx += font::CELL_W * scale;
        }
        cx
    }

    /// Draw text with a one-pixel offset copy behind it — faux bold,
    /// exactly the way title bars did it.
    pub fn text_bold(&mut self, x: i32, y: i32, s: &str, c: Color, scale: i32) -> i32 {
        self.text(x + scale.max(1), y, s, c, scale);
        self.text(x, y, s, c, scale)
    }

    /// Pixel width of a string at the given scale.
    pub fn text_width(s: &str, scale: i32) -> i32 {
        s.chars().count() as i32 * font::CELL_W * scale.max(1)
    }
}
