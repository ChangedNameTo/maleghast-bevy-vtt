use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ascii_terminal::*;
use maleghast_vtt::Board;

fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, setup)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let board = Board::new();

    // camera
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: (10.0),
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Z),
    ));

    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(*Dir3::Z, Vec2 { x: 5.0, y: 5.0 }).mesh())),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    for tile in board.render_board_tiles(meshes, materials) {
        commands.spawn(tile);
    }

    // light
    commands.spawn((PointLight::default(), Transform::from_xyz(3.0, 8.0, 5.0)));
}
