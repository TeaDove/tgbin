use std::sync::Arc;
use crate::repositories::user_repository::UserRepository;

pub struct UserService{
    user_repository: Arc<UserRepository>
}

impl UserService{
    pub fn new(user_repository: Arc<UserRepository>)->Self{
        Self{user_repository}
    }

    pub async fn save_username(&self, username: String, user_id: u64) -> Result<(), String>{
        match self.user_repository.insert_username(&username, user_id) {
            Ok(_) => {},
            Err(err) => return Err(String::from(format!("failed to insert username: {}", err.to_string()))),
        };

        log::info!("username.saved {}", username);
        Ok(())
    }

    pub async fn get_user_id(&self, username: &String) -> Result<Option<u64>, String>{
        match self.user_repository.get_user_id(username){
            Ok(user_id) => Ok(user_id),
            Err(err) => return Err(String::from(format!("failed to get user: {}", err.to_string()))),
        }
    }
}