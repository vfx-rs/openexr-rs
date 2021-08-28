/// Latitude-Longitude Map
///
/// The environment is projected onto the image using polar coordinates
/// (latitude and longitude).  A pixel's x coordinate corresponds to
/// its longitude, and the y coordinate corresponds to its latitude.
/// Pixel (dataWindow.min.x, dataWindow.min.y) has latitude +pi/2 and
/// longitude +pi; pixel (dataWindow.max.x, dataWindow.max.y) has
/// latitude -pi/2 and longitude -pi.
///
/// In 3D space, latitudes -pi/2 and +pi/2 correspond to the negative and
/// positive y direction.  Latitude 0, longitude 0 points into positive
/// z direction; and latitude 0, longitude pi/2 points into positive x
/// direction.
///
/// The size of the data window should be 2*N by N pixels (width by height),
/// where N can be any integer greater than 0.
///
pub mod latlong {
    use openexr_sys as sys;

    use imath_traits::{Bound2, Vec2, Vec3};

    /// Convert a 3D direction to a 2D vector whose x and y components represent
    /// the corresponding latitude and longitude.
    ///
    pub fn latlong_from_direction<V3: Vec3<f32>, V2: Vec2<f32>>(
        direction: &V3,
    ) -> V2 {
        let mut v = [0.0f32; 2];
        unsafe {
            sys::Imf_LatLongMap_latlong_from_direction(
                v.as_mut_ptr() as *mut sys::Imath_V2f_t,
                direction.as_ptr() as *const sys::Imath_V3f_t,
            );
        }

        V2::from_slice(&v)
    }

    /// Convert the position of a pixel to a 2D vector whose x and y components
    /// represent the corresponding latitude and longitude.
    ///
    pub fn latlong_from_pixel_position<B: Bound2<i32>, V: Vec2<f32>>(
        data_window: &B,
        pixel_position: &V,
    ) -> V {
        let mut v = [0.0f32; 2];

        unsafe {
            sys::Imf_LatLongMap_latlong_from_pixel_position(
                v.as_mut_ptr() as *mut sys::Imath_V2f_t,
                data_window.as_ptr() as *const sys::Imath_Box2i_t,
                pixel_position.as_ptr() as *const sys::Imath_V2f_t,
            );
        }

        V::from_slice(&v)
    }

    /// Convert a 2D vector, whose x and y components represent longitude and
    /// latitude, into a corresponding pixel position.
    ///
    pub fn pixel_position_from_latlong<B: Bound2<i32>, V: Vec2<f32>>(
        data_window: &B,
        latlong: &V,
    ) -> V {
        let mut v = [0.0f32; 2];

        unsafe {
            sys::Imf_LatLongMap_pixelPosition_from_latlong(
                v.as_mut_ptr() as *mut sys::Imath_V2f_t,
                data_window.as_ptr() as *const sys::Imath_Box2i_t,
                latlong.as_ptr() as *const sys::Imath_V2f_t,
            );
        }

        V::from_slice(&v)
    }

    /// Convert a 2D vector, whose x and y components represent longitude and
    /// latitude, into a corresponding pixel position.
    ///
    pub fn pixel_position_from_direction<
        B: Bound2<i32>,
        V2: Vec2<f32>,
        V3: Vec3<f32>,
    >(
        data_window: &B,
        direction: &V3,
    ) -> V2 {
        let mut v = [0.0f32; 2];

        unsafe {
            sys::Imf_LatLongMap_pixelPosition_from_direction(
                v.as_mut_ptr() as *mut sys::Imath_V2f_t,
                data_window.as_ptr() as *const sys::Imath_Box2i_t,
                direction.as_ptr() as *const sys::Imath_V3f_t,
            );
        }

        V2::from_slice(&v)
    }

    /// Convert a 2D vector, whose x and y components represent longitude and
    /// latitude, into a corresponding pixel position.
    ///
    pub fn direction_from_pixel_position<
        B: Bound2<i32>,
        V2: Vec2<f32>,
        V3: Vec3<f32>,
    >(
        data_window: &B,
        position: &V2,
    ) -> V3 {
        let mut v = [0.0f32; 3];

        unsafe {
            sys::Imf_LatLongMap_direction_from_pixel_position(
                v.as_mut_ptr() as *mut sys::Imath_V3f_t,
                data_window.as_ptr() as *const sys::Imath_Box2i_t,
                position.as_ptr() as *const sys::Imath_V2f_t,
            );
        }

        V3::from_slice(&v)
    }
}

