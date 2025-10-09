// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 7]
        color -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    focus_session (id) {
        id -> Uuid,
        task_id -> Nullable<Uuid>,
        category_id -> Uuid,
        #[max_length = 20]
        session_type -> Varchar,
        planned_duration_minutes -> Int4,
        actual_duration_minutes -> Nullable<Int4>,
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

diesel::joinable!(tasks -> categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(categories, focus_session, tasks,);
