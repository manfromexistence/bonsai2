//! Native Win32 host for the `ui` engine.
//!
//! Opens a real window on Windows, renders a synthetic application-chrome scene
//! through the `ui` rasterizer, and reports its own CPU and memory footprint
//! while it runs.
//!
//! This exists to answer three questions that the offline benchmark cannot:
//!
//! 1. What does the engine cost in CPU time and resident memory *inside* a
//!    real window, against a real compositor, on real hardware?
//! 2. Does the idle-gating actually work — does an idle window draw nothing?
//! 3. Does the frame budget hold when the scene is realistic rather than a
//!    single rectangle?
//!
//! Run it with:
//!
//! ```text
//! cargo run --release --example win32
//! ```
//!
//! Close the window to exit. The status line is printed to stdout every
//! second.

#![allow(unsafe_code)]
#![allow(clippy::field_reassign_with_default)]

use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU64, Ordering};
use std::time::Instant;

use ui::layout::{self, EdgeInsets, FlexDirection, Justify, LayoutProps, Rect, Size};
use ui::raster::{Paint, Rasterizer, Rgba};
use ui::scheduler::{FrameCost, Route, Scheduler};
use ui::spring::Spring;
use ui::text::{GlyphOutline, Vec2};
use ui::tree::NodeTree;

// ---------------------------------------------------------------------------
// Win32 FFI
// ---------------------------------------------------------------------------

type Handle = *mut core::ffi::c_void;

