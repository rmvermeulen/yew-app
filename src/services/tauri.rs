use js_sys::{Function, Promise, Reflect};
use serde::{Deserialize, Serialize};
use yew::services::ConsoleService;
use yew::worker::*;

use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use web_sys::Window;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    EventBusMsg(String),
}

pub struct EventBus {
    link: AgentLink<EventBus>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for EventBus {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Request;
    type Output = String;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            Request::EventBusMsg(s) => {
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, s.clone());
                }
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}

#[derive(Serialize)]
struct Arg {
    cmd: String,
    argument: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum Msg {
    SayHi,
    AddNumber(i32),
}
pub fn send_message(msg: Msg) {
    // web-sys
    let window: Window = web_sys::window().expect("window not available");

    let key = JsValue::from_str("__TAURI__");
    let tauri = Reflect::get(&window, &key).expect("tauri");

    let promisified = Reflect::get(&tauri, &JsValue::from_str("promisified"))
        .expect("tauri::promisifed() function not found!");
    let promisified = Function::from(promisified);
    let arg = JsValue::from_serde(&Arg {
        cmd: String::from("myCustomCommand"),
        argument: String::from("some text"),
    })
    .expect("Failed )to serialize Arg");
    let promise: Promise = promisified
        .call1(&tauri, &arg)
        .expect("expected a promise")
        .into();
    handle_promise(window, promise);
    ConsoleService::log(&format!("tauri::send_message {:?}", msg));
}

async fn handle_promise(window: Window, promise: Promise) -> Result<JsValue, JsValue> {
    let future = wasm_bindgen_futures::JsFuture::from(promise);
    let response = future.await?;
    window
        .alert_with_message(&format!("response: {:?}", response))
        .expect("failed to window::alert()");
    Ok(response)
}
