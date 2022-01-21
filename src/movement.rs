use bevy::prelude::*;

#[derive(Component)]
pub struct Movement {
    pub speed: f32,
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Movement, &mut Transform)>,
) {
    let (player, mut transform) = query.single_mut();
    let mut x_direction = 0.0;
    let mut y_direction = 0.0;
    if keyboard_input.pressed(KeyCode::Left) {
        x_direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        x_direction += 1.0;
    }
    
    if keyboard_input.pressed(KeyCode::Up) {
        y_direction += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        y_direction -= 1.0;
    }

    let translation = &mut transform.translation;

    translation.x += x_direction * player.speed;
    translation.y += y_direction * player.speed;
    translation.x = translation.x.min(310.0).max(-310.0);
    translation.y = translation.y.min(250.0).max(-275.0);
}

pub fn sprite_animation_system(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}
