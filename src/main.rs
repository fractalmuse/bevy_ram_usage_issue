use bevy::{prelude::*, utils::HashMap, window::PresentMode};
use iyes_loopless::prelude::*;
use iyes_progress::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Phase {
    SplashScreen,
    MainMenu,
}

#[derive(Default, Resource)]
struct GraphicsPack {
    handles: HashMap<String, HandleUntyped>,
}

impl GraphicsPack {
    #[cfg(feature = "all_assets")]
    fn asset_file_list() -> Vec<(String, String)> {
        let mut files: Vec<(String, String)> = vec![
            ("heading_font", "fonts/MacedoniaOld.ttf"),
            ("body_font", "fonts/TimesSansSerif.ttf"),
            ("light_background", "images/ui/parchment_texture.png"),
            ("dark_background", "images/ui/stone_texture.png"),
            ("loading_icon", "images/ui/loading.png"),
            ("minotaur", "images/minotaur.png"),
            ("skeleton_mage", "images/skeleton_mage.png"),
            ("skeleton", "images/skeleton.png"),
        ].iter().map(|(a, b)| (a.to_string(), b.to_string())).collect();

        for character in ["male", "female"] {
            for part in ["feet", "hands", "head", "legs", "torso"] {
                for material in ["chain", "default", "leather", "mage_alt1", "mage_alt2", "plate"] {
                    let name = [character, part, material].join("/");
                    let path = format!("images/{}.png", &name);
                    files.push((name, path));
                }
            }

            for weapon in ["buckler", "greatbow", "iron_buckler", "longbow", "longsword", "rod", "shield", "shortbow", "shortsword", "staff", "wand", "zweihander"] {
                let name = [character, "weapons", weapon].join("/");
                let path = format!("images/{}.png", &name);
                files.push((name, path));
            }
        }
    
        files
    }

    fn title_style(&self) -> TextStyle {
        TextStyle {
            font: self.handles.get("heading_font").unwrap().clone().typed(),
            font_size: 96.0,
            color: Color::WHITE,
        }
    }
}

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa { samples: 4 })
        .init_resource::<GraphicsPack>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "RAM Usage".to_string(),
                width: 1280.,
                height: 720.,
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_loopless_state(Phase::SplashScreen)
        .add_plugin(
            ProgressPlugin::new(Phase::SplashScreen)
                .continue_to(Phase::MainMenu)
                .track_assets(),
        )
        .add_enter_system(Phase::MainMenu, main_menu_setup)
        .add_startup_system(camera_setup);

    #[cfg(feature = "all_assets")]
    {
        app.add_enter_system(Phase::SplashScreen, load_assets);
    }

    #[cfg(feature = "only_font")]
    {
        app.add_enter_system(Phase::SplashScreen, load_single_image);
    }

    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[cfg(feature = "only_font")]
fn load_single_image(
    mut pack: ResMut<GraphicsPack>,
    asset_server: Res<AssetServer>,
    mut progress_tracker: ResMut<AssetsLoading>,
) {
    let handle = asset_server.load_untyped("fonts/MacedoniaOld.ttf");
    progress_tracker.add(&handle);
    pack.handles.insert("heading_font".into(), handle);
}

#[cfg(feature = "all_assets")]
fn load_assets(
    mut pack: ResMut<GraphicsPack>,
    asset_server: Res<AssetServer>,
    mut progress_tracker: ResMut<AssetsLoading>,
) {
    for (name, path) in GraphicsPack::asset_file_list() {
        let handle = asset_server.load_untyped(path);
        progress_tracker.add(&handle);
        pack.handles.insert(name, handle);
    }
}

const MB_SIZE: f32 = 1_000_000.;

fn main_menu_setup(mut commands: Commands, pack: Res<GraphicsPack>, images: Res<Assets<Image>>) {
    #[cfg(feature = "all_assets")]
    {
        // Log the memory that the loaded images are using
        bevy::log::info!("**Image Memory Usage**");
        let mut total_mb_size: f32 = 0.;

        for (name, handle) in &pack.handles {
            if name == "heading_font" || name == "body_font" { continue; }
    
            let mb_size = (images.get(&handle.typed_weak()).unwrap().data.len() as f32) / MB_SIZE;
            bevy::log::info!("{}: {:.3}MB", name, mb_size);
            total_mb_size += mb_size;
        }

        bevy::log::info!("Total: {:.3}MB", total_mb_size);
    }

    commands
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                margin: UiRect::all(Val::Px(0.)),
                ..default()
            },
            ..default()
        })
        // .insert(Decoration::Image(theme.light_background()))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("RAM Usage", pack.title_style()));
        });
}
