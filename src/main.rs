use rand::Rng;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
struct Player1;

#[derive(Component)]
struct Player2;

#[derive(Component)]
struct Player1ScoreText;

#[derive(Component)]
struct Player2ScoreText;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball {
    velocity: Vec2
}

struct ScoreBoard {
    player1: i32,
    player2: i32,
}

// TODO: Add an in-game main menu and a state for it?
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    InGame,
    Reset
}

enum MoveDirection {
    UP,
    DOWN
}

// TODO: Player paddle and ball colors
// TODO: Maybe font colors also etc..?  :)
struct Config {
    player1_start_position: Vec3,
    player2_start_position: Vec3,
    paddle_size: Vec3,
    paddle_half_height: f32,
    window_half_height: f32,
    window_half_width: f32,
}

impl FromWorld for Config {
    fn from_world(world: &mut World) -> Self {
        let window = world.get_resource::<Windows>().unwrap().get_primary().unwrap();
        let p1_start_x = -window.width() / 2.0 + 70.0;
        let p2_start_x = window.width() / 2.0 - 70.0;

        let paddle_size = Vec3::new(50.0, window.height() / 4.0, 10.0);
        let paddle_half_height = window.height() / 8.0;
        let window_half_height = window.height() / 2.0;
        let window_half_width = window.width() / 2.0;

        Config {
            player1_start_position: Vec3::new(p1_start_x, 0.0, 0.0),
            player2_start_position: Vec3::new(p2_start_x, 0.0, 0.0),
            paddle_size,
            paddle_half_height,
            window_half_height,
            window_half_width
        }
    }
}

const PADDLE_SPEED: f32 = 10.0;
const PADDLE_WIDTH: f32 = 50.0;
const BALL_RADIUS: f32 = 15.0;
const BALL_SPAWN_SPEED: f32 = 7.0;
const BALL_ACCEL: f32 = 1.0;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Pong!".to_string(),
            width: 1280.0,
            height: 720.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(ScoreBoard { player1: 0, player2: 0})
        .init_resource::<Config>()
        .add_startup_system(setup_cameras)
        .add_startup_system(setup_ui)
        .add_startup_system(create_paddles)
        .add_startup_system(spawn_ball)
        .add_state(AppState::InGame)
        // system sets for InGame and Reset states
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(move_ball)
                .with_system(player1_input)
                .with_system(player2_input)
                .with_system(check_collisions)
                .with_system(scoreboard_system)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Reset)
                .with_system(reset_ball.label("reset_ball"))
                // Original PONG did not reset paddles 
                // Still leacing this here, because of the scoring system
                // Might want to do something like "first to ten points"
                // and then reset the whole game, paddles included
                // .with_system(reset_paddles.after("reset_ball"))
        )
        .run();
}

fn setup_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>
) {
    let window = windows.get_primary().unwrap();

    let offset = window.height() / 10.0;

    // Create the 'net'
    for i in 0..10 {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 20.0, 0.0),
                translation: Vec3::new(0.0, offset * i as f32 - window.height() / 2.0 + 20.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        });
    }

    // Player 1 score
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                // Player 1 score section
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(1.0, 1.0, 1.0),
                    },
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(10.0),
                left: Val::Px(window.width() / 2.0 - 50.0 - 20.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }).insert(Player1ScoreText);

    // Player 2 score
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(1.0, 1.0, 1.0),
                    },
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(10.0),
                left: Val::Px(window.width() / 2.0 + 50.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }).insert(Player2ScoreText);
}

fn create_paddles(config: Res<Config>, mut commands: Commands) {
    // first paddle
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            ..Default::default()
        },
        transform: Transform {
            scale: config.paddle_size,
            translation: config.player1_start_position,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Paddle)
    .insert(Player1);

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            ..Default::default()
        },
        transform: Transform {
            scale: config.paddle_size,
            translation: config.player2_start_position,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Paddle)
    .insert(Player2);
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let starting_direction = generate_ball_start_direction();

    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere::default())).into(),
        transform: Transform {
            scale: Vec3::new(BALL_RADIUS, BALL_RADIUS, 0.0),
            ..Default::default()
        },
        material: materials.add(ColorMaterial::from(Color::rgb(1.0, 1.0, 1.0))).into(),
        ..Default::default()
    })
    .insert(Ball{
        velocity: starting_direction.normalize() * BALL_SPAWN_SPEED,
    });
}

fn reset_ball(
    mut app_state: ResMut<State<AppState>>,
    mut ball_query: Query<(&mut Transform, &mut Ball), Without<Paddle>>
) {
    println!("RESET BALL");

    // Reset ball and randomize starting velocity again
    let (mut ball_transform, mut ball) = ball_query.single_mut();

    ball_transform.translation = Vec3::new(0.0, 0.0, 0.0);
    ball.velocity = generate_ball_start_direction().normalize() * BALL_SPAWN_SPEED;

    // Go to InGame state to start another round
    app_state.set(AppState::InGame).unwrap();
}

fn _reset_paddles(
    config: Res<Config>,
    mut app_state: ResMut<State<AppState>>,
    mut player1_query: Query<(&mut Transform, &Player1), Without<Player2>>,
    mut player2_query: Query<(&mut Transform, &Player2), Without<Player1>>
) {
    println!("RESET PADDLES");

    let (mut p1, _) = player1_query.single_mut();
    let (mut p2, _) = player2_query.single_mut();

    p1.translation = config.player1_start_position;
    p2.translation = config.player2_start_position;

    // Go to InGame state to start another round
    app_state.set(AppState::InGame).unwrap();
}

