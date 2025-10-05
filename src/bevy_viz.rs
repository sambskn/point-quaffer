use bevy::prelude::*;

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
        .add_systems(Update, hello_world_system)
        .run();
}

fn hello_world_system() {
    println!("sup it me da bevy app");
}
