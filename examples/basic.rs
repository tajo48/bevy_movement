//! A basic example demonstrating the character controller.
//!
//! This example shows:
//! - How to set up a character controller
//! - First-person movement with WASD/arrow keys
//! - Mouse look for camera control
//! - Jumping with spacebar
//! - Gamepad support
//! - A simple environment to move around in

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
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.15)))
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
            CharacterControllerBundle::new(Collider::capsule(0.4, 1.0)).with_movement(
                30.0,                          // acceleration
                0.92,                          // damping factor
                7.0,                           // jump impulse
                (30.0 as Scalar).to_radians(), // max slope angle
            ),
            // Additional physics properties for better character feel
            Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            GravityScale(2.0),
            // Add FPS controller for input management
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

    // A cube to push around
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.4, 0.4))),
        Transform::from_xyz(3.0, 2.0, 3.0),
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
    println!("First-Person Character Controller Example");
    println!("Controls:");
    println!("  WASD / Arrow Keys - Move");
    println!("  Mouse - Look around (cursor is grabbed by default)");
    println!("  Space - Jump");
    println!("  Right Click - Grab cursor");
    println!("  Escape - Release cursor");
    println!("  Gamepad Left Stick - Move");
    println!("  Gamepad South Button (A/X) - Jump");
}
