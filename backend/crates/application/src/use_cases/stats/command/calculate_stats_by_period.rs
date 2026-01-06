use domain::entities::user::UserId;

pub struct StatsPeriod {
    pub user_id: UserId,
    pub start_date: i64,
    pub end_date: Option<i64>,
}
