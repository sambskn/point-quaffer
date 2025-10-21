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
        // let mut indices = Vec::with_capacity(triangles.len() * 3);

        let bbox = dt.get_bbox();
        let x_min = bbox[0] as f32;
        let y_min = bbox[1] as f32;
        let mut counter = 0;
        let vertices = vecs_to_arrays(dt.all_vertices());
        let mut z_offset = 0.0;
        for triangle in triangles {
            let a = vertices[triangle.v[0]];
            let b = vertices[triangle.v[1]];
            let c = vertices[triangle.v[2]];
            if z_offset == 0.0 {
                z_offset = c[2];
            }
            let triangle = Triangle3d::new(
                vec3(
                    (a[0] - x_min) as f32,
                    (a[1] - y_min) as f32,
                    a[2] as f32 - z_offset,
                ),
                vec3(
                    (b[0] - x_min) as f32,
                    (b[1] - y_min) as f32,
                    b[2] as f32 - z_offset,
                ),
                vec3(
                    (c[0] - x_min) as f32,
                    (c[1] - y_min) as f32,
                    c[2] as f32 - z_offset,
                ),
            );
            let mesh = Mesh::from(triangle);
            commands.spawn((
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(materials.add(Color::srgb_u8(
                    (12 + counter / 2) % 255,
                    (40 + counter) % 255,
                    (1 + counter / 4) % 255,
                ))),
                Transform::default(),
            ));
            if counter == 0 {
                info!("debug triangle");
                info!("{:?}", triangle);
            }
            counter += 1;
            if counter > 155 {
                counter = 0;
            }
        }

        // let uvs: Vec<Vec2> = dt
        //     .all_vertices()
        //     .into_iter()
        //     .map(|v| Vec2::new(0.5, 0.5))
        //     .collect();
        // let mut mesh = Mesh::new(
        //     PrimitiveTopology::TriangleList,
        //     RenderAssetUsages::default(),
        // )
        // .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
        // .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        // .with_inserted_indices(Indices::U32(indices));
        // mesh.compute_area_weighted_normals();
        // commands.spawn((
        //     Mesh3d(meshes.add(mesh)),
        //     MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        //     Transform::default(),
        // ));

        // also spawn some test stuff
        // circular base
        commands.spawn((
            Mesh3d(meshes.add(Circle::new(4.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ));
        // cube
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::from_xyz(0.0, 0.5, 0.0),
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
        Transform::from_xyz(-2.5, -2.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
