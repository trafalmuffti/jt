//! # J Toolkit (`jt`)
//!
//! A cross-platform retro toolkit in Rust, reminiscent of mid-1990s
//! corporate design: silver faces, hard bevels, navy title bars, segmented
//! progress bars, and menus that mean business.
//!
//! The J does not stand for anything. It is a nice looking glyph.
//!
//! `jt` has **no dependencies**. Everything renders into an in-memory
//! [`Framebuffer`] with jt's own software renderer, so the same pixels come
//! out on every platform. A platform backend only has to:
//!
//! 1. open a window (Win32, X11, Cocoa, ... — the example ships a Win32 one),
//! 2. blit the framebuffer, and
//! 3. forward mouse clicks to the application.
//!
//! ## Quick start
//!
//! ```rust
//! use jt::{Framebuffer, Theme, widgets};
//!
//! let theme = Theme::corporate();
//! let mut fb = Framebuffer::new(320, 200);
//! fb.clear(theme.desktop);
//! widgets::window(&mut fb, &theme, 10, 10, 300, 180, "J Toolkit", true);
//! widgets::button(&mut fb, &theme, 20, 160, 80, 23, "OK", widgets::ButtonState::Default);
//! ```

pub mod font;
pub mod framebuffer;
pub mod png;
pub mod theme;
pub mod widgets;

pub use framebuffer::{Framebuffer, Rect};
pub use theme::{Color, Theme};
