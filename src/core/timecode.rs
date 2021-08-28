use crate::core::error::Error;
use openexr_sys as sys;
use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::mem::MaybeUninit;
use std::os::raw::{c_int, c_uint};

type Result<T, E = Error> = std::result::Result<T, E>;

/// Bit packing variants
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeCodePacking {
    /// packing for 60-field television
    ///
    Tv60,
    /// packing for 50-field television
    ///
    Tv50,
    /// packing for 24-frame film
    ///
    Film24,
}

impl TimeCodePacking {
    fn from_sys_packing(packing: sys::Imf_TimeCode_Packing) -> Self {
        match packing {
            sys::Imf_TimeCode_Packing_TV60_PACKING => TimeCodePacking::Tv60,
            sys::Imf_TimeCode_Packing_TV50_PACKING => TimeCodePacking::Tv50,
            sys::Imf_TimeCode_Packing_FILM24_PACKING => TimeCodePacking::Film24,
            _ => panic!("Packing is invalid"),
        }
    }

    fn to_sys_packing(&self) -> sys::Imf_TimeCode_Packing {
        match self {
            Self::Tv60 => sys::Imf_TimeCode_Packing_TV60_PACKING,
            Self::Tv50 => sys::Imf_TimeCode_Packing_TV50_PACKING,
            Self::Film24 => sys::Imf_TimeCode_Packing_FILM24_PACKING,
        }
    }
}

/// A TimeCode object stores time and control codes as described in SMPTE
/// standard 12M-1999.  A TimeCode object contains the following fields:
///
/// Time Address:
/// - hours: integer, range 0 - 23
/// - minutes: integer, range 0 - 59
/// - seconds: integer, range 0 - 59
/// - frame: integer, range 0 - 29
///
/// Flags:
/// - drop frame flag: boolean
/// - color frame flag: boolean
/// - field/phase flag: boolean
/// - bgf0: boolean
/// - bgf1: boolean
/// - bgf2: boolean
///
/// Binary groups for user-defined data and control codes:
/// - binary group 1: integer, range 0 - 15
/// - binary group 2: integer, range 0 - 15
/// - ...
/// - binary group 8: integer, range 0 - 15
///
/// TimeCode contains methods to convert between the fields listed above and a
/// more compact representation where the fields are packed into two unsigned
/// 32-bit integers. In the packed integer representations, bit 0 is the least
/// significant bit, and bit 31 is the most significant bit of the integer
/// value. The time address and flags fields can be packed in three different
/// ways:
///
/// | bits    | packing for 24-frame film | packing for 60-field television | packing for 50-field television |
/// | ------- | ------------------------- | ------------------------------- | ------------------------------- |
/// | 0 - 3   | frame units               | frame units                     | frame units                     |
/// | 4 - 5   | frame tens                | frame tens                      | frame tens                      |
/// | 6       | unused, set to 0          | drop frame flag                 | unused, set to 0                |
/// | 7       | unused, set to 0          | color frame flag                | color frame flag                |
/// | 8 - 11  | seconds units             | seconds units                   | seconds units                   |
/// | 12 - 14 | seconds tens              | seconds tens                    | seconds tens                    |
/// | 15      | phase flag                | field/phase flag                | bgf0                            |
/// | 16 - 19 | minutes units             | minutes units                   | minutes units                   |
/// | 20 - 22 | minutes tens              | minutes tens                    | minutes tens                    |
/// | 23      | bgf0                      | bgf0                            | bgf2                            |
/// | 24 - 27 | hours units               | hours units                     | hours units                     |
/// | 28 - 29 | hours tens                | hours tens                      | hours tens                      |
/// | 30      | bgf1                      | bgf1                            | bgf1                            |
/// | 31      | bgf2                      | bgf2                            | field/phase flag                |
///
/// User-defined data and control codes are packed as follows:
///
/// | bits    | field          |
/// | ------- | -------------- |
/// | 0 - 3   | binary group 1 |
/// | 4 - 7   | binary group 2 |
/// | 8 - 11  | binary group 3 |
/// | 12 - 15 | binary group 4 |
/// | 16 - 19 | binary group 5 |
/// | 20 - 23 | binary group 6 |
/// | 24 - 27 | binary group 7 |
/// | 28 - 31 | binary group 8 |
///
pub struct TimeCode {
    inner: sys::Imf_TimeCode_t,
}

