use std::sync::Arc;
use tgbin::presentations::{api_presentation, tg_presentation};
use tgbin::services::user_service::UserService;
use pretty_env_logger;
use redb::Database;
use teloxide::Bot;
use tgbin::repositories::user_repository::UserRepository;
use tgbin::services::paste_service;
use tgbin::settings::settings;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let settings = settings::Settings::must_new();

    let bot = Bot::new(settings.tg_token.trim());

    let db = Database::create(settings.db_path).unwrap();
    let user_service = Arc::new(UserService::new(Arc::new(UserRepository::new(db))));
    let paste_service = Arc::new(paste_service::PasteService::new(user_service.clone(), bot.clone()));

    tokio::join!(
        tg_presentation::build_and_run(user_service, bot),
        api_presentation::build_and_run(paste_service, &settings.url),
    );
}
