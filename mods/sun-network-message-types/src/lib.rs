use serde::{Deserialize, Serialize};
use sun_api::SunSettings;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SunSettingsChanged {
    pub settings: SunSettings,
}
