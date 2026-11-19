//! J Toolkit example — "Corporate Workstation", a mid-90s quarterly
//! report dialog rendered entirely with jt's software renderer.
//!
//! * On Windows it opens a real Win32 window (see `win32` below).
//! * On every platform, `jt-demo --screenshot <path.png>` renders the
//!   same frame headlessly and writes a PNG — that is how
//!   `example/screenshot.png` in this repository was produced.

#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

use jt::widgets::{self, ButtonState};
use jt::{Framebuffer, Rect, Theme};

pub const CANVAS_W: i32 = 880;
pub const CANVAS_H: i32 = 620;

const WIN: Rect = Rect::new(40, 28, 800, 562);
const LIST: Rect = Rect::new(52, 268, 300, 110);
const BTN_OK: Rect = Rect::new(540, 462, 84, 28);
const BTN_CANCEL: Rect = Rect::new(632, 462, 84, 28);
const BTN_HELP: Rect = Rect::new(724, 462, 84, 28);
const CHK_OVERHEAD: Rect = Rect::new(564, 146, 230, 16);
const CHK_CONSOLIDATE: Rect = Rect::new(564, 166, 230, 16);
const CHK_AUDIT: Rect = Rect::new(564, 186, 230, 16);
const RADIO_SUMMARY: Rect = Rect::new(564, 212, 100, 16);
const RADIO_DETAIL: Rect = Rect::new(676, 212, 100, 16);
const CHK_PRINT: Rect = Rect::new(68, 418, 90, 16);
const CHK_FILE: Rect = Rect::new(168, 418, 120, 16);

const REGIONS: [&str; 5] = [
    "North America",
    "Europe",
    "Pacific Rim",
    "Latin America",
    "Middle East + Africa",
];

#[derive(Clone)]
pub struct App {
    pub region: usize,
    pub overhead: bool,
    pub consolidate: bool,
    pub audit: bool,
    pub detail: bool,
    pub print_copy: bool,
    pub file_copy: bool,
    pub progress: f32,
    pub status: String,
}

impl Default for App {
    fn default() -> Self {
        App {
            region: 0,
            overhead: true,
            consolidate: true,
            audit: false,
            detail: false,
            print_copy: true,
            file_copy: false,
            progress: 0.62,
            status: "Ready".to_string(),
        }
    }
}

impl App {
    pub fn click(&mut self, x: i32, y: i32) {
        if CHK_OVERHEAD.contains(x, y) {
            self.overhead = !self.overhead;
        } else if CHK_CONSOLIDATE.contains(x, y) {
            self.consolidate = !self.consolidate;
        } else if CHK_AUDIT.contains(x, y) {
            self.audit = !self.audit;
        } else if RADIO_SUMMARY.contains(x, y) {
            self.detail = false;
        } else if RADIO_DETAIL.contains(x, y) {
            self.detail = true;
        } else if CHK_PRINT.contains(x, y) {
            self.print_copy = !self.print_copy;
        } else if CHK_FILE.contains(x, y) {
            self.file_copy = !self.file_copy;
        } else if LIST.contains(x, y) {
            let row = (y - LIST.y - 3) / 14;
            if row >= 0 && (row as usize) < REGIONS.len() {
                self.region = row as usize;
                self.status = format!("Region: {}", REGIONS[self.region]);
            }
        } else if BTN_OK.contains(x, y) {
            self.progress = 1.0;
            self.status = "Report saved - UNTITLED.JTK".to_string();
        } else if BTN_CANCEL.contains(x, y) {
            self.progress = 0.0;
            self.status = "Cancelled".to_string();
        } else if BTN_HELP.contains(x, y) {
            self.status = "Help is a 400-page binder. It is excellent.".to_string();
        }
    }
}

