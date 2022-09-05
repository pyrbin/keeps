use crate::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct AssetsPlugin {
    pub next_state: AppState,
}

impl AssetsPlugin {
    pub fn continue_to(next_state: AppState) -> Self {
        Self { next_state }
    }
}

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_exit_system(AppState::AssetLoading, despawn_all_with::<LoadingMenu>)
            .add_loading_state(
                LoadingState::new(AppState::AssetLoading)
                    .with_collection::<FontAssets>()
                    .with_collection::<TextureAssets>()
                    .with_collection::<AudioAssets>()
                    .continue_to_state(AppState::WorldGen),
            );
    }
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy_logo: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(Component)]
pub struct LoadingMenu;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: UiColor(Color::hex("101010").unwrap()),
            ..Default::default()
        })
        .insert(LoadingMenu)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    "Loading...",
                    TextStyle {
                        font_size: 100.0,
                        color: Color::WHITE,
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    },
                ),
                ..Default::default()
            });
        });
}
