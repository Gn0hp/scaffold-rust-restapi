// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        #[max_length = 255]
        title -> Varchar,
        content -> Text,
        user_id -> Integer,
        created_at -> Timestamp,
        created_by -> Nullable<Integer>,
        updated_at -> Nullable<Timestamp>,
        updated_by -> Nullable<Integer>,
        deleted_at -> Nullable<Timestamp>,
        deleted_by -> Nullable<Integer>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 250]
        first_name -> Varchar,
        #[max_length = 50]
        last_name -> Varchar,
        age -> Integer,
        #[max_length = 10]
        gender -> Varchar,
        #[max_length = 50]
        email -> Varchar,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
        #[max_length = 255]
        address -> Nullable<Varchar>,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
        created_by -> Nullable<Integer>,
        updated_at -> Nullable<Timestamp>,
        updated_by -> Nullable<Integer>,
        deleted_at -> Nullable<Timestamp>,
        deleted_by -> Nullable<Integer>,
    }
}

diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
