use bevy::{
    prelude::*,
    diagnostic::{FrameTimeDiagnosticsPlugin, DiagnosticsStore},
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin))
        .add_systems(Startup, (setup_camera, setup_ui))
        .add_systems(Update, (update_square, display_info))
        .run();
}

#[derive(Component)]
struct CursorSquare;

#[derive(Component)]
struct InfoText;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_ui(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {

    // Spawn the square, and save its entity id in the component.
    commands.spawn((
    	Mesh2d(meshes.add(Rectangle::new(50.0, 50.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
        Transform::from_xyz(0.,0.,1.),
        CursorSquare, // Mark the entity
    ));


    // Spawn the text for displaying information.
    commands.spawn((
        Text::default(),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        InfoText, // Use a marker component.
    ));
}



fn update_square(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut square_query: Query<&mut Transform, With<CursorSquare>>,
) {
    let (camera, camera_transform) = camera_q.single();
    let Ok(window) = windows.get_single() else {
        return;
    };

    if let Some(cursor_position) = window.cursor_position() {
        if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
            for mut transform in &mut square_query {
                transform.translation = world_position.extend(1.0);
            }
        }
    }
}


fn display_info(
    diagnostics: Res<DiagnosticsStore>,
    windows: Query<&Window>,
    mut query: Query<&mut Text, With<InfoText>>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera.single();
    let Ok(window) = windows.get_single() else {
        return;
    };
    let mut text = query.single_mut();

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    text.clear();
    text.push_str(&format!("World Pos: {:.2?}\n", world_position));
    text.push_str(&format!("Screen Pos: {:.2?}\n", cursor_position));

    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            // Update the value of the second section
            text.push_str(&format!("FPS: {:.2}\n", value));
        }
    }
}