pub mod cube_map {
    use crate::core::CubeMapFace;
    use imath_traits::{Bound2, Vec2, Vec3};
    use openexr_sys as sys;

    /// Width and height of a cube's face, in pixels
    ///
    pub fn size_of_face<B: Bound2<i32>>(data_window: &B) -> i32 {
        let mut v = 0i32;
        unsafe {
            sys::Imf_CubeMap_sizeOfFace(
                &mut v,
                data_window.as_ptr() as *const sys::Imath_Box2i_t,
            )
            .into_result()
            .unwrap();
        }
        v
    }

    /// Compute the region in the environment map that is covered by the
    /// specified face.
    ///
    pub fn data_window_for_face<B: Bound2<i32>>(
        face: CubeMapFace,
        data_window: &B,
    ) -> B {
        let mut v = [0i32; 4];
        unsafe {
            sys::Imf_CubeMap_dataWindowForFace(
                v.as_mut_ptr() as *mut sys::Imath_Box2i_t,
                face.into(),
                data_window.as_ptr() as *const sys::Imath_Box2i_t,
            )
            .into_result()
            .unwrap();
        }

        B::from_slice(&v)
    }

    /// Convert the coordinates of a pixel within a face in the range from
    /// `(0,0)` to `(s-1,s-1)`, where `s == size_of_face(data_window)` to pixel
    /// coordinates in the environment map.
    ///
    pub fn pixel_position_from_position_in_face<
        B: Bound2<i32>,
        V: Vec2<f32>,
    >(
        face: sys::CubeMapFace,
        data_window: &B,
        position_in_face: V,
    ) -> V {
        let mut v = [0f32; 2];
        unsafe {
            sys::Imf_CubeMap_pixelPosition_from_position_in_face(
                v.as_mut_ptr() as *mut sys::Imath_V2f_t,
                face.into(),
                data_window.as_ptr() as *const sys::Imath_Box2i_t,
                sys::Imath_V2f_t {
                    x: position_in_face.as_slice()[0],
                    y: position_in_face.as_slice()[1],
                },
            )
            .into_result()
            .unwrap();
        }

        V::from_slice(&v)
    }

    /// Convert a 3D direction into a cube face, and a pixel position
    /// within that face.
    ///
    /// If you have a 3D direction, `dir`, the following code fragment
    /// finds the position, `pos`, of the corresponding pixel in an
    /// environment map with data window `dw`:
    /// ```no_run
    /// use openexr::prelude::*;
    ///
    /// let dir = [0.0f32, 1.0, 0.0];
    /// let dw = [0, 0, 255, 127];
    ///
    /// let (face, position_in_face): (CubeMapFace, [f32; 2]) =
    ///     cube_map::face_and_position_in_face_from_direction(
    ///         &dir,
    ///         &dw
    ///     );
    /// let pos = cube_map::pixel_position_from_position_in_face(
    ///     face,
    ///     &dw,
    ///     position_in_face
    /// );
    /// ```
    ///
    pub fn face_and_position_in_face_from_direction<
        V3: Vec3<f32>,
        B: Bound2<i32>,
        V2: Vec2<f32>,
    >(
        direction: &V3,
        data_window: &B,
    ) -> (CubeMapFace, V2) {
        let mut face = CubeMapFace::PosY;
        let mut position_in_face = [0f32; 2];

        unsafe {
            sys::Imf_CubeMap_faceAndPixelPosition_from_direction(
                direction.as_ptr() as *const sys::Imath_V3f_t,
                data_window.as_ptr() as *const sys::Imath_Box2i_t,
                &mut face as *mut CubeMapFace as *mut sys::Imf_CubeMapFace,
                position_in_face.as_mut_ptr() as *mut sys::Imath_V2f_t,
            )
            .into_result()
            .unwrap();
        }

        (face, V2::from_slice(&position_in_face))
    }

