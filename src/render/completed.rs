use bevy::prelude::*;

pub fn escape_forever(
    keys: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>
) {
    if keys.just_pressed(KeyCode::Escape) {
        app_exit_events.send(bevy::app::AppExit::default());
    }
}
