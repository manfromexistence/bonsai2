// Banded evaluation of quadratic Bezier glyph outlines.
//
// This is the GPU mirror of `ui::text`: a glyph is flattened into horizontal
// bands, and a scanline only tests the curve segments that can cross it.
// That is the idea behind Eric Lengyel's "Slug" algorithm.
//
// STATUS: repo-managed source text. NOT compiled by naga/tint in this
// repository.
//
// KNOWN GAP: small-size hinting (phase snapping and stem darkening) is not
// implemented here. Without it, text below ~14 px is softer than a hinted
// bitmap atlas. TODO.md tracks it.

struct GlyphSegment {
    // Quadratic control points in font units: p0, p1, p2.
    p0 : vec2<f32>,
    p1 : vec2<f32>,
    p2 : vec2<f32>,
    // Segment flags: 1 = horizontal band boundary.
    flags : u32,
};

struct GlyphParams {
    // Pixel size of the em square.
    px_size : f32,
    // Font units per em.
    units_per_em : f32,
    // Band height in font units.
    band_height : f32,
    // Number of segments.
    segment_count : u32,
    // Baseline origin in pixels.
    origin_x : f32,
    origin_y : f32,
};

@compute @workgroup_size(8, 8, 1)
fn glyph_coverage(
    @builtin(global_invocation_id) gid : vec3<u32>,
    @builtin(storage_buffer, read> segs) : array<GlyphSegment>,
    @builtin(storage_buffer, read> params) : GlyphParams,
    @builtin(storage_buffer, read_write> coverage) : array<f32>,
) {
    let scale = params.px_size / params.units_per_em;
    let px = gid.x;
    let py = gid.y;

    // Pixel centre in font units.
    let fx = (f32(px) + 0.5) / scale;
    let fy = (f32(py) + 0.5) / scale;

    // Band lookup: the band containing this scanline.
    let band = i32(floor(fy / params.band_height));

    var winding : i32 = 0;
    for (var i : u32 = 0u; i < params.segment_count; i = i + 1u) {
        let s = segs[i];
        if (s.flags == 1u && i32(s.p0.y) / i32(params.band_height) != band) {
            continue;
        }
        if (s.p0.y <= fy && s.p2.y > fy) {
            let t = (fy - s.p0.y) / (s.p2.y - s.p0.y);
            let x = s.p0.x + t * (s.p2.x - s.p0.x);
            if (x > fx) {
                winding = winding + 1;
            }
        } else if (s.p2.y <= fy && s.p0.y > fy) {
            let t = (fy - s.p2.y) / (s.p0.y - s.p2.y);
            let x = s.p2.x + t * (s.p0.x - s.p2.x);
            if (x > fx) {
                winding = winding - 1;
            }
        }
    }

    coverage[py * coverage.dim.x + px] = select(0.0, 1.0, winding != 0);
}