#[repr(C)]
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct RectRaw {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

#[repr(C)]
struct Msg {
    hwnd: Handle,
    message: u32,
    wparam: usize,
    lparam: isize,
    time: u32,
    point: Point,
}

#[repr(C)]
struct PaintStruct {
    hdc: Handle,
    f_erase: i32,
    rc_paint: RectRaw,
    f_restore: i32,
    f_inc_update: i32,
    rgb_reserved: [u8; 32],
}

#[repr(C)]
struct BitmapInfoHeader {
    size: u32,
    width: i32,
    height: i32,
    planes: u16,
    bit_count: u16,
    compression: u32,
    size_image: u32,
    x_ppm: i32,
    y_ppm: i32,
    clr_used: u32,
    clr_important: u32,
}

#[repr(C)]
struct BitmapInfo {
    header: BitmapInfoHeader,
    colors: [u32; 3],
}

#[repr(C)]
struct WindowClassEx {
    cb_size: u32,
    style: u32,
    wnd_proc: Option<unsafe extern "system" fn(Handle, u32, usize, isize) -> isize>,
    cb_cls_extra: i32,
    cb_wnd_extra: i32,
    h_instance: Handle,
    h_icon: Handle,
    h_cursor: Handle,
    h_br_background: Handle,
    menu_name: *const u16,
    class_name: *const u16,
    h_icon_small: Handle,
}

#[repr(C)]
struct ProcessMemoryCounters {
    cb: u32,
    page_fault_count: u32,
    peak_working_set: usize,
    working_set: usize,
    quota_peak_paged: usize,
    quota_paged: usize,
    quota_peak_non_paged: usize,
    quota_non_paged: usize,
    pagefile_usage: usize,
    peak_pagefile_usage: usize,
}

#[repr(C)]
struct FileTime {
    low: u32,
    high: u32,
}

const WM_DESTROY: u32 = 0x0002;
const WM_PAINT: u32 = 0x000F;
const WM_CLOSE: u32 = 0x0010;
const WM_ERASEBKGND: u32 = 0x0014;
const WM_TIMER: u32 = 0x0113;
const WM_MOUSEMOVE: u32 = 0x0200;
const WM_KEYDOWN: u32 = 0x0100;

const WS_OVERLAPPEDWINDOW: u32 = 0x00CF_0000;
const CS_HREDRAW: u32 = 0x0002;
const CS_VREDRAW: u32 = 0x0001;
const SW_SHOW: i32 = 5;
const BI_RGB: u32 = 0;
const DIB_RGB_COLORS: u32 = 0;
const SRCCOPY: u32 = 0x00CC_0020;

#[link(name = "user32")]
unsafe extern "system" {
    fn RegisterClassExW(class: *const WindowClassEx) -> u16;
    fn CreateWindowExW(
        ex_style: u32,
        class_name: *const u16,
        window_name: *const u16,
        style: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        parent: Handle,
        menu: Handle,
        instance: Handle,
        param: Handle,
    ) -> Handle;
    fn DefWindowProcW(hwnd: Handle, msg: u32, wparam: usize, lparam: isize) -> isize;
    fn DestroyWindow(hwnd: Handle) -> i32;
    fn PostQuitMessage(code: i32);
    fn ShowWindow(hwnd: Handle, cmd: i32) -> i32;
    fn UpdateWindow(hwnd: Handle) -> i32;
    fn GetClientRect(hwnd: Handle, rect: *mut RectRaw) -> i32;
    fn InvalidateRect(hwnd: Handle, rect: *const RectRaw, erase: i32) -> i32;
    fn BeginPaint(hwnd: Handle, paint: *mut PaintStruct) -> Handle;
    fn EndPaint(hwnd: Handle, paint: *const PaintStruct) -> i32;
    fn GetMessageW(msg: *mut Msg, hwnd: Handle, min: u32, max: u32) -> i32;
    fn TranslateMessage(msg: *mut Msg) -> i32;
    fn DispatchMessageW(msg: *mut Msg) -> isize;
    fn SetTimer(hwnd: Handle, id: usize, millis: u32) -> usize;
    fn KillTimer(hwnd: Handle, id: usize) -> i32;
    fn LoadCursorW(instance: Handle, name: *const u16) -> Handle;
}

#[link(name = "gdi32")]
unsafe extern "system" {
    fn CreateCompatibleDC(dc: Handle) -> Handle;
    fn CreateDIBSection(
        dc: Handle,
        info: *const BitmapInfo,
        usage: u32,
        bits: *mut *mut u8,
        section: Handle,
        offset: u32,
    ) -> Handle;
    fn SelectObject(dc: Handle, obj: Handle) -> Handle;
    fn BitBlt(
        dc: Handle,
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        src: Handle,
        sx: i32,
        sy: i32,
        rop: u32,
    ) -> i32;
    fn DeleteObject(obj: Handle) -> i32;
    fn DeleteDC(dc: Handle) -> i32;
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetModuleHandleW(module: *const u16) -> Handle;
    fn GetCurrentProcess() -> Handle;
    fn GetProcessTimes(
        process: Handle,
        creation: *mut FileTime,
        exit: *mut FileTime,
        kernel: *mut FileTime,
        user: *mut FileTime,
    ) -> i32;
}

// `GetProcessMemoryInfo` lives in psapi.dll, not kernel32.
#[link(name = "psapi")]
unsafe extern "system" {
    fn GetProcessMemoryInfo(process: Handle, counters: *mut ProcessMemoryCounters, cb: u32) -> i32;
}

// ---------------------------------------------------------------------------
// Engine state
// ---------------------------------------------------------------------------

/// Everything the window procedure needs.
struct Engine {
    tree: NodeTree,
    scheduler: Scheduler,
    mouse_x: f32,
    mouse_y: f32,
    /// Spring that follows the mouse: the animated element.
    card: Spring,
    /// Spring for the panel fade-in.
    fade: Spring,
    glyph: GlyphOutline,
    /// Cached atlas-free glyph coverage. Re-rasterising every frame is the
    /// obvious way to waste time; see TODO.md 2.1.
    glyph_bitmap: Option<std::sync::Arc<ui::text::GlyphBitmap>>,
    width: usize,
    height: usize,
    frames: u64,
    idle_frames: u64,
    last_render_us: u64,
    layout_us: u64,
    build_us: u64,
    raster_us: u64,
    glyph_us: u64,
    route_micro: u64,
    route_macro: u64,
    pixels_shaded: u64,
    pixels_total: u64,
}

static ENGINE: OnceLock<std::sync::Mutex<Engine>> = OnceLock::new();
static MOUSE_X: AtomicI32 = AtomicI32::new(-1);
static MOUSE_Y: AtomicI32 = AtomicI32::new(-1);
static RUNNING: AtomicBool = AtomicBool::new(true);
static FRAME_COUNTER: AtomicU64 = AtomicU64::new(0);
/// Demo mode: orbit the card on a sine wave so the window renders
/// continuously without needing (or stealing) the mouse.
static DEMO: AtomicBool = AtomicBool::new(false);
static START: OnceLock<Instant> = OnceLock::new();

fn engine() -> &'static std::sync::Mutex<Engine> {
    ENGINE.get_or_init(|| {
        let mut tree = NodeTree::new();
        build_scene(&mut tree);

        let mut card = Spring::new(0.0);
        card.stiffness = 120.0;
        card.damping = 18.0;

        let mut fade = Spring::new(0.0);
        fade.target = 1.0;
        fade.stiffness = 90.0;
        fade.damping = 20.0;

        std::sync::Mutex::new(Engine {
            tree,
            scheduler: Scheduler::default(),
            mouse_x: 240.0,
            mouse_y: 180.0,
            card,
            fade,
            glyph: GlyphOutline::circle(Vec2::new(500.0, 500.0), 420.0, 96),
            glyph_bitmap: None,
            width: 1280,
            height: 800,
            frames: 0,
            idle_frames: 0,
            last_render_us: 0,
            layout_us: 0,
            build_us: 0,
            raster_us: 0,
            glyph_us: 0,
            route_micro: 0,
            route_macro: 0,
            pixels_shaded: 0,
            pixels_total: 0,
        })
    })
}

