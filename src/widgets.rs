//! The widget set, drawn exactly the way a 1995 corporate style guide
//! demanded: silver faces, two-pixel bevels, navy selections, and a
//! segmented progress bar you can hear clicking in your mind.

use crate::framebuffer::Framebuffer;
use crate::theme::{Color, Theme};

/// The J mark: a bold serif-ish J on a navy tile.
/// The J does not stand for anything; it is a nice looking glyph.
pub fn j_mark(fb: &mut Framebuffer, t: &Theme, x: i32, y: i32, size: i32) {
    fb.fill_rect(x, y, size, size, t.accent);
    fb.outline(x, y, size, size, t.highlight);
    let u = (size as f32 / 16.0).max(1.0);
    let rx = |v: f32| x + (v * u) as i32;
    let ry = |v: f32| y + (v * u) as i32;
    let pw = |v: f32| (v * u).ceil() as i32;
    let white = Color::rgb(0xFF, 0xFF, 0xFF);
    // Top bar of the J
    fb.fill_rect(rx(3.0), ry(2.0), pw(10.0), pw(2.5), white);
    // Stem
    fb.fill_rect(rx(9.5), ry(2.0), pw(3.0), pw(11.5), white);
    // Hook along the bottom, curving up on the left
    fb.fill_rect(rx(3.5), ry(11.0), pw(9.0), pw(2.5), white);
    fb.fill_rect(rx(3.5), ry(8.5), pw(3.0), pw(5.0), white);
}

/// A top-level window: drop shadow, raised frame, gradient title bar
/// with the J mark and the minimise / maximise / close furniture.
/// Returns the client area origin `(x, y)` — content starts below the
/// title bar.
pub fn window(
    fb: &mut Framebuffer,
    t: &Theme,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    title: &str,
    active: bool,
) -> (i32, i32) {
    // Drop shadow
    fb.fill_rect(x + 4, y + 4, w, h, Color::rgb(0x00, 0x40, 0x40));
    // Face + frame
    fb.fill_rect(x, y, w, h, t.face);
    fb.bevel_raised(t, x, y, w, h);
    // Title bar
    let (c0, c1) = if active {
        (t.title_active, t.title_active_end)
    } else {
        (t.title_inactive, t.title_inactive)
    };
    let tw = w - 8;
    for i in 0..tw {
        let f = i as f32 / tw.max(1) as f32;
        let mix = |a: u8, b: u8| (a as f32 + (b as f32 - a as f32) * f) as u8;
        fb.vline(
            x + 4 + i,
            y + 4,
            18,
            Color::rgb(mix(c0.r(), c1.r()), mix(c0.g(), c1.g()), mix(c0.b(), c1.b())),
        );
    }
    j_mark(fb, t, x + 6, y + 6, 14);
    fb.text_bold(x + 24, y + 9, title, t.title_text, 1);
    // Title-bar buttons: minimise, maximise, close
    let by = y + 6;
    for (i, kind) in ["min", "max", "close"].iter().enumerate() {
        let bx = x + w - 22 - (2 - i as i32) * 18;
        title_button(fb, t, bx, by, kind);
    }
    (x + 4, y + 26)
}

fn title_button(fb: &mut Framebuffer, t: &Theme, x: i32, y: i32, kind: &str) {
    fb.fill_rect(x, y, 16, 14, t.face);
    fb.bevel_raised(t, x, y, 16, 14);
    match kind {
        "min" => fb.fill_rect(x + 4, y + 10, 7, 2, t.text),
        "max" => {
            fb.outline(x + 3, y + 3, 10, 8, t.text);
            fb.hline(x + 3, y + 4, 10, t.text);
        }
        _ => {
            // An X, drawn as two honest diagonals
            for i in 0..7 {
                fb.set_pixel(x + 4 + i, y + 3 + i, t.text);
                fb.set_pixel(x + 4 + i, y + 4 + i, t.text);
                fb.set_pixel(x + 10 - i, y + 3 + i, t.text);
                fb.set_pixel(x + 10 - i, y + 4 + i, t.text);
            }
        }
    }
}

/// A menu bar. Returns the y just below it. The first letter of each
/// entry gets the traditional underlined-hotkey treatment.
pub fn menu_bar(fb: &mut Framebuffer, t: &Theme, x: i32, y: i32, w: i32, items: &[&str]) -> i32 {
    fb.fill_rect(x, y, w, 20, t.face);
    fb.hline(x, y + 19, w, t.shadow);
    fb.hline(x, y + 20, w, t.highlight);
    let mut cx = x + 8;
    for item in items {
        let end = fb.text(cx, y + 6, item, t.text, 1);
        fb.hline(cx, y + 14, 5, t.text); // hotkey underline
        cx = end + 14;
    }
    y + 21
}

