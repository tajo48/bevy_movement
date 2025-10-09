//! A basic implementation of a character controller for a kinematic rigid body.
//!
//! This showcases the following:
//!
//! - Basic directional movement and jumping
//! - Support for both keyboard and gamepad input
//! - A configurable maximum slope angle
//! - Collision response for kinematic bodies
//! - Loading a platformer environment from a glTF
//!
//! The character controller logic is contained within the `bevy_movement` crate.
//!
//! For a dynamic character controller, see the `dynamic_character_3d` example.
//!
//! # Warning
//!
//! Note that this is *not* intended to be a fully featured character controller,
//! and the collision logic is quite basic.
//!
//! For a better solution, consider implementing a "collide-and-slide" algorithm,
//! or use an existing third party character controller plugin like Bevy Tnua
//! (a dynamic character controller).

use avian3d::{math::*, prelude::*};
use bevy::prelude::*;
use bevy_movement::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            CharacterControllerPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Player character with first-person camera
    let player_id = commands
        .spawn((
            Mesh3d(meshes.add(Capsule3d::new(0.4, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
            Transform::from_xyz(0.0, 1.5, 0.0),
            CharacterControllerBundle::new(Collider::capsule(0.4, 1.0), Vector::NEG_Y * 9.81 * 2.0)
                .with_movement(30.0, 0.92, 7.0, (30.0 as Scalar).to_radians()),
            FpsController::default(),
        ))
        .id();

    // First-person camera attached to player
    commands.entity(player_id).with_children(|parent| {
        parent.spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 0.8, 0.0), // Eye level height
        ));
    });

    // A cube to move around
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(3.0, 2.0, 3.0),
    ));

    // Ground plane
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(20.0, 0.1, 20.0),
        Mesh3d(meshes.add(Cuboid::new(20.0, 0.1, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, -0.05, 0.0),
    ));

    // Some platforms to jump on
    for i in 0..5 {
        let x = (i as f32 - 2.0) * 4.0;
        let height = (i as f32) * 0.5 + 1.0;

        commands.spawn((
            RigidBody::Static,
            Collider::cuboid(1.5, 0.1, 1.5),
            Mesh3d(meshes.add(Cuboid::new(1.5, 0.1, 1.5))),
            MeshMaterial3d(materials.add(Color::srgb(0.7, 0.4, 0.2))),
            Transform::from_xyz(x, height, 5.0),
        ));
    }

    // A ramp to test slope climbing
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(2.0, 0.1, 4.0),
        Mesh3d(meshes.add(Cuboid::new(2.0, 0.1, 4.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.5, 0.5, 0.7))),
        Transform::from_xyz(-8.0, 1.0, 0.0).with_rotation(Quat::from_rotation_z(0.3)),
    ));

    // Some walls to create boundaries
    let wall_positions = [
        (Vec3::new(0.0, 2.0, 12.0), Vec3::new(20.0, 4.0, 0.5)), // Back wall
        (Vec3::new(0.0, 2.0, -12.0), Vec3::new(20.0, 4.0, 0.5)), // Front wall
        (Vec3::new(12.0, 2.0, 0.0), Vec3::new(0.5, 4.0, 20.0)), // Right wall
        (Vec3::new(-12.0, 2.0, 0.0), Vec3::new(0.5, 4.0, 20.0)), // Left wall
    ];

    for (position, size) in wall_positions.iter() {
        commands.spawn((
            RigidBody::Static,
            Collider::cuboid(size.x, size.y, size.z),
            Mesh3d(meshes.add(Cuboid::new(size.x, size.y, size.z))),
            MeshMaterial3d(materials.add(Color::srgb(0.4, 0.4, 0.4))),
            Transform::from_translation(*position),
        ));
    }

    // Light
    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            range: 50.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 15.0, 0.0),
    ));

    // Instructions
    println!("FPS Character Controller Example");
    println!("Controls:");
    println!("  WASD / Arrow Keys - Move");
    println!("  Mouse - Look around (after grabbing cursor)");
    println!("  Space - Jump");
    println!("  Right Click - Grab cursor and enable FPS controls");
    println!("  Escape - Release cursor and disable FPS controls");
    println!("  Gamepad Left Stick - Move");
    println!("  Gamepad Right Stick - Look around");
    println!("  Gamepad South Button (A/X) - Jump");
    println!();
    println!("This uses a kinematic character controller with first-person camera.");
    println!("Right-click to grab the cursor and start playing!");
}
