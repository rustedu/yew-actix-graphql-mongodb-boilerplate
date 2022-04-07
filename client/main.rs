use gloo::storage::{LocalStorage, Storage};
use state::{Entry, Filter, State};
use strum::IntoEnumIterator;
use types::TodoListInfo;
use web_sys::HtmlInputElement as InputElement;
use yew::{
    classes,
    events::{FocusEvent, KeyboardEvent},
    html,
    html::Scope,
    Classes, Component, Context, Html, NodeRef, TargetCast,
};

extern crate dotenv;

use dotenv::dotenv;


mod state;
mod utils;
mod types;
mod services;


use crate::types::{ TodoCreateUpdateInfoWrapper, TodoCreateUpdateInfo };
use crate::services::todos;

const KEY: &str = "yew.todomvc.self";

pub enum Msg {
    Add(String),
    Edit((usize, String)),
    Remove(usize),
    SetFilter(Filter),
    ToggleAll,
    ToggleEdit(usize),
    Toggle(usize),
    ClearCompleted,
    Focus,
    InitialEntries(Vec<Entry>),
    RestApiTest
}

pub struct App {
    state: State,
    focus_ref: NodeRef,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        // let mut entries = LocalStorage::get(KEY).unwrap_or_else(|_| Vec::new());
        let entries: Vec<Entry> = vec![];
        let state = State {
            entries,
            filter: Filter::All,
            edit_value: "".into(),
        };
        let focus_ref = NodeRef::default();
        Self { state, focus_ref }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        log::info!("first_render: {:?}", first_render);
        if first_render {
            let link = ctx.link().clone();
            let fetch_todos =  async move {
                let link = link.clone();
                let todos_list = todos::get_all().await;
                log::info!("todos_list: {:?}", todos_list);
                if let Some(todos_list) = &todos_list.ok() {
                    link.send_message(Msg::InitialEntries(todos_list.todos.clone()));
                }
            };
            wasm_bindgen_futures::spawn_local(fetch_todos );
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InitialEntries(entries) => {
                log::info!("update: {:?}", entries);
                self.state.entries = entries
            },
            Msg::RestApiTest => {
                let rest_todos =  async move {
                    // todos::get("1".to_string()).await;
                    // todos::update("1".to_string(), TodoCreateUpdateInfoWrapper { todo: TodoCreateUpdateInfo { description: "cooooooook".to_string() } }).await;
                    // todos::del("5".to_string()).await;
                };
                wasm_bindgen_futures::spawn_local( rest_todos );
            },
            Msg::Add(description) => {
                if !description.is_empty() {
                    let entry = Entry {
                        id: 10,
                        description: description.trim().to_string(),
                        completed: false,
                        editing: false,
                    };
                    self.state.entries.push(entry);
                }
            }
            Msg::Edit((idx, edit_value)) => {
                self.state.complete_edit(idx, edit_value.trim().to_string());
                self.state.edit_value = "".to_string();
            }
            Msg::Remove(idx) => {
                self.state.remove(idx);
            }
            Msg::SetFilter(filter) => {
                self.state.filter = filter;
            }
            Msg::ToggleEdit(idx) => {
                self.state.edit_value = self.state.entries[idx].description.clone();
                self.state.clear_all_edit();
                self.state.toggle_edit(idx);
            }
            Msg::ToggleAll => {
                let status = !self.state.is_all_completed();
                self.state.toggle_all(status);
            }
            Msg::Toggle(idx) => {
                self.state.toggle(idx);
            }
            Msg::ClearCompleted => {
                self.state.clear_completed();
            }
            Msg::Focus => {
                if let Some(input) = self.focus_ref.cast::<InputElement>() {
                    input.focus().unwrap();
                }
            }
        }
        LocalStorage::set(KEY, &self.state.entries).expect("failed to set");
        // post to /api/todos to update mongodb

        let entries = self.state.entries.clone();
        wasm_bindgen_futures::spawn_local(async move {

            let info = TodoListInfo {
                todos: entries,
            };

            let _r = todos::set_all(info).await;
        });

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let hidden_class = if self.state.entries.is_empty() {
            "hidden"
        } else {
            ""
        };
        html! {
            <div class="todomvc-wrapper">
                <section class="todoapp">
                    <header class="header">
                        <h1>{ "todos" }</h1>
                        { self.view_input(ctx.link()) }
                    </header>
                    <section class={classes!("main", hidden_class)}>
                        <input
                            type="checkbox"
                            class="toggle-all"
                            id="toggle-all"
                            checked={self.state.is_all_completed()}
                            onclick={ctx.link().callback(|_| Msg::ToggleAll)}
                        />
                        <label for="toggle-all" />
                        <ul class="todo-list">
                            { for self.state.entries.iter().filter(|e| self.state.filter.fits(e)).enumerate().map(|e| self.view_entry(e, ctx.link())) }
                        </ul>
                    </section>
                    <footer class={classes!("footer", hidden_class)}>
                        <span class="todo-count">
                            <strong>{ self.state.total() }</strong>
                            { " item(s) left" }
                        </span>
                        <ul class="filters">
                            { for Filter::iter().map(|flt| self.view_filter(flt, ctx.link())) }
                        </ul>
                        <button class="clear-completed" onclick={ctx.link().callback(|_| Msg::ClearCompleted)}>
                            { format!("Clear completed ({})", self.state.total_completed()) }
                        </button>
                    </footer>
                </section>
                <footer class="info">
                    <p>{ "Double-click to edit a todo" }</p>
                    <p>{ "Written by " }<a href="https://github.com/DenisKolodin/" target="_blank">{ "Denis Kolodin" }</a></p>
                    <p>{ "Part of " }<a href="http://todomvc.com/" target="_blank">{ "TodoMVC" }</a></p>
                </footer>
            </div>
        }
    }
}

