use std::sync::Weak;
use log::debug;
use statig::prelude::*;
use tokio::sync::Mutex;
use crate::network::handler::login_packet_handler::LoginPacketHandler;
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

pub struct SessionStateMachine {
    session: Weak<Mutex<Session>>,
}

impl SessionStateMachine {
    pub fn new(session: Weak<Mutex<Session>>) -> Self {
        Self { session }
    }
}

#[state_machine(initial = "State::start()")]
impl SessionStateMachine {
    #[state(exit_action = "exit_start")]
    async fn start(event: &SessionState) -> Response<State> {
        match event {
            SessionState::Login => Transition(State::login()),
            _ => Super
        }
    }

    #[state(entry_action = "enter_login", exit_action = "exit_login")]
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

    #[action]
    async fn exit_start() {
        debug!("Waiting for login packet.")
    }

    #[action]
    async fn enter_login(&mut self) {
        let Some(mut session) = self.session.upgrade() else { return; };
        let mut session = session.lock().await;
        
        session.packet_handler = Some(
            Box::new(
                LoginPacketHandler::new(
                    self.session.clone()
                )
            )
        )
    }

    #[action]
    async fn exit_login(&mut self) {
        debug!("Login completed.");
        let Some(mut session) = self.session.upgrade() else { return; };
        let mut session = session.lock().await;
        
        session.on_login_success().await;
    }
}