fn move_ball(
    config: Res<Config>,
    mut scoreboard: ResMut<ScoreBoard>,
    mut app_state: ResMut<State<AppState>>,
    mut ball_query: Query<(&mut Transform, &mut Ball)>
) {
    let (mut transform, mut ball) = ball_query.single_mut();

    // TODO: Add timestep
    transform.translation.x += ball.velocity.x;
    transform.translation.y += ball.velocity.y;

    // check ball collision with ceiling and floor and reflect
    if transform.translation.y + BALL_RADIUS >= config.window_half_height
        || transform.translation.y - BALL_RADIUS <= -config.window_half_height
         {
        ball.velocity.y = -ball.velocity.y;
    }

    // Check ball collision with either side of the screen, give points
    // and transition to Reset state
    if transform.translation.x - BALL_RADIUS < -config.window_half_width{
        println!("Player2 scores");
        scoreboard.player2 += 1;
        app_state.set(AppState::Reset).unwrap();
    } else if transform.translation.x + BALL_RADIUS > config.window_half_width {
        println!("Player1 score");
        scoreboard.player1 += 1;
        app_state.set(AppState::Reset).unwrap();
    }
}

fn player1_input(
    keyboard_input: Res<Input<KeyCode>>,
    config: Res<Config>,
    mut player1_query: Query<&mut Transform, With<Player1>>
) {
    let mut transform = player1_query.single_mut();

    if keyboard_input.pressed(KeyCode::W) {
        move_and_cap_paddle(&config, &mut transform, MoveDirection::UP);
    }

    if keyboard_input.pressed(KeyCode::S) {
        move_and_cap_paddle(&config, &mut transform, MoveDirection::DOWN);
    }
}

fn player2_input(
    keyboard_input: Res<Input<KeyCode>>,
    config: Res<Config>,
    mut player2_query: Query<&mut Transform, With<Player2>>
) {
    let mut transform = player2_query.single_mut();

    if keyboard_input.pressed(KeyCode::Up) {
        move_and_cap_paddle(&config, &mut transform, MoveDirection::UP);
    }
    
    if keyboard_input.pressed(KeyCode::Down) {
        move_and_cap_paddle(&config, &mut transform, MoveDirection::DOWN);
    }
}

fn move_and_cap_paddle(
    config: &Res<Config>, 
    transform: &mut Mut<Transform>,
    direction: MoveDirection
) {
    // move paddle
    match direction {
        MoveDirection::UP => transform.translation.y += PADDLE_SPEED,        
        MoveDirection::DOWN => transform.translation.y -= PADDLE_SPEED
    }
    
    // clamp to upper/lower bounds
    let min = -config.window_half_height + config.paddle_half_height;
    let max = config.window_half_height - config.paddle_half_height;
    transform.translation.y = transform.translation.y.clamp(min, max);
}

fn check_collisions(
    windows: Res<Windows>,
    mut ball_query: Query<(&Transform, &mut Ball)>,
    mut paddle_query: Query<&Transform, With<Paddle>>
) {
    let window = windows.get_primary().unwrap();
    let paddle_half = window.height() as f32 / 8.0;

    let (ball_transform, mut ball) = ball_query.single_mut();

    let b_trans = ball_transform.translation;

    for paddle in paddle_query.iter_mut() {
        // Check collision with Paddle objects
        if b_trans.x + BALL_RADIUS >= paddle.translation.x - PADDLE_WIDTH / 2.0 
        && b_trans.y - BALL_RADIUS <= paddle.translation.y + paddle_half
        && b_trans.y + BALL_RADIUS >= paddle.translation.y - paddle_half
        && b_trans.x - BALL_RADIUS <= paddle.translation.x + PADDLE_WIDTH / 2.0
        {

            // ball colliding left or right side
            if ball.velocity.x > 0.0 && b_trans.x < paddle.translation.x 
            || ball.velocity.x < 0.0 && b_trans.x > paddle.translation.x 
            {
                // Change direction and accelerate
                ball.velocity.x = -ball.velocity.x;
                
                if ball.velocity.x < 0.0 {
                    ball.velocity.x -= BALL_ACCEL;
                } else {
                    ball.velocity.x += BALL_ACCEL;
                }

            // TODO: Here be some bug, which in some cases makes the ball reverse direction instead of bouncing
            // Maybe fix, or let it be a Feature :)
            } else if ball.velocity.y < 0.0 && b_trans.y > paddle.translation.y // ball colliding top or bottom side
                || ball.velocity.y > 0.0 && b_trans.y < paddle.translation.y 
            {
                ball.velocity.y = -ball.velocity.y;

                if ball.velocity.y < 0.0 {
                    ball.velocity.y -= BALL_ACCEL;
                } else {
                    ball.velocity.y += BALL_ACCEL;
                }
            }
        }
    }
}

// TODO: Horrible with/without, should figure out a better way..
fn scoreboard_system(
    scoreboard: Res<ScoreBoard>,
    mut player1_text_query: Query<&mut Text, (With<Player1ScoreText>, Without<Player2ScoreText>)>,
    mut player2_text_query: Query<&mut Text, (With<Player2ScoreText>, Without<Player1ScoreText>)>,
) {
    let mut player1_text = player1_text_query.single_mut();
    let mut player2_text = player2_text_query.single_mut();

    player1_text.sections[0].value = format!("{}", scoreboard.player1);
    player2_text.sections[0].value = format!("{}", scoreboard.player2);
}

fn generate_ball_start_direction() -> Vec2 {
    let mut rng = rand::thread_rng();
    let x: f32 = rng.gen_range(-1.0..=1.0);
    let y: f32 = rng.gen_range(-1.0..=1.0);
    Vec2::new(x, y)
}