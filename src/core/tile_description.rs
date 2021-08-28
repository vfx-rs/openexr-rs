use openexr_sys as sys;

use crate::core::{LevelMode, LevelRoundingMode};

#[derive(Debug, Copy, Clone)]
pub struct TileDescription {
    x_size: u32,
    y_size: u32,
    mode: LevelMode,
    rounding_mode: LevelRoundingMode,
}

impl TileDescription {
    pub fn new(
        x_size: u32,
        y_size: u32,
        mode: LevelMode,
        rounding_mode: LevelRoundingMode,
    ) -> TileDescription {
        TileDescription {
            x_size,
            y_size,
            mode,
            rounding_mode,
        }
    }
}

impl From<sys::Imf_TileDescription_t> for TileDescription {
    fn from(td: sys::Imf_TileDescription_t) -> TileDescription {
        TileDescription::new(
            td.x_size,
            td.y_size,
            td.mode.into(),
            td.rounding_mode.into(),
        )
    }
}

impl From<TileDescription> for sys::Imf_TileDescription_t {
    fn from(td: TileDescription) -> Self {
        sys::Imf_TileDescription_t {
            x_size: td.x_size,
            y_size: td.y_size,
            mode: td.mode.into(),
            rounding_mode: td.rounding_mode.into(),
        }
    }
}
