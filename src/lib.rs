//! A character controller crate for Bevy using Avian3D physics.
//!
//! This crate provides a flexible character controller system that supports:
//! - Basic directional movement and jumping
//! - Both keyboard and gamepad input
//! - Configurable maximum slope angle for jumping
//! - Dynamic rigid body physics integration
//!
//! # Quick Start
//!
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_movement::prelude::*;
//! use avian3d::prelude::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins((
//!             DefaultPlugins,
//!             PhysicsPlugins::default(),
//!             CharacterControllerPlugin,
//!         ))
//!         .add_systems(Startup, setup)
//!         .run();
//! }
//!
//! fn setup(mut commands: Commands) {
//!     // Spawn a character controller
//!     commands.spawn(CharacterControllerBundle::new(
//!         Collider::capsule(0.4, 1.0)
//!     ));
//! }
//! ```

pub mod controller;
pub mod input;
pub mod movement;

pub use controller::*;
pub use input::*;
pub use movement::*;

/// Common imports for the character controller crate.
pub mod prelude {
    pub use crate::controller::*;
    pub use crate::input::*;
    pub use crate::movement::*;
}