    /// Given a cube face and a pixel position within that face, compute the
    /// corresponding 3D direction.
    ///
    pub fn direction_from_face_and_position_in_face<
        B: Bound2<i32>,
        V2: Vec2<f32>,
        V3: Vec3<f32>,
    >(
        face: CubeMapFace,
        data_window: &B,
        position_in_face: &V2,
    ) -> V3 {
        let mut v = [0f32; 3];
        unsafe {
            sys::Imf_CubeMap_direction_from_position_in_face(
                v.as_mut_ptr() as *mut sys::Imath_V3f_t,
                face.into(),
                data_window.as_ptr() as *const sys::Imath_Box2i_t,
                position_in_face.as_ptr() as *const sys::Imath_V2f_t,
            )
            .into_result()
            .unwrap();
        }

        V3::from_slice(&v)
    }
}

#[cfg(test)]
#[test]
fn test_latlong_from_direction() {
    let ll: [f32; 2] = latlong::latlong_from_direction(&[0.0f32, 1.0, 0.0]);
    assert_eq!(ll, [1.5707964f32, 0.0]);
}

#[cfg(test)]
#[test]
fn test_latlong_from_pixel_position() {
    let dw = [0, 0, 255, 127];
    let pp = [255.0f32, 127.0];
    let ll = latlong::latlong_from_pixel_position(&dw, &pp);
    assert_eq!(ll, [-1.5707964, -3.1415927]);
}

#[cfg(test)]
#[test]
fn test_pixel_position_from_latlong() {
    let dw = [0, 0, 255, 127];
    let ll = [std::f32::consts::FRAC_PI_2, std::f32::consts::PI];
    let pp = latlong::pixel_position_from_latlong(&dw, &ll);
    assert_eq!(pp, [0.0f32, 0.0]);
}

#[cfg(test)]
#[test]
fn test_pixel_position_from_direction() {
    let dw = [0, 0, 255, 127];
    let dir = [0.0f32, 1.0, 0.0];
    let pp: [f32; 2] = latlong::pixel_position_from_direction(&dw, &dir);
    assert_eq!(pp, [127.5f32, 0.0]);
}

#[cfg(test)]
#[test]
fn test_direction_from_pixel_position() {
    let dw = [0, 0, 255, 127];
    let pp = [127.5f32, 0.0];
    let dir: [f32; 3] = latlong::direction_from_pixel_position(&dw, &pp);

    let eps = 1.0e-5f32;
    assert!(
        dir[0].abs() < eps
            && (dir[1] - 1.0f32).abs() < eps
            && dir[2].abs() < eps
    );
}

#[cfg(test)]
#[test]
fn test_size_of_face() {
    let dw = [0, 0, 399, 299];
    let sz = cube_map::size_of_face(&dw);
    assert_eq!(sz, 50);
}

#[cfg(test)]
#[test]
fn test_data_window_for_face() {
    let dw = [0, 0, 399, 299];
    let fdw =
        cube_map::data_window_for_face(crate::core::CubeMapFace::PosY, &dw);
    assert_eq!(fdw, [0, 100, 49, 149]);
}

#[cfg(test)]
#[test]
fn test_pixel_position_from_position_in_face() {
    let dw = [0, 0, 399, 299];
    let pp = cube_map::pixel_position_from_position_in_face(
        crate::core::CubeMapFace::PosY,
        &dw,
        [0.0f32, 0.0],
    );
    assert_eq!(pp, [0.0f32, 149.0]);
}

#[cfg(test)]
#[test]
fn test_face_and_position_in_face_from_direction() {
    let dir = [0.0f32, 1.0, 0.0];
    let dw = [0, 0, 255, 127];

    let (face, position_in_face): (crate::core::CubeMapFace, [f32; 2]) =
        cube_map::face_and_position_in_face_from_direction(&dir, &dw);

    assert_eq!(face, crate::core::CubeMapFace::PosY);
    assert_eq!(position_in_face, [10.0f32, 10.0]);
}

#[cfg(test)]
#[test]
fn test_direction_from_face_and_position_in_face() {
    let dw = [0, 0, 255, 127];
    let dir: [f32; 3] = cube_map::direction_from_face_and_position_in_face(
        crate::core::CubeMapFace::PosY,
        &dw,
        &[10.0f32, 10.0],
    );

    assert_eq!(dir, [0.0f32, 1.0, 0.0]);
}
