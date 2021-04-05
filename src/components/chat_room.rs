use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};

use crate::types::ChatMessage;

pub struct ChatRoom {
  messages: Vec<ChatMessage>,
}

pub enum Msg {}

impl Component for ChatRoom {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    ChatRoom {
      messages: vec![
        ChatMessage {
          id: String::from(""),
          chatroom_id: String::from(""),
          text: String::from("메시지1"),
        },
        ChatMessage {
          id: String::from(""),
          chatroom_id: String::from(""),
          text: String::from("메시지2"),
        },
        ChatMessage {
          id: String::from(""),
          chatroom_id: String::from(""),
          text: String::from("메시지3"),
        },
      ],
    }
  }
  fn update(&mut self, _: Self::Message) -> ShouldRender {
    todo!()
  }
  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    todo!()
  }
  fn view(&self) -> Html {
    html! {
      <div>{for self.messages.iter().map(|message| self.render_message(&message))}</div>
    }
  }
}

impl ChatRoom {
  fn render_message(&self, message: &ChatMessage) -> Html {
    html! {
      <div>{&message.text}</div>
    }
  }
}