impl Default for TimeCode {
    fn default() -> Self {
        let mut inner = MaybeUninit::<sys::Imf_TimeCode_t>::uninit();

        unsafe {
            // Function does not raise errors, so skipping error checking.
            sys::Imf_TimeCode_default(inner.as_mut_ptr());

            Self {
                inner: inner.assume_init(),
            }
        }
    }
}

impl Debug for TimeCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TimeCode")
            .field("hours", &self.hours())
            .field("minutes", &self.minutes())
            .field("seconds", &self.seconds())
            .field("frame", &self.frame())
            .field("drop_frame", &self.drop_frame())
            .field("color_frame", &self.color_frame())
            .field("field_phase", &self.field_phase())
            .field("bgf0", &self.bgf0())
            .field("bgf1", &self.bgf1())
            .field("bgf2", &self.bgf2())
            .field("binary_group1", &self.binary_group(1).unwrap())
            .field("binary_group2", &self.binary_group(2).unwrap())
            .field("binary_group3", &self.binary_group(3).unwrap())
            .field("binary_group4", &self.binary_group(4).unwrap())
            .field("binary_group5", &self.binary_group(5).unwrap())
            .field("binary_group6", &self.binary_group(6).unwrap())
            .field("binary_group7", &self.binary_group(7).unwrap())
            .field("binary_group8", &self.binary_group(8).unwrap())
            .finish()
    }
}

impl Clone for TimeCode {
    fn clone(&self) -> Self {
        let mut other = MaybeUninit::<sys::Imf_TimeCode_t>::uninit();

        unsafe {
            // Function does not raise errors, so skipping error checking.
            sys::Imf_TimeCode_copy(other.as_mut_ptr(), &self.inner);

            Self {
                inner: other.assume_init(),
            }
        }
    }
}

impl Drop for TimeCode {
    fn drop(&mut self) {
        unsafe {
            // Function does not raise errors, so skipping error checking.
            sys::Imf_TimeCode_dtor(&mut self.inner);
        }
    }
}

impl PartialEq for TimeCode {
    fn eq(&self, other: &Self) -> bool {
        let mut result = false;

        unsafe {
            // Function does not raise errors, so skipping error checking.
            sys::Imf_TimeCode__eq(&self.inner, &mut result, &other.inner);
        }

        result
    }
}

impl Eq for TimeCode {}

