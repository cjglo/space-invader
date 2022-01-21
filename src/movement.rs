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



