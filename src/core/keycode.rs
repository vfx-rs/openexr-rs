use openexr_sys as sys;

use crate::core::error::Error;

type Result<T, E = Error> = std::result::Result<T, E>;

/// A KeyCode object uniquely identifies a motion picture film frame.
/// The following fields specifiy film manufacturer, film type, film
/// roll and the frame's position within the roll.
///
/// # Fields
///
/// * `film_mfc_code` - Film manufacturer code.
///     Range: `[0, 99]`
/// * `filmType` - Film type code.
///     Range: `[0, 99]`
/// * `prefix` - Prefix to identify film roll.
///     Range: `[0, 999999]`
/// * `count` - Count, increments once every perfs_per_count perforations.
///     Range: `[0, 9999]`
/// * `perf_offset` - Offset of frame, in perforations from zero-frame reference mark
///     Range: `[0, 119]`
/// * `perfs_per_frame` - Number of perforations per frame. Typical values are 1 for 16mm film; 3, 4 or 8 for 35mm film; 5, 8 or 15 for 65mm film.
///     Range: `[1, 15]`
/// * `perfs_per_count` - Number of perforations per count. Typical values are 20 for 16mm film, 64 for 35mm film, 80 or 120 for 65mm film.
///     Range: `[20, 120]`
///
/// # Further Reading
/// For more information about the interpretation of those fields see
/// the following standards and recommended practice publications:
/// * SMPTE 254 Motion-Picture Film (35-mm) - Manufacturer-Printed
///         Latent Image Identification Information
/// * SMPTE 268M    File Format for Digital Moving-Picture Exchange (DPX)
///         (section 6.1)
/// * SMPTE 270 Motion-Picture Film (65-mm) - Manufacturer- Printed
///         Latent Image Identification Information
/// * SMPTE 271 Motion-Picture Film (16-mm) - Manufacturer- Printed
///         Latent Image Identification Information
///
#[repr(transparent)]
pub struct KeyCode(sys::Imf_KeyCode_t);

impl KeyCode {
    /// Get the film manufacturer code. Valid range `[0, 99]`
    ///
    pub fn film_mfc_code(&self) -> i32 {
        let mut v = 0i32;
        unsafe {
            sys::Imf_KeyCode_filmMfcCode(&self.0, &mut v)
                .into_result()
                .expect("Unexpected exception from Imf_KeyCode_filmMfcCode");
        }
        v
    }

    /// Set the film manufacturer code. Valid range `[0, 99]`
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - If `code` is not in the range `[0, 99]`
    ///
    /// TODO: Do we want to implement a bounded integer here to specify the range?
    pub fn set_film_mfc_code(&mut self, code: i32) -> Result<()> {
        unsafe {
            sys::Imf_KeyCode_setFilmMfcCode(&mut self.0, code).into_result()?;
        }
        Ok(())
    }

    /// Get the film type code. Valid range `[0, 99]`
    ///
    pub fn film_type(&self) -> i32 {
        let mut v = 0i32;
        unsafe {
            sys::Imf_KeyCode_filmType(&self.0, &mut v)
                .into_result()
                .expect("Unexpected exception from Imf_KeyCode_filmType");
        }
        v
    }

    /// Set the film type code. Valid range `[0, 99]`
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - If `code` is not in the range `[0, 99]`
    ///
    pub fn set_film_type(&mut self, code: i32) -> Result<()> {
        unsafe {
            sys::Imf_KeyCode_setFilmType(&mut self.0, code).into_result()?;
        }
        Ok(())
    }

    /// Get the prefix code which identifies the film roll.
    /// Valid range `[0, 999999]`
    ///
    pub fn prefix(&self) -> i32 {
        let mut v = 0i32;
        unsafe {
            sys::Imf_KeyCode_filmType(&self.0, &mut v)
                .into_result()
                .expect("Unexpected exception from Imf_KeyCode_filmType");
        }
        v
    }

    /// Set the prefix code which identifies the film roll.
    /// Valid range `[0, 999999]`
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - If `code` is not in the range `[0, 999999]`
    ///
    pub fn set_prefix(&mut self, v: i32) -> Result<()> {
        unsafe {
            sys::Imf_KeyCode_setPrefix(&mut self.0, v).into_result()?;
        }
        Ok(())
    }