impl TimeCode {
    /// Create a new TimeCode object.
    ///
    /// Errors:
    ///
    /// The following cases will return an error:
    ///
    /// - Setting hours outside of the range of 0 - 23
    /// - Setting minutes outside of the range of 0 - 59
    /// - Setting seconds outside of the range of 0 - 59
    /// - Setting frame outside of the range of 0 - 29
    ///
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        hours: i32,
        minutes: i32,
        seconds: i32,
        frame: i32,
        drop_frame: bool,
        color_frame: bool,
        field_phase: bool,
        bgf0: bool,
        bgf1: bool,
        bgf2: bool,
        binary_group1: i32,
        binary_group2: i32,
        binary_group3: i32,
        binary_group4: i32,
        binary_group5: i32,
        binary_group6: i32,
        binary_group7: i32,
        binary_group8: i32,
    ) -> Result<Self> {
        let mut inner = MaybeUninit::<sys::Imf_TimeCode_t>::uninit();

        unsafe {
            sys::Imf_TimeCode_ctor(
                inner.as_mut_ptr(),
                hours as c_int,
                minutes as c_int,
                seconds as c_int,
                frame as c_int,
                drop_frame,
                color_frame,
                field_phase,
                bgf0,
                bgf1,
                bgf2,
                binary_group1 as c_int,
                binary_group2 as c_int,
                binary_group3 as c_int,
                binary_group4 as c_int,
                binary_group5 as c_int,
                binary_group6 as c_int,
                binary_group7 as c_int,
                binary_group8 as c_int,
            )
            .into_result()?;

            Ok(Self {
                inner: inner.assume_init(),
            })
        }
    }

    /// Create a new TimeCode object from the packed representation of the
    /// object.
    ///
    pub fn from_time_and_flags(
        time_and_flags: u32,
        user_data: u32,
        packing: TimeCodePacking,
    ) -> Self {
        let mut inner = MaybeUninit::<sys::Imf_TimeCode_t>::uninit();

        unsafe {
            // Function does not raise errors, so skipping error checking.
            sys::Imf_TimeCode_from_time_and_flags(
                inner.as_mut_ptr(),
                time_and_flags as c_uint,
                user_data as c_uint,
                packing.to_sys_packing(),
            );

            Self {
                inner: inner.assume_init(),
            }
        }
    }

    /// Return the hours from 0 - 23.
    ///
    pub fn hours(&self) -> i32 {
        let mut result: c_int = 0;

        unsafe {
            // Function does not raise errors, so skipping error checking.
            sys::Imf_TimeCode_hours(&self.inner, &mut result);
        }

        result as i32
    }

    /// Set the hours of the time code.
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] if `hours` is less than 0 or greater than 23
    ///
    pub fn set_hours(&mut self, hours: i32) -> Result<()> {
        unsafe {
            sys::Imf_TimeCode_setHours(&mut self.inner, hours as c_int)
                .into_result()?;
        }

        Ok(())
    }

    /// Return the minutes from 0 - 59.
    ///
    pub fn minutes(&self) -> i32 {
        let mut result: c_int = 0;

        unsafe {
            // Function does not raise errors, so skipping error checking.
            sys::Imf_TimeCode_minutes(&self.inner, &mut result);
        }

        result as i32
    }

    /// Set the minutes of the time code.
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] if `minutes` is less than 0 or greater than 59
    ///
    pub fn set_minutes(&mut self, minutes: i32) -> Result<()> {
        unsafe {
            sys::Imf_TimeCode_setMinutes(&mut self.inner, minutes as c_int)
                .into_result()?;
        }

        Ok(())
    }

    /// Return the seconds from 0 - 59.
    ///
    pub fn seconds(&self) -> i32 {
        let mut result: c_int = 0;

        unsafe {
            // Function does not raise errors, so skipping error checking.
            sys::Imf_TimeCode_seconds(&self.inner, &mut result);
        }

        result as i32
    }

    /// Set the seconds of the time code.
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] if `seconds` is less than 0 or greater than 59
    ///
    pub fn set_seconds(&mut self, seconds: i32) -> Result<()> {
        unsafe {
            sys::Imf_TimeCode_setSeconds(&mut self.inner, seconds as c_int)
                .into_result()?;
        }

        Ok(())
    }

    /// Return the frame from 0 - 29.
    ///
    pub fn frame(&self) -> i32 {
        let mut result: c_int = 0;

        unsafe {
            // Function does not raise errors, so skipping error checking.
            sys::Imf_TimeCode_frame(&self.inner, &mut result);
        }

        result as i32
    }

    /// Set the frame of the time code.
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] if `frame` is less than 0 or greater than 29
    ///
    pub fn set_frame(&mut self, frame: i32) -> Result<()> {
        unsafe {
            sys::Imf_TimeCode_setFrame(&mut self.inner, frame as c_int)
                .into_result()?;
        }

        Ok(())
    }

    /// Return if there is a drop frame.
    ///
    pub fn drop_frame(&self) -> bool {
        let mut result: bool = false;

        unsafe {
            // Function does not raise errors, so skipping error checking.
            sys::Imf_TimeCode_dropFrame(&self.inner, &mut result);
        }

        result
    }

    /// Set if there is a drop frame.
    ///
    pub fn set_drop_frame(&mut self, drop_frame: bool) {
        unsafe {
            sys::Imf_TimeCode_setDropFrame(&mut self.inner, drop_frame);
        }
    }

    /// Return if there is a color frame.
    ///
    pub fn color_frame(&self) -> bool {
        let mut result: bool = false;

        unsafe {
            // Function does not raise errors, so skipping error checking.
            sys::Imf_TimeCode_colorFrame(&self.inner, &mut result);
        }

        result
    }

    /// Set if there is a color frame.
    ///
    pub fn set_color_frame(&mut self, color_frame: bool) {
        unsafe {
            sys::Imf_TimeCode_setColorFrame(&mut self.inner, color_frame);
        }
    }

    /// Return if there is a field phase.
    ///
    pub fn field_phase(&self) -> bool {
        let mut result: bool = false;

        unsafe {
            // Function does not raise errors, so skipping error checking.
            sys::Imf_TimeCode_fieldPhase(&self.inner, &mut result);
        }

        result
    }

    /// Set if there is a field phase.
    ///
    pub fn set_field_phase(&mut self, field_phase: bool) {
        unsafe {
            sys::Imf_TimeCode_setFieldPhase(&mut self.inner, field_phase);
        }
    }

    /// Return the bgf0.
    ///
    pub fn bgf0(&self) -> bool {
        let mut result: bool = false;

        unsafe {
            // Function does not raise errors, so skipping error checking.
            sys::Imf_TimeCode_bgf0(&self.inner, &mut result);
        }

        result
    }

    /// Set the bgf0.
    ///
    pub fn set_bgf0(&mut self, bgf0: bool) {
        unsafe {
            sys::Imf_TimeCode_setBgf0(&mut self.inner, bgf0);
        }
    }

    /// Return the bgf1.
    ///
    pub fn bgf1(&self) -> bool {
        let mut result: bool = false;

        unsafe {
            // Function does not raise errors, so skipping error checking.
            sys::Imf_TimeCode_bgf1(&self.inner, &mut result);
        }

        result
    }

    /// Set the bgf1.
    ///
    pub fn set_bgf1(&mut self, bgf1: bool) {
        unsafe {
            sys::Imf_TimeCode_setBgf1(&mut self.inner, bgf1);
        }
    }

    /// Return the bgf2.
    ///
    pub fn bgf2(&self) -> bool {
        let mut result: bool = false;

        unsafe {
            // Function does not raise errors, so skipping error checking.
            sys::Imf_TimeCode_bgf2(&self.inner, &mut result);
        }

        result
    }

    /// Set the bgf2.
    ///
    pub fn set_bgf2(&mut self, bgf2: bool) {
        unsafe {
            sys::Imf_TimeCode_setBgf2(&mut self.inner, bgf2);
        }
    }

    /// Return the value for the binary group.
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] if `group` is not between 1 and 8.
    ///
    pub fn binary_group(&self, group: i32) -> Result<i32> {
        let mut result: c_int = 0;

        unsafe {
            // Function does not raise errors, so skipping error checking.
            sys::Imf_TimeCode_binaryGroup(
                &self.inner,
                &mut result,
                group as c_int,
            )
            .into_result()?;
        }

        Ok(result as i32)
    }

    /// Set the binary group value.
    ///
    /// # Errors
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] if `group` is not between 1 and 8 or `value`
    /// is not between 1 and 15.
    ///
    pub fn set_binary_group(&mut self, group: i32, value: i32) -> Result<()> {
        if !(0..=15).contains(&value) {
            return Err(Error::InvalidArgument(
                "Binary group value must be between 0 and 15.".into(),
            ));
        }

        unsafe {
            sys::Imf_TimeCode_setBinaryGroup(
                &mut self.inner,
                group as c_int,
                value as c_int,
            )
            .into_result()?;
        }

        Ok(())
    }

    /// Return the time and flags for the given packing.
    ///
    pub fn time_and_flags(&self, packing: TimeCodePacking) -> u32 {
        let mut result: c_uint = 0;

        unsafe {
            // Function does not raise errors, so skipping error checking.
            sys::Imf_TimeCode_timeAndFlags(
                &self.inner,
                &mut result,
                packing.to_sys_packing(),
            );
        }

        result as u32
    }

    /// Set the time and flags for the given packing.
    ///
    pub fn set_time_and_flags(&mut self, value: u32, packing: TimeCodePacking) {
        unsafe {
            sys::Imf_TimeCode_setTimeAndFlags(
                &mut self.inner,
                value as c_uint,
                packing.to_sys_packing(),
            );
        }
    }

    /// Return arbitrary user data.
    ///
    pub fn user_data(&self) -> u32 {
        let mut result: c_uint = 0;

        unsafe {
            // Function does not raise errors, so skipping error checking.
            sys::Imf_TimeCode_userData(&self.inner, &mut result);
        }

        result as u32
    }

    /// Set arbitrary user data.
    ///
    pub fn set_user_data(&mut self, value: u32) {
        unsafe {
            sys::Imf_TimeCode_setUserData(&mut self.inner, value as c_uint);
        }
    }
}

