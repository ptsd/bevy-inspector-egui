use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: bevy::prelude::Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        material: bevy::prelude::MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        ..default()
    });
    // cube
    commands.spawn((
        Name::new("My Cube"),
        PbrBundle {
            mesh: bevy::prelude::Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            material: bevy::prelude::MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
    ));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 2_000_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
