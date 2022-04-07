
use std::os::windows;

use rand::Rng;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
struct Player1;

#[derive(Component)]
struct Player2;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball {
    velocity: Vec2,
}

enum MoveDirection {
    UP,
    DOWN
}

const PADDLE_SPEED: f32 = 10.0;
const PADDLE_WIDTH: f32 = 50.0;
const BALL_RADIUS: f32 = 10.0;
const BALL_SPAWN_SPEED: f32 = 7.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Pong!".to_string(),
            width: 800.0,
            height: 600.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(setup_camera)
        .add_startup_system(create_paddles)
        .add_startup_system(spawn_ball)
        .add_system(move_ball)
        .add_system(player1_input)
        .add_system(player2_input)
        .add_system(check_collisions)
        // .add_system(player2_paddle_collision)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn create_paddles(windows: Res<Windows>, mut commands: Commands) {
    let window = windows.get_primary().unwrap();
    let paddle_height = window.height() as f32 / 4.0;

    let player1_position = -window.width() as f32 / 2.0 + 70.0;
    let player2_position = window.width() as f32 / 2.0 - 70.0;

    // first paddle
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            ..Default::default()
        },
        transform: Transform {
            scale: Vec3::new(50.0, paddle_height, 10.0),
            translation: Vec3::new(player1_position, 0.0, 0.0),
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
            scale: Vec3::new(50.0, paddle_height, 10.0),
            translation: Vec3::new(player2_position, 0.0, 0.0),
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
    let mut rng = rand::thread_rng();
    let x: f32 = rng.gen_range(-1.0..=1.0);
    let y: f32 = rng.gen_range(-1.0..=1.0);
    let starting_direction = Vec2::new(x, y);

    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere::default())).into(),
        transform: Transform {
            scale: Vec3::new(2.0 * BALL_RADIUS, 2.0 * BALL_RADIUS, 1.0),
            ..Default::default()
        },
        material: materials.add(ColorMaterial::from(Color::rgb(1.0, 1.0, 1.0))).into(),
        ..Default::default()
    })
    .insert(Ball{
        velocity: starting_direction.normalize() * BALL_SPAWN_SPEED
    });
}

fn move_ball(
    windows: Res<Windows>, 
    mut ball_query: Query<(&mut Transform, &mut Ball)>
) {
    let window = windows.get_primary().unwrap();
    let win_half = window.height() as f32 / 2.0;
    let (mut transform, mut ball) = ball_query.single_mut();

    // TODO: Add timestep
    transform.translation.x += ball.velocity.x;
    transform.translation.y += ball.velocity.y;

    // check ball collision with ceiling and floor and reflect
    if transform.translation.y + BALL_RADIUS >= win_half  
        || transform.translation.y - BALL_RADIUS <= -win_half
         {
        ball.velocity.y = -ball.velocity.y;
    }
}

fn player1_input(
    keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut player1_query: Query<&mut Transform, With<Player1>>
) {
    let mut transform = player1_query.single_mut();

    if keyboard_input.pressed(KeyCode::W) {
        move_and_cap_paddle(&windows, &mut transform, MoveDirection::UP);
    }

    if keyboard_input.pressed(KeyCode::S) {
        move_and_cap_paddle(&windows, &mut transform, MoveDirection::DOWN);
    }
}

fn player2_input(
    keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut player2_query: Query<&mut Transform, With<Player2>>
) {
    let mut transform = player2_query.single_mut();

    if keyboard_input.pressed(KeyCode::Up) {
        move_and_cap_paddle(&windows, &mut transform, MoveDirection::UP);
    }
    
    if keyboard_input.pressed(KeyCode::Down) {
        move_and_cap_paddle(&windows, &mut transform, MoveDirection::DOWN);
    }
}

fn move_and_cap_paddle(
    windows: &Res<Windows>, 
    transform: &mut Mut<Transform>,
    direction: MoveDirection
) {
    let window = windows.get_primary().unwrap();
    let win_half = window.height() as f32 / 2.0;
    let paddle_half = window.height() as f32 / 8.0;

    // move paddle
    match direction {
        MoveDirection::UP => transform.translation.y += PADDLE_SPEED,        
        MoveDirection::DOWN => transform.translation.y -= PADDLE_SPEED
    }
    
    // cap to bounds
    if transform.translation.y + paddle_half >= win_half {
        transform.translation.y = win_half - paddle_half;
    } else if transform.translation.y - paddle_half <= -win_half {
        transform.translation.y = -win_half + paddle_half;
    }
}

fn check_collisions(
    windows: Res<Windows>,
    mut ball_query: Query<(&Transform, &mut Ball)>,
    mut paddle_query: Query<&Transform, (With<Paddle>)>
) {
    let window = windows.get_primary().unwrap();
    let paddle_half = window.height() as f32 / 8.0;

    let (ball_transform, mut ball) = ball_query.single_mut();

    let mut b_trans = ball_transform.translation;

    for paddle in paddle_query.iter_mut() {
        // Check collision with Paddle objects
        if b_trans.x + BALL_RADIUS >= paddle.translation.x - PADDLE_WIDTH / 2.0 
        && b_trans.y - BALL_RADIUS <= paddle.translation.y + paddle_half
        && b_trans.y + BALL_RADIUS >= paddle.translation.y - paddle_half
        && b_trans.x - BALL_RADIUS <= paddle.translation.x + PADDLE_WIDTH / 2.0
        {
            println!("collision");
            
            // ball colliding left or right side
            if ball.velocity.x > 0.0 && b_trans.x < paddle.translation.x 
            || ball.velocity.x < 0.0 && b_trans.x > paddle.translation.x {
                ball.velocity.x = -ball.velocity.x;
            }

            // ball colliding top or bottom side
            if ball.velocity.y < 0.0 && b_trans.y > paddle.translation.y
            || ball.velocity.y > 0.0 && b_trans.y < paddle.translation.y {
                ball.velocity.y = -ball.velocity.y;
            }
        }
    }
}

fn player1_paddle_collision(
    windows: Res<Windows>, 
    mut ball_query: Query<(&Transform, &mut Ball)>,
    mut paddle_query: Query<&Transform, (With<Player1>, Without<Ball>)>
) {
    let window = windows.get_primary().unwrap();
    let paddle_half = window.height() as f32 / 8.0;
    
    let (ball_transform, mut ball) = ball_query.single_mut();

    let player1_transform = paddle_query.single_mut();

    let right_collide = ball_transform.translation.x - BALL_RADIUS <= player1_transform.translation.x + PADDLE_WIDTH / 2.0;
    let below = ball_transform.translation.y - BALL_RADIUS > player1_transform.translation.y - paddle_half;
    let over = ball_transform.translation.y + BALL_RADIUS > player1_transform.translation.y + paddle_half;

    if right_collide && (below || over){
        if ball.velocity.x < 0.0 {
            println!("player1 right collision");
            ball.velocity.x = -ball.velocity.x;
        }
    } 
}

fn player2_paddle_collision(
    mut ball_query: Query<(&Transform, &mut Ball)>,
    mut paddle_query: Query<&Transform, (With<Player2>, Without<Ball>)>
) {
    let (ball_transform, mut ball) = ball_query.single_mut();

    let player2_transform = paddle_query.single_mut();

    if ball_transform.translation.x + BALL_RADIUS >= player2_transform.translation.x - PADDLE_WIDTH / 2.0 {
        if ball.velocity.x > 0.0 {
            println!("player2 left collision");
            ball.velocity.x = -ball.velocity.x;
        }
    } 
}