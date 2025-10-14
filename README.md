# Bevy Movement

A kinematic character controller crate for Bevy using Avian3D physics.

## Features

- **Kinematic Character Controller**: Smooth, responsive character movement without being affected by external forces
- **First-Person Controls**: Mouse look with configurable sensitivity and pitch constraints
- **Multiple Input Support**: Both keyboard/mouse and gamepad input
- **Slope Climbing**: Configurable maximum slope angle for realistic terrain traversal
- **Jump Mechanics**: Grounded detection and jumping with customizable impulse
- **Collision Response**: Manual collision handling for kinematic bodies with wall sliding
- **Gravity Simulation**: Customizable gravity for realistic falling behavior

## Quick Start

Add the plugin to your Bevy app and spawn a character controller:

```rust
use bevy::prelude::*;
use bevy_movement::prelude::*;
use avian3d::prelude::*;

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

fn setup(mut commands: Commands) {
    // Spawn a kinematic character controller
    commands.spawn((
        CharacterControllerBundle::new(
            Collider::capsule(0.4, 1.0),
            Vec3::NEG_Y * 9.81 * 2.0
        )
        .with_movement(30.0, 0.92, 7.0, std::f32::consts::PI * 0.25),
        FpsController::default(),
    ));
}
```

## Controls

### Keyboard & Mouse
- **WASD / Arrow Keys** - Move
- **Mouse** - Look around (after grabbing cursor)
- **Space** - Jump
- **Right Click** - Grab cursor and enable FPS controls
- **Escape** - Release cursor and disable FPS controls

### Gamepad
- **Left Stick** - Move
- **Right Stick** - Look around
- **South Button (A/X)** - Jump

## Configuration

The character controller can be customized with various parameters:

- **Movement Acceleration** - How quickly the character accelerates
- **Damping Factor** - How quickly movement slows down when no input is applied
- **Jump Impulse** - The strength of jumps
- **Max Slope Angle** - Maximum angle of slopes the character can climb
- **Gravity** - Custom gravity vector
- **Mouse Sensitivity** - Look sensitivity for mouse input

## Example

Run the basic example to see the character controller in action:

```bash
cargo run --example basic
```

This example includes:
- A first-person character controller
- A test environment with platforms, ramps, and walls
- Demonstration of all movement features

## Dependencies

- [Bevy](https://bevyengine.org/) - Game engine
- [Avian3D](https://github.com/Jondolf/avian) - Physics engine

## License

Licensed under the MIT License.