impl App {
    fn view_filter(&self, filter: Filter, link: &Scope<Self>) -> Html {
        let cls = if self.state.filter == filter {
            "selected"
        } else {
            "not-selected"
        };
        html! {
            <li>
                <a class={cls}
                   href={filter.as_href()}
                   onclick={link.callback(move |_| Msg::SetFilter(filter))}
                >
                    { filter }
                </a>
            </li>
        }
    }

    fn view_input(&self, link: &Scope<Self>) -> Html {
        let onkeypress = link.batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: InputElement = e.target_unchecked_into();
                let value = input.value();
                input.set_value("");
                Some(Msg::Add(value))
            } else {
                None
            }
        });
        html! {
            // You can use standard Rust comments. One line:
            // <li></li>
            <input
                class="new-todo"
                placeholder="What needs to be done?"
                {onkeypress}
            />
            /* Or multiline:
            <ul>
                <li></li>
            </ul>
            */
        }
    }

    fn view_entry(&self, (idx, entry): (usize, &Entry), link: &Scope<Self>) -> Html {
        let mut class = Classes::from("todo");
        if entry.editing {
            class.push(" editing");
        }
        if entry.completed {
            class.push(" completed");
        }
        html! {
            <li {class}>
                <div class="view">
                    <input
                        type="checkbox"
                        class="toggle"
                        checked={entry.completed}
                        onclick={link.callback(move |_| Msg::Toggle(idx))}
                    />
                    <label ondblclick={link.callback(move |_| Msg::ToggleEdit(idx))}>{ &entry.description }</label>
                    <button class="destroy" onclick={link.callback(move |_| Msg::Remove(idx))} />
                </div>
                { self.view_entry_edit_input((idx, entry), link) }
            </li>
        }
    }

    fn view_entry_edit_input(&self, (idx, entry): (usize, &Entry), link: &Scope<Self>) -> Html {
        let edit = move |input: InputElement| {
            let value = input.value();
            input.set_value("");
            Msg::Edit((idx, value))
        };

        let onblur = link.callback(move |e: FocusEvent| edit(e.target_unchecked_into()));

        let onkeypress = link.batch_callback(move |e: KeyboardEvent| {
            (e.key() == "Enter").then(|| edit(e.target_unchecked_into()))
        });

        if entry.editing {
            html! {
                <input
                    class="edit"
                    type="text"
                    ref={self.focus_ref.clone()}
                    value={self.state.edit_value.clone()}
                    onmouseover={link.callback(|_| Msg::Focus)}
                    {onblur}
                    {onkeypress}
                />
            }
        } else {
            html! { <input type="hidden" /> }
        }
    }
}

fn main() {
    dotenv().ok();
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

// use yew::prelude::*;

// enum Msg {
//     AddOne,
// }

// struct Model {
//     value: i64,
// }

// #[derive(Debug, Default)]
// pub struct App {
//     password: String,
// }

// impl Component for Model {
//     type Message = Msg;
//     type Properties = ();

//     fn create(_ctx: &Context<Self>) -> Self {
//         Self {
//             value: 0,
//         }
//     }

//     fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             Msg::AddOne => {
//                 self.value += 1;
//                 // the value has changed so we need to
//                 // re-render for it to appear on the page
//                 true
//             }
//         }
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
//         let link = ctx.link();
//         html! {
//             <div>
//                 <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
//                 <p>{ self.value }</p>
//             </div>
//         }
//     }
// }

// fn main() {
//     yew::start_app::<Model>();
// }

// use wasm_bindgen::JsCast;
// use web_sys::{EventTarget, HtmlInputElement};
// use yew::{
//     events::Event,
//     html,
//     Component, Context, Html,
// };

// pub struct Comp;

// pub enum Msg {
//     InputValue(String),
// }

// impl Component for Comp {
//     type Message = Msg;
//     type Properties = ();

//     fn create(_: &Context<Self>) -> Self {
//         Self
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         let link = ctx.link();

//         // Use batch_callback so if something unexpected happens we can return
//         // None and do nothing
//         let on_cautious_change = link.batch_callback(|e: Event| {
//             // When events are created the target is undefined, it's only
//             // when dispatched does the target get added.
//             let target: Option<EventTarget> = e.target();
//             // Events can bubble so this listener might catch events from child
//             // elements which are not of type HtmlInputElement
//             //highlight-next-line
//             let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
//             input.map(|input| Msg::InputValue(input.value()))
//         });

//         let on_dangerous_change = link.callback(|e: Event| {
//             let target: EventTarget = e
//                 .target()
//                 .expect("Event should have a target when dispatched");
//             // You must KNOW target is a HtmlInputElement, otherwise
//             // the call to value would be Undefined Behaviour (UB).
//             //highlight-next-line
//             Msg::InputValue(target.unchecked_into::<HtmlInputElement>().value())
//         });

//         html! {
//             <>
//                 <label for="cautious-input">
//                     { "My cautious input:" }
//                     <input onchange={on_cautious_change}
//                         id="cautious-input"
//                         type="text"
//                     />
//                 </label>
//                 <br />
//                 <label for="dangerous-input">
//                     { "My dangerous input:" }
//                     <input onchange={on_dangerous_change}
//                         id="dangerous-input"
//                         type="text"
//                     />
//                 </label>
//             </>
//         }
//     }
// }

// fn main() {
//     yew::start_app::<Comp>();
// }
