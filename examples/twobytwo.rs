extern crate bevy_grid;
use bevy::prelude::*;
use bevy_grid::GridComponents;

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        // ui camera
        .spawn(UiCameraComponents::default())
        // root node
        .spawn(NodeComponents {
            style: Style {
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::SpaceBetween,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(GridComponents{
                    style: Style {
                        size: Size::new(Val::Percent(48.0), Val::Percent(48.0)),
                        border: Rect::all(Val::Percent(1.0)),
                        margin: Rect::all(Val::Percent(1.0)),
                        ..Default::default()
                    },
                    material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                    ..Default::default()
                })
                .spawn(GridComponents {
                    style: Style {
                        size: Size::new(Val::Percent(48.0), Val::Percent(48.0)),
                        border: Rect::all(Val::Percent(1.0)),
                        margin: Rect::all(Val::Percent(1.0)),
                        ..Default::default()
                    },
                    material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
                    ..Default::default()
                })
                .spawn(GridComponents {
                    style: Style {
                        size: Size::new(Val::Percent(48.0), Val::Percent(48.0)),
                        border: Rect::all(Val::Percent(1.0)),
                        margin: Rect::all(Val::Percent(1.0)),
                        ..Default::default()
                    },
                    material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
                    ..Default::default()
                })
                .spawn(GridComponents {
                    style: Style {
                        size: Size::new(Val::Percent(48.0), Val::Percent(48.0)),
                        border: Rect::all(Val::Percent(1.0)),
                        margin: Rect::all(Val::Percent(1.0)),
                        ..Default::default()
                    },
                    material: materials.add(Color::rgb(1.0, 1.0, 0.0).into()),
                    ..Default::default()
                });
        });
}
