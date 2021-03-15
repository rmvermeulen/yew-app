use js_sys::{Function, Reflect};
use serde::Serialize;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use web_sys::Window;
use yew::agent::*;
use yew::services::ConsoleService;

// use std::collections::HashSet;
// use std::time::Duration;
// use yew::utils::window;
// use yew::prelude::*;
// use yew::services::interval::{IntervalService, IntervalTask};
// use yew::services::ConsoleService;
// use yew::worker::*;

#[derive(Serialize)]
struct ArgObject {
    cmd: String,
    argument: String,
}

pub enum Msg {
    SendCommand(String),
}

pub struct TauriService {
    link: AgentLink<TauriService>,
    subscribers: HashSet<HandlerId>,
}

impl TauriService {
    // pub fn send_command(&self, cmd: String) {
    //     let cmd = Msg::SendCommand(cmd);
    //     self.link.send_message(cmd)
    // }
}

impl Agent for TauriService {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Msg;
    type Output = ();
    fn create(link: AgentLink<Self>) -> Self {
        // web-sys
        // let window: Window = web_sys::window().expect("window not available");

        // let key = JsValue::from_str("__TAURI__");
        // let tauri = Reflect::get(&window, &key).expect("tauri");

        // let promisified = Reflect::get(&tauri, &JsValue::from_str("promisified"))
        //     .expect("tauri::promisifed() function not found!");
        // let promisified = Function::from(promisified);
        // let arg = JsValue::from_serde(&ArgObject {
        //     cmd: String::from("myCustomCommand"),
        //     argument: String::from("some text"),
        // })
        // .expect("Failed to serialize ArgObject");
        // let response = promisified.call1(&tauri, &arg).expect("expected a promise");
        // window
        //     .alert_with_message(&format!("response: {:?}", response))
        //     .expect("failed to window::alert()");
        // create the service
        return TauriService {
            link,
            subscribers: HashSet::new(),
        };
    }
    fn update(&mut self, _msg: Self::Message) {}
    fn handle_input(&mut self, msg: Self::Input, hid: HandlerId) {
        match msg {
            Msg::SendCommand(cmd) => {
                ConsoleService::log(&format!("TauriCmd: {}", cmd));
            }
        }
    }
}
