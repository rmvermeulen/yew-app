use crate::services::event_bus::{EventBus, Request};
use crate::services::tauri::{Msg as TMsg, TauriService};
use yew::agent::Dispatcher;
use yew::prelude::*;

pub enum Msg {
    SendMessage,
    UseTauriAPI,
}

pub struct Producer {
    link: ComponentLink<Producer>,
    event_bus: Dispatcher<EventBus>,
    tauri: Dispatcher<TauriService>,
}

impl Component for Producer {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            event_bus: EventBus::dispatcher(),
            tauri: TauriService::dispatcher(),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SendMessage => {
                self.event_bus
                    .send(Request::EventBusMsg("producer::messages".to_owned()));
                false
            }
            Msg::UseTauriAPI => {
                self.tauri
                    .send(TMsg::SendCommand("some command".to_owned()));
                false
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                <button onclick=self.link.callback(|_| Msg::SendMessage)>
                { "PRESS ME" }
                </button>
                // <button onclick=self.link.callback(|_| Msg::UseTauriAPI)>
                // { "USE TAURI" }
                // </button>
            </>
        }
    }
}