#[cfg(test)]
mod tests {
    use openexr_sys as sys;
    #[test]
    fn test_packing_from_sys_success() {
        let result = super::TimeCodePacking::from_sys_packing(
            sys::Imf_TimeCode_Packing_TV60_PACKING,
        );
        assert_eq!(result, super::TimeCodePacking::Tv60);

        let result = super::TimeCodePacking::from_sys_packing(
            sys::Imf_TimeCode_Packing_TV50_PACKING,
        );
        assert_eq!(result, super::TimeCodePacking::Tv50);

        let result = super::TimeCodePacking::from_sys_packing(
            sys::Imf_TimeCode_Packing_FILM24_PACKING,
        );
        assert_eq!(result, super::TimeCodePacking::Film24);
    }

    #[test]
    #[should_panic]
    fn test_packing_from_sys_failure_invalid_value() {
        super::TimeCodePacking::from_sys_packing(sys::Imf_TimeCode_Packing(
            u32::MAX,
        ));
    }

    #[test]
    fn test_packing_to_sys_success() {
        let result = super::TimeCodePacking::Tv60.to_sys_packing();
        assert_eq!(result, sys::Imf_TimeCode_Packing_TV60_PACKING);

        let result = super::TimeCodePacking::Tv50.to_sys_packing();
        assert_eq!(result, sys::Imf_TimeCode_Packing_TV50_PACKING);

        let result = super::TimeCodePacking::Film24.to_sys_packing();
        assert_eq!(result, sys::Imf_TimeCode_Packing_FILM24_PACKING);
    }

