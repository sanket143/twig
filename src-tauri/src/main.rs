#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

use dotenv;
use serde::Serialize;

extern crate imap;
extern crate native_tls;

#[derive(Serialize)]
struct Reply {
  data: String,
}

fn fetch_inbox_top() -> imap::error::Result<Option<String>> {
  dotenv::dotenv().ok();

  let domain = "imap.gmail.com";
  let tls = native_tls::TlsConnector::builder().build().unwrap();

  // we pass in the domain twice to check that the server's TLS
  // certificate is valid for the domain we're connecting to.
  let client = imap::connect((domain, 993), domain, &tls).unwrap();

  // the client we have here is unauthenticated.
  // to do anything useful with the e-mails, we need to log in


  let email = dotenv::var("TWIG_EMAIL").unwrap();
  let password = dotenv::var("TWIG_PASSWORD").unwrap();

  let mut imap_session = client
    .login(email, password)
    .map_err(|e| e.0)?;

  // we want to fetch the first email in the INBOX mailbox
  imap_session.select("INBOX")?;

  // fetch message number 1 in this mailbox, along with its RFC822 field.
  // RFC 822 dictates the format of the body of e-mails
  let messages = imap_session.fetch("1", "RFC822")?;
  let message = if let Some(m) = messages.iter().next() {
    m
  } else {
    return Ok(None);
  };

  // extract the message's body
  let body = message.body().expect("message did not have a body!");
  let body = std::str::from_utf8(body)
    .expect("message was not valid utf-8")
    .to_string();

  // be nice to the server and log out
  imap_session.logout()?;

  Ok(Some(body))
}

fn sign_in_handler(data: Option<String>) {
  println!("signInHandler '{:?}'", data);
  println!("{:?}", fetch_inbox_top().unwrap().unwrap());
}

fn main() {
  tauri::AppBuilder::new()
    .setup(|webview, _source| {
      let mut webview = webview.as_mut();

      tauri::event::listen(String::from("sign-in"), sign_in_handler);

      tauri::event::listen(String::from("js-event"), move |msg| {

        let reply = Reply {
          data: msg.unwrap(),
        };

        tauri::event::emit(
          &mut webview,
          String::from("rust-event"),
          Some(serde_json::to_string(&reply).unwrap()),
        )
        .expect("failed to emit");
      });
    })
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => {
          Err(e.to_string())
        }
        Ok(command) => {
          match command {
            // definitions for your custom commands from Cmd here
            MyCustomCommand { argument } => {
              //  your command code
              println!("{}", argument);
            }
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
