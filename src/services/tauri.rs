pub enum Msg {
    SendCommand(String)
}

pub struct TauriService {}

impl Agent for TauriService {}
