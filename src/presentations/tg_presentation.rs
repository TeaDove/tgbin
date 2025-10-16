use std::sync::Arc;
use teloxide::prelude::*;
use crate::services::user_service::UserService;

pub async fn build_and_run(user_service: Arc<UserService>, bot: Bot) {
    let handler = Update::filter_message().endpoint(
        |bot: Bot, user_service: Arc<UserService>, msg: Message| async move {
            let from = match msg.from {
                None => return respond(()),
                Some(x) => x,
            };

            let username = match from.clone().username{
                None => return respond(()),
                Some(x) => x.to_lowercase()
            };

            let res = user_service.save_username(username, from.id.0).await;
            if let Err(x) = res {
                log::error!("failed.to.save.username: {}", x);
            }

            bot.send_message(msg.chat.id, "I have saved your data!").await?;
            respond(())
        },
    );

    log::info!("bot.staring");
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![user_service])
        .build()
        .dispatch()
        .await;
}
