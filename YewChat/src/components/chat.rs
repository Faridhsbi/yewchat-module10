use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use crate::services::event_bus::EventBus;
use crate::{User, services::websocket::WebsocketService, Route};
use yew_router::components::Link;

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    wss: WebsocketService,
    messages: Vec<MessageData>,
    _producer: Box<dyn Bridge<EventBus>>,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let register_msg = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username),
            data_array: None,
        };
        let _ = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&register_msg).unwrap());

        Self {
            users: Vec::new(),
            messages: Vec::new(),
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(raw) => {
                if let Ok(ws_msg) = serde_json::from_str::<WebSocketMessage>(&raw) {
                    match ws_msg.message_type {
                        MsgTypes::Users => {
                            let list = ws_msg.data_array.unwrap_or_default();
                            self.users = list
                                .into_iter()
                                .map(|u| UserProfile {
                                    name: u.clone(),
                                    avatar: format!(
                                        "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                        u
                                    ),
                                })
                                .collect();
                            return true;
                        }
                        MsgTypes::Message => {
                            if let Some(data) = ws_msg.data {
                                if let Ok(md) = serde_json::from_str::<MessageData>(&data) {
                                    self.messages.push(md);
                                    return true;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                false
            }
            Msg::SubmitMessage => {
                if let Some(input) = self.chat_input.cast::<HtmlInputElement>() {
                    let text = input.value();
                    if !text.is_empty() {
                        let ws_msg = WebSocketMessage {
                            message_type: MsgTypes::Message,
                            data: Some(text.clone()),
                            data_array: None,
                        };
                        let _ = self
                            .wss
                            .tx
                            .clone()
                            .try_send(serde_json::to_string(&ws_msg).unwrap());
                        input.set_value("");
                    }
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        html! {
            <div class="flex w-screen bg-gray-900 text-white">
                <div class="flex-none w-56 h-screen bg-gray-800">
                    <div class="text-xl p-3 border-b border-gray-700">{"Users"}</div>
                    { for self.users.iter().map(|u| html! {
                        <div class="flex m-3 bg-gray-700 rounded-lg p-2">
                            <img class="w-12 h-12 rounded-full" src="https://cdn.idntimes.com/content-images/post/20250304/tangkapan-layar-2025-03-04-pukul-130602-bcf3757607919df15775907d72c06307_600x400.png" alt="avatar"/>
                            <div class="flex-grow p-3">
                                <div class="text-xs font-semibold">{ &u.name }</div>
                                <div class="text-xs text-gray-400">{"Hi there!"}</div>
                            </div>
                        </div>
                    }) }
                </div>
                // Area chat utama
                <div class="grow h-screen flex flex-col">
                    // Header dengan link ke profil
                    <div class="w-full h-14 border-b-2 border-gray-300 flex justify-between items-center px-4">
                        <div class="text-xl">{"💬 Chat!"}</div>
                        <Link<Route> to={Route::Profile}>
                            <img
                                src="https://static.vecteezy.com/system/resources/previews/051/558/143/non_2x/shock-cat-meme-sticker-cute-illustration-vector.jpg"
                                alt="Profile"
                                class="w-8 h-8 cursor-pointer hover:opacity-75"
                            />
                        </Link<Route>>
                    </div>
                    // Daftar pesan
                    <div class="w-full grow overflow-auto border-b-2 border-gray-300 px-4">
                        { for self.messages.iter().filter_map(|m| {
                            self.users.iter().find(|u| u.name == m.from).map(|u| html! {
                                <div class="flex items-end w-3/6 bg-gray-100 m-4 rounded-tl-lg rounded-tr-lg rounded-br-lg">
                                    <img class="w-8 h-8 rounded-full m-3" src={u.avatar.clone()} alt="avatar"/>
                                    <div class="p-3">
                                        <div class="text-sm font-medium text-gray-800">{ &m.from }</div>
                                        <div class="text-xs text-gray-500">
                                            {
                                                if m.message.ends_with(".gif") {
                                                    html!{ <img class="mt-3" src={m.message.clone()} /> }
                                                } else {
                                                    html!{ &m.message }
                                                }
                                            }
                                        </div>
                                    </div>
                                </div>
                            })
                        }) }
                    </div>
                    // Input untuk mengirim pesan
                    <div class="w-full h-14 flex px-3 items-center">
                        <input
                            ref={self.chat_input.clone()}
                            type="text"
                            placeholder="Message"
                            class="block w-full py-2 pl-4 mx-3 bg-gray-100 rounded-full outline-none focus:text-gray-700"
                            required=true
                        />
                        <button onclick={submit}
                                class="p-3 shadow-sm bg-blue-600 w-10 h-10 rounded-full flex justify-center items-center text-white hover:bg-blue-500 transition">
                            <svg fill="currentColor" viewBox="0 0 24 24" class="w-6 h-6">
                                <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"/>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}