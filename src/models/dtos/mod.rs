pub mod post_dto;
pub mod user_dto;

pub trait IBaseDto<T>{
    fn is_valid(&self) -> bool;
    fn to_json(&self) -> String;
    fn to_entity(&self) -> T;
}