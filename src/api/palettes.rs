use std::borrow::Cow;

use super::types::Palette;

pub const MAIN_PALETTE: Palette = Palette {
    light: Cow::Borrowed("#769518"),
    accent: Cow::Borrowed("#3c4d0c"),
    dark: Cow::Borrowed("#3c4d0c"),
    background: Cow::Borrowed("#182005"),
};
