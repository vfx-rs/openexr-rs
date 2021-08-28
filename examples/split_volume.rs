fn split_volume_sample(
    a: f32,
    c: f32, // opacity and colour of original sample
    zf: f32,
    zb: f32, // front and back of original sample
    z: f32,  // position of split
) -> ((f32, f32), (f32, f32)) {
    // Given a volume sample whose front and back are at depths zf and zb
    // respectively, split the sample at depth z. Return the opacities
    // and colors of the two parts that result from the split.
    //
    // The code below is written to avoid excessive rounding errors when
    // the opacity of the original sample is very small:
    //
    // The straightforward computation of the opacity of either part
    // requires evaluating an expression of the form
    //
    // 1.0 - (1.0 - a).pow(x)
    //
    // However, if a is very small, then 1-a evaluates to 1.0 exactly,
    // and the entire expression evaluates to 0.0.
    //
    // We can avoid this by rewriting the expression as
    //
    // 1.0 - (x * (1.0 - a).log()).exp()
    //
    // and replacing the call to log() with a call to the method ln_1p(),
    // which computes the logarithm of 1+x without attempting to evaluate
    // the expression 1+x when x is very small.
    //
    // Now we have
    //
    // 1.0 - (x * (-a).ln_1p()).exp()
    //
    // However, if a is very small then the call to exp() returns 1.0, and
    // the overall expression still evaluates to 0.0. We can avoid that
    // by replacing the call to exp() with a call to expm1():
    //
    // -(x * (-a).ln_1p()).exp_m1()
    //
    // x.exp_m1() computes exp(x) - 1 in such a way that the result is accurate
    // even if x is very small.
    //
    assert!(zb > zf && z >= zf && z <= zb);

    let a = a.clamp(0.0, 1.0);

    if a == 1.0f32 {
        ((1.0f32, c), (1.0f32, c))
    } else {
        let xf = (z - zf) / (zb - zf);
        let xb = (zb - z) / (zb - zf);
        if a > f32::MIN_POSITIVE {
            //let af = -expm1 (xf * log1p (-a));
            let af = -((-a).ln_1p() * xf).exp_m1();
            let cf = (af / a) * c;
            //ab = -expm1 (xb * log1p (-a));
            let ab = -((-a).ln_1p() * xb).exp_m1();
            let cb = (ab / a) * c;
            ((af, cf), (ab, cb))
        } else {
            ((a * xf, c * xf), (a * xb, c * xb))
        }
    }
}

fn main() {
    let a = 0.5f32;
    let c = 1.0f32;
    let zf = 0.0f32;
    let zb = 1.0f32;

    assert_eq!(
        split_volume_sample(a, c, zf, zb, 0.5),
        ((0.29289323, 0.58578646), (0.29289323, 0.58578646))
    );
    assert_eq!(
        split_volume_sample(a, c, zf, zb, 1.0e-7),
        (
            (0.000000069314716, 0.00000013862943),
            (0.49999997, 0.99999994)
        )
    );
}
