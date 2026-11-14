//! Box blur via a summed-area table.
//!
//! # The claim under test
//!
//! "`backdrop-filter: blur()` needs a ping-pong render target per layer, so
//! nested blurs are capped and VRAM explodes."
//!
//! The cap is real in engines that implement blur as a chain of separable
//! Gaussian passes over render targets. It is **not** inherent to blurring: a
//! Gaussian is well approximated by three cascaded box filters (central limit
//! theorem), and a box filter of any radius is computable in **O(1) per pixel**
//! from a summed-area table (2D prefix sum).
//!
//! That removes the radius-dependent cost, but it does **not** remove the
//! dependency on neighbouring pixels — a blur still needs data from outside the
//! pixel it writes. So:
//!
//! * The *allocation* problem goes away: one sum table, no ping-pong targets.
//! * The *nesting* problem goes away: no cap, because there is no target
//!   budget to exhaust.
//! * The *single-pass early-exit* problem stays: a blurred region is a barrier
//!   that front-to-back blending cannot cross.
//!
//! Both of those are implemented and testable here.

/// Blurs a single float plane with a box filter of the given radius.
///
/// Uses a summed-area table, so the cost per pixel is independent of the
/// radius: four table reads and three additions regardless of `radius`.
#[must_use]
pub fn box_blur_plane(src: &[f32], width: usize, height: usize, radius: usize) -> Vec<f32> {
    if src.len() < width * height || width == 0 || height == 0 {
        return src.to_vec();
    }
    let sat_width = width + 1;
    let mut sat = vec![0f32; sat_width * (height + 1)];
    for y in 0..height {
        let row = y * width;
        let sat_row = (y + 1) * sat_width;
        let prev_row = y * sat_width;
        let mut running = 0f32;
        for x in 0..width {
            running += src[row + x];
            sat[sat_row + x + 1] = sat[prev_row + x + 1] + running;
        }
    }

    let r = radius as i32;
    let mut out = vec![0f32; width * height];
    for y in 0..height {
        let y0 = (y as i32 - r).max(0);
        let y1 = (y as i32 + r + 1).min(height as i32);
        let area = (y1 - y0) as f32;
        let (top, bottom) = (y0 as usize * sat_width, y1 as usize * sat_width);
        for x in 0..width {
            let x0 = (x as i32 - r).max(0);
            let x1 = (x as i32 + r + 1).min(width as i32);
            let s = sat[bottom + x1 as usize] - sat[bottom + x0 as usize]
                - sat[top + x1 as usize]
                + sat[top + x0 as usize];
            out[y * width + x] = s / (area * (x1 - x0) as f32);
        }
    }
    out
}

/// Blurs a premultiplied RGBA8 image in place.
///
/// The data is assumed premultiplied, which is the correct space to blur in:
/// blurring straight alpha separately would bleed colour out of transparent
/// regions.
pub fn box_blur_rgba8(data: &mut [u8], width: usize, height: usize, radius: usize) {
    if radius == 0 || data.len() < width * height * 4 {
        return;
    }
    let mut planes = [
        vec![0f32; width * height],
        vec![0f32; width * height],
        vec![0f32; width * height],
        vec![0f32; width * height],
    ];
    for (i, px) in data.chunks_exact(4).enumerate() {
        planes[0][i] = f32::from(px[0]) / 255.0;
        planes[1][i] = f32::from(px[1]) / 255.0;
        planes[2][i] = f32::from(px[2]) / 255.0;
        planes[3][i] = f32::from(px[3]) / 255.0;
    }
    for plane in &mut planes {
        *plane = box_blur_plane(plane, width, height, radius);
    }
    for (i, px) in data.chunks_exact_mut(4).enumerate() {
        px[0] = (planes[0][i] * 255.0).round().clamp(0.0, 255.0) as u8;
        px[1] = (planes[1][i] * 255.0).round().clamp(0.0, 255.0) as u8;
        px[2] = (planes[2][i] * 255.0).round().clamp(0.0, 255.0) as u8;
        px[3] = (planes[3][i] * 255.0).round().clamp(0.0, 255.0) as u8;
    }
}
