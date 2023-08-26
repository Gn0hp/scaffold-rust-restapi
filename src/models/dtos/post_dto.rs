use crate::models::dtos::IBaseDto;
use crate::models::post::Post;

pub struct PostDto {
    pub post: Post
}
impl PostDto {
    pub fn new(post: Post) -> Self {
        Self {
            post
        }
    }
}
impl IBaseDto<Post> for PostDto {
    fn is_valid(&self) -> bool {
        true
    }

    fn to_json(&self) -> String {
        String::from("PostDto of json")
    }

    fn to_entity(&self) -> Post {
        self.post.clone()
    }
}