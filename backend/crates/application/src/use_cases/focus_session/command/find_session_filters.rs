use domain::entities::focus_session_type::FocusSessionType;
use domain::entities::user::UserId;

#[derive(Debug, Clone)]
pub struct FindSessionFiltersCommand {
    pub user_id: UserId,
    pub date_range: Option<FocusSessionDateFilter>,
    pub category_ids: Option<Vec<String>>,
    pub session_type: Option<FocusSessionType>,
    pub concentration_score_range: Option<ConcentrationScoreFilter>,
    pub has_notes: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct FocusSessionDateFilter {
    pub start_date: i64,
    pub end_date: i64,
}

#[derive(Debug, Clone)]
pub struct ConcentrationScoreFilter {
    pub min: i32,
    pub max: i32,
}
