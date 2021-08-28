use openexr_sys as sys;

use crate::core::rational::Rational;

/// Get a [`Rational`] representing 23.976023... fps (24/1.001)
///
pub fn fps_23_976() -> Rational {
    let mut r = Rational::default();
    unsafe {
        sys::Imf_fps_23_976(
            &mut r as *mut Rational as *mut sys::Imf_Rational_t,
        )
        .into_result()
        .expect("Unexpected expection in fps_23_976");
    }
    r
}

/// Get a [`Rational`] representing 24 fps
///
pub fn fps_24() -> Rational {
    let mut r = Rational::default();
    unsafe {
        sys::Imf_fps_24(&mut r as *mut Rational as *mut sys::Imf_Rational_t)
            .into_result()
            .expect("Unexpected expection in fps_24");
    }
    r
}

/// Get a [`Rational`] representing 25 fps
///
pub fn fps_25() -> Rational {
    let mut r = Rational::default();
    unsafe {
        sys::Imf_fps_25(&mut r as *mut Rational as *mut sys::Imf_Rational_t)
            .into_result()
            .expect("Unexpected expection in fps_25");
    }
    r
}

/// Get a [`Rational`] representing 29.970029... fps (30/1.001)
///
pub fn fps_29_97() -> Rational {
    let mut r = Rational::default();
    unsafe {
        sys::Imf_fps_29_97(&mut r as *mut Rational as *mut sys::Imf_Rational_t)
            .into_result()
            .expect("Unexpected expection in fps_29_97");
    }
    r
}

/// Get a [`Rational`] representing 30 fps
///
pub fn fps_30() -> Rational {
    let mut r = Rational::default();
    unsafe {
        sys::Imf_fps_30(&mut r as *mut Rational as *mut sys::Imf_Rational_t)
            .into_result()
            .expect("Unexpected expection in fps_30");
    }
    r
}

/// Get a [`Rational`] representing 47.952047... fps (48/1.001)
///
pub fn fps_47_952() -> Rational {
    let mut r = Rational::default();
    unsafe {
        sys::Imf_fps_47_952(
            &mut r as *mut Rational as *mut sys::Imf_Rational_t,
        )
        .into_result()
        .expect("Unexpected expection in fps_47_952");
    }
    r
}

/// Get a [`Rational`] representing 48 fps
///
pub fn fps_48() -> Rational {
    let mut r = Rational::default();
    unsafe {
        sys::Imf_fps_48(&mut r as *mut Rational as *mut sys::Imf_Rational_t)
            .into_result()
            .expect("Unexpected expection in fps_48");
    }
    r
}

/// Get a [`Rational`] representing 50 fps
///
pub fn fps_50() -> Rational {
    let mut r = Rational::default();
    unsafe {
        sys::Imf_fps_50(&mut r as *mut Rational as *mut sys::Imf_Rational_t)
            .into_result()
            .expect("Unexpected expection in fps_50");
    }
    r
}

/// Get a [`Rational`] representing 59.940059... fps (60/1.001)
///
pub fn fps_59_94() -> Rational {
    let mut r = Rational::default();
    unsafe {
        sys::Imf_fps_59_94(&mut r as *mut Rational as *mut sys::Imf_Rational_t)
            .into_result()
            .expect("Unexpected expection in fps_59_94");
    }
    r
}

/// Get a [`Rational`] representing 60 fps
///
pub fn fps_60() -> Rational {
    let mut r = Rational::default();
    unsafe {
        sys::Imf_fps_60(&mut r as *mut Rational as *mut sys::Imf_Rational_t)
            .into_result()
            .expect("Unexpected expection in fps_60");
    }
    r
}

/// Tries to convert a floating-point frame rate to the nearest "good" frame
/// rate.
///
/// If none is close enough then it returns the closest rational approximation
/// to `fps`.
///
/// ```no_run
/// use openexr::core::Rational;
/// use openexr::core::frames_per_second::{guess_exact_fps_f64, fps_59_94};
///
/// assert_eq!(guess_exact_fps_f64(59.941), fps_59_94());
/// ```
///
pub fn guess_exact_fps_f64(fps: f64) -> Rational {
    let mut r = Rational::default();
    unsafe {
        sys::Imf_guessExactFps_f64(
            &mut r as *mut Rational as *mut sys::Imf_Rational_t,
            fps,
        )
        .into_result()
        .expect("Unexpected expection in guessExactFps_f64");
    }
    r
}

// TODO: can't get this to do anything useful
// pub fn guess_exact_fps(fps: Rational) -> Rational {
//     let mut r = Rational::default();
//     unsafe {
//         sys::Imf_guessExactFps(
//             &mut r as *mut Rational as *mut sys::Imf_Rational_t,
//             &fps as *const Rational as *const sys::Imf_Rational_t,
//         )
//         .into_result()
//         .expect("Unexpected expection in guessExactFps_f64");
//     }
//     r
// }

#[cfg(test)]
#[test]
fn test_fps() {
    assert_eq!(fps_23_976(), Rational::new(24000, 1001));
    assert_eq!(fps_24(), Rational::new(24, 1));
    assert_eq!(fps_25(), Rational::new(25, 1));
    assert_eq!(fps_29_97(), Rational::new(30000, 1001));
    assert_eq!(fps_30(), Rational::new(30, 1));
    assert_eq!(fps_47_952(), Rational::new(48000, 1001));
    assert_eq!(fps_48(), Rational::new(48, 1));
    assert_eq!(fps_50(), Rational::new(50, 1));
    assert_eq!(fps_59_94(), Rational::new(60000, 1001));
    assert_eq!(fps_60(), Rational::new(60, 1));

    assert_eq!(guess_exact_fps_f64(59.941), fps_59_94());
}
