use actix_web::Result as AwResult;
use actix_web::{Scope, get, web};
use maud::{Markup, html};

mod chat;
mod navbar;
mod todolist;

pub use todolist::render_item;

use crate::database::{self, DBClient};
use crate::routes::ChatMessage;

pub fn scope() -> Scope {
    web::scope("/ui")
        .service(chat::scope())
        .service(todolist::scope())
        .service(index_route)
}

#[get("")]
async fn index_route(client: web::Data<DBClient>) -> AwResult<Markup> {
    let sender = "You";
    let client = client.get_ref();
    let messages = database::get_messages(client, sender).await;
    Ok(index(None, messages.as_slice()))
}

pub fn css(path: impl Into<String>) -> Markup {
    let path: String = path.into();
    html! {link href=(path) rel="stylesheet" type="text/css";}
}

pub fn js(path: impl Into<String>) -> Markup {
    let path: String = path.into();
    html! {script src=(path) {}}
}

pub fn index(content: Option<Markup>, messages: &[ChatMessage]) -> Markup {
    let content = content.unwrap_or_else(|| chat::render(&messages));
    html! {
        (maud::DOCTYPE)
        head {
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            title {
                "Todo & Chat App"
            }
            (js("/assets/tw.js"))
            (js("/assets/theme-switcher.js"))
            (js("/assets/htmx.js"))
            (css("/assets/daisy.css"))
            (css("/assets/themes.css"))
            (css("/assets/app.css"))

        }
        body {
            (js("/assets/htmxListener.js"))
            (navbar::render())

            div class="container mx-auto p-4" {
                div class="grid grid-cols-1 gap-6" {
                  (content)
                }
            }
        }
    }
}
