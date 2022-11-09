// pub mod gui;

use bevy::prelude::*;

use crate::states::{Views, CombatPhases};

pub struct GuiPlugin;

const NORMAL_BUTTON: Color = Color::rgb(0.75, 0.75, 0.75);
const HOVERED_BUTTON: Color = Color::rgb(0.55, 0.55, 0.55);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);
pub enum CombatActions {
    Attack,
    Defend,
    Item,
}

#[derive(Default)]
struct CombatButtons {
    attack: Option<Entity>,
    defend: Option<Entity>,
    item: Option<Entity>,
}

pub struct CombatButtonEvent {
    pub action: CombatActions,
}

fn setup_combat(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut buttons: ResMut<CombatButtons>,
) {
    // root node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(17.5)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_items: AlignItems::FlexStart,
                                justify_content: JustifyContent::SpaceBetween,
                                flex_direction: FlexDirection::Row,
                                padding: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Row,
                                        ..default()
                                    },
                                    color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    buttons.attack = parent
                                        .spawn_bundle(ButtonBundle {
                                            button: Button::default(),
                                            style: Style {
                                                size: Size {
                                                    width: Val::Px(200.0),
                                                    height: Val::Percent(50.0),
                                                },
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                padding: UiRect::new(
                                                    Val::Px(0.0),
                                                    Val::Px(0.0),
                                                    Val::Px(50.0),
                                                    Val::Px(50.0),
                                                ),
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            parent.spawn_bundle(
                                                TextBundle::from_section(
                                                    "Attack",
                                                    TextStyle {
                                                        font: asset_server
                                                            .load("fonts/SourceCodePro.ttf"),
                                                        font_size: 24.0,
                                                        color: Color::BLACK,
                                                    },
                                                )
                                                .with_style(Style { ..default() }),
                                            );
                                        })
                                        .id()
                                        .into();
                                    buttons.defend = parent
                                        .spawn_bundle(ButtonBundle {
                                            button: Button::default(),
                                            style: Style {
                                                size: Size {
                                                    width: Val::Px(200.0),
                                                    height: Val::Percent(50.0),
                                                },
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                padding: UiRect::new(
                                                    Val::Px(0.0),
                                                    Val::Px(0.0),
                                                    Val::Px(50.0),
                                                    Val::Px(50.0),
                                                ),
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            parent.spawn_bundle(
                                                TextBundle::from_section(
                                                    "Defend",
                                                    TextStyle {
                                                        font: asset_server
                                                            .load("fonts/SourceCodePro.ttf"),
                                                        font_size: 24.0,
                                                        color: Color::BLACK,
                                                    },
                                                )
                                                .with_style(Style { ..default() }),
                                            );
                                        })
                                        .id()
                                        .into();
                                    buttons.item = parent
                                        .spawn_bundle(ButtonBundle {
                                            button: Button::default(),
                                            style: Style {
                                                size: Size {
                                                    width: Val::Px(200.0),
                                                    height: Val::Percent(50.0),
                                                },
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                padding: UiRect::new(
                                                    Val::Px(0.0),
                                                    Val::Px(0.0),
                                                    Val::Px(50.0),
                                                    Val::Px(50.0),
                                                ),
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            parent.spawn_bundle(
                                                TextBundle::from_section(
                                                    "Item",
                                                    TextStyle {
                                                        font: asset_server
                                                            .load("fonts/SourceCodePro.ttf"),
                                                        font_size: 24.0,
                                                        color: Color::BLACK,
                                                    },
                                                )
                                                .with_style(Style { ..default() }),
                                            );
                                        })
                                        .id()
                                        .into();
                                });
                        });
                });
        });
}

fn teardown_combat() {}

fn combat_button_events(
    mut buttons_q: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
    buttons: Res<CombatButtons>,
    mut phase: ResMut<State<CombatPhases>>,
) {
    if let Some(button) = buttons.attack {
        if let Ok((interaction, mut color)) = buttons_q.get_mut(button) {
            match interaction {
                Interaction::Clicked => {
                    *color = PRESSED_BUTTON.into();
                    if phase.overwrite_set(CombatPhases::SelectTarget).is_ok() {
                    }
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON.into();
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON.into();
                }
            }
        }
    }
    if let Some(button) = buttons.defend {
        if let Ok((interaction, mut color)) = buttons_q.get_mut(button) {
            match interaction {
                Interaction::Clicked => {
                    *color = PRESSED_BUTTON.into();
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON.into();
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON.into();
                }
            }
        }
    }
    if let Some(button) = buttons.item {
        if let Ok((interaction, mut color)) = buttons_q.get_mut(button) {
            match interaction {
                Interaction::Clicked => {
                    *color = PRESSED_BUTTON.into();
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON.into();
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON.into();
                }
            }
        }
    }
}
impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CombatButtons>()
            .add_system_set(SystemSet::on_enter(Views::Combat).with_system(setup_combat))
            .add_system_set(SystemSet::on_update(Views::Combat).with_system(combat_button_events))
            .add_system_set(SystemSet::on_exit(Views::Combat).with_system(teardown_combat));
    }
}