pub fn render(app: &App) -> Framebuffer {
    let t = Theme::corporate();
    let mut fb = Framebuffer::new(CANVAS_W, CANVAS_H);
    fb.clear(t.desktop);

    // The window itself
    let (cx, cy) = widgets::window(
        &mut fb,
        &t,
        WIN.x,
        WIN.y,
        WIN.w,
        WIN.h,
        "J Toolkit  -  Corporate Workstation  -  UNTITLED.JTK",
        true,
    );
    let menu_y = widgets::menu_bar(
        &mut fb,
        &t,
        cx,
        cy,
        WIN.w - 8,
        &["File", "Edit", "View", "Data", "Window", "Help"],
    );

    // Toolbar
    let labels = ["New", "Open", "Save", "Print", "Cut", "Copy", "Paste"];
    for (i, label) in labels.iter().enumerate() {
        widgets::button(&mut fb, &t, 52 + i as i32 * 72, menu_y + 6, 64, 24, label, ButtonState::Normal);
    }
    widgets::separator(&mut fb, &t, cx + 4, menu_y + 36, WIN.w - 16);

    // Quarterly figures
    widgets::group_box(&mut fb, &t, 52, 126, 480, 118, "Quarterly Figures");
    widgets::label(&mut fb, &t, 68, 146, "Company:");
    widgets::text_field(&mut fb, &t, 150, 142, 360, "J Consolidated Industries, Inc.");
    widgets::label(&mut fb, &t, 68, 172, "Quarter:");
    widgets::text_field(&mut fb, &t, 150, 168, 150, "Q3 FY1995");
    widgets::label(&mut fb, &t, 316, 172, "Division:");
    widgets::text_field(&mut fb, &t, 388, 168, 122, "All");
    widgets::label(&mut fb, &t, 68, 198, "Prepared by:");
    widgets::text_field(&mut fb, &t, 150, 194, 150, "Finance Dept.");
    widgets::label(&mut fb, &t, 316, 198, "Date:");
    widgets::text_field(&mut fb, &t, 388, 194, 122, "17 Oct 1995");

    // Report options
    widgets::group_box(&mut fb, &t, 548, 126, 276, 118, "Report Options");
    widgets::check_box(&mut fb, &t, CHK_OVERHEAD.x, CHK_OVERHEAD.y, "Include overhead", app.overhead);
    widgets::check_box(&mut fb, &t, CHK_CONSOLIDATE.x, CHK_CONSOLIDATE.y, "Consolidate subsidiaries", app.consolidate);
    widgets::check_box(&mut fb, &t, CHK_AUDIT.x, CHK_AUDIT.y, "Audit trail", app.audit);
    widgets::radio(&mut fb, &t, RADIO_SUMMARY.x, RADIO_SUMMARY.y, "Summary", !app.detail);
    widgets::radio(&mut fb, &t, RADIO_DETAIL.x, RADIO_DETAIL.y, "Detail", app.detail);

    // Regions
    widgets::label(&mut fb, &t, 52, 254, "Regions:");
    widgets::list_box(&mut fb, &t, LIST.x, LIST.y, LIST.w, LIST.h, &REGIONS, app.region);

    // Consolidation progress
    widgets::label(&mut fb, &t, 368, 254, "Consolidation:");
    widgets::progress_bar(&mut fb, &t, 368, 268, 456, app.progress);
    let pct = format!(
        "Consolidating ledger for {}... {}%",
        REGIONS[app.region],
        (app.progress * 100.0).round() as i32
    );
    widgets::label(&mut fb, &t, 368, 294, &pct);

    // Ledger preview — a little spreadsheet, drawn with raw primitives
    draw_ledger(&mut fb, &t, 368, 318);

    // Distribution
    widgets::group_box(&mut fb, &t, 52, 390, 300, 52, "Distribution");
    widgets::check_box(&mut fb, &t, CHK_PRINT.x, CHK_PRINT.y + 2, "Print", app.print_copy);
    widgets::check_box(&mut fb, &t, CHK_FILE.x, CHK_FILE.y + 2, "File copy", app.file_copy);

    // A word from the toolkit
    fb.text_bold(52, 466, "J Toolkit 0.1", t.text, 1);
    widgets::label(&mut fb, &t, 52, 482, "One toolkit. Every platform.");
    widgets::label(&mut fb, &t, 52, 494, "No surprises.");

    // Action buttons
    widgets::button(&mut fb, &t, BTN_OK.x, BTN_OK.y, BTN_OK.w, BTN_OK.h, "OK", ButtonState::Default);
    widgets::button(&mut fb, &t, BTN_CANCEL.x, BTN_CANCEL.y, BTN_CANCEL.w, BTN_CANCEL.h, "Cancel", ButtonState::Normal);
    widgets::button(&mut fb, &t, BTN_HELP.x, BTN_HELP.y, BTN_HELP.w, BTN_HELP.h, "Help", ButtonState::Normal);

    // Status bar
    widgets::status_bar(
        &mut fb,
        &t,
        WIN.x + 4,
        WIN.y + WIN.h - 28,
        WIN.w - 8,
        &[
            (250, app.status.as_str()),
            (170, "Records: 1,024"),
            (140, "Ledger: OPEN"),
            (0, "17 Oct 1995"),
        ],
    );
    fb
}

