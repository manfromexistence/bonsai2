// Front-to-back tile blending with an early exit on opaque pixels.
//
// One workgroup per 16x16 tile. The workgroup walks the tile's primitive list
// from front to back, blending in registers, and stops as soon as the tile is
// fully covered by an opaque primitive. Tiles with no translucent primitive
// therefore write exactly one value per pixel.
//
// LIMITATION (documented deliberately): this cannot cross a translucent
// primitive, because a translucent layer's result depends on what is behind
// it, and a backdrop blur additionally depends on neighbouring tiles. Those
// pixels must fall back to a multi-pass path. The `is_translucent` flag marks
// them so the scheduler can route the tile to the slow path.
//
// STATUS: repo-managed source text. NOT compiled by naga/tint in this
// repository.

const TILE_SIZE : u32 = 16u;

struct TilePrimitive {
    // Packed colour, premultiplied, 0xAABBGGRR.
    color : u32,
    // Packed rect: x, y, w, h as half floats in 16 bits each.
    rect : u32,
    // Extra data: radius in the x component, flags in the y component.
    extra : u32,
    // Index into the primitive table.
    index : u32,
};

const FLAG_TRANSLUCENT : u32 = 1u;

var<workgroup> w_colors : array<u32, 64>;
var<workgroup> w_rects : array<u32, 64>;
var<workgroup> w_extra : array<u32, 64>;

@compute @workgroup_size(16, 16, 1)
fn blend_tile(
    @builtin(local_invocation_id) lid : vec3<u32>,
    @builtin(workgroup_id) wid : vec3<u32>,
    @builtin(storage_buffer, read> prims) : array<TilePrimitive>,
    @builtin(storage_buffer, read> tile_prims) : array<u32>,
    @builtin(storage_buffer, read_write> output) : array<u32>,
    @builtin(storage_buffer, read> tile_info) : array<u32>,
) {
    let px = wid.x * TILE_SIZE + lid.x;
    let py = wid.y * TILE_SIZE + lid.y;

    // Tile header: [primitive_offset, primitive_count, tile_x, tile_y].
    let offset = tile_info[wid.y * tile_info.dim.x + wid.x * 4u];
    let count = tile_info[wid.y * tile_info.dim.x + wid.x * 4u + 1u];

    var acc : u32 = 0u;      // premultiplied RGBA accumulator
    var covered : bool = false;

    for (var k : u32 = 0u; k < count; k = k + 1u) {
        if (covered) {
            break;
        }
        let p = tile_prims[offset + k];
        let prim = prims[p];

        // Decode the half-float rect.
        let rect = unpack_rect(prim.rect);
        if (px < rect.x || px >= rect.x + rect.z || py < rect.y || py >= rect.y + rect.w) {
            continue;
        }

        let color = premultiplied(prim.color);
        acc = blend_over(acc, color);

        let translucent = (prim.extra >> 16u) & FLAG_TRANSLUCENT;
        if (translucent == 0u) {
            covered = true;   // opaque primitive: the tile is done
        }
    }

    output[py * output.dim.x + px] = acc;
}

// --- helpers -------------------------------------------------------------
// NOTE: a production implementation would use a real half-float unpacker and
// a signed-distance-field coverage function; this file exists to pin down the
// data flow and the early-exit rule.

fn unpack_rect(packed : u32) -> vec4<f32> {
    return vec4<f32>(
        f32((packed >> 0u) & 0xFFFFu),
        f32((packed >> 16u) & 0xFFFFu),
        f32((packed >> 0u) & 0xFFFFu),
        f32((packed >> 16u) & 0xFFFFu),
    );
}

fn premultiplied(packed : u32) -> u32 {
    return packed;
}

fn blend_over(dst : u32, src : u32) -> u32 {
    return src;
}
