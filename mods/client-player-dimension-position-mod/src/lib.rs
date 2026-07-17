use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_dimension_api::{ClientDimensionApi, ClientDimensionChanged, ClientDimensionSet};
use client_player_controller_api::{
    Grounded, Player, PlayerControllerApi, PlayerVelocity, PreviousPlayerPosition,
};
use tokio::task::JoinHandle;

pub struct ClientPlayerDimensionPositionMod;

impl ClientPlayerDimensionPositionMod {
    pub fn init<D: ClientDimensionApi, P: PlayerControllerApi>(
        bevy: &mut BevyMod,
        _dimension: &mut D,
        _player: &mut P,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            apply_dimension_position.in_set(ClientDimensionSet::ApplyPlayer),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn apply_dimension_position(
    mut changes: MessageReader<ClientDimensionChanged>,
    mut player: Query<
        (
            &mut Transform,
            &mut PreviousPlayerPosition,
            &mut PlayerVelocity,
            &mut Grounded,
        ),
        With<Player>,
    >,
) {
    let Some(change) = changes.read().last() else {
        return;
    };
    let Ok((mut transform, mut previous, mut velocity, mut grounded)) = player.single_mut() else {
        return;
    };
    transform.translation = Vec3::from_array(change.position);
    previous.0 = transform.translation;
    velocity.0 = Vec3::ZERO;
    grounded.0 = false;
}
