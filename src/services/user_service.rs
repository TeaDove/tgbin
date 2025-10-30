use std::sync::Arc;
use crate::repositories::user_repository::UserRepository;

pub struct UserService{
    user_repository: Arc<UserRepository>
}

impl UserService{
    pub fn new(user_repository: Arc<UserRepository>)->Self{
        Self{user_repository}
    }

    pub async fn save_username(&self, username: String, user_id: u64) -> anyhow::Result<()>{
        self.user_repository.insert_username(&username, user_id)?;
        log::info!("username.saved {}", username);
        Ok(())
    }

    pub async fn get_user_id(&self, username: &String) -> anyhow::Result<Option<u64>>{
        self.user_repository.get_user_id(username)
    }
}