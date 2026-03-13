// https://github.com/jorgecarleitao/perlin-rs/blob/main/src/lib.rs

static PERM: [usize; 512] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180, // copy of the first part
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180,
];

#[inline]
fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

#[inline]
fn lerp(t: f32, a: f32, b: f32) -> f32 {
    a + t * (b - a)
}

#[inline]
fn grad_2d(hash: usize, x: f32, y: f32) -> f32 {
    // in 2D, we only select from 4 different gradients.
    // http://riven8192.blogspot.com/2010/08/calculate-perlinnoise-twice-as-fast.html
    match hash % 4 {
        0 => x + y,  // (1, 1)
        1 => -x + y, // (-1, 1)
        2 => x - y,  // (1, -1)
        3 => -x - y, // (-1, -1)
        _ => 0.0,    // unreachable
    }
}

#[inline]
fn perlin_2d_grad(x: f32, y: f32, g00: usize, g10: usize, g01: usize, g11: usize) -> f32 {
    // compute the gradients
    // note: each corner has its own independent direction (derived from the permutation table)
    // g00 represents the dot product of (x,y) with one of the directions assigned to the corner `(0,0)` (e.g. `(1,1)`)
    let g00 = grad_2d(g00, x, y); // (x,y) - (0,0)
    let g10 = grad_2d(g10, x - 1.0, y); // (x,y) - (1,0)

    let g01 = grad_2d(g01, x, y - 1.0); // (x,y) - (0,1)
    let g11 = grad_2d(g11, x - 1.0, y - 1.0); // (x,y) - (1,1)

    // smoothed x (continuous second derivative)
    let u = fade(x);
    // smoothed y (continuous second derivative)
    let v = fade(y);

    // g00 + f(x) * (g10 - g00) + f(y) * (g01 + f(x) * (g11 - g01) - (g00 + f(x) * (g10 - g00)))
    lerp(v, lerp(u, g00, g10), lerp(u, g01, g11))
    // in particular
    // x = 0 and y = 0 => g00 => 0
    // x = 1 and y = 0 => g10 => 0
    // x = 0 and y = 1 => g01 => 0
    // x = 1 and y = 1 => g11 => 0
    // i.e. noise at each corner equals to zero
    // x = 0.5 and y = 0.5 => (g00+g10+g01+g11)/4
    // i.e. noise at the center equals to the average of the gradients
}

/// Returns the evaluation of perlin noise at position (x, y)
/// This function does not allocate
/// It uses the improved implementation of perlin noise
/// whose reference implementation is available here: https://mrl.cs.nyu.edu/~perlin/noise/
/// The modifications are:
/// * made it 2d, ignoring the z coordinate
/// * the grad computation was modified
pub fn perlin_2d(mut x: f32, mut y: f32) -> f32 {
    let x0 = x as usize;
    let y0 = y as usize;

    x -= x0 as f32;
    y -= y0 as f32;
    // x = x % 1.0;
    // y = y % 1.0;
    // at this point (x, y) is bounded to [0, 1]

    debug_assert!((x >= 0.0) && (x <= 1.0) && (y >= 0.0) && (y <= 1.0));

    let gx = x0 % 256;
    let gy = y0 % 256;

    // derive a permutation from the indices.
    // This behaves like a weak hash
    // note that the +1's must be consistent with the relative position in the box
    let a00 = gy + PERM[gx];
    let a10 = gy + PERM[gx + 1];

    let g00 = PERM[a00];
    let g10 = PERM[a10];
    let g01 = PERM[1 + a00];
    let g11 = PERM[1 + a10];

    perlin_2d_grad(x, y, g00, g10, g01, g11)
}
