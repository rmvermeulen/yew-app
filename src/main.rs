use serde::Serialize;
use std::time::Duration;
use yew::prelude::*;
use yew::services::interval::{IntervalService, IntervalTask};
use yew::services::ConsoleService;
use yew::utils::window;

mod services;
use services::tauri;

enum Msg {
    IncrementCounter,
    DecrementCounter,
    IncTimer,
    Reload,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    counter: i64,
    timer: i64,
    _handles: Vec<IntervalTask>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // timer
        let handle =
            IntervalService::spawn(Duration::from_secs(1), link.callback(|_| Msg::IncTimer));

        Self {
            link,
            counter: 0,
            timer: 0,
            _handles: vec![handle],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;
        match msg {
            Reload => {
                window().location().reload().unwrap();
                false
            }
            IncTimer => {
                self.timer += 1;
                true
            }
            IncrementCounter => {
                ConsoleService::log(&format!(
                    "increment {} -> {}",
                    self.counter,
                    self.counter + 1
                ));
                self.counter += 1;
                // the counter has changed so we need to
                // re-render for it to appear on the page
                true
            }
            DecrementCounter => {
                ConsoleService::log(&format!(
                    "decrement {} -> {}",
                    self.counter,
                    self.counter - 1
                ));
                self.counter -= 1;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let header = format!("my fancy app, alive for {} seconds", self.timer);
        let reload = self.link.callback(|_| Msg::Reload);
        let inc_counter = self.link.callback(|_| Msg::IncrementCounter);
        let dec_counter = self.link.callback(|_| Msg::DecrementCounter);
        html! {
            <div>
                <p>{header}</p>
                <button onclick=reload>{"reload app"}</button>
                <div>
                    <button onclick=inc_counter>{ "+1" }</button>
                    <p>{ self.counter }</p>
                    <button onclick=dec_counter>{ "-1" }</button>
                </div>
            </div>
        }
    }
}

fn main() {
    ConsoleService::log(&format!("sending message..."));
    tauri::send_message(tauri::Msg::SayHi);
    ConsoleService::log(&format!("sent message!"));
    yew::start_app::<Model>();
}
