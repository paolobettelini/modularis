use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_tick_api::{ServerTickApi, ServerTickMetrics, ServerTickRate};
use tokio::task::JoinHandle;

pub struct ServerTickMetricsMod;

impl ServerTickMetricsMod {
    pub fn init<T: ServerTickApi>(bevy: &mut BevyMod, _tick: &mut T) -> Self {
        let target = bevy.app.world().resource::<ServerTickRate>().target_tps;
        bevy.app
            .insert_resource(ServerTickMetrics::new(target))
            .add_systems(Update, measure_server_tick);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn measure_server_tick(time: Res<Time>, mut metrics: ResMut<ServerTickMetrics>) {
    metrics.record_tick(time.delta_secs_f64());
}
