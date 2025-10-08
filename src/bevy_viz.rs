use crate::bevy_web_file_drop::WebFileDropPlugin;
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub fn start_bevy() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Window {
                    title: "point-quaffer-viz".to_string(),
                    fit_canvas_to_parent: true,
                    ..default()
                }
                .into(),
                ..default()
            }),
        )
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(WebFileDropPlugin)
        .add_systems(Startup, lights_camera)
        .add_systems(Update, handle_drag_n_drop)
        .run();
}

fn lights_camera(mut commands: Commands) {
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // camera
    commands.spawn((
        PanOrbitCamera::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn handle_drag_n_drop(mut drag_and_drop_reader: MessageReader<FileDragAndDrop>) {
    for drag_and_drop in drag_and_drop_reader.read() {
        info!("{:?}", drag_and_drop);
    }
}

// fn visuals(
//     mut cmds: Commands,
//     mut mesh_assets: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // points to be used
//     let points: Vec<[f64; 3]> = vec![
//         [-50.0, -50.0, 50.0],
//         [50.0, -50.0, 50.0],
//         [50.0, -50.0, 50.0],
//         [-50.0, -50.0, 50.0],
//         [-50.0, 50.0, 50.0],
//         [50.0, 50.0, 50.0],
//         [50.0, 50.0, 50.0],
//         [-50.0, 53.0, 50.0],
//         [5.0, 0.0, 0.0],
//         [50.0, 20.0, 20.0],
//         [50.0, -20.0, 40.0],
//     ];
//     let mut dt = startin::Triangulation::new();
//     dt.insert(&points, startin::InsertionStrategy::AsIs);
// }
