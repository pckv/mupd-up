use std::borrow::Cow;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Packet {
    pub message: Option<String>,
    pub palette: Option<Palette>,
}

#[derive(Serialize, Deserialize)]
pub struct Palette {
    pub light: Cow<'static, str>,
    pub accent: Cow<'static, str>,
    pub dark: Cow<'static, str>,
    pub background: Cow<'static, str>,
}
