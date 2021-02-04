use crate::components::chat_icon::ChatIcon;
use crate::components::user_icon::UserIcon;
use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};

use crate::types::User;
use crate::types::{Chat, ChatType};

pub struct Nav {
  user: User,
  chats: Vec<Chat>,
}

pub enum Msg {}

impl Component for Nav {
  type Message = Msg;
  type Properties = ();
  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Nav {
      user: User {
        id: String::from("#2339"),
        name: String::from("낙타"),
        icon: String::from("https://placeimg.com/40/40/puppy"),
      },
      chats: vec![
        Chat {
          id: String::from("chatId1"),
          icon: String::from("https://placeimg.com/40/40/puppy"),
          members: vec![String::from("member1"), String::from("member2")],
          title: String::from("대화방 제목"),
          r#type: ChatType::Direct,
        },
        Chat {
          id: String::from("chatId2"),
          icon: String::from("https://placeimg.com/40/40/puppy"),
          members: vec![String::from("member1"), String::from("member2")],
          title: String::from("대화방 제목22"),
          r#type: ChatType::Direct,
        },
        Chat {
          id: String::from("chatId3"),
          icon: String::from("https://placeimg.com/40/40/puppy"),
          members: vec![String::from("member1"), String::from("member2")],
          title: String::from("대화방 제목22"),
          r#type: ChatType::Direct,
        },
      ],
    }
  }
  fn update(&mut self, _: <Self as yew::Component>::Message) -> ShouldRender {
    todo!()
  }
  fn change(&mut self, _: <Self as yew::Component>::Properties) -> ShouldRender {
    todo!()
  }
  fn view(&self) -> yew::virtual_dom::VNode {
    html! {
      <nav class="flex-none flex flex-col w-60 bg-gray-800 text-gray-400">
        <div class="flex-none px-2 py-2 border-b border-gray-900">
          <input class="bg-gray-900 outline-none p-1 px-2 rounded w-full" placeholder="대화 찾기 또는 시작하기"/>
        </div>
        <div class="flex-1 px-2">{for self.chats.iter().map(|chat| self.render_chat(&chat))}</div>
        <div class="flex-none">
          <div class="flex items-center w-full p-3 bg-gray-700">
            <UserIcon user=&self.user/>
            <div class="flex flex-col pl-2">
              <div class="flex text-gray-50">{&self.user.name}</div>
              <div class="flex text-sm text-gray-100">{&self.user.id}</div>
            </div>
          </div>
        </div>
      </nav>
    }
  }
}

impl Nav {
  fn render_chat(&self, chat: &Chat) -> Html {
    html! {
      <div class="flex items-center py-2">
        <span>
          <ChatIcon chat=chat/>
        </span>
        <span class="pl-2">{&chat.title}</span>
      </div>
    }
  }
}