    #[test]
    fn test_timecode_default_success() {
        let result = super::TimeCode::default();
        let mut is_eq = false;

        unsafe {
            sys::Imf_TimeCode__eq(
                &result.inner,
                &mut is_eq,
                &sys::Imf_TimeCode_t::default(),
            );
        }

        assert!(is_eq);
    }

    #[test]
    fn test_timecode_new_success() {
        let hours = 1;
        let minutes = 2;
        let seconds = 3;
        let frame = 4;
        let drop_frame = true;
        let color_frame = true;
        let field_phase = true;
        let bgf0 = true;
        let bgf1 = true;
        let bgf2 = true;
        let binary_group1 = 1;
        let binary_group2 = 2;
        let binary_group3 = 3;
        let binary_group4 = 4;
        let binary_group5 = 5;
        let binary_group6 = 6;
        let binary_group7 = 7;
        let binary_group8 = 8;

        let result = super::TimeCode::new(
            hours,
            minutes,
            seconds,
            frame,
            drop_frame,
            color_frame,
            field_phase,
            bgf0,
            bgf1,
            bgf2,
            binary_group1,
            binary_group2,
            binary_group3,
            binary_group4,
            binary_group5,
            binary_group6,
            binary_group7,
            binary_group8,
        )
        .unwrap();

        assert_eq!(result.hours(), hours);
        assert_eq!(result.minutes(), minutes);
        assert_eq!(result.seconds(), seconds);
        assert_eq!(result.frame(), frame);
        assert_eq!(result.drop_frame(), drop_frame);
        assert_eq!(result.color_frame(), color_frame);
        assert_eq!(result.field_phase(), field_phase);
        assert_eq!(result.bgf0(), bgf0);
        assert_eq!(result.bgf1(), bgf1);
        assert_eq!(result.bgf2(), bgf2);
        assert_eq!(result.binary_group(1).unwrap(), binary_group1);
        assert_eq!(result.binary_group(2).unwrap(), binary_group2);
        assert_eq!(result.binary_group(3).unwrap(), binary_group3);
        assert_eq!(result.binary_group(4).unwrap(), binary_group4);
        assert_eq!(result.binary_group(5).unwrap(), binary_group5);
        assert_eq!(result.binary_group(6).unwrap(), binary_group6);
        assert_eq!(result.binary_group(7).unwrap(), binary_group7);
        assert_eq!(result.binary_group(8).unwrap(), binary_group8);
    }

