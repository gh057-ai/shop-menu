use bevy::{
    input::{mouse::MouseButton, ButtonInput},
    prelude::*,
    ui::Interaction,
};

// Constants (if you add any)

// States
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
}

// Resources
#[derive(Resource)]
pub struct PlayerStats {
    coins: u32,
    damage: f32,
    speed: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            coins: 100,
            damage: 10.0,
            speed: 1.0,
        }
    }
}

// Components
#[derive(Component, Clone, Copy)]
struct ShopButton {
    upgrade_type: UpgradeType,
    cost: u32,
}

#[derive(Component)]
struct HoverDebugText;

#[derive(Component)]
struct CoinsText;

#[derive(Component)]
struct DamageText;

#[derive(Component)]
struct SpeedText;

// Enums
#[derive(Clone, Copy, Debug)]
pub enum UpgradeType {
    Damage,
    Speed,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ShopSystemSet {
    Input,
    Update,
    Display,
}

// Events
#[derive(Event)]
pub struct PurchaseEvent {
    pub upgrade_type: UpgradeType,
}

// Plugin
pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerStats>()
            .add_event::<PurchaseEvent>()
            .add_systems(Startup, setup_shop)
            .configure_sets(
                Update,
                (ShopSystemSet::Input, ShopSystemSet::Update, ShopSystemSet::Display).chain(),
            )
            .add_systems(
                Update,
                (
                    handle_shop_buttons.in_set(ShopSystemSet::Input),
                    handle_purchases.in_set(ShopSystemSet::Update),
                    update_shop_ui.in_set(ShopSystemSet::Display),
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

// Systems
fn setup_shop(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(20.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                position_type: PositionType::Absolute,
                left: Val::Percent(40.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.9)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "SHOP",
                    TextStyle {
                        font_size: 30.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                style: Style {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            });

            spawn_stats_display(parent);

            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(20.0),
                    ..default()
                },
                ..default()
            });

            spawn_shop_button(parent, "Damage +5 (Cost: 50)", UpgradeType::Damage, 50);
            spawn_shop_button(parent, "Speed +0.2 (Cost: 30)", UpgradeType::Speed, 30);

            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Hover State: None",
                        TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    style: Style {
                        margin: UiRect::top(Val::Px(20.0)),
                        ..default()
                    },
                    ..default()
                },
                HoverDebugText,
            ));
        });
}

fn spawn_stats_display(parent: &mut ChildBuilder) {
    parent.spawn((
        TextBundle::from_section(
            format!("Coins: {}", 100),
            TextStyle {
                font_size: 20.0,
                color: Color::srgb(1.0, 0.84, 0.0),
                ..default()
            },
        ),
        CoinsText,
    ));

    parent.spawn((
        TextBundle::from_section(
            format!("Damage: {}", 10.0),
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        ),
        DamageText,
    ));

    parent.spawn((
        TextBundle::from_section(
            format!("Speed: {}", 1.0),
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        ),
        SpeedText,
    ));
}

fn spawn_shop_button(parent: &mut ChildBuilder, text: &str, upgrade_type: UpgradeType, cost: u32) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    margin: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                border_color: BorderColor(Color::WHITE),
                background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                interaction: Interaction::None,
                ..default()
            },
            ShopButton {
                upgrade_type,
                cost,
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

fn handle_shop_buttons(
    mut interaction_query: Query<(&Interaction, &ShopButton, &mut BackgroundColor), Changed<Interaction>>,
    mut debug_text_query: Query<&mut Text, With<HoverDebugText>>,
    mut player_stats: ResMut<PlayerStats>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut purchase_events: EventWriter<PurchaseEvent>,
) {
    for (interaction, shop_button, mut color) in interaction_query.iter_mut() {
        debug!("Processing interaction: {:?}", interaction);
        
        match *interaction {
            Interaction::Pressed => {
                info!("Purchase attempted for {:?}", shop_button.upgrade_type);
                *color = BackgroundColor(Color::srgb(0.4, 0.4, 0.4));
                
                if mouse_input.just_pressed(MouseButton::Left) {
                    if player_stats.coins >= shop_button.cost {
                        info!("Purchase successful: {:?} for {} coins", shop_button.upgrade_type, shop_button.cost);
                        purchase_events.send(PurchaseEvent {
                            upgrade_type: shop_button.upgrade_type,
                        });
                        player_stats.coins -= shop_button.cost;
                        *color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
                        
                        if let Ok(mut debug_text) = debug_text_query.get_single_mut() {
                            debug_text.sections[0].value = "Purchase Successful".to_string();
                        }
                    } else if let Ok(mut debug_text) = debug_text_query.get_single_mut() {
                        debug_text.sections[0].value = "Not enough coins!".to_string();
                    }
                }
            }
            Interaction::Hovered => {
                debug!("Button hovered");
                *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
            }
            Interaction::None => {
                debug!("Button interaction ended");
                *color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
            }
        }
    }
}

fn handle_purchases(mut events: EventReader<PurchaseEvent>, mut player_stats: ResMut<PlayerStats>) {
    for event in events.read() {
        info!("Processing purchase: {:?}", event.upgrade_type);
        match event.upgrade_type {
            UpgradeType::Damage => {
                player_stats.damage += 5.0;
                info!("Damage upgraded to: {}", player_stats.damage);
            }
            UpgradeType::Speed => {
                player_stats.speed += 0.2;
                info!("Speed upgraded to: {}", player_stats.speed);
            }
        }
    }
}

// For some reason this is needed to silence a warning
#[allow(clippy::type_complexity)]
fn update_shop_ui(
    player_stats: Res<PlayerStats>,
    mut coins_query: Query<&mut Text, With<CoinsText>>,
    mut damage_query: Query<&mut Text, (With<DamageText>, Without<CoinsText>)>,
    mut speed_query: Query<&mut Text, (With<SpeedText>, Without<CoinsText>, Without<DamageText>)>,
) {
    if let Ok(mut coins_text) = coins_query.get_single_mut() {
        coins_text.sections[0].value = format!("Coins: {}", player_stats.coins);
    }

    if let Ok(mut damage_text) = damage_query.get_single_mut() {
        damage_text.sections[0].value = format!("Damage: {}", player_stats.damage);
    }

    if let Ok(mut speed_text) = speed_query.get_single_mut() {
        speed_text.sections[0].value = format!("Speed: {}", player_stats.speed);
    }
}