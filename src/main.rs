// external libs
use::bevy::prelude::*;
// internal imports
mod entities;
mod movement;

// const CAMERA_SPEED: f32 = 1000.0;

// struct Player(u64);


fn main() {
     
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(movement::player_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
   
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("town.png"),
        transform: Transform::from_scale(Vec3::new(2.3, 2.3, 0.0)),
        ..Default::default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("player.png"),
        transform: Transform {
                scale: Vec3::new(0.07, 0.07, 0.0),
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..Default::default()
            }, 
        ..Default::default()
    })
    .insert(movement::Movement { speed: 9.0});
}