    #[test]
    fn test_timecode_from_time_and_flags_success() {
        let hours = 1;
        let minutes = 2;
        let seconds = 3;
        let frame = 4;
        let drop_frame = false;
        let color_frame = false;
        let field_phase = false;
        let bgf0 = true;
        let bgf1 = true;
        let bgf2 = true;
        let binary_group1 = 1;
        let binary_group2 = 2;
        let binary_group3 = 3;
        let binary_group4 = 4;
        let binary_group5 = 5;
        let binary_group6 = 6;
        let binary_group7 = 7;
        let binary_group8 = 8;

        let example = super::TimeCode::new(
            hours,
            minutes,
            seconds,
            frame,
            drop_frame,
            color_frame,
            field_phase,
            bgf0,
            bgf1,
            bgf2,
            binary_group1,
            binary_group2,
            binary_group3,
            binary_group4,
            binary_group5,
            binary_group6,
            binary_group7,
            binary_group8,
        )
        .unwrap();

        let packing = super::TimeCodePacking::Film24;
        let result = super::TimeCode::from_time_and_flags(
            example.time_and_flags(packing),
            example.user_data(),
            packing,
        );

        assert_eq!(example, result);
    }

    #[test]
    fn test_timecode_clone_success() {
        let left = super::TimeCode::default();
        let right = left.clone();

        assert_eq!(left, right);
    }

    #[test]
    fn test_timecode_eq_success() {
        let left = super::TimeCode::default();
        let right = super::TimeCode::default();

        assert_eq!(left, right);
    }

    #[test]
    fn test_timecode_ne_success() {
        let left = super::TimeCode::default();
        let mut right = super::TimeCode::default();
        right.set_hours(1).unwrap();

        assert_ne!(left, right);
    }

    #[test]
    fn test_timecode_set_hours_to_min_success() {
        let mut timecode = super::TimeCode::default();
        timecode.set_hours(0).unwrap();

        assert_eq!(timecode.hours(), 0);
    }

    #[test]
    fn test_timecode_set_hours_to_max_success() {
        let mut timecode = super::TimeCode::default();
        timecode.set_hours(23).unwrap();

        assert_eq!(timecode.hours(), 23);
    }

    #[test]
    fn test_timecode_set_hours_to_less_than_min_failure() {
        let mut timecode = super::TimeCode::default();
        assert_eq!(timecode.set_hours(-1).is_err(), true);
    }

    #[test]
    fn test_timecode_set_hours_to_greater_than_max_failure() {
        let mut timecode = super::TimeCode::default();
        assert_eq!(timecode.set_hours(24).is_err(), true);
    }

