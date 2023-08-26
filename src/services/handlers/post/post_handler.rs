// use crate::internals::db::mysql::config::establish_connection;
// use diesel::prelude::*;
// use crate::models::post::Post;
//
// pub fn get_all_post() {
//     use crate::schema::posts::dsl::*;
//     let connection = &mut establish_connection();
//     let results = posts
//         .limit(5)
//         .load::<Post>(connection)
//         .expect("Error loading posts");
//     println!("Displaying {} posts", results.len());
//     for post in results {
//         println!("{}", post.title);
//         println!("----------\n");
//     }
// }