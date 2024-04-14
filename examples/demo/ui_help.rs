use bevy::prelude::*;

/// Plugin for highlighting a scene control
/// instruction. For debug, demo purpose.
pub struct UiHelpPlugin;

impl Plugin for UiHelpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build_diagnostics_ui);
    }
}

/// Debug, demo UI init
fn build_diagnostics_ui(mut commands: Commands) {
    let text_header_style = TextStyle {
        font_size: 16.0,
        color: Color::WHITE,
        ..default()
    };

    let text_help_style = TextStyle {
        font_size: 11.0,
        color: Color::rgb(0.85, 0.85, 0.85),
        ..default()
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.),
                        height: Val::Px(100.),
                        border: UiRect::all(Val::Px(1.0)),
                        margin: UiRect::all(Val::Px(5.0)),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        ..default()
                    },
                    border_color: Color::rgb(0.85, 0.85, 0.85).into(),
                    background_color: Color::rgba(0.2, 0.2, 0.2, 1.0).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section("Help:", text_header_style).with_style(Style {
                            margin: UiRect::all(Val::Px(5.)),
                            ..default()
                        }),
                        Label,
                    ));

                    parent
                        .spawn((
                            TextBundle::from_section(
                                "Width: W + scroll",
                                text_help_style.clone(),
                            )
                            .with_style(Style {
                                margin: UiRect::left(Val::Px(10.)),
                                ..default()
                            }),
                            Label,
                        ));

                    parent
                        .spawn((
                            TextBundle::from_section(
                                "Outline dependency from time: R",
                                text_help_style.clone(),
                            )
                            .with_style(Style {
                                margin: UiRect::left(Val::Px(10.)),
                                ..default()
                            }),
                            Label,
                        ));

                    parent
                        .spawn((
                            TextBundle::from_section(
                                "Flickering speed: T + scroll",
                                text_help_style.clone(),
                            )
                            .with_style(Style {
                                margin: UiRect::left(Val::Px(10.)),
                                ..default()
                            }),
                            Label,
                        ));
                });
        });
}