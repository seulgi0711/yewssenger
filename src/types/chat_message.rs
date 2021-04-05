#[derive(Clone, Debug)]
pub struct ChatMessage {
  pub id: String,
  pub text: String,
  pub chatroom_id: String,
}
