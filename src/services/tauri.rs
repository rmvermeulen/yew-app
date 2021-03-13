use js_sys::{Function, Reflect};
use serde::Serialize;
use wasm_bindgen::prelude::*;
use web_sys::Window;
use yew::agent::*;

// use std::collections::HashSet;
// use std::time::Duration;
// use yew::utils::window;
// use yew::prelude::*;
// use yew::services::interval::{IntervalService, IntervalTask};
// use yew::services::ConsoleService;
// use yew::worker::*;

use crate::services::event_bus::EventBus;

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
    event_bus: Dispatcher<EventBus>,
}

impl Agent for TauriService {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = ();
    type Output = ();
    fn create(link: AgentLink<Self>) -> Self {
        // web-sys
        let window: Window = web_sys::window().expect("window not available");

        let key = JsValue::from_str("__TAURI__");
        let tauri = Reflect::get(&window, &key).expect("tauri");

        let promisified = Reflect::get(&tauri, &JsValue::from_str("promisified"))
            .expect("tauri::promisifed() function not found!");
        let promisified = Function::from(promisified);
        let arg = JsValue::from_serde(&ArgObject {
            cmd: String::from("myCustomCommand"),
            argument: String::from("some text"),
        })
        .expect("Failed to serialize ArgObject");
        let response = promisified.call1(&tauri, &arg).expect("expected a promise");
        window
            .alert_with_message(&format!("response: {:?}", response))
            .expect("failed to window::alert()");
        // create the service
        return TauriService {
            link,
            event_bus: EventBus::dispatcher(),
        };
    }
    fn update(&mut self, msg: Msg) {}
    fn handle_input(&mut self, input: Self::Input, hid: HandlerId) {}
}
