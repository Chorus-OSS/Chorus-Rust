use std::sync::Weak;
use log::{debug, info};
use statig::prelude::*;
use tokio::sync::Mutex;
use crate::network::handler::packet_handler::PacketHandler;
use crate::network::session::Session;
use crate::server::Server;

pub enum SessionState {
    Start,
    Login,
    ResourcePack,
    Encryption,
    PreSpawn,
    InGame,
    Death
}

pub struct SessionStateMachine;

impl SessionStateMachine {
    pub fn new() -> Self {
        Self { }
    }
}

#[state_machine(initial = "State::start()")]
impl SessionStateMachine {
    #[state]
    async fn start(event: &SessionState) -> Response<State> {
        match event {
            SessionState::Login => Transition(State::login()),
            _ => Super
        }
    }

    #[state]
    async fn login(event: &SessionState) -> Response<State> {
        match event {
            SessionState::Encryption => {
                if (Server::get().await.properties.network_encryption) {
                    Transition(State::encryption())
                } else { Super }
            }
            SessionState::ResourcePack => Transition(State::resource_pack()),
            _ => Super
        }
    }

    #[state]
    async fn encryption(event: &SessionState) -> Response<State> {
        match event {
            _ => Super
        }
    }

    #[state]
    async fn resource_pack(event: &SessionState) -> Response<State> {
        match event {
            _ => Super
        }
    }
}