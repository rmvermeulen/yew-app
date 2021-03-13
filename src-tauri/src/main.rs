#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => {
          match command {
            // definitions for your custom commands from Cmd here
            MyCustomCommand { argument,callback,error } => {
              //  your command code
              println!("argument: '{}'", argument); 
              tauri::execute_promise(_webview, move || Ok(15), callback, error)
            }
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
