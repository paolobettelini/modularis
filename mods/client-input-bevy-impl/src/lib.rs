use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};
use bevy_mod::BevyMod;
use client_game_state_api::{GameStateApi, InGameOverlayState};
use client_input_api::{InputApi, PlayerInput};
use tokio::task::JoinHandle;

pub struct InputBevyImpl;

impl InputBevyImpl {
    pub fn init<G: GameStateApi>(bevy: &mut BevyMod, _game_state: &mut G) -> Self {
        bevy.app
            .init_resource::<PlayerInput>()
            .add_systems(
                OnEnter(InGameOverlayState::Playing),
                (reset_input, grab_cursor),
            )
            .add_systems(
                OnExit(InGameOverlayState::Playing),
                (release_cursor, reset_input),
            )
            .add_systems(
                Update,
                (keep_cursor_hidden, update_input).run_if(in_state(InGameOverlayState::Playing)),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl InputApi for InputBevyImpl {}

fn update_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse: Res<AccumulatedMouseMotion>,
    mut input: ResMut<PlayerInput>,
) {
    let mut movement = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        movement.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        movement.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        movement.x += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        movement.x -= 1.0;
    }
    input.movement = movement.normalize_or_zero();
    input.look_delta = mouse.delta;
    input.break_block_pressed = mouse_buttons.just_pressed(MouseButton::Left);
    input.use_item_pressed = mouse_buttons.just_pressed(MouseButton::Right);
}

fn reset_input(mut input: ResMut<PlayerInput>) {
    *input = PlayerInput::default();
}

fn grab_cursor(mut cursors: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    if let Ok(mut cursor) = cursors.single_mut() {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
    }
}

fn keep_cursor_hidden(mut cursors: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    let Ok(mut cursor) = cursors.single_mut() else {
        return;
    };
    if cursor.grab_mode != CursorGrabMode::Locked {
        cursor.grab_mode = CursorGrabMode::Locked;
    }
    if cursor.visible {
        cursor.visible = false;
    }
}

fn release_cursor(mut cursors: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    if let Ok(mut cursor) = cursors.single_mut() {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
    }
}
