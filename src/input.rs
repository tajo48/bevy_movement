//! Input handling for character controllers.

use avian3d::math::*;
use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

/// Component for FPS controller input enablement
#[derive(Component)]
pub struct FpsController {
    pub enable_input: bool,
}

impl Default for FpsController {
    fn default() -> Self {
        Self {
            enable_input: false,
        }
    }
}

/// A [`Message`] written for a movement input action.
#[derive(Message)]
pub enum MovementAction {
    /// Move in a direction (x, z plane)
    Move(Vector2),
    /// Jump action
    Jump,
    /// Look around (mouse delta x, y)
    Look(Vector2),
}

/// Sends [`MovementAction`] events based on keyboard input.
pub fn keyboard_input(
    mut movement_writer: MessageWriter<MovementAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: MessageReader<bevy::input::mouse::MouseMotion>,
    fps_controllers: Query<&FpsController>,
) {
    // Check if any FPS controller has input enabled
    let input_enabled = fps_controllers
        .iter()
        .any(|controller| controller.enable_input);

    let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let horizontal = right as i8 - left as i8;
    let vertical = up as i8 - down as i8;
    let direction = Vector2::new(horizontal as Scalar, vertical as Scalar).clamp_length_max(1.0);

    if direction != Vector2::ZERO {
        movement_writer.write(MovementAction::Move(direction));
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        movement_writer.write(MovementAction::Jump);
    }

    // Handle mouse look only if input is enabled
    if input_enabled {
        for mouse_event in mouse_motion.read() {
            movement_writer.write(MovementAction::Look(Vector2::new(
                mouse_event.delta.x as Scalar,
                mouse_event.delta.y as Scalar,
            )));
        }
    }
}

/// Sends [`MovementAction`] events based on gamepad input.
pub fn gamepad_input(
    mut movement_writer: MessageWriter<MovementAction>,
    gamepads: Query<&Gamepad>,
    fps_controllers: Query<&FpsController>,
) {
    // Check if any FPS controller has input enabled
    let input_enabled = fps_controllers
        .iter()
        .any(|controller| controller.enable_input);

    if !input_enabled {
        return;
    }

    for gamepad in gamepads.iter() {
        if let (Some(x), Some(y)) = (
            gamepad.get(GamepadAxis::LeftStickX),
            gamepad.get(GamepadAxis::LeftStickY),
        ) {
            movement_writer.write(MovementAction::Move(
                Vector2::new(x as Scalar, y as Scalar).clamp_length_max(1.0),
            ));
        }

        if gamepad.just_pressed(GamepadButton::South) {
            movement_writer.write(MovementAction::Jump);
        }

        // Handle gamepad look input
        if let (Some(x), Some(y)) = (
            gamepad.get(GamepadAxis::RightStickX),
            gamepad.get(GamepadAxis::RightStickY),
        ) {
            let look_sensitivity = 2.0; // Adjust as needed
            movement_writer.write(MovementAction::Look(Vector2::new(
                x as Scalar * look_sensitivity,
                -y as Scalar * look_sensitivity, // Invert Y axis
            )));
        }
    }
}

/// Manages cursor grab mode and FPS controller input
/// Right click to grab cursor and enable FPS controls
/// Escape to release cursor and disable FPS controls
pub fn manage_cursor(
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut cursor_options: Single<&mut CursorOptions>,
    mut controller_query: Query<&mut FpsController>,
) {
    let mut cursor_grabbed = false;
    let mut cursor_released = false;

    if btn.just_pressed(MouseButton::Left) {
        cursor_grabbed = true;
    }
    if key.just_pressed(KeyCode::Escape) {
        cursor_released = true;
    }

    // Update cursor options
    if cursor_grabbed {
        cursor_options.grab_mode = CursorGrabMode::Locked;
        cursor_options.visible = false;
    }
    if cursor_released {
        cursor_options.grab_mode = CursorGrabMode::None;
        cursor_options.visible = true;
    }

    // Update FPS controllers
    for mut controller in &mut controller_query {
        if cursor_grabbed {
            controller.enable_input = true;
        }
        if cursor_released {
            controller.enable_input = false;
        }
    }
}
