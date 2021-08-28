use openexr_sys as sys;

/// A rational number is represented as pair of integers, n and d.
///
/// The value of of the rational number is:
/// * `n/d  for d > 0`
/// * `+inf for n > 0, d == 0`
/// * `-inf for n < 0, d == 0`
/// * `NaN  for n == 0, d == 0`
///
#[repr(C)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub struct Rational {
    pub num: i32,
    pub denom: u32,
}

// we're going to transmute Rational and sys::Imf_Rational_t so make sure
// they're the same size
const _: fn() = || {
    let _ = std::mem::transmute::<Rational, sys::Imf_Rational_t>;
};

impl Rational {
    /// Create a new `Rational` specified as `num/denom`
    ///
    pub fn new(num: i32, denom: u32) -> Self {
        Rational { num, denom }
    }
}

impl Default for Rational {
    fn default() -> Self {
        Rational::new(0, 1)
    }
}

impl From<sys::Imf_Rational_t> for Rational {
    fn from(s: sys::Imf_Rational_t) -> Self {
        Rational::new(s.n, s.d)
    }
}

impl From<Rational> for sys::Imf_Rational_t {
    fn from(r: Rational) -> Self {
        sys::Imf_Rational_t {
            n: r.num,
            d: r.denom,
        }
    }
}

impl From<f64> for Rational {
    fn from(f: f64) -> Self {
        let mut r = Rational::default();
        unsafe {
            sys::Imf_Rational_from_f64(
                &mut r as *mut Rational as *mut sys::Imf_Rational_t,
                f,
            )
            .into_result()
            .expect("Unexpected exception from Imf_Rational_from_f64");
        }
        r
    }
}

impl From<Rational> for f64 {
    fn from(r: Rational) -> f64 {
        let mut f = 0.0;
        let mut r = r;
        unsafe {
            sys::Imf_Rational_as_f64(
                &mut r as *mut Rational as *mut sys::Imf_Rational_t,
                &mut f,
            )
            .into_result()
            .expect("Unexpected exception from Imf_Rational_as_f64");
        }
        f
    }
}

#[cfg(test)]
#[test]
fn test_rational() {
    let r = Rational::new(0, 0);
    assert!(f64::from(r).is_nan());

    let r = Rational::new(1, 0);
    assert!(f64::from(r).is_infinite() && f64::from(r).is_sign_positive());

    let r = Rational::new(-1, 0);
    assert!(f64::from(r).is_infinite() && f64::from(r).is_sign_negative());

    let r = Rational::new(1, 2);
    assert_eq!(f64::from(r), 0.5);
}
