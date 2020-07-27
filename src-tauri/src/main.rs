#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

use dotenv;
use serde::Serialize;
use mailparse::MailHeaderMap;

extern crate imap;
extern crate mailparse;
extern crate native_tls;

#[derive(Serialize)]
struct Reply {
  data: String,
}

fn fetch_inbox_top(msg_number: String) -> imap::error::Result<Option<String>> {
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
  let messages = imap_session.fetch(&msg_number, "RFC822")?;
  let message = if let Some(m) = messages.iter().next() {
    m
  } else {
    return Ok(None);
  };

  // extract the message's body
  let body = message.body().unwrap();

  // be nice to the server and log out
  imap_session.logout()?;

  let pm = mailparse::parse_mail(&body).unwrap();

  pm.headers
    .get_first_value("From")
    .map(|a| println!("{:?}", mailparse::addrparse(&a).unwrap()));
  pm.headers
    .get_first_value("To")
    .map(|a| println!("{:?}", mailparse::addrparse(&a).unwrap()));
  pm.headers
    .get_first_value("Cc")
    .map(|a| println!("{:?}", mailparse::addrparse(&a).unwrap()));
  pm.headers
    .get_first_value("Bcc")
    .map(|a| println!("{:?}", mailparse::addrparse(&a).unwrap()));

  let body = match pm.subparts.len() {
    0 => {
      pm.get_body().unwrap()
    },
    _ => {
      pm.subparts[pm.subparts.len() - 1].get_body().unwrap()
    }
  };

  Ok(Some(body))
}

fn sign_in_handler(msg: Option<String>) -> String {

  fetch_inbox_top(msg.unwrap()).unwrap().unwrap()
}

fn main() {
  tauri::AppBuilder::new()
    .setup(|webview, _source| {
      let webview = webview.as_mut();
      let mut webview_clone = webview.clone();

      tauri::event::listen(String::from("sign-in"), move |msg| {
        let reply = Reply {
          data: sign_in_handler(msg),
        };

        tauri::event::emit(
          &mut webview_clone,
          String::from("mail-fetch"),
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
