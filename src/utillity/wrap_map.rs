use bevy::prelude::*;

use crate::{WORLD_HEIGHT, WORLD_WIDTH};

pub fn world_wrap_position(
    mut transforms: Query<&mut Transform>
) {
    // Get the primary window
    let x_bound = WORLD_WIDTH/ 2.0;
    let y_bound = WORLD_HEIGHT / 2.0;
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