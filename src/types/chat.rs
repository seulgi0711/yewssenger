#[derive(Clone, Debug)]
pub enum ChatType {
  Private,
  Direct,
  Group,
}

#[derive(Clone, Debug)]
pub struct Chat {
  pub id: String,
  pub title: String,
  pub members: Vec<String>,
  pub r#type: ChatType,
}
