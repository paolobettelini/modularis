use player_network_message_types::PlayerId;
use server_dimension_api::{
    RequestPlayerDimensionChange, ServerDimensions, ServerPlayerDimensionChanged,
};
use server_player_registry_api::ServerPlayerRegistry;

pub fn initialize_default_dimension(
    dimensions: &ServerDimensions,
    registry: &mut ServerPlayerRegistry,
    player_id: PlayerId,
) -> Option<ServerPlayerDimensionChanged> {
    let definition = dimensions.default_dimension()?;
    let previous = dimensions
        .set_player(player_id, definition.id)
        .unwrap_or(definition.id);
    registry.set_player_position(player_id, definition.spawn)?;
    Some(ServerPlayerDimensionChanged {
        player_id,
        previous,
        current: definition.clone(),
        position: definition.spawn,
    })
}

pub fn apply_dimension_change(
    dimensions: &ServerDimensions,
    registry: &mut ServerPlayerRegistry,
    request: &RequestPlayerDimensionChange,
) -> Option<ServerPlayerDimensionChanged> {
    let definition = dimensions.definition(request.target)?;
    let previous = dimensions.set_player(request.player_id, request.target)?;
    let position = request.position.unwrap_or(definition.spawn);
    registry.set_player_position(request.player_id, position)?;
    Some(ServerPlayerDimensionChanged {
        player_id: request.player_id,
        previous,
        current: definition,
        position,
    })
}
