use std::sync::Arc;
use tokio::task;
use tgbin::presentations::tg_presentation;
use tgbin::services::user_service::UserService;
use pretty_env_logger;
use redb::Database;
use tgbin::repositories::user_repository::UserRepository;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let db = Database::create(".data/user.redb").unwrap();
    let user_service = Arc::new(UserService::new(Arc::new(UserRepository::new(db))));

    let tg_handler = task::spawn(async move {
        tg_presentation::build_and_run(user_service).await;
    });

    tg_handler.await.unwrap();
}
