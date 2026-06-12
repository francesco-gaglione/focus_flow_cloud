// @generated automatically by Diesel CLI.

diesel::table! {
    flashcard_folder_items (flashcard_id, folder_id) {
        flashcard_id -> Uuid,
        folder_id -> Uuid,
    }
}

diesel::table! {
    flashcard_folders (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        parent_id -> Nullable<Uuid>,
        path -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    flashcards (id) {
        id -> Uuid,
        user_id -> Uuid,
        front -> Text,
        back -> Text,
        stability -> Float4,
        difficulty -> Float4,
        due_date -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    _sqlx_migrations (version) {
        version -> Int8,
        description -> Text,
        installed_on -> Timestamptz,
        success -> Bool,
        checksum -> Bytea,
        execution_time -> Int8,
    }
}

diesel::table! {
    categories (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
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
    push_subscriptions (id) {
        id -> Uuid,
        user_id -> Uuid,
        endpoint -> Text,
        p256dh -> Text,
        auth -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    reminders (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        date -> Timestamptz,
        reminder_sent -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        task_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    subtasks (id) {
        id -> Uuid,
        task_id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        completed_at -> Nullable<Timestamptz>,
        sort_order -> Int2,
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
        title -> Varchar,
        description -> Nullable<Text>,
        scheduled_date -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        completed_at -> Nullable<Timestamptz>,
        #[max_length = 10]
        priority -> Nullable<Varchar>,
        deleted_at -> Nullable<Timestamptz>,
        #[max_length = 12]
        schedule_type -> Varchar,
        schedule_all_day_date -> Nullable<Date>,
        schedule_duration_secs -> Nullable<Int8>,
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

diesel::joinable!(flashcard_folder_items -> flashcards (flashcard_id));
diesel::joinable!(flashcard_folder_items -> flashcard_folders (folder_id));
diesel::joinable!(flashcard_folders -> users (user_id));
diesel::joinable!(flashcards -> users (user_id));
diesel::joinable!(categories -> users (user_id));
diesel::joinable!(focus_session -> tasks (task_id));
diesel::joinable!(focus_session -> users (user_id));
diesel::joinable!(push_subscriptions -> users (user_id));
diesel::joinable!(reminders -> tasks (task_id));
diesel::joinable!(reminders -> users (user_id));
diesel::joinable!(subtasks -> tasks (task_id));
diesel::joinable!(subtasks -> users (user_id));
diesel::joinable!(tasks -> categories (category_id));
diesel::joinable!(tasks -> users (user_id));
diesel::joinable!(user_settings -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    _sqlx_migrations,
    categories,
    flashcard_folder_items,
    flashcard_folders,
    flashcards,
    focus_session,
    push_subscriptions,
    reminders,
    subtasks,
    tasks,
    user_settings,
    users,
);