/// State of a push button.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ButtonState {
    Normal,
    /// The default action in a dialog: extra-bold frame.
    Default,
    Pushed,
    Disabled,
}

/// A push button, 23px tall in period dress.
pub fn button(fb: &mut Framebuffer, t: &Theme, x: i32, y: i32, w: i32, h: i32, label: &str, state: ButtonState) {
    if state == ButtonState::Default {
        fb.outline(x, y, w, h, t.dark_shadow);
    }
    let (ox, oy) = if state == ButtonState::Default { (1, 1) } else { (0, 0) };
    fb.fill_rect(x + ox, y + oy, w - 2 * ox, h - 2 * oy, t.face);
    if state == ButtonState::Pushed {
        fb.bevel_sunken(t, x + ox, y + oy, w - 2 * ox, h - 2 * oy);
    } else {
        fb.bevel_raised(t, x + ox, y + oy, w - 2 * ox, h - 2 * oy);
    }
    let colour = if state == ButtonState::Disabled { t.disabled_text } else { t.text };
    let tw = Framebuffer::text_width(label, 1);
    let tx = x + (w - tw) / 2 + if state == ButtonState::Pushed { 1 } else { 0 };
    let ty = y + (h - 7) / 2 + if state == ButtonState::Pushed { 1 } else { 0 };
    fb.text(tx, ty, label, colour, 1);
    if state == ButtonState::Default {
        fb.dotted_rect(x + 4, y + 4, w - 8, h - 8, t.text);
    }
}

/// A plain text label.
pub fn label(fb: &mut Framebuffer, t: &Theme, x: i32, y: i32, s: &str) {
    fb.text(x, y, s, t.text, 1);
}

/// A sunken single-line edit field with a text cursor's worth of confidence.
pub fn text_field(fb: &mut Framebuffer, t: &Theme, x: i32, y: i32, w: i32, value: &str) {
    fb.fill_rect(x, y, w, 21, t.field);
    fb.bevel_sunken(t, x, y, w, 21);
    // Clip the value to the field, like a good citizen
    let max_chars = ((w - 10) / 6).max(0) as usize;
    let shown: String = value.chars().take(max_chars).collect();
    fb.text(x + 5, y + 7, &shown, t.text, 1);
}

/// A group box: etched frame with a label sitting in the top edge.
pub fn group_box(fb: &mut Framebuffer, t: &Theme, x: i32, y: i32, w: i32, h: i32, label: &str) {
    // Etched (groove) frame: shadow line outside, highlight inside
    fb.outline(x, y + 4, w, h - 4, t.shadow);
    fb.outline(x + 1, y + 5, w - 2, h - 6, t.highlight);
    let tw = Framebuffer::text_width(label, 1) + 6;
    fb.fill_rect(x + 8, y, tw, 9, t.face);
    fb.text(x + 11, y + 1, label, t.text, 1);
}

/// A checkbox, 12x12, with a proper heavy tick.
pub fn check_box(fb: &mut Framebuffer, t: &Theme, x: i32, y: i32, label: &str, checked: bool) {
    fb.fill_rect(x, y, 12, 12, t.field);
    fb.bevel_sunken(t, x, y, 12, 12);
    if checked {
        // Tick: down-stroke then a long rise
        for i in 0..4 {
            fb.set_pixel(x + 2 + i, y + 5 + i, t.text);
            fb.set_pixel(x + 2 + i, y + 6 + i, t.text);
        }
        for i in 0..6 {
            fb.set_pixel(x + 5 + i, y + 8 - i, t.text);
            fb.set_pixel(x + 5 + i, y + 9 - i, t.text);
        }
    }
    fb.text(x + 17, y + 3, label, t.text, 1);
}

/// A radio button: a sunken diamond-disc with a bullet when selected.
/// (Period toolkits drew circles; jt draws them one honest pixel at a time.)
pub fn radio(fb: &mut Framebuffer, t: &Theme, x: i32, y: i32, label: &str, selected: bool) {
    // A 12x12 disc, rasterised by distance: white fill, shadow arc on the
    // upper-left, highlight arc on the lower-right.
    for dy in 0..12 {
        for dx in 0..12 {
            let (cx, cy) = (dx as f32 - 5.5, dy as f32 - 5.5);
            let d2 = cx * cx + cy * cy;
            if d2 <= 30.0 {
                fb.set_pixel(x + dx, y + dy, t.field);
            }
            if (16.0..=34.0).contains(&d2) {
                let c = if cx + cy < 0.0 { t.shadow } else { t.highlight };
                fb.set_pixel(x + dx, y + dy, c);
            }
        }
    }
    if selected {
        fb.fill_rect(x + 4, y + 4, 4, 4, t.text);
        fb.fill_rect(x + 5, y + 3, 2, 6, t.text);
        fb.fill_rect(x + 3, y + 5, 6, 2, t.text);
    }
    fb.text(x + 17, y + 3, label, t.text, 1);
}

