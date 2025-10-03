//! Core character controller components and plugin.

use avian3d::{math::*, prelude::*};
use bevy::prelude::*;

use crate::{input::*, movement::*};

/// A plugin that adds character controller functionality to a Bevy app.
pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<MovementAction>().add_systems(
            Update,
            (
                manage_cursor,
                keyboard_input,
                gamepad_input,
                update_grounded,
                movement,
                mouse_look,
                apply_movement_damping,
            )
                .chain(),
        );
    }
}

/// A marker component indicating that an entity is using a character controller.
#[derive(Component)]
pub struct CharacterController;

/// A marker component indicating that an entity is on the ground.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

/// A bundle that contains the components needed for a basic
/// dynamic character controller.
#[derive(Bundle)]
pub struct CharacterControllerBundle {
    pub character_controller: CharacterController,
    pub body: RigidBody,
    pub collider: Collider,
    pub ground_caster: ShapeCaster,
    pub locked_axes: LockedAxes,
    pub movement: MovementBundle,
}

impl CharacterControllerBundle {
    /// Creates a new character controller bundle with the given collider.
    ///
    /// # Arguments
    ///
    /// * `collider` - The collider shape for the character
    ///
    /// # Example
    ///
    /// ```no_run
    /// use bevy::prelude::*;
    /// use bevy_movement::prelude::*;
    /// use avian3d::prelude::*;
    ///
    /// fn setup(mut commands: Commands) {
    ///     commands.spawn(CharacterControllerBundle::new(
    ///         Collider::capsule(0.4, 1.0)
    ///     ));
    /// }
    /// ```
    pub fn new(collider: Collider) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            character_controller: CharacterController,
            body: RigidBody::Dynamic,
            collider,
            ground_caster: ShapeCaster::new(
                caster_shape,
                Vector::ZERO,
                Quaternion::default(),
                Dir3::NEG_Y,
            )
            .with_max_distance(0.2),
            locked_axes: LockedAxes::new().lock_rotation_x().lock_rotation_z(),
            movement: MovementBundle::default(),
        }
    }

    /// Creates a character controller with custom movement parameters.
    ///
    /// # Arguments
    ///
    /// * `collider` - The collider shape for the character
    /// * `acceleration` - The acceleration used for movement
    /// * `damping` - The damping factor for slowing down movement
    /// * `jump_impulse` - The strength of jumps
    /// * `max_slope_angle` - Maximum angle for climbable slopes in radians
    ///
    /// # Example
    ///
    /// ```no_run
    /// use bevy::prelude::*;
    /// use bevy_movement::prelude::*;
    /// use avian3d::prelude::*;
    ///
    /// fn setup(mut commands: Commands) {
    ///     commands.spawn(
    ///         CharacterControllerBundle::new(Collider::capsule(0.4, 1.0))
    ///             .with_movement(30.0, 0.92, 7.0, std::f32::consts::PI * 0.25)
    ///     );
    /// }
    /// ```
    pub fn with_movement(
        mut self,
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(acceleration, damping, jump_impulse, max_slope_angle);
        self
    }
}

/// Updates the [`Grounded`] status for character controllers.
fn update_grounded(
    mut commands: Commands,
    mut query: Query<
        (Entity, &ShapeHits, &Rotation, Option<&MaxSlopeAngle>),
        With<CharacterController>,
    >,
) {
    for (entity, hits, rotation, max_slope_angle) in &mut query {
        // The character is grounded if the shape caster has a hit with a normal
        // that isn't too steep.
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                (rotation * -hit.normal2).angle_between(Vector::Y).abs() <= angle.0
            } else {
                true
            }
        });

        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}
