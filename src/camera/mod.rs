use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, camera_setup);
    }
}


fn camera_setup(
    mut commands: Commands
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.25, 1.5)
            .looking_at(Vec3::new(0.0, 0.25, 0.0), Vec3::Y),
        ..default()
    });
}