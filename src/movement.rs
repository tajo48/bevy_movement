//! Movement components and systems for character controllers.

use avian3d::{math::*, prelude::*};
use bevy::{ecs::query::Has, prelude::*};

use crate::{controller::*, input::*};

/// Mouse sensitivity for look around.
#[derive(Component)]
pub struct MouseSensitivity(pub Scalar);

impl Default for MouseSensitivity {
    fn default() -> Self {
        Self(0.002)
    }
}

/// Pitch angle for camera (up/down rotation).
#[derive(Component)]
pub struct Pitch {
    pub angle: Scalar,
    pub max: Scalar,
}

impl Default for Pitch {
    fn default() -> Self {
        Self {
            angle: 0.0,
            max: std::f32::consts::FRAC_PI_2 - 0.1, // Almost 90 degrees
        }
    }
}

/// The acceleration used for character movement.
#[derive(Component)]
pub struct MovementAcceleration(pub Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component)]
pub struct MovementDampingFactor(pub Scalar);

/// The strength of a jump.
#[derive(Component)]
pub struct JumpImpulse(pub Scalar);

/// The maximum angle a slope can have for a character controller
/// to be able to climb and jump. If the slope is steeper than this angle,
/// the character will slide down.
#[derive(Component)]
pub struct MaxSlopeAngle(pub Scalar);

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    pub acceleration: MovementAcceleration,
    pub damping: MovementDampingFactor,
    pub jump_impulse: JumpImpulse,
    pub max_slope_angle: MaxSlopeAngle,
    pub mouse_sensitivity: MouseSensitivity,
    pub pitch: Pitch,
}

impl MovementBundle {
    /// Creates a new movement bundle with custom parameters.
    ///
    /// # Arguments
    ///
    /// * `acceleration` - The acceleration used for movement
    /// * `damping` - The damping factor for slowing down movement (0.0 = no damping, 1.0 = no movement)
    /// * `jump_impulse` - The strength of jumps
    /// * `max_slope_angle` - Maximum angle for climbable slopes in radians
    ///
    /// # Example
    ///
    /// ```no_run
    /// use bevy_movement::prelude::*;
    /// use std::f32::consts::PI;
    ///
    /// let movement = MovementBundle::new(25.0, 0.9, 8.0, PI * 0.4);
    /// ```
    pub const fn new(
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
            jump_impulse: JumpImpulse(jump_impulse),
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
            mouse_sensitivity: MouseSensitivity(0.002),
            pitch: Pitch {
                angle: 0.0,
                max: std::f32::consts::FRAC_PI_2 - 0.1,
            },
        }
    }
}

impl Default for MovementBundle {
    /// Default movement parameters suitable for most character controllers.
    fn default() -> Self {
        Self::new(30.0, 0.9, 7.0, PI * 0.45)
    }
}

/// Responds to [`MovementAction`] events and moves character controllers accordingly.
pub fn movement(
    time: Res<Time>,
    mut movement_reader: MessageReader<MovementAction>,
    mut controllers: Query<
        (
            &MovementAcceleration,
            &JumpImpulse,
            &mut LinearVelocity,
            &Rotation,
            Has<Grounded>,
            &FpsController,
        ),
        With<CharacterController>,
    >,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_secs_f64().adjust_precision();

    for event in movement_reader.read() {
        for (
            movement_acceleration,
            jump_impulse,
            mut linear_velocity,
            rotation,
            is_grounded,
            fps_controller,
        ) in &mut controllers
        {
            // Skip processing if input is disabled
            if !fps_controller.enable_input {
                continue;
            }
            match event {
                MovementAction::Move(direction) => {
                    // Convert local movement direction to world space based on character rotation
                    let forward = rotation * Vector::NEG_Z;
                    let right = rotation * Vector::X;

                    // Calculate movement in world space
                    let movement_vector = (right * direction.x + forward * direction.y)
                        * movement_acceleration.0
                        * delta_time;

                    linear_velocity.x += movement_vector.x;
                    linear_velocity.z += movement_vector.z;
                }
                MovementAction::Jump => {
                    if is_grounded {
                        linear_velocity.y = jump_impulse.0;
                    }
                }
                MovementAction::Look(_) => {
                    // Look actions are handled by the mouse_look system
                }
            }
        }
    }
}

/// Handles mouse look input for character rotation and camera pitch.
pub fn mouse_look(
    mut movement_reader: MessageReader<MovementAction>,
    mut controllers: Query<
        (
            Entity,
            &MouseSensitivity,
            &mut Rotation,
            &mut Pitch,
            &FpsController,
        ),
        With<CharacterController>,
    >,
    mut cameras: Query<&mut Transform, (With<Camera3d>, Without<CharacterController>)>,
    children: Query<&Children>,
) {
    for event in movement_reader.read() {
        if let MovementAction::Look(delta) = event {
            for (entity, sensitivity, mut rotation, mut pitch, fps_controller) in &mut controllers {
                // Skip processing if input is disabled
                if !fps_controller.enable_input {
                    continue;
                }
                // Rotate around Y axis (yaw) based on mouse X movement
                let yaw_delta = -delta.x * sensitivity.0;
                let yaw_rotation = Quaternion::from_rotation_y(yaw_delta);
                rotation.0 = yaw_rotation * rotation.0;

                // Update pitch based on mouse Y movement
                let pitch_delta = -delta.y * sensitivity.0;
                pitch.angle += pitch_delta;
                pitch.angle = pitch.angle.clamp(-pitch.max, pitch.max);

                // Apply pitch to camera (if it's a child of the controller)
                if let Ok(children) = children.get(entity) {
                    for child in children.iter() {
                        if let Ok(mut camera_transform) = cameras.get_mut(child) {
                            camera_transform.rotation = Quaternion::from_rotation_x(pitch.angle);
                        }
                    }
                }
            }
        }
    }
}

/// Slows down movement in the XZ plane.
pub fn apply_movement_damping(
    mut query: Query<(&MovementDampingFactor, &mut LinearVelocity), With<CharacterController>>,
) {
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= damping_factor.0;
        linear_velocity.z *= damping_factor.0;
    }
}
