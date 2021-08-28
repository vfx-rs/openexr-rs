fn merge_overlapping(
    a1: f32,
    c1: f32, // Opacity and color of first sample
    a2: f32,
    c2: f32, // Opacity and color of second sample
) -> (f32, f32) {
    // This function merges two perfectly overlapping volume or point
    // samples. Given the color and opacity of two samples, it returns
    // the color and opacity of the merged sample.
    //
    // The code below is written to avoid very large rounding errors when
    // the opacity of one or both samples is very small:
    //
    // * The merged opacity must not be computed as 1 - (1-a1) * (1-a2).
    // If a1 and a2 are less than about half a floating-point epsilon,
    // the expressions (1-a1) and (1-a2) evaluate to 1.0 exactly, and the
    // merged opacity becomes 0.0. The error is amplified later in the
    // calculation of the merged color.
    //
    // Changing the calculation of the merged opacity to a1 + a2 - a1\*a2
    // avoids the excessive rounding error.
    //
    // * For small x, the logarithm of 1+x is approximately equal to x,
    // but log(1+x) returns 0 because 1+x evaluates to 1.0 exactly.
    // This can lead to large errors in the calculation of the merged
    // color if a1 or a2 is very small.
    //
    // x.ln_1p() returns the logarithm of 1+x, but without attempting to
    // evaluate the expression 1+x when x is very small.
    //
    let a1 = a1.clamp(0.0, 1.0);
    let a2 = a2.clamp(0.0, 1.0);
    let am = a1 + a2 - a1 * a2;
    if a1 == 1.0 && a2 == 1.0 {
        (am, (c1 + c2) / 2.0)
    } else if a1 == 1.0 {
        (am, c1)
    } else if a2 == 1.0 {
        (am, c2)
    } else {
        let u1 = -((-a1).ln_1p());
        let v1 = if u1 < a1 * f32::MAX { u1 / a1 } else { 1.0 };
        let u2 = -((-a2).ln_1p());
        let v2 = if u2 < a2 * f32::MAX { u2 / a2 } else { 1.0 };
        let u = u1 + u2;
        let w = if u > 1.0 || am < u * f32::MAX {
            am / u
        } else {
            1.0
        };
        (am, (c1 * v1 + c2 * v2) * w)
    }
}

fn main() {
    let a1 = 0.5f32;
    let c1 = 0.2f32;
    let a2 = 0.3f32;
    let c2 = 0.4f32;

    assert_eq!(merge_overlapping(a1, c1, a2, c2), (0.65, 0.46611378))
}
