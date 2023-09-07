use bevy::{prelude::*, window::PrimaryWindow};


#[derive(Resource)]
struct HelloTimer(Timer);


#[derive(Component)]
struct MainCamera;


#[derive(Component)]
struct ChessBoard;


#[derive(Component)]
struct LevelText { level: u32 }


#[derive(Component)]
struct GridPosition { x: u8, y: u8 }


#[derive(Component)]
struct Pawn;


#[derive(Component)]
struct MouseState {
    pressed: bool,
    just_pressed: bool,
    just_released: bool,
    grid_position: Option<(u8, u8)>,
}


fn spawn_camera(
    mut commands: Commands,
    // window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // let window = window_query.get_single().unwrap();
    commands.spawn((
        Camera2dBundle::new_with_far(1.0),
        MainCamera,
    ));
}


fn spawn_board(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    const SPRITE_SIZE: f32 = 72.0;
    const BOARD_SIZE: i32 = 6;

    let full_board_size: f32 = (SPRITE_SIZE * ((BOARD_SIZE as f32) - 1.0)) / 2.0;

    for k in [SPRITE_SIZE * 3.0, SPRITE_SIZE * -3.0] {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            SPRITE_SIZE * (i as f32) - full_board_size + k,
                            SPRITE_SIZE * (j as f32) - full_board_size,
                            0.0,
                        ),
                        texture: asset_server.load(format!(
                            "sprites/pieces/square gray {} _1x_ns.png",
                            if i%2 == j%2 { "dark" } else { "light" },
                        )),
                        ..default()
                    },
                    ChessBoard,
                ));
            }
        }
    }
}


fn spawn_text(mut commands: Commands) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Level 1", TextStyle {
                font_size: 36.0,
                ..default()
            }).with_alignment(TextAlignment::Center),
            transform: Transform::from_translation(4.0 * 72.0 * Vec3::Y),
            ..default()
        },
        LevelText { level: 1 },
    ));
}


fn xy_from_gridposition(x: u8, y: u8) -> (f32, f32) {
    (
        (x as f32 - 5.5) * 72.0,
        (y as f32 - 2.5) * 72.0,
    )
}


fn grid_position_from_xy(x: f32, y: f32) -> (u8, u8) {
    (
        ((x / 72.0) + 6.0) as u8,
        ((y / 72.0) + 3.0) as u8,
    )
}


fn start_mouse_state(mut commands: Commands) {
    commands.spawn(MouseState {
        pressed: false,
        just_pressed: false,
        just_released: false,
        grid_position: None,
    });
}


fn mouse_tracker(
    buttons: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut query_mouse: Query<&mut MouseState>,
    mut query_pawn: Query<(&mut Transform, &mut GridPosition), With<Pawn>>,
) {
    if let Ok(mut mouse_state) = query_mouse.get_single_mut() {
        if let Ok((mut transform, mut grid_position)) = query_pawn.get_single_mut() {

            if buttons.pressed(MouseButton::Left) {
                mouse_state.just_pressed = !mouse_state.pressed;
                mouse_state.pressed = true;
                
                if let Some(position) = q_windows.single().cursor_position() {
                    let (camera, camera_transform) = q_camera.single();
                    if let Some(world_position) = camera.viewport_to_world(camera_transform, position).map(|ray| ray.origin.truncate()) {
                        let (grid_x, grid_y) = grid_position_from_xy(world_position.x, world_position.y);
                        // println!("{}, {}", grid_x, grid_y);
                        grid_position.x = grid_x;
                        grid_position.y = grid_y;
                        let (x, y) = xy_from_gridposition(grid_x, grid_y);
                        transform.translation.x = x;
                        transform.translation.y = y;
                    }
                }

            } else {
                mouse_state.just_released = mouse_state.pressed;
                mouse_state.pressed = false;
            }

        }
    }
}


fn add_pawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    let (x, y) = xy_from_gridposition(2, 3);

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.1),
            texture: asset_server.load("sprites/pieces/w_pawn_1x_ns.png"),
            ..default()
        },
        GridPosition { x: 0, y: 0 },
        Pawn,
    ));
}


fn hello_world(time: Res<Time>, mut timer: ResMut<HelloTimer>, mut query: Query<(&mut LevelText, &mut Text)>) {
    if timer.0.tick(time.delta()).just_finished() {
        if let Ok((mut level_text, mut text)) = query.get_single_mut() {
            let new_level = level_text.level + 1;
            level_text.level = new_level;
            let style = text.sections[0].style.clone();
            text.sections = vec![TextSection::new(format!("Level {}", new_level), style)];
        }
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(HelloTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_board)
        .add_systems(Startup, spawn_text)
        .add_systems(Startup, start_mouse_state)
        .add_systems(Startup, add_pawn)
        .add_systems(Update, hello_world)
        .add_systems(Update, mouse_tracker)
        .run();
}
