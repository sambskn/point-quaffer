use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy_http_client::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use serde::Deserialize;
use startin;

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

fn vecs_to_arrays(vecs: Vec<Vec<f64>>) -> Vec<Vec3> {
    vecs.into_iter()
        .filter_map(|v| Some(Vec3::new(v[0] as f32, v[1] as f32, v[2] as f32)))
        .collect()
}

fn handle_response(
    mut events: ResMut<Messages<TypedResponse<Points>>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for response in events.drain() {
        info!("got points from endpoint ({} total)", response.points.len());
        let response: Points = response.into_inner();
        // make tin
        let mut dt = startin::Triangulation::new();
        dt.insert(&response.points, startin::InsertionStrategy::AsIs);
        info!("{}", dt);
        let triangles = dt.all_finite_triangles();
        let mut indices = Vec::with_capacity(triangles.len() * 3);

        for triangle in triangles {
            indices.push(triangle.v[0] as u32);
            indices.push(triangle.v[1] as u32);
            indices.push(triangle.v[2] as u32);
        }

        let vertices = vecs_to_arrays(dt.all_vertices());
        let mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
        .with_inserted_indices(Indices::U32(indices));
        commands.spawn((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(StandardMaterial { ..default() })),
            Transform::default(),
        ));
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
