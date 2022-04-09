use bevy::{prelude::*, app::AppExit};
use super::AppState;

pub struct MainMenuPlugin;

struct MainMenu {
    ui_root: Entity,
    ui_camera: Entity,
}

// TODO: Restart, which transitions into Restart state, which in turn starts the game from scratch
#[derive(Component)]
enum MenuButton {
    Play,
    Continue,
    Quit
}

struct MenuColors {
    play_button_normal: Color,
    play_button_hover: Color,
    quit_button_normal: Color,
    quit_button_hover: Color,
}

// TODO: other UI colors also
impl FromWorld for MenuColors {
    fn from_world(_world: &mut World) -> Self {
        MenuColors { 
            play_button_normal: Color::rgb(0.17, 0.78, 0.19),
            play_button_hover: Color::rgb(0.16, 1.0, 0.18),
            quit_button_normal: Color::rgb(1.0, 0.12, 0.11),
            quit_button_hover: Color::rgb(0.84, 0.0, 0.04),
        }
    }
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<MenuColors>()
        .add_system_set(
            SystemSet::on_enter(AppState::Start)
                .with_system(setup_menu)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Start)
                .with_system(button_system)
                .with_system(close_menu)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Start)
                .with_system(cleanup)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::MainMenu)
                .with_system(setup_menu)
        )
        .add_system_set(
            SystemSet::on_update(AppState::MainMenu)
                .with_system(button_system)
                .with_system(close_menu)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::MainMenu)
                .with_system(cleanup)
        );
    }
}

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    app_state: Res<State<AppState>>,
    colors: Res<MenuColors>
) {
    // Stash the id for cleanup
    let ui_camera = commands.spawn_bundle(UiCameraBundle::default()).id();

    // root node
    let ui_root = commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: Color::NONE.into(),
        ..Default::default()
    })
    .with_children(|parent| {
        // border node
        parent.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
                border: Rect::all(Val::Px(20.0)),
                ..Default::default()
            },
            color: Color::rgb(0.92, 0.33, 0.20).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // content node
            parent.spawn_bundle(NodeBundle{
                style: Style {
                    size: Size::new(Val::Percent(100.0),Val::Percent(100.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                color: Color::rgb(0.92, 0.39, 0.20).into(),
                ..Default::default()
            })
            .with_children(|parent| {
                // Header text container node
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                }).with_children(|parent| {
                    // Header text
                    parent.spawn_bundle(TextBundle {
                        style: Style {
                            ..Default::default()
                        },
                        text: Text::with_section(
                        "PONG!",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 50.0,
                            color: Color::rgb(1.0, 1.0, 1.0)
                        },
                        Default::default()),
                        ..Default::default()
                    });
                });

                // button container node
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(70.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                }).with_children(|parent| {

                    let component = if *app_state.current() == AppState::Start { MenuButton::Play } else { MenuButton::Continue };
                    let text = if *app_state.current() == AppState::Start { "Play" } else { "Continue" };

                    // TODO: Refactor button creation into a function
                    // Play/Continue 
                    parent.spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        color: colors.play_button_normal.into(),
                        ..Default::default()
                    }).with_children(|parent| {
                        // Button text
                        parent.spawn_bundle(TextBundle{
                            text: Text::with_section(text, TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::rgb(0.0, 0.0, 0.0),
                            }, Default::default()),
                            ..Default::default()
                        });
                    }).insert(component);

                    // Quit
                    parent.spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        color: colors.quit_button_normal.into(),
                        ..Default::default()
                    }).with_children(|parent| {
                        // Button text
                        parent.spawn_bundle(TextBundle{
                            text: Text::with_section("Quit", TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::rgb(1.0, 1.0, 1.0),
                            }, Default::default()),
                            ..Default::default()
                        });
                    }).insert(MenuButton::Quit);
                });
            });
        });
    }).id();

    commands.insert_resource(MainMenu {
        ui_root,
        ui_camera,
    });
}

fn button_system(
    mut app_state: ResMut<State<AppState>>,
    colors: Res<MenuColors>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &MenuButton),
        (Changed<Interaction>, With<Button>)>,
    mut exit: EventWriter<AppExit>
) {
    for (interaction, mut color, menu_button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                match *menu_button {
                    MenuButton::Play | MenuButton::Continue => *color = colors.play_button_hover.into(),
                    MenuButton::Quit => *color = colors.quit_button_hover.into(),
                }
            },
            Interaction::Clicked => {
                match *menu_button {
                    MenuButton::Continue | MenuButton::Play => app_state.set(AppState::InGame).unwrap(),
                    MenuButton::Quit => exit.send(AppExit),
                }
            },
            Interaction::None => {
                match *menu_button {
                    MenuButton::Continue | MenuButton::Play => *color = colors.play_button_normal.into(),
                    MenuButton::Quit => *color = colors.quit_button_normal.into(),
                }
            }
        }
    }
}

fn close_menu(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>
) {
    // TODO: Only check Esc when in MainMenu state, Start needs an interaction with the Play/Continue button
    if *app_state.current() == AppState::MainMenu || 
        *app_state.current() == AppState::Start {
        if keyboard_input.just_pressed(KeyCode::Escape) {
            app_state.set(AppState::InGame).unwrap();
            keyboard_input.reset(KeyCode::Escape);
        }
    }
}

fn cleanup(mut commands: Commands, menu: Res<MainMenu>){
    // TODO: despawn_recursive for main_menu root and menu UI camera
    commands.entity(menu.ui_root).despawn_recursive();
    commands.entity(menu.ui_camera).despawn_recursive();
}