/// Builds a synthetic application-chrome tree.
fn build_scene(tree: &mut NodeTree) {
    let root = tree.root();
    let mut root_props = LayoutProps::default();
    root_props.direction = FlexDirection::Row;
    root_props.width = Some(1280.0);
    root_props.height = Some(800.0);
    if let Some(n) = tree.node_mut(root) {
        n.props = root_props;
    }

    // Sidebar.
    let mut sidebar = LayoutProps::default();
    sidebar.direction = FlexDirection::Column;
    sidebar.width = Some(240.0);
    sidebar.height = Some(800.0);
    sidebar.padding = EdgeInsets {
        left: 16.0,
        top: 16.0,
        right: 16.0,
        bottom: 16.0,
    };
    sidebar.gap = 10.0;
    let sidebar_id = tree.alloc(sidebar);
    tree.insert_child(root, sidebar_id, None);

    for i in 0..10 {
        let mut p = LayoutProps::default();
        p.direction = FlexDirection::Row;
        p.height = Some(34.0);
        p.grow = 0.0;
        p.gap = 10.0;
        p.justify = Justify::Start;
        let row = tree.alloc(p);
        tree.insert_child(sidebar_id, row, None);
        for _ in 0..3 {
            let mut chip = LayoutProps::default();
            chip.width = Some(14.0);
            chip.height = Some(14.0);
            let c = tree.alloc(chip);
            tree.insert_child(row, c, None);
        }
        let _ = i;
    }

    // Content pane with a grid of cards.
    let mut content = LayoutProps::default();
    content.direction = FlexDirection::Column;
    content.width = Some(1040.0);
    content.height = Some(800.0);
    content.padding = EdgeInsets {
        left: 24.0,
        top: 24.0,
        right: 24.0,
        bottom: 24.0,
    };
    content.gap = 14.0;
    let content_id = tree.alloc(content);
    tree.insert_child(root, content_id, None);

    for _ in 0..6 {
        let mut row = LayoutProps::default();
        row.direction = FlexDirection::Row;
        row.height = Some(110.0);
        row.gap = 14.0;
        let row_id = tree.alloc(row);
        tree.insert_child(content_id, row_id, None);
        for _ in 0..3 {
            let mut card = LayoutProps::default();
            card.width = Some(318.0);
            card.height = Some(110.0);
            let card_id = tree.alloc(card);
            tree.insert_child(row_id, card_id, None);
        }
    }
}

// ---------------------------------------------------------------------------
// 3x5 bitmap font for the HUD (drawn through the real rasterizer)
// ---------------------------------------------------------------------------

fn glyph_rows(c: char) -> [u8; 5] {
    match c.to_ascii_uppercase() {
        '0' => [7, 5, 5, 5, 7],
        '1' => [2, 6, 2, 2, 7],
        '2' => [7, 1, 7, 4, 7],
        '3' => [7, 1, 7, 1, 7],
        '4' => [5, 5, 7, 1, 1],
        '5' => [7, 4, 7, 1, 7],
        '6' => [7, 4, 7, 5, 7],
        '7' => [7, 1, 1, 1, 1],
        '8' => [7, 5, 7, 5, 7],
        '9' => [7, 5, 7, 1, 7],
        'A' => [7, 5, 7, 5, 5],
        'B' => [6, 5, 6, 5, 6],
        'C' => [7, 4, 4, 4, 7],
        'D' => [6, 5, 5, 5, 6],
        'E' => [7, 4, 7, 4, 7],
        'F' => [7, 4, 7, 4, 4],
        'G' => [7, 4, 5, 5, 7],
        'H' => [5, 5, 7, 5, 5],
        'I' => [7, 2, 2, 2, 7],
        'J' => [1, 1, 1, 5, 7],
        'K' => [5, 5, 6, 5, 5],
        'L' => [4, 4, 4, 4, 7],
        'M' => [5, 7, 7, 5, 5],
        'N' => [5, 7, 7, 7, 5],
        'O' => [7, 5, 5, 5, 7],
        'P' => [7, 5, 7, 4, 4],
        'Q' => [7, 5, 5, 7, 1],
        'R' => [7, 5, 7, 6, 5],
        'S' => [7, 4, 7, 1, 7],
        'T' => [7, 2, 2, 2, 2],
        'U' => [5, 5, 5, 5, 7],
        'V' => [5, 5, 5, 5, 2],
        'W' => [5, 5, 7, 7, 5],
        'X' => [5, 5, 2, 5, 5],
        'Y' => [5, 5, 2, 2, 2],
        'Z' => [7, 1, 2, 4, 7],
        ':' => [0, 2, 0, 2, 0],
        '.' => [0, 0, 0, 0, 2],
        ',' => [0, 0, 0, 2, 4],
        '-' => [0, 0, 7, 0, 0],
        '/' => [1, 1, 2, 4, 4],
        '%' => [5, 1, 2, 4, 5],
        '(' => [1, 2, 2, 2, 1],
        ')' => [4, 2, 2, 2, 4],
        '+' => [0, 2, 7, 2, 0],
        '=' => [0, 7, 0, 7, 0],
        '#' => [5, 7, 5, 7, 5],
        ' ' => [0, 0, 0, 0, 0],
        _ => [0, 0, 0, 0, 0],
    }
}

