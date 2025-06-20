use crate::database::{self, DBClient};
use crate::routes::Item;
use actix_web::Result as AwResult;
use actix_web::{Scope, get, web};
use maud::{Markup, html};

pub fn scope() -> Scope {
    web::scope("list").service(index_route)
}

#[get("")]
async fn index_route(client: web::Data<DBClient>) -> AwResult<Markup> {
    let client = client.get_ref();
    let items: Vec<Item> = database::get_items(client).await;

    let sender = "You";
    let messages = database::get_messages(client, sender).await;

    Ok(super::index(Some(render(&items)), messages.as_slice()))
}

pub fn render(items: &[Item]) -> Markup {
    html! {
        div class="card bg-base-200 shadow-xl" {
            div class="card-body" {
                h2 class="card-title text-2xl mb-4" {
                    svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                        path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4" {
                        }
                    }
                    "List"
                }
                form class="flex gap-2 mb-4" hx-post="/api/todos" hx-target="#todo-list" hx-swap="beforeend" hx-on--after-request="this.reset()" {
                    input class="input input-bordered flex-1" type="text" name="task" placeholder="Add a new task..." required;
                    button class="btn btn-primary" type="submit" {
                        svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                            path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" {
                            }
                        }
                        "Add"
                    }
                }
                div id="todo-list" class="todo-container space-y-2" {
                    @for item in items {
                        (render_item(item))
                    }
                }
            }
        }
    }
}

pub fn render_item(item: &Item) -> Markup {
    html! {
        div class="flex items-center gap-3 p-3 bg-base-100 rounded-lg" id=(format!("c-todo-{}", item.id)) {

            input class="checkbox checkbox-primary" type="checkbox"
                id=(format!("todo-{}", item.id))
                name=(format!("todo-{}", item.id))
                checked[item.completed]

                hx-patch=(format!("/api/todos/{}/toggle", item.id))
                hx-target=(format!("#c-todo-{}", item.id))
                hx-swap="outerHTML";
            span class={
                @if item.completed {
                    "flex-1 line-through opacity-60"
                } @else {
                    "flex-1"
                }
            } {
                (item.task)
            }
            button class="btn btn-sm btn-error btn-outline"
                hx-delete=(format!("/api/todos/{}", item.id))
                hx-target="closest div"
                hx-swap="outerHTML"
                hx-confirm="Are you sure you want to delete this item?" {
                svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" {
                    }
                }
            }
        }
    }
}