fn draw_ledger(fb: &mut Framebuffer, t: &Theme, x: i32, y: i32) {
    let (w, h) = (456, 124);
    fb.fill_rect(x, y, w, h, t.field);
    fb.bevel_sunken(t, x, y, w, h);
    // Header row
    fb.fill_rect(x + 2, y + 2, w - 4, 18, t.selection);
    for (i, head) in ["Account", "Q3 1995", "Q2 1995", "Change"].iter().enumerate() {
        fb.text(x + 10 + i as i32 * 112, y + 8, head, t.selection_text, 1);
    }
    let rows = [
        ["Revenue", "1,204", "1,096", "+9.9%"],
        ["Expenses", "842", "811", "+3.8%"],
        ["Payroll", "391", "377", "+3.7%"],
        ["Profit", "362", "285", "+27.0%"],
    ];
    for (r, row) in rows.iter().enumerate() {
        let ry = y + 24 + r as i32 * 24;
        for (c, cell) in row.iter().enumerate() {
            fb.text(x + 10 + c as i32 * 112, ry + 4, cell, t.text, 1);
        }
        if r < rows.len() - 1 {
            fb.hline(x + 4, ry + 19, w - 8, Theme::corporate().light);
        }
    }
    for c in 1..4 {
        fb.vline(x + c * 112, y + 4, h - 8, Theme::corporate().light);
    }
}

fn screenshot_mode(args: &[String]) -> Option<std::path::PathBuf> {
    let pos = args.iter().position(|a| a == "--screenshot")?;
    let path = args
        .get(pos + 1)
        .cloned()
        .unwrap_or_else(|| "screenshot.png".to_string());
    Some(std::path::PathBuf::from(path))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(path) = screenshot_mode(&args) {
        let fb = render(&App::default());
        match jt::png::write_png(&fb, &path) {
            Ok(()) => println!("jt-demo: wrote {}", path.display()),
            Err(e) => {
                eprintln!("jt-demo: cannot write {}: {e}", path.display());
                std::process::exit(1);
            }
        }
        return;
    }

    #[cfg(target_os = "windows")]
    {
        win32::run();
    }
    #[cfg(not(target_os = "windows"))]
    {
        // Headless platforms: render one frame so the program still does
        // something honest, and say how to get the picture.
        let fb = render(&App::default());
        let path = std::path::Path::new("screenshot.png");
        let _ = jt::png::write_png(&fb, path);
        println!("jt-demo: this build has no window backend; wrote screenshot.png");
        println!("jt-demo: run with --screenshot <path.png> to choose the file.");
    }
}

#[cfg(target_os = "windows")]
#[allow(static_mut_refs)] // single-threaded UI thread owns the App
mod win32 {
    use super::{render, App, CANVAS_H, CANVAS_W};
    use std::ffi::c_void;

    type Hwnd = *mut c_void;
    type Hdc = *mut c_void;

    #[repr(C)]
    struct Point {
        x: i32,
        y: i32,
    }
    #[repr(C)]
    struct Msg {
        hwnd: Hwnd,
        message: u32,
        w_param: usize,
        l_param: isize,
        time: u32,
        pt: Point,
    }
    #[repr(C)]
    struct WndClassW {
        style: u32,
        lpfn_wnd_proc: Option<unsafe extern "system" fn(Hwnd, u32, usize, isize) -> isize>,
        cb_cls_extra: i32,
        cb_wnd_extra: i32,
        h_instance: *mut c_void,
        h_icon: *mut c_void,
        h_cursor: *mut c_void,
        hbr_background: *mut c_void,
        lpsz_menu_name: *const u16,
        lpsz_class_name: *const u16,
    }
    #[repr(C)]
    struct PaintStruct {
        hdc: Hdc,
        f_erase: i32,
        rc_paint: [i32; 4],
        f_restore: i32,
        f_inc_update: i32,
        rgb_reserved: [u8; 32],
    }
    #[repr(C)]
    struct BitmapInfoHeader {
        bi_size: u32,
        bi_width: i32,
        bi_height: i32,
        bi_planes: u16,
        bi_bit_count: u16,
        bi_compression: u32,
        bi_size_image: u32,
        bi_x_pels_per_meter: i32,
        bi_y_pels_per_meter: i32,
        bi_clr_used: u32,
        bi_clr_important: u32,
    }
    #[repr(C)]
    struct BitmapInfo {
        bmi_header: BitmapInfoHeader,
        bmi_colors: [u32; 1],
    }
    #[repr(C)]
    struct RectW {
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    }

    const WM_DESTROY: u32 = 0x0002;
    const WM_PAINT: u32 = 0x000F;
    const WM_LBUTTONDOWN: u32 = 0x0201;
    const GWLP_USERDATA: i32 = -21;
    const WS_OVERLAPPED: u32 = 0x0000_0000;
    const WS_CAPTION: u32 = 0x00C0_0000;
    const WS_SYSMENU: u32 = 0x0008_0000;
    const WS_MINIMIZEBOX: u32 = 0x0002_0000;
    const WS_VISIBLE: u32 = 0x1000_0000;
    const SRCCOPY: u32 = 0x00CC_0020;

    extern "system" {
        fn RegisterClassW(lp_wnd_class: *const WndClassW) -> u16;
        fn CreateWindowExW(
            ex_style: u32,
            class_name: *const u16,
            window_name: *const u16,
            style: u32,
            x: i32,
            y: i32,
            w: i32,
            h: i32,
            parent: Hwnd,
            menu: *mut c_void,
            instance: *mut c_void,
            param: *mut c_void,
        ) -> Hwnd;
        fn DefWindowProcW(hwnd: Hwnd, msg: u32, w: usize, l: isize) -> isize;
        fn GetMessageW(msg: *mut Msg, hwnd: Hwnd, min: u32, max: u32) -> i32;
        fn TranslateMessage(msg: *const Msg) -> i32;
        fn DispatchMessageW(msg: *const Msg) -> isize;
        fn PostQuitMessage(code: i32);
        fn BeginPaint(hwnd: Hwnd, ps: *mut PaintStruct) -> Hdc;
        fn EndPaint(hwnd: Hwnd, ps: *const PaintStruct) -> i32;
        fn InvalidateRect(hwnd: Hwnd, rect: *const RectW, erase: i32) -> i32;
        fn SetWindowLongPtrW(hwnd: Hwnd, index: i32, new_long: isize) -> isize;
        fn GetWindowLongPtrW(hwnd: Hwnd, index: i32) -> isize;
        fn AdjustWindowRectEx(rect: *mut RectW, style: u32, menu: i32, ex_style: u32) -> i32;
        fn GetModuleHandleW(name: *const u16) -> *mut c_void;
    }