    #[test]
    fn test_timecode_set_minutes_to_min_success() {
        let mut timecode = super::TimeCode::default();
        timecode.set_minutes(0).unwrap();

        assert_eq!(timecode.minutes(), 0);
    }

    #[test]
    fn test_timecode_set_minutes_to_max_success() {
        let mut timecode = super::TimeCode::default();
        timecode.set_minutes(59).unwrap();

        assert_eq!(timecode.minutes(), 59);
    }

    #[test]
    fn test_timecode_set_minutes_to_less_than_min_failure() {
        let mut timecode = super::TimeCode::default();
        assert_eq!(timecode.set_minutes(-1).is_err(), true);
    }

    #[test]
    fn test_timecode_set_minutes_to_greater_than_max_failure() {
        let mut timecode = super::TimeCode::default();
        assert_eq!(timecode.set_minutes(60).is_err(), true);
    }

    #[test]
    fn test_timecode_set_seconds_to_min_success() {
        let mut timecode = super::TimeCode::default();
        timecode.set_seconds(0).unwrap();

        assert_eq!(timecode.seconds(), 0);
    }

    #[test]
    fn test_timecode_set_seconds_to_max_success() {
        let mut timecode = super::TimeCode::default();
        timecode.set_seconds(59).unwrap();

        assert_eq!(timecode.seconds(), 59);
    }

    #[test]
    fn test_timecode_set_seconds_to_less_than_min_failure() {
        let mut timecode = super::TimeCode::default();
        assert_eq!(timecode.set_seconds(-1).is_err(), true);
    }

    #[test]
    fn test_timecode_set_seconds_to_greater_than_max_failure() {
        let mut timecode = super::TimeCode::default();
        assert_eq!(timecode.set_seconds(60).is_err(), true);
    }

    #[test]
    fn test_timecode_set_frame_to_min_success() {
        let mut timecode = super::TimeCode::default();
        timecode.set_frame(0).unwrap();

        assert_eq!(timecode.frame(), 0);
    }

    #[test]
    fn test_timecode_set_frame_to_max_success() {
        let mut timecode = super::TimeCode::default();
        // NOTE: There is a bug with OpenEXR where the max time code should be 29, but v3.0.1 doesn't include that fix.
        timecode.set_frame(29).unwrap();

        assert_eq!(timecode.frame(), 29);
    }

    #[test]
    fn test_timecode_set_frame_to_less_than_min_failure() {
        let mut timecode = super::TimeCode::default();
        assert_eq!(timecode.set_frame(-1).is_err(), true);
    }

    #[test]
    fn test_timecode_set_frame_to_greater_than_max_failure() {
        let mut timecode = super::TimeCode::default();
        assert_eq!(timecode.set_frame(60).is_err(), true);
    }

    #[test]
    fn test_timecode_set_drop_frame_success() {
        let mut timecode = super::TimeCode::default();
        timecode.set_drop_frame(true);
        assert_eq!(timecode.drop_frame(), true);
        timecode.set_drop_frame(false);
        assert_eq!(timecode.drop_frame(), false);
    }

    #[test]
    fn test_timecode_set_color_frame_success() {
        let mut timecode = super::TimeCode::default();
        timecode.set_color_frame(true);
        assert_eq!(timecode.color_frame(), true);
        timecode.set_color_frame(false);
        assert_eq!(timecode.color_frame(), false);
    }

    #[test]
    fn test_timecode_set_field_phase_success() {
        let mut timecode = super::TimeCode::default();
        timecode.set_field_phase(true);
        assert_eq!(timecode.field_phase(), true);
        timecode.set_field_phase(false);
        assert_eq!(timecode.field_phase(), false);
    }

    #[test]
    fn test_timecode_set_bgf0_success() {
        let mut timecode = super::TimeCode::default();
        timecode.set_bgf0(true);
        assert_eq!(timecode.bgf0(), true);
        timecode.set_bgf0(false);
        assert_eq!(timecode.bgf0(), false);
    }