/// Emits one solid paint per lit pixel of a 3x5 glyph.
fn text_paints(text: &str, x: f32, y: f32, scale: f32, color: Rgba, out: &mut Vec<Paint>) {
    let mut cursor = x;
    for ch in text.chars() {
        let rows = glyph_rows(ch);
        for (ry, bits) in rows.iter().enumerate() {
            for rx in 0..3 {
                if bits & (1 << (2 - rx)) != 0 {
                    out.push(Paint::Solid {
                        rect: Rect::new(
                            cursor + rx as f32 * scale,
                            y + ry as f32 * scale,
                            scale,
                            scale,
                        ),
                        color,
                    });
                }
            }
        }
        cursor += 4.0 * scale;
    }
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

fn build_paints(engine: &mut Engine) -> Vec<Paint> {
    let mut paints = Vec::with_capacity(512);

    // Window background.
    paints.push(Paint::Solid {
        rect: Rect::new(0.0, 0.0, engine.width as f32, engine.height as f32),
        color: Rgba::new(0.06, 0.07, 0.09, 1.0),
    });

    // Sidebar.
    paints.push(Paint::Solid {
        rect: Rect::new(0.0, 0.0, 240.0, engine.height as f32),
        color: Rgba::new(0.10, 0.11, 0.14, 1.0),
    });

    // Cards: rounded rects with translucent fills, deliberately stacked so
    // the occlusion culler has something to do.
    let card_alpha = 0.28;
    for row in 0..6 {
        for col in 0..3 {
            let x = 264.0 + col as f32 * 332.0;
            let y = 24.0 + row as f32 * 124.0;
            paints.push(Paint::Solid {
                rect: Rect::new(x, y, 318.0, 110.0),
                color: Rgba::new(0.16, 0.17, 0.21, 1.0),
            });
            paints.push(Paint::RoundedRect {
                rect: Rect::new(x + 16.0, y + 16.0, 120.0, 14.0),
                radius: 7.0,
                color: Rgba::new(0.35, 0.45, 0.65, card_alpha),
            });
            paints.push(Paint::RoundedRect {
                rect: Rect::new(x + 16.0, y + 42.0, 240.0, 10.0),
                radius: 5.0,
                color: Rgba::new(0.30, 0.32, 0.38, card_alpha),
            });
            paints.push(Paint::RoundedRect {
                rect: Rect::new(x + 16.0, y + 62.0, 180.0, 10.0),
                radius: 5.0,
                color: Rgba::new(0.30, 0.32, 0.38, card_alpha),
            });
        }
    }

    // The moving card: a shadow plus a rounded rect, driven by the spring.
    let cx = engine.card.value;
    let cy = engine.card.value * 0.6 + 300.0;
    paints.push(Paint::Shadow {
        rect: Rect::new(cx, cy, 180.0, 120.0),
        radius: 16.0,
        blur: 22.0,
        color: Rgba::new(0.0, 0.0, 0.0, 0.55),
    });
    paints.push(Paint::RoundedRect {
        rect: Rect::new(cx, cy, 180.0, 120.0),
        radius: 16.0,
        color: Rgba::new(0.95, 0.55, 0.25, 1.0),
    });

    // Atlas-free glyph, rendered from a quadratic outline with no atlas.
    // Cached: re-rasterising a static glyph every frame is pure waste.
    if engine.glyph_bitmap.is_none() {
        let t = Instant::now();
        let bmp = engine.glyph.rasterize(96.0, 4);
        engine.glyph_us = t.elapsed().as_micros() as u64;
        engine.glyph_bitmap = Some(std::sync::Arc::new(bmp));
    }
    if let Some(bitmap) = engine.glyph_bitmap.clone() {
        paints.push(Paint::Glyph {
            bitmap,
            origin: (engine.width as f32 - 150.0, 40.0),
            color: Rgba::new(0.35, 0.85, 0.70, 1.0),
        });
    }

    // HUD.
    let hud_color = Rgba::new(0.85, 0.88, 0.95, 1.0);
    text_paints(
        &format!(
            "UI ENGINE  {} FPS  {} US/FRAME  ROUTE {}/{}",
            engine.frames, engine.last_render_us, engine.route_micro, engine.route_macro
        ),
        16.0,
        engine.height as f32 - 40.0,
        2.0,
        hud_color,
        &mut paints,
    );
    text_paints(
        "MOVE THE MOUSE - THE CARD FOLLOWS A CLOSED-FORM SPRING",
        16.0,
        engine.height as f32 - 22.0,
        2.0,
        Rgba::new(0.55, 0.60, 0.70, 1.0),
        &mut paints,
    );

    paints
}

fn render(engine: &mut Engine) -> Vec<u8> {
    let start = Instant::now();

    // Layout the tree.
    let t = Instant::now();
    layout::layout(
        &mut engine.tree,
        Size::new(engine.width as f32, engine.height as f32),
    );
    engine.layout_us = t.elapsed().as_micros() as u64;

    // Advance the springs.
    if DEMO.load(Ordering::Relaxed) {
        let t = START.get_or_init(Instant::now).elapsed().as_secs_f64();
        engine.card.target = 300.0 + ((t * 1.6).sin() * 320.0) as f32;
    } else {
        engine.card.target = engine.mouse_x - 90.0;
    }
    engine.card.step(1.0 / 60.0);
    engine.fade.step(1.0 / 60.0);

    let t = Instant::now();
    let paints = build_paints(engine);
    engine.build_us = t.elapsed().as_micros() as u64;

    // Route the frame.
    let dirty = engine.width as u64 * engine.height as u64;
    let cost = FrameCost {
        dirty_px: dirty,
        layer_depth: 3,
        animated_curves: if engine.card.is_settled() { 0 } else { 4 },
    };
    match engine.scheduler.route(&cost) {
        Route::CpuMicro => engine.route_micro += 1,
        Route::GpuMacro => engine.route_macro += 1,
    }

    // Rasterize.
    let t = Instant::now();
    let mut raster = Rasterizer::new(engine.width, engine.height);
    let stats = raster.paint_all(&paints);
    engine.raster_us = t.elapsed().as_micros() as u64;
    engine.pixels_shaded += stats.pixels_painted;
    engine.pixels_total += stats.pixels_total;
    engine.frames += 1;
    FRAME_COUNTER.fetch_add(1, Ordering::Relaxed);

    let fb = raster.into_frame_buffer();
    engine.last_render_us = start.elapsed().as_micros() as u64;

    // GDI wants BGRX; the engine produces RGBA8.
    let mut bgra = vec![0u8; fb.data.len()];
    for (src, dst) in fb.data.chunks_exact(4).zip(bgra.chunks_exact_mut(4)) {
        dst[0] = src[2];
        dst[1] = src[1];
        dst[2] = src[0];
        dst[3] = src[3];
    }
    bgra
}

// ---------------------------------------------------------------------------
// Process accounting
// ---------------------------------------------------------------------------

fn working_set_bytes() -> usize {
    let mut counters = ProcessMemoryCounters {
        cb: 0,
        page_fault_count: 0,
        peak_working_set: 0,
        working_set: 0,
        quota_peak_paged: 0,
        quota_paged: 0,
        quota_peak_non_paged: 0,
        quota_non_paged: 0,
        pagefile_usage: 0,
        peak_pagefile_usage: 0,
    };
    counters.cb = std::mem::size_of::<ProcessMemoryCounters>() as u32;
    unsafe {
        GetProcessMemoryInfo(
            GetCurrentProcess(),
            &mut counters,
            std::mem::size_of::<ProcessMemoryCounters>() as u32,
        );
    }
    counters.working_set
}

fn peak_working_set_bytes() -> usize {
    let mut counters = ProcessMemoryCounters {
        cb: 0,
        page_fault_count: 0,
        peak_working_set: 0,
        working_set: 0,
        quota_peak_paged: 0,
        quota_paged: 0,
        quota_peak_non_paged: 0,
        quota_non_paged: 0,
        pagefile_usage: 0,
        peak_pagefile_usage: 0,
    };
    counters.cb = std::mem::size_of::<ProcessMemoryCounters>() as u32;
    unsafe {
        GetProcessMemoryInfo(
            GetCurrentProcess(),
            &mut counters,
            std::mem::size_of::<ProcessMemoryCounters>() as u32,
        );
    }
    counters.peak_working_set
}

fn process_cpu_seconds() -> f64 {
    let mut creation = FileTime { low: 0, high: 0 };
    let mut exit = FileTime { low: 0, high: 0 };
    let mut kernel = FileTime { low: 0, high: 0 };
    let mut user = FileTime { low: 0, high: 0 };
    unsafe {
        GetProcessTimes(
            GetCurrentProcess(),
            &mut creation,
            &mut exit,
            &mut kernel,
            &mut user,
        );
    }
    let hz = 10_000_000.0f64;
    let k = (kernel.high as f64 * 4_294_967_296.0 + kernel.low as f64) / hz;
    let u = (user.high as f64 * 4_294_967_296.0 + user.low as f64) / hz;
    k + u
}

// ---------------------------------------------------------------------------
// Window procedure
// ---------------------------------------------------------------------------

const TIMER_ID: usize = 1;

unsafe extern "system" fn window_proc(
    hwnd: Handle,
    msg: u32,
    wparam: usize,
    lparam: isize,
) -> isize {
    window_proc_impl(hwnd, msg, wparam, lparam)
}
/// The message handler. Every Win32 call below sits inside one `unsafe`
/// block, so the `unsafe fn` body does not need per-call annotations.
fn window_proc_impl(hwnd: Handle, msg: u32, wparam: usize, lparam: isize) -> isize {
    unsafe {
        match msg {
            WM_PAINT => {
                let mut ps = PaintStruct {
                    hdc: std::ptr::null_mut(),
                    f_erase: 0,
                    rc_paint: RectRaw {
                        left: 0,
                        top: 0,
                        right: 0,
                        bottom: 0,
                    },
                    f_restore: 0,
                    f_inc_update: 0,
                    rgb_reserved: [0; 32],
                };
                let dc = BeginPaint(hwnd, &mut ps);
                let mut eng = engine().lock().unwrap_or_else(|e| e.into_inner());

                let mut client = RectRaw {
                    left: 0,
                    top: 0,
                    right: 0,
                    bottom: 0,
                };
                GetClientRect(hwnd, &mut client);
                let w = (client.right - client.left) as usize;
                let h = (client.bottom - client.top) as usize;
                if w > 0 && h > 0 {
                    eng.width = w;
                    eng.height = h;
                    let pixels = render(&mut eng);

                    // Blit through a top-down 32-bit DIB.
                    let bmi = BitmapInfo {
                        header: BitmapInfoHeader {
                            size: std::mem::size_of::<BitmapInfoHeader>() as u32,
                            width: w as i32,
                            height: -(h as i32), // top-down
                            planes: 1,
                            bit_count: 32,
                            compression: BI_RGB,
                            size_image: 0,
                            x_ppm: 0,
                            y_ppm: 0,
                            clr_used: 0,
                            clr_important: 0,
                        },
                        colors: [0; 3],
                    };
                    let mut bits: *mut u8 = std::ptr::null_mut();
                    let dib = CreateDIBSection(
                        dc,
                        &bmi,
                        DIB_RGB_COLORS,
                        &mut bits,
                        std::ptr::null_mut(),
                        0,
                    );
                    if !dib.is_null() && !bits.is_null() {
                        std::ptr::copy_nonoverlapping(
                            pixels.as_ptr(),
                            bits,
                            std::cmp::min(pixels.len(), w * h * 4),
                        );
                        let mem_dc = CreateCompatibleDC(dc);
                        let old = SelectObject(mem_dc, dib);
                        BitBlt(dc, 0, 0, w as i32, h as i32, mem_dc, 0, 0, SRCCOPY);
                        SelectObject(mem_dc, old);
                        DeleteDC(mem_dc);
                        DeleteObject(dib);
                    }
                }
                EndPaint(hwnd, &ps);
                0
            }
            WM_TIMER => {
                // Only wake the compositor when there is something to draw.
                let mut eng = engine().lock().unwrap_or_else(|e| e.into_inner());
                let animating = !eng.card.is_settled() || !eng.fade.is_settled();
                if animating {
                    // NULL rect means "the whole client area". A zeroed RECT is an
                    // EMPTY region and invalidates nothing.
                    InvalidateRect(hwnd, std::ptr::null(), 0);
                } else {
                    eng.idle_frames += 1;
                }
                0
            }
            WM_MOUSEMOVE => {
                let x = (lparam & 0xFFFF) as i32 as f32;
                let y = ((lparam >> 16) & 0xFFFF) as i32 as f32;
                MOUSE_X.store(x as i32, Ordering::Relaxed);
                MOUSE_Y.store(y as i32, Ordering::Relaxed);
                let mut eng = engine().lock().unwrap_or_else(|e| e.into_inner());
                eng.mouse_x = x;
                eng.mouse_y = y;
                0
            }
            WM_ERASEBKGND => 1,
            WM_CLOSE => {
                DestroyWindow(hwnd);
                0
            }
            WM_DESTROY => {
                RUNNING.store(false, Ordering::Relaxed);
                PostQuitMessage(0);
                0
            }
            WM_KEYDOWN => {
                if wparam == 0x1B {
                    DestroyWindow(hwnd);
                }
                0
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "demo") {
        DEMO.store(true, Ordering::Relaxed);
        START.get_or_init(Instant::now);
    }
    let class_name = wide("DxUiWindow");
    let window_name = wide("ui - native Win32 host");

    unsafe {
        let instance = GetModuleHandleW(std::ptr::null());

        let class = WindowClassEx {
            cb_size: std::mem::size_of::<WindowClassEx>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            wnd_proc: Some(window_proc),
            cb_cls_extra: 0,
            cb_wnd_extra: 0,
            h_instance: instance,
            h_icon: std::ptr::null_mut(),
            h_cursor: LoadCursorW(std::ptr::null_mut(), std::ptr::null()),
            h_br_background: std::ptr::null_mut(),
            menu_name: std::ptr::null(),
            class_name: class_name.as_ptr(),
            h_icon_small: std::ptr::null_mut(),
        };
        RegisterClassExW(&class);

        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            window_name.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            80,
            60,
            1296,
            860,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            instance,
            std::ptr::null_mut(),
        );
        if hwnd.is_null() {
            eprintln!("failed to create window");
            return;
        }
        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);
        SetTimer(hwnd, TIMER_ID, 16);

        // Stats thread: prints the footprint once a second.
        std::thread::spawn(|| {
            let mut last_cpu = process_cpu_seconds();
            let mut last_frames = 0u64;
            let start = Instant::now();
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
                if !RUNNING.load(Ordering::Relaxed) {
                    break;
                }
                let eng = engine().lock().unwrap_or_else(|e| e.into_inner());
                let cpu = process_cpu_seconds();
                let dcpu = cpu - last_cpu;
                last_cpu = cpu;
                let frames = eng.frames;
                let fps = frames - last_frames;
                last_frames = frames;
                let elapsed = start.elapsed().as_secs_f64();
                println!(
                    "t={:>5.1}s  fps={fps:<4} render={:>6}us (layout {:>5} + build {:>5} + raster {:>6})  \
cpu={dcpu:>5.2}s ({:>4.1}% of one core)  ws={:>6.1}MB  frames={frames}  idle_skips={}  \
card={:.2}/{:.2} v={:.4} fade={:.4} settled={}",
                    elapsed,
                    eng.last_render_us,
                    eng.layout_us,
                    eng.build_us,
                    eng.raster_us,
                    dcpu * 100.0,
                    working_set_bytes() as f64 / 1048576.0,
                    eng.idle_frames,
                    eng.card.value,
                    eng.card.target,
                    eng.card.velocity,
                    eng.fade.value,
                    eng.card.is_settled() && eng.fade.is_settled()
                );
                drop(eng);
            }
        });

        let mut msg = Msg {
            hwnd: std::ptr::null_mut(),
            message: 0,
            wparam: 0,
            lparam: 0,
            time: 0,
            point: Point { x: 0, y: 0 },
        };
        while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&mut msg);
            DispatchMessageW(&mut msg);
        }
        KillTimer(hwnd, TIMER_ID);
    }

    let eng = engine().lock().unwrap_or_else(|e| e.into_inner());
    println!(
        "\nfinal: {} frames rendered, {} idle skips, cpu {:.2}s, ws {:.1} MB, peak ws {:.1} MB",
        eng.frames,
        eng.idle_frames,
        process_cpu_seconds(),
        working_set_bytes() as f64 / 1048576.0,
        peak_working_set_bytes() as f64 / 1048576.0
    );
    let _ = FRAME_COUNTER.load(Ordering::Relaxed);
    let _ = MOUSE_X.load(Ordering::Relaxed);
    let _ = MOUSE_Y.load(Ordering::Relaxed);
}
