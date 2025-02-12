use bevy::{prelude::*, window::WindowMode};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub const LAUNCHER_TITLE: &str = "Bevy Shell - Template";

pub fn app(fullscreen: bool) -> App {
    let mode = if fullscreen {
        WindowMode::BorderlessFullscreen
    } else {
        WindowMode::Windowed
    };
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: LAUNCHER_TITLE.to_string(),
            canvas: Some("#bevy".to_string()),
            fit_canvas_to_parent: true,
            mode,
            ..default()
        },
        ..default()
    }))
    .add_plugins(EguiPlugin)
    //.add_startup_system(load_icon);
    //app
    .add_systems(Update, ui_example_system)
    .run();
}

fn ui_example_system(mut contexts: EguiContexts) {
  egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
    ui.label("world");
  });
}

/*fn load_icon(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("bevy.png"),
        ..default()
    });
}*/
