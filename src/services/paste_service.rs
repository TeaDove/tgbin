use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::services::user_service::UserService;
use teloxide::prelude::*;

#[derive(Clone)]
pub struct PasteService {
    user_service: Arc<UserService>,
    tg_bot: Bot
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasteTextRequest{
    pub text: String,
    pub user: String,
    pub with_code: bool,
}

impl PasteService {
    pub fn new(user_service: Arc<UserService>, tg_bot: Bot) -> Self {
        Self {user_service, tg_bot}
    }

    pub async fn paste_text(&self, req: &PasteTextRequest) -> anyhow::Result<()> {
        let user = &req.user.to_lowercase().trim_start_matches('@').to_string();

        let user_id: u64 = match user.parse(){
            Ok(x) => x,
            Err(_) => match self.user_service.get_user_id(user).await?{
                Some(x) => x,
                None => anyhow::bail!("User does not exist"),
            }
        };

        let texts = PasteService::make_text_msgs(&req.text, req.with_code);
        for text in texts {
            self.tg_bot.send_message(ChatId(user_id as i64), text).parse_mode(teloxide::types::ParseMode::Html).await?;
        }

        log::info!("text.pasted: {:?} {:?}", req.user.len(), user);
        Ok(())
    }

    fn wrap_text(text: &String, with_code: bool) -> String {
        if !with_code{
            return text.clone();
        }

        format!("<code>{}</code>", text)
    }

    fn make_text_msgs(text: &String, with_code: bool) -> Vec<String> {
        let mut text = html_escape::encode_text(&text).to_string();

        const MAX_TG_MSG_SIZE: usize = 4095 - "<code></code>".len();
        let mut msgs = Vec::new();
        loop {
            if let Some((first, second)) = text.clone().split_at_checked(MAX_TG_MSG_SIZE) {
                msgs.push(PasteService::wrap_text(&first.to_string(), with_code));

                text = second.to_string();
            } else {
                msgs.push(PasteService::wrap_text(&text, with_code));
                break;
            }
        };

        msgs
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_make_text_msgs(){
        let res = PasteService::make_text_msgs(&String::from("text").repeat(20), true);
        println!("{:?}", res);
    }
}