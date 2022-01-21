// external libs
use::bevy::prelude::*;
// internal imports
mod entities;
mod movement;

fn main() {
     
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(movement::player_movement)
        .add_system(movement::sprite_animation_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
   
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("town.png"),
        transform: Transform::from_scale(Vec3::new(2.3, 2.3, 0.0)),
        ..Default::default()
    });
    let texture_handle = asset_server.load("playerspr.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(31.0,32.0), 3, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform {
                scale: Vec3::new(1.1, 1.1, 0.0),
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..Default::default()
            }, 
        ..Default::default()
    })
    .insert(Timer::from_seconds(0.1,true))
    .insert(movement::Movement { speed: 9.0});
}