/// A sunken list box. The selected row wears navy, as is traditional.
/// Row height is 14px.
pub fn list_box(fb: &mut Framebuffer, t: &Theme, x: i32, y: i32, w: i32, h: i32, items: &[&str], selected: usize) {
    fb.fill_rect(x, y, w, h, t.field);
    fb.bevel_sunken(t, x, y, w, h);
    // Scroll bar on the right edge
    scroll_bar_vertical(fb, t, x + w - 16, y + 2, h - 4);
    for (i, item) in items.iter().enumerate() {
        let ry = y + 3 + i as i32 * 14;
        if ry + 12 > y + h - 3 {
            break;
        }
        if i == selected {
            fb.fill_rect(x + 3, ry, w - 22, 13, t.selection);
            fb.text(x + 6, ry + 3, item, t.selection_text, 1);
        } else {
            fb.text(x + 6, ry + 3, item, t.text, 1);
        }
    }
}

/// A vertical scroll bar with arrow buttons at both ends and a thumb.
pub fn scroll_bar_vertical(fb: &mut Framebuffer, t: &Theme, x: i32, y: i32, h: i32) {
    // Track: a tasteful dither
    for yy in y..y + h {
        for xx in x..x + 14 {
            let c = if (xx + yy) % 2 == 0 { t.highlight } else { t.face };
            fb.set_pixel(xx, yy, c);
        }
    }
    // Up arrow button
    fb.fill_rect(x, y, 14, 14, t.face);
    fb.bevel_raised(t, x, y, 14, 14);
    for i in 0..5 {
        fb.hline(x + 3 + i, y + 8 - i, 9 - 2 * i, t.text);
    }
    // Down arrow button
    let dy = y + h - 14;
    fb.fill_rect(x, dy, 14, 14, t.face);
    fb.bevel_raised(t, x, dy, 14, 14);
    for i in 0..5 {
        fb.hline(x + 3 + i, dy + 4 + i, 9 - 2 * i, t.text);
    }
    // Thumb near the top
    fb.fill_rect(x, y + 14, 14, 30, t.face);
    fb.bevel_raised(t, x, y + 14, 14, 30);
}

/// The segmented progress bar. Accept no smooth substitutes.
/// `fraction` is clamped to 0.0..=1.0.
pub fn progress_bar(fb: &mut Framebuffer, t: &Theme, x: i32, y: i32, w: i32, fraction: f32) {
    fb.fill_rect(x, y, w, 18, t.field);
    fb.bevel_sunken(t, x, y, w, 18);
    let inner = w - 6;
    let block = 10;
    let gap = 2;
    let total = inner / (block + gap);
    let lit = (fraction.clamp(0.0, 1.0) * total as f32).round() as i32;
    for i in 0..lit {
        fb.fill_rect(x + 3 + i * (block + gap), y + 3, block, 12, t.accent);
    }
}

/// A status bar along the bottom of a window, with sunken panes.
/// Each pane is `(width, text)`; a width of 0 stretches to fill the rest.
pub fn status_bar(fb: &mut Framebuffer, t: &Theme, x: i32, y: i32, w: i32, panes: &[(i32, &str)]) {
    fb.fill_rect(x, y, w, 20, t.face);
    fb.hline(x, y, w, t.shadow);
    fb.hline(x, y + 1, w, t.highlight);
    let mut cx = x + 2;
    for (i, (pw, text)) in panes.iter().enumerate() {
        let width = if *pw == 0 || i == panes.len() - 1 { x + w - cx - 2 } else { *pw };
        fb.bevel_sunken(t, cx, y + 3, width, 15);
        fb.text(cx + 5, y + 7, text, t.text, 1);
        cx += width + 3;
    }
}

/// A raised separator line with its highlight companion.
pub fn separator(fb: &mut Framebuffer, t: &Theme, x: i32, y: i32, w: i32) {
    fb.hline(x, y, w, t.shadow);
    fb.hline(x, y + 1, w, t.highlight);
}
