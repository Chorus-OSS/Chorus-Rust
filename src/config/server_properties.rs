use std::fs;
use std::path::PathBuf;
use std::process::exit;
use log::{debug, error};
use serde::{Deserialize, Serialize};

const SERVER_PROPERTIES_PATH: &str = "server.properties";

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ServerProperties {
    pub motd: String,
    #[serde(rename = "sub-motd")]
    pub sub_motd: String,
    #[serde(rename = "server-port")]
    pub server_port: u16,
    #[serde(rename = "server-ip")]
    pub server_ip: String,
    #[serde(rename = "view-distance")]
    pub view_distance: u32,
    #[serde(rename = "white-list")]
    pub white_list: bool,
    pub achievements: bool,
    #[serde(rename = "announce-player-achievements")]
    pub announce_player_achievements: bool,
    #[serde(rename = "spawn-protection")]
    pub spawn_protection: u64,
    #[serde(rename = "max-players")]
    pub max_players: u32,
    #[serde(rename = "allow-flight")]
    pub allow_flight: bool,
    #[serde(rename = "spawn-animals")]
    pub spawn_animals: bool,
    #[serde(rename = "spawn-mobs")]
    pub spawn_mobs: bool,
    pub gamemode: u32,
    #[serde(rename = "force-gamemode")]
    pub force_gamemode: bool,
    pub hardcore: bool,
    pub pvp: bool,
    pub difficulty: u32,
    #[serde(rename = "level-name")]
    pub level_name: String,
    #[serde(rename = "level-seed")]
    pub level_seed: String,
    #[serde(rename = "allow-nether")]
    pub allow_nether: bool,
    #[serde(rename = "allow-the-end")]
    pub allow_the_end: bool,
    #[serde(rename = "auto-save")]
    pub auto_save: bool,
    #[serde(rename = "force-resources")]
    pub force_resources: bool,
    #[serde(rename = "force-resources-allow-client-packs")]
    pub force_resources_allow_client_packs: bool,
    #[serde(rename = "xbox-auth")]
    pub xbox_auth: bool,
    #[serde(rename = "check-login-time")]
    pub check_login_time: bool,
    #[serde(rename = "server-authoritative-movement")]
    pub server_authoritative_movement: String,
    #[serde(rename = "network-encryption")]
    pub network_encryption: bool,
}

impl Default for ServerProperties {
    fn default() -> Self {
        Self {
            motd: String::from("Chorus"),
            sub_motd: String::from("chorus-oss.org"),
            server_ip: String::from("0.0.0.0"),
            server_port: 19132,
            view_distance: 8,
            white_list: false,
            achievements: true,
            announce_player_achievements: true,
            spawn_protection: 16,
            max_players: 20,
            allow_flight: false,
            spawn_animals: true,
            spawn_mobs: true,
            gamemode: 0,
            force_gamemode: false,
            hardcore: false,
            pvp: true,
            difficulty: 1,
            level_name: String::from("world"),
            level_seed: String::from(""),
            allow_nether: true,
            allow_the_end: true,
            auto_save: true,
            force_resources: false,
            force_resources_allow_client_packs: false,
            xbox_auth: true,
            check_login_time: false,
            server_authoritative_movement: String::from("server-auth"),
            network_encryption: true,
        }
    }
}

pub fn setup_properties() -> ServerProperties {
    let properties = if PathBuf::from(SERVER_PROPERTIES_PATH).exists() {
        let text = fs::read_to_string(SERVER_PROPERTIES_PATH).unwrap_or_else(|err| {
            error!("Failed to read {SERVER_PROPERTIES_PATH:?}, Err: {err}");
            exit(1);
        });

        serde_java_properties::from_str(&text).unwrap_or_else(|err| {
            error!("Failed to deserialize {SERVER_PROPERTIES_PATH:?}, Err: {err}");
            exit(1);
        })
    } else {
        let properties = ServerProperties::default();

        let text = toml::to_string(&properties).unwrap_or_else(|err| {
            error!("Failed to serialize {properties:?}, Err: {err}");
            exit(1);
        });

        fs::write(SERVER_PROPERTIES_PATH, text).unwrap_or_else(|err| {
            error!("Failed to write missing properties to {SERVER_PROPERTIES_PATH:?}, Err: {err}");
        });

        properties
    };
    
    properties
}