    /// Get the count, which increments every `perfs_per_count` perforations.
    /// Valid range [0, 9999]
    ///
    pub fn count(&self) -> i32 {
        let mut v = 0i32;
        unsafe {
            sys::Imf_KeyCode_filmType(&self.0, &mut v)
                .into_result()
                .expect("Unexpected exception from Imf_KeyCode_filmType");
        }
        v
    }

    /// Set the count, which increments every `perfs_per_count` perforations.
    /// Valid range [0, 9999]
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - If `count` is not in the range `[0, 9999]`
    ///
    pub fn set_count(&mut self, count: i32) -> Result<()> {
        unsafe {
            sys::Imf_KeyCode_setCount(&mut self.0, count).into_result()?;
        }
        Ok(())
    }

    /// Get the offset of the frame in perforations from the zero-frame reference mark.
    /// Valid range [0, 119]
    ///
    pub fn perf_offset(&self) -> i32 {
        let mut v = 0i32;
        unsafe {
            sys::Imf_KeyCode_filmType(&self.0, &mut v)
                .into_result()
                .expect("Unexpected exception from Imf_KeyCode_filmType");
        }
        v
    }

    /// Set the offset of the frame in perforations from the zero-frame reference mark.
    /// Valid range [0, 119]
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - If `offset` is not in the range `[0, 119]`
    ///
    pub fn set_perf_offset(&mut self, offset: i32) -> Result<()> {
        unsafe {
            sys::Imf_KeyCode_setPerfOffset(&mut self.0, offset)
                .into_result()?;
        }
        Ok(())
    }

    /// Get the number of perforations per frame.
    /// Valid range [1, 15]
    ///
    /// Typical values:
    /// * 1 for 16mm film
    /// * 3, 4 or 8 for 35mm film
    /// * 5, 8, or 15 for 65mm film
    ///
    pub fn perfs_per_frame(&self) -> i32 {
        let mut v = 0i32;
        unsafe {
            sys::Imf_KeyCode_filmType(&self.0, &mut v)
                .into_result()
                .expect("Unexpected exception from Imf_KeyCode_filmType");
        }
        v
    }

    /// Set the number of perforations per frame.
    /// Valid range [1, 15]
    ///
    /// Typical values:
    /// * 1 for 16mm film
    /// * 3, 4 or 8 for 35mm film
    /// * 5, 8, or 15 for 65mm film
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - If `perfs` is not in the range `[1, 15]`
    ///
    pub fn set_perfs_per_frame(&mut self, perfs: i32) -> Result<()> {
        unsafe {
            sys::Imf_KeyCode_setPerfsPerFrame(&mut self.0, perfs)
                .into_result()?;
        }
        Ok(())
    }

    /// Get the number of perforations per count.
    /// Valid range [2, 120]
    ///
    /// Typical values:
    /// * 20 for 16mm film
    /// * 64 for 35mm film
    /// * 80 or 120 for 65mm film
    ///
    pub fn perfs_per_count(&self) -> i32 {
        let mut v = 0i32;
        unsafe {
            sys::Imf_KeyCode_filmType(&self.0, &mut v)
                .into_result()
                .expect("Unexpected exception from Imf_KeyCode_filmType");
        }
        v
    }

    /// Set the number of perforations per count.
    /// Valid range [2, 120]
    ///
    /// Typical values:
    /// * 20 for 16mm film
    /// * 64 for 35mm film
    /// * 80 or 120 for 65mm film
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - If `perfs` is not in the range `[2, 120]`
    ///
    pub fn set_perfs_per_count(&mut self, perfs: i32) -> Result<()> {
        unsafe {
            sys::Imf_KeyCode_setPerfsPerCount(&mut self.0, perfs)
                .into_result()?;
        }
        Ok(())
    }
}

impl Default for KeyCode {
    fn default() -> Self {
        let mut inner = sys::Imf_KeyCode_t::default();
        unsafe {
            sys::Imf_KeyCode_ctor(&mut inner, 0, 0, 0, 0, 0, 4, 64)
                .into_result()
                .expect("Unexpected exception from Imf_KeyCode_ctor");
        }
        KeyCode(inner)
    }
}

#[cfg(test)]
#[test]
fn test_keycode() {
    let mut k = KeyCode::default();

    assert!(k.set_film_mfc_code(-1).is_err());
    assert!(k.set_film_mfc_code(1).is_ok());
    assert_eq!(k.film_mfc_code(), 1);
}
