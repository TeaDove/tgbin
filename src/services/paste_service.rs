use std::sync::Arc;
use crate::services::user_service::UserService;
use teloxide::prelude::*;

#[derive(Clone)]
pub struct PasteService {
    user_service: Arc<UserService>,
    tg_bot: Arc<teloxide::Bot>
}

impl PasteService {
    pub fn new(user_service: Arc<UserService>, tg_bot: Arc<teloxide::Bot>) -> Self {
        Self {user_service, tg_bot}
    }

    pub async fn paste_text(self, text: String, user: &String) -> Result<(), String> {
        let user_id: u64 = match user.parse(){
            Ok(x) => x,
            Err(_) => match self.user_service.get_user_id(user).await?{
                Some(x) => x,
                None => return Err(String::from("user not found, please login first")),
            }
        };

        // TODO devide long text
        // TODO add `code`
        match self.tg_bot.send_message(ChatId(user_id as i64), format!("Pasted text: `{}`", text)).await{
            Ok(_) => {},
            Err(err) => return Err(format!("failed to send pasted text: {}", err.to_string())),
        };

        log::info!("text.pasted: {:?} {:?}", text.len(), user);
        Ok(())
    }
}