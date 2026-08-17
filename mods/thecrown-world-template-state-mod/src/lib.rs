use bevy_mod::BevyMod;
use thecrown_world_template_api::{TheCrownWorldTemplateApi, TheCrownWorldTemplates};
use tokio::task::JoinHandle;

pub struct TheCrownWorldTemplateStateMod;

impl TheCrownWorldTemplateStateMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app.init_resource::<TheCrownWorldTemplates>();
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

impl TheCrownWorldTemplateApi for TheCrownWorldTemplateStateMod {}

