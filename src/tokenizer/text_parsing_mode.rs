#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TextParsingMode {
    PlainText,
    RCData,
    RawText,
    ScriptData,

    // NOTE: these two can be constructed only in the test code.
    // Prevent rustc from complaining on release builds.
    #[allow(dead_code)]
    Data,

    #[allow(dead_code)]
    CDataSection,
}

#[cfg(feature = "testing_api")]
impl TextParsingMode {
    pub fn should_replace_unsafe_null_in_text(self) -> bool {
        self != TextParsingMode::Data && self != TextParsingMode::CDataSection
    }

    pub fn allows_text_entitites(self) -> bool {
        self == TextParsingMode::Data || self == TextParsingMode::RCData
    }
}

#[cfg(feature = "testing_api")]
impl<'s> From<&'s str> for TextParsingMode {
    fn from(mode: &'s str) -> Self {
        match mode {
            "Data state" => TextParsingMode::Data,
            "PLAINTEXT state" => TextParsingMode::PlainText,
            "RCDATA state" => TextParsingMode::RCData,
            "RAWTEXT state" => TextParsingMode::RawText,
            "Script data state" => TextParsingMode::ScriptData,
            "CDATA section state" => TextParsingMode::CDataSection,
            _ => panic!("Unknown text parsing mode"),
        }
    }
}

#[cfg(feature = "testing_api")]
#[derive(Copy, Clone, Debug)]
pub struct TextParsingModeSnapshot {
    pub mode: TextParsingMode,
    pub last_start_tag_name_hash: Option<u64>,
}

#[cfg(feature = "testing_api")]
pub trait TextParsingModeChangeHandler {
    fn handle(&mut self, mode_snapshot: TextParsingModeSnapshot);
}

#[cfg(feature = "testing_api")]
impl<H: FnMut(TextParsingModeSnapshot)> TextParsingModeChangeHandler for H {
    fn handle(&mut self, mode_snapshot: TextParsingModeSnapshot) {
        self(mode_snapshot);
    }
}