    #[test]
    fn test_timecode_set_bgf1_success() {
        let mut timecode = super::TimeCode::default();
        timecode.set_bgf1(true);
        assert_eq!(timecode.bgf1(), true);
        timecode.set_bgf1(false);
        assert_eq!(timecode.bgf1(), false);
    }

    #[test]
    fn test_timecode_set_bgf2_success() {
        let mut timecode = super::TimeCode::default();
        timecode.set_bgf2(true);
        assert_eq!(timecode.bgf2(), true);
        timecode.set_bgf2(false);
        assert_eq!(timecode.bgf2(), false);
    }

    #[test]
    fn test_timecode_set_binary_group_min_group_success() {
        let mut timecode = super::TimeCode::default();
        timecode.set_binary_group(1, 1).unwrap();
        assert_eq!(timecode.binary_group(1).unwrap(), 1);
    }

    #[test]
    fn test_timecode_set_binary_group_max_group_success() {
        let mut timecode = super::TimeCode::default();
        timecode.set_binary_group(8, 1).unwrap();
        assert_eq!(timecode.binary_group(8).unwrap(), 1);
    }

    #[test]
    fn test_timecode_set_binary_group_less_than_min_group_failure() {
        let mut timecode = super::TimeCode::default();
        assert_eq!(timecode.set_binary_group(0, 1).is_err(), true);
    }

    #[test]
    fn test_timecode_set_binary_group_greater_than_max_group_failure() {
        let mut timecode = super::TimeCode::default();
        assert_eq!(timecode.set_binary_group(9, 1).is_err(), true);
    }

    #[test]
    fn test_timecode_set_binary_group_min_value_success() {
        let mut timecode = super::TimeCode::default();
        timecode.set_binary_group(1, 0).unwrap();
        assert_eq!(timecode.binary_group(1).unwrap(), 0);
    }

    #[test]
    fn test_timecode_set_binary_group_max_value_success() {
        let mut timecode = super::TimeCode::default();
        timecode.set_binary_group(1, 15).unwrap();
        assert_eq!(timecode.binary_group(1).unwrap(), 15);
    }

    #[test]
    fn test_timecode_set_binary_group_less_than_min_value_failure() {
        let mut timecode = super::TimeCode::default();
        assert_eq!(timecode.set_binary_group(1, -1).is_err(), true);
    }

    #[test]
    fn test_timecode_set_binary_group_greater_than_max_value_failure() {
        let mut timecode = super::TimeCode::default();
        assert_eq!(timecode.set_binary_group(1, 16).is_err(), true);
    }

    #[test]
    fn test_timecode_set_time_and_flags_success() {
        let mut timecode = super::TimeCode::default();
        let hours = 1;
        let minutes = 2;
        let seconds = 3;
        let frame = 4;
        let drop_frame = false;
        let color_frame = false;
        let field_phase = false;
        let bgf0 = true;
        let bgf1 = true;
        let bgf2 = true;
        let binary_group1 = 1;
        let binary_group2 = 2;
        let binary_group3 = 3;
        let binary_group4 = 4;
        let binary_group5 = 5;
        let binary_group6 = 6;
        let binary_group7 = 7;
        let binary_group8 = 8;

        let example = super::TimeCode::new(
            hours,
            minutes,
            seconds,
            frame,
            drop_frame,
            color_frame,
            field_phase,
            bgf0,
            bgf1,
            bgf2,
            binary_group1,
            binary_group2,
            binary_group3,
            binary_group4,
            binary_group5,
            binary_group6,
            binary_group7,
            binary_group8,
        )
        .unwrap();
        let packing = super::TimeCodePacking::Film24;
        timecode.set_time_and_flags(example.time_and_flags(packing), packing);

        assert_eq!(
            timecode.time_and_flags(packing),
            example.time_and_flags(packing)
        );
    }

    #[test]
    fn test_timecode_set_user_data_success() {
        let mut timecode = super::TimeCode::default();
        timecode.set_user_data(1);

        assert_eq!(timecode.user_data(), 1);
    }
}
