use bevy::prelude::*;


#[derive(Resource)]
struct HelloTimer(Timer);


#[derive(Component)]
struct MainCamera;


#[derive(Component)]
struct ChessBoard;


#[derive(Component)]
struct GridPosition { x: u8, y: u8 }


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

    for k in [SPRITE_SIZE * 4.0, SPRITE_SIZE * -4.0] {
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


fn add_things(mut commands: Commands) {
    commands.spawn(GridPosition { x: 0, y: 0 });
    commands.spawn(GridPosition { x: 1, y: 2 });
}


fn hello_world(time: Res<Time>, mut timer: ResMut<HelloTimer>, query: Query<&GridPosition>) {
    if timer.0.tick(time.delta()).just_finished() {
        for position in &query {
            println!("Thing at: ({}, {})", position.x, position.y);
        }
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(HelloTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_board)
        .add_systems(Startup, add_things)
        .add_systems(Update, hello_world)
        .run();
}
