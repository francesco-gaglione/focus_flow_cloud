use uuid::Uuid;

pub struct StatsPeriod {
    pub user_id: Uuid,
    pub start_date: i64,
    pub end_date: Option<i64>,
}
