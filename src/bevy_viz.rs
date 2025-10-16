use bevy::prelude::*;
use bevy_http_client::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Points {
    pub points: Vec<[f64; 3]>,
}

pub fn start_bevy() {
    let mut app = App::new();
    app.add_plugins(
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
    .add_plugins((PanOrbitCameraPlugin, HttpClientPlugin))
    .add_systems(Startup, (lights_camera, fetch_points))
    .add_systems(Update, (handle_response, handle_error));
    app.register_request_type::<Points>();
    app.run();
}

fn fetch_points(mut ev_request: MessageWriter<TypedRequest<Points>>) {
    if let Ok(request) = HttpClient::new()
        .get("http://localhost:8000/points")
        .headers(&[("Content-Type", "application/json"), ("Accept", "*/*")])
        .try_with_type::<Points>()
    {
        ev_request.write(request);
    }
}

fn handle_response(mut events: ResMut<Messages<TypedResponse<Points>>>) {
    for response in events.drain() {
        let response: Points = response.into_inner();
        info!("first point: {:?}", response.points[0]);
    }
}

fn handle_error(mut ev_error: MessageReader<TypedResponseError<Points>>) {
    for error in ev_error.read() {
        info!("err getting points: {:?}", error.err);
    }
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