    #[link(name = "gdi32")]
    extern "system" {
        fn StretchDIBits(
            hdc: Hdc,
            x_dest: i32,
            y_dest: i32,
            w_dest: i32,
            h_dest: i32,
            x_src: i32,
            y_src: i32,
            w_src: i32,
            h_src: i32,
            bits: *const c_void,
            bmi: *const BitmapInfo,
            usage: u32,
            rop: u32,
        ) -> i32;
    }

    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    static mut APP: Option<App> = None;

    unsafe extern "system" fn wnd_proc(hwnd: Hwnd, msg: u32, w: usize, l: isize) -> isize {
        match msg {
            WM_PAINT => {
                let app = APP.as_ref().unwrap();
                let fb = render(app);
                let mut ps = PaintStruct {
                    hdc: std::ptr::null_mut(),
                    f_erase: 0,
                    rc_paint: [0; 4],
                    f_restore: 0,
                    f_inc_update: 0,
                    rgb_reserved: [0; 32],
                };
                let hdc = BeginPaint(hwnd, &mut ps);
                let bmi = BitmapInfo {
                    bmi_header: BitmapInfoHeader {
                        bi_size: std::mem::size_of::<BitmapInfoHeader>() as u32,
                        bi_width: CANVAS_W,
                        bi_height: -CANVAS_H, // top-down
                        bi_planes: 1,
                        bi_bit_count: 32,
                        bi_compression: 0,
                        bi_size_image: 0,
                        bi_x_pels_per_meter: 0,
                        bi_y_pels_per_meter: 0,
                        bi_clr_used: 0,
                        bi_clr_important: 0,
                    },
                    bmi_colors: [0],
                };
                // jt pixels are 0x00RRGGBB; in little-endian memory that is
                // B,G,R,0 — exactly what a 32bpp DIB wants.
                StretchDIBits(
                    hdc,
                    0,
                    0,
                    CANVAS_W,
                    CANVAS_H,
                    0,
                    0,
                    CANVAS_W,
                    CANVAS_H,
                    fb.pixels.as_ptr() as *const c_void,
                    &bmi,
                    0,
                    SRCCOPY,
                );
                EndPaint(hwnd, &ps);
                0
            }
            WM_LBUTTONDOWN => {
                let x = (l & 0xFFFF) as i16 as i32;
                let y = ((l >> 16) & 0xFFFF) as i16 as i32;
                if let Some(app) = APP.as_mut() {
                    app.click(x, y);
                }
                InvalidateRect(hwnd, std::ptr::null(), 0);
                0
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                0
            }
            _ => DefWindowProcW(hwnd, msg, w, l),
        }
    }

    pub fn run() {
        unsafe {
            APP = Some(App::default());
            let class_name = wide("JToolkitWindow");
            let wc = WndClassW {
                style: 0,
                lpfn_wnd_proc: Some(wnd_proc),
                cb_cls_extra: 0,
                cb_wnd_extra: 0,
                h_instance: GetModuleHandleW(std::ptr::null()),
                h_icon: std::ptr::null_mut(),
                h_cursor: std::ptr::null_mut(),
                hbr_background: std::ptr::null_mut(),
                lpsz_menu_name: std::ptr::null(),
                lpsz_class_name: class_name.as_ptr(),
            };
            RegisterClassW(&wc);
            let style = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_VISIBLE;
            let mut rect = RectW {
                left: 0,
                top: 0,
                right: CANVAS_W,
                bottom: CANVAS_H,
            };
            AdjustWindowRectEx(&mut rect, style, 0, 0);
            let hwnd = CreateWindowExW(
                0,
                class_name.as_ptr(),
                wide("J Toolkit - Corporate Workstation").as_ptr(),
                style,
                60,
                40,
                rect.right - rect.left,
                rect.bottom - rect.top,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                GetModuleHandleW(std::ptr::null()),
                std::ptr::null_mut(),
            );
            if hwnd.is_null() {
                return;
            }
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
            let mut msg = Msg {
                hwnd: std::ptr::null_mut(),
                message: 0,
                w_param: 0,
                l_param: 0,
                time: 0,
                pt: Point { x: 0, y: 0 },
            };
            while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }
}
