use crate::models::dtos::IBaseDto;
use crate::models::user::User;

pub struct UserDto {
    pub user: User
}

impl UserDto {
    pub fn new(user: User) -> Self {
        Self {
            user
        }
    }
}

impl IBaseDto<User> for UserDto {
    fn is_valid(&self) -> bool {
        true
    }

    fn to_json(&self) -> String {
        String::from("UserDto of json type")
    }

    fn to_entity(&self) -> User {
        self.user.clone()
    }
}