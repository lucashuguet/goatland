use bevy::prelude::*;

mod controls;
mod voxel;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.8, 0.9)))
        .insert_resource(WindowDescriptor {
            title: "GoatLand".to_string(),
            width: 1920.,
            height: 1080.,
            resizable: false,
            cursor_visible: false,
            decorations: false,
            canvas: Some("#canvas".to_string()),
            fit_canvas_to_parent: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(controls::player::PlayerControllerPlugin)
        .add_plugin(voxel::terrain::TerrainGen)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.,
            // shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(2., 8., 2.),
        ..default()
    });

    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0., 10., 0.),
            ..default()
        })
        .insert(controls::player::PlayerController);
}
