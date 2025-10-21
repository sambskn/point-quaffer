use bevy::prelude::*;
use bevy_http_client::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use colorgrad;
use colorgrad::Gradient;
use serde::Deserialize;
use startin;
use std::f32::consts::PI;

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

        // color stuff
        let gradient = colorgrad::GradientBuilder::new()
            .html_colors(&["gold", "hotpink", "darkturquoise"])
            .domain(&[-0.5, 0.5])
            .mode(colorgrad::BlendMode::Rgb)
            .build::<colorgrad::LinearGradient>()
            .unwrap();

        let bbox = dt.get_bbox();
        // ideally this offset centers the point mesh
        // for now just grab min values to stick corner of mesh at origin
        let x_offset = -bbox[0] as f32;
        let y_offset = -bbox[1] as f32;
        info!("x_offset {:?}", x_offset);
        info!("y_offset {:?}", y_offset);
        let vertices = vecs_to_arrays(dt.all_vertices());
        let mut z_offset = 0.0;
        // loop through all finite triangles and spawn as primitve bevy meshes
        // (ideally this just makes one mesh with all the triangles instead)
        for triangle in triangles {
            let a = vertices[triangle.v[0]];
            let b = vertices[triangle.v[1]];
            let c = vertices[triangle.v[2]];
            if z_offset == 0.0 {
                z_offset = c[2];
            }
            // swtiching y and z values because that's what looked right???
            let triangle = Triangle3d::new(
                vec3(
                    (c[0] + x_offset) as f32,
                    c[2] as f32 - z_offset,
                    (c[1] + y_offset) as f32,
                ),
                vec3(
                    (b[0] + x_offset) as f32,
                    b[2] as f32 - z_offset,
                    (b[1] + y_offset) as f32,
                ),
                vec3(
                    (a[0] + x_offset) as f32,
                    a[2] as f32 - z_offset,
                    (a[1] + y_offset) as f32,
                ),
            );
            let mesh = Mesh::from(triangle);
            // assign color from gradient based on z value
            let avg_z = (c[2] + b[2] + a[2]) / 3.0 - z_offset;
            let color_rgb8 = gradient.at(avg_z).to_rgba8();
            // spawn primative mesh
            commands.spawn((
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(materials.add(Color::srgb_u8(
                    color_rgb8[0],
                    color_rgb8[1],
                    color_rgb8[2],
                ))),
                Transform::default(),
            ));
        }
    }
}

fn handle_error(mut ev_error: MessageReader<TypedResponseError<Points>>) {
    for error in ev_error.read() {
        info!("err getting points: {:?}", error.err);
    }
}

fn lights_camera(mut commands: Commands) {
    // light
    // copied from light example, should be tuned to specific mesh more or editable
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
    ));

    // camera
    // needs better defaults!
    commands.spawn((
        PanOrbitCamera::default(),
        Transform::from_xyz(2.5, 2.5, 9.0).looking_at(Vec3::new(5.0, 0., -5.0), Vec3::Y),
    ));
}
