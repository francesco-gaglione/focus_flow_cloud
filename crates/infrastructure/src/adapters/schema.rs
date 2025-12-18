// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 7]
        color -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    focus_session (id) {
        id -> Uuid,
        user_id -> Uuid,
        task_id -> Nullable<Uuid>,
        category_id -> Nullable<Uuid>,
        #[max_length = 20]
        session_type -> Varchar,
        actual_duration -> Nullable<Int8>,
        concentration_score -> Nullable<Int4>,
        notes -> Nullable<Text>,
        started_at -> Timestamptz,
        ended_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    tasks (id) {
        id -> Uuid,
        user_id -> Uuid,
        template_id -> Nullable<Uuid>,
        category_id -> Nullable<Uuid>,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        scheduled_date -> Nullable<Date>,
        created_at -> Timestamptz,
        completed_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    user_settings (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        key -> Varchar,
        value -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        hashed_password -> Varchar,
        #[max_length = 20]
        role -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(categories -> users (user_id));
diesel::joinable!(focus_session -> categories (category_id));
diesel::joinable!(focus_session -> tasks (task_id));
diesel::joinable!(focus_session -> users (user_id));
diesel::joinable!(tasks -> categories (category_id));
diesel::joinable!(tasks -> users (user_id));
diesel::joinable!(user_settings -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    focus_session,
    tasks,
    user_settings,
    users,
);
