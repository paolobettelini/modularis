use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingType {
    Bool,
    I32,
    F32,
    String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SettingValue {
    Bool(bool),
    I32(i32),
    F32(f32),
    String(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SettingDefault {
    Bool(bool),
    I32(i32),
    F32(f32),
    String(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SettingDefinition {
    pub id: &'static str,
    pub label: &'static str,
    pub kind: SettingType,
    /// Identifier of the independently provided UI editor for this setting.
    pub input: &'static str,
    pub default: SettingDefault,
    /// Optional bounds for numeric settings. They are part of the setting
    /// contract, not a detail of a particular UI provider.
    pub number_range: Option<SettingNumberRange>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SettingNumberRange {
    pub min: Option<f64>,
    pub max: Option<f64>,
}

/// Optional menu grouping contributed alongside a setting.
///
/// Sections are presentation metadata: the setting store remains flat and
/// consumers continue to address values only through generated setting keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettingSection {
    pub id: &'static str,
    pub label: &'static str,
    pub parent: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettingParseError {
    pub expected: SettingType,
    pub input: String,
}

impl SettingDefault {
    pub fn to_value(self) -> SettingValue {
        match self {
            Self::Bool(value) => SettingValue::Bool(value),
            Self::I32(value) => SettingValue::I32(value),
            Self::F32(value) => SettingValue::F32(value),
            Self::String(value) => SettingValue::String(value.to_string()),
        }
    }
}

impl SettingValue {
    pub const fn kind(&self) -> SettingType {
        match self {
            Self::Bool(_) => SettingType::Bool,
            Self::I32(_) => SettingType::I32,
            Self::F32(_) => SettingType::F32,
            Self::String(_) => SettingType::String,
        }
    }

    pub fn parse(kind: SettingType, input: &str) -> Result<Self, SettingParseError> {
        let parsed = match kind {
            SettingType::Bool => input.trim().parse().ok().map(Self::Bool),
            SettingType::I32 => input.trim().parse().ok().map(Self::I32),
            SettingType::F32 => input.trim().parse().ok().map(Self::F32),
            SettingType::String => Some(Self::String(input.to_string())),
        };

        parsed.ok_or_else(|| SettingParseError {
            expected: kind,
            input: input.to_string(),
        })
    }
}

impl fmt::Display for SettingValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool(value) => write!(formatter, "{value}"),
            Self::I32(value) => write!(formatter, "{value}"),
            Self::F32(value) => write!(formatter, "{value}"),
            Self::String(value) => formatter.write_str(value),
        }
    }
}
