use crate::services::event_bus::EventBus;
use yew::{html, Bridge, Bridged, Component, ComponentLink, Html, ShouldRender};

pub enum Msg {
    NewMessage(String),
}

pub struct Subscriber {
    messages: Vec<String>,
    _producer: Box<dyn Bridge<EventBus>>,
}

impl Component for Subscriber {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            messages: vec![],
            _producer: EventBus::bridge(link.callback(Msg::NewMessage)),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NewMessage(mut s) => {
                self.messages.push(s);
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <h1>{ format!("{:?}",&self.messages) }</h1>
        }
    }
}
