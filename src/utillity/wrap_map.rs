use bevy::prelude::*;

pub fn wrap_position_to_window_size(
    windows: Single<&Window>,
    mut transforms: Query<&mut Transform>
) {
    // Get the primary window
    let window = windows.into_inner();
    let x_bound = window.width() / 2.0;
    let y_bound = window.height() / 2.0;
    for mut transform in transforms.iter_mut() {
        if transform.translation.x > x_bound {
            transform.translation.x = -x_bound;
        } else if transform.translation.x < -x_bound {
            transform.translation.x = x_bound;
        }
        if transform.translation.y > y_bound {
            transform.translation.y = -y_bound;
        } else if transform.translation.y < -y_bound {
            transform.translation.y = y_bound;
        }
    }
}