use strum::{Display, VariantArray};

#[derive(Display, VariantArray)]
pub enum Preset {
    Fast,
    Balanced,
    Quality,
    Custom,
}

impl Preset {
    pub fn get_args(&self) -> Option<&[&'static str]> {
        match self {
            Self::Fast => Some(&["-preset", "fast", "-crf", "28", "-x265-params", "aq-mode=3"]),
            Self::Balanced => Some(&[
                "-preset",
                "medium",
                "-crf",
                "28",
                "-x265-params",
                "aq-mode=3:rd=4",
            ]),
            Self::Quality => Some(&[
                "-preset",
                "medium",
                "-crf",
                "26",
                "-x265-params",
                "aq-mode=3:rd=4",
            ]),
            Self::Custom => None,
        }
    }
}
