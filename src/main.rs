use js_sys::{Function, Promise, Reflect};
use serde::Serialize;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
enum Msg {
    IncrementCounter,
    DecrementCounter,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::IncrementCounter => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::DecrementCounter => {
                self.value -= 1;
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
        html! {
            <div>
                <p>{"my fancy app!"}</p>
                <button onclick=self.link.callback(|_| Msg::IncrementCounter)>{ "+1" }</button>
                <p>{ self.value }</p>
                <button onclick=self.link.callback(|_| Msg::DecrementCounter)>{ "-1" }</button>
            </div>
        }
    }
}

#[derive(Serialize)]
struct Arg {
    cmd: String,
    argument: String,
}

fn main() {
    // web-sys
    let window: web_sys::Window = web_sys::window().expect("window not available");
    let key = JsValue::from_str("__TAURI__");
    let tauri = Reflect::get(&window, &key).expect("tauri");

    let promisified = Reflect::get(&tauri, &JsValue::from_str("promisified"))
        .expect("tauri::promisifed() function not found!");
    let promisified = Function::from(promisified);
    let arg = JsValue::from_serde(&Arg {
        cmd: String::from("myCustomCommand"),
        argument: String::from("some text"),
    })
    .expect("Failed to serialize Arg");
    let response = promisified.call1(&tauri, &arg).expect("expected a promise");
    let cb = Closure::wrap(Box::new(|result| {
        println!("got response: {:?}", result);
    }) as Box<dyn FnMut(JsValue)>);
    Promise::resolve(&response).then(&cb);
    // window
    //     .alert_with_message(&format!("response: {:?}", response))
    //     .expect("failed to window::alert()");
    yew::start_app::<Model>();
}
