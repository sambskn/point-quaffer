use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use voronoi_mosaic::prelude::*;

const DELAUNAY_EDGE_COLOUR: Color = Color::srgb(1.0, 0.0, 0.0);

/// Colour of Delaunay vertices
const DELAUNAY_VERTEX_COLOUR: Color = Color::srgb(0.0, 0.0, 1.0);

/// Colour of Voronoi edges
const VORONOI_EDGE_COLOUR: Color = Color::srgb(1.0, 0.5, 0.0);

/// Colour of the Voronoi vertices
const VORONOI_VERTEX_COLOUR: Color = Color::srgb(0.5, 1.0, 0.0);

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
        .add_systems(Startup, (lights_camera, visuals))
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

fn visuals(
    mut cmds: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // points to be used
    let points = vec![
        Vec3::new(-50.0, -50.0, 50.0),
        Vec3::new(50.0, -50.0, 50.0),
        Vec3::new(50.0, -50.0, 50.0),
        Vec3::new(-50.0, -50.0, 50.0),
        Vec3::new(-50.0, 50.0, 50.0),
        Vec3::new(50.0, 50.0, 50.0),
        Vec3::new(50.0, 50.0, 50.0),
        Vec3::new(-50.0, 53.0, 50.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(20.0, 20.0, 20.0),
        Vec3::new(10.0, -20.0, 40.0),
    ];

    // compute data
    let mosaic = Mosaic3d::new(&points);

    if let Some(delaunay) = mosaic.get_delaunay() {
        create_delaunay_visuals(&mut cmds, &mut mesh_assets, &mut materials, delaunay);
        if let Some(voronoi) = mosaic.get_voronoi() {
            create_voronoi_cell_visuals(&mut cmds, &mut mesh_assets, &mut materials, voronoi);
            create_mesh_visuals(&mut cmds, &mut mesh_assets, &mut materials, voronoi);
        }
    } else {
        warn!("Data computation failed");
    }
}

/// Labels an entity in the Delaunay view for querying
#[derive(Component)]
struct DelaunayLabel;

/// Create simple shapes to illustrate the raw delaunay data
fn create_delaunay_visuals(
    cmds: &mut Commands,
    mesh_assets: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    delaunay: &mosaic_3d::delaunay::Delaunay3d,
) {
    let tetrahedra = delaunay.get_tetrahedra();

    let vertex_lookup = delaunay.get_vertex_lookup();

    for (_, tetra) in tetrahedra.iter() {
        // create markers for vertices
        let mesh = mesh_assets.add(Sphere::new(0.5));
        let material = materials.add(StandardMaterial {
            base_color: DELAUNAY_VERTEX_COLOUR,

            ..default()
        });

        // vertices
        let translations = [
            vertex_lookup.get(&tetra.get_vertex_a_id()).unwrap(),
            vertex_lookup.get(&tetra.get_vertex_b_id()).unwrap(),
            vertex_lookup.get(&tetra.get_vertex_c_id()).unwrap(),
            vertex_lookup.get(&tetra.get_vertex_d_id()).unwrap(),
        ];

        for translation in translations.iter() {
            cmds.spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(**translation),
                Visibility::Hidden,
                DelaunayLabel,
            ));
        }

        // create markers for edges
        let mat = materials.add(StandardMaterial {
            base_color: DELAUNAY_EDGE_COLOUR,
            ..default()
        });

        for edge in tetra.get_edges().iter() {
            let start = vertex_lookup.get(&edge.get_vertex_a_id()).unwrap();
            let end = vertex_lookup.get(&edge.get_vertex_b_id()).unwrap();
            let len = (end - start).length();
            let mesh = mesh_assets.add(Cuboid::new(0.25, 0.25, len));
            let translation = (end + start) / 2.0;
            let mut tform = Transform::from_translation(translation);
            tform.look_at(*end, Vec3::Y);
            cmds.spawn((
                Mesh3d(mesh),
                MeshMaterial3d(mat.clone()),
                tform,
                Visibility::Hidden,
                DelaunayLabel,
            ));
        }
    }
}

/// Labels an entity in the Voronoi view for querying
#[derive(Component)]
struct VoronoiLabel;

/// Create simple shapes to illustrate the raw voronoi data
fn create_voronoi_cell_visuals(
    cmds: &mut Commands,
    mesh_assets: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    voronoi: &Voronoi3d,
) {
    let cells = voronoi.get_cells();

    let vertex_lookup = voronoi.get_vertex_lookup();

    for (_, cell) in cells.iter() {
        let cell_vertex_ids = cell.get_vertex_ids();

        for vertex_id in cell_vertex_ids.iter() {
            // mark each vertex of every cell

            let mesh = mesh_assets.add(Sphere::new(0.5));

            let material = materials.add(StandardMaterial {
                base_color: VORONOI_VERTEX_COLOUR,

                ..default()
            });

            let pos = vertex_lookup.get(vertex_id).unwrap();

            cmds.spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(*pos),
                Visibility::Hidden,
                VoronoiLabel,
            ));
        }

        // mark the edges
        let edges = cell.get_edges();

        for edge in edges.iter() {
            let start = vertex_lookup.get(&edge.get_vertex_a_id()).unwrap();
            let end = vertex_lookup.get(&edge.get_vertex_b_id()).unwrap();
            let len = (end - start).length();
            let mesh = mesh_assets.add(Cuboid::new(0.25, 0.25, len));
            let mat = materials.add(StandardMaterial {
                base_color: VORONOI_EDGE_COLOUR,
                ..default()
            });

            let translation = (end + start) / 2.0;
            let mut tform = Transform::from_translation(translation);

            tform.look_at(*end, Vec3::Y);
            cmds.spawn((
                Mesh3d(mesh),
                MeshMaterial3d(mat.clone()),
                tform,
                Visibility::Hidden,
                VoronoiLabel,
            ));
        }
    }
}

/// Labels an entity in the bevy mesh view for querying
#[derive(Component)]
struct MeshLabel;

/// Create the meshes

fn create_mesh_visuals(
    cmds: &mut Commands,
    mesh_assets: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    voronoi: &Voronoi3d,
) {
    let meshes = voronoi.as_bevy3d_meshes();

    for (i, (mesh, position)) in meshes.values().enumerate() {
        // randomise mesh colour
        let colour = Color::hsl(360. * i as f32 / meshes.len() as f32, 0.95, 0.7);
        let tform = Transform::from_translation(*position);
        let mat = StandardMaterial {
            base_color: colour,
            ..default()
        };

        cmds.spawn((
            Mesh3d(mesh_assets.add(mesh.clone())),
            MeshMaterial3d(materials.add(mat)),
            tform,
            MeshLabel,
            Visibility::Visible,
        ));
    }
}
