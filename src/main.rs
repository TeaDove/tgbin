use std::sync::Arc;
use tgbin::presentations::{api_presentation, tg_presentation};
use tgbin::services::user_service::UserService;
use pretty_env_logger;
use redb::Database;
use tgbin::repositories::user_repository::UserRepository;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let db = Database::create(".data/user.redb").unwrap(); // TODO move to settings
    let user_service = Arc::new(UserService::new(Arc::new(UserRepository::new(db))));

    tokio::join!(
        tg_presentation::build_and_run(user_service),
        api_presentation::build_and_run(),
    );
}
