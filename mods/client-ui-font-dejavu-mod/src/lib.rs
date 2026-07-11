use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_bevy_default_plugins_mod::ClientBevyDefaultPluginsMod;
use client_ui_font_api::{ClientUiFont, ClientUiFontApi};
use tokio::task::JoinHandle;

pub struct ClientUiFontDejavuMod;

impl ClientUiFontDejavuMod {
    pub fn init(bevy: &mut BevyMod, _plugins: &mut ClientBevyDefaultPluginsMod) -> Self {
        bevy.app.add_systems(Startup, load_dejavu_font);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientUiFontApi for ClientUiFontDejavuMod {}

fn load_dejavu_font(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(ClientUiFont(
        assets.load("client-ui-font-dejavu-mod/DejaVuSans.ttf"),
    ));
}
