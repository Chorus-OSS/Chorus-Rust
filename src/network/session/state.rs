use log::debug;
use statig::prelude::*;

pub enum SessionState {
    Start,
    Login,
    ResourcePack,
    Encryption,
    PreSpawn,
    InGame,
    Death
}

#[derive(Default)]
pub struct SessionStateMachine;

#[state_machine(initial = "State::start()")]
impl SessionStateMachine {
    #[state(exit_action = "exit_start")]
    fn start(event: &SessionState) -> Response<State> {
        match event {
            SessionState::Login => Transition(State::login()),
            _ => Super
        }
    }
    
    #[state]
    fn login(event: &SessionState) -> Response<State> {
        match event {
            _ => Super
        }
    }

    #[action]
    fn exit_start() {
        debug!("Waiting for login packet.")
    }
}