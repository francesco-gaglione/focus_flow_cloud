use adapters::http::stats::get_stats::GetStatsResponseDto;
use adapters::http::{
    dto::common::session_type_enum::SessionTypeEnum,
    session::create_manual_session::CreateManualSessionDto,
};
use chrono::{Duration, Utc};
use common::setup;

mod common;

#[tokio::test]
async fn test_stats_empty() {
    let ctx = setup().await;

    let response = ctx
        .client
        .get(format!("{}/api/stats", ctx.base_url))
        .send()
        .await
        .expect("Failed to get stats");

    assert_eq!(response.status(), 200);
    let stats: GetStatsResponseDto = response.json().await.unwrap();

    assert_eq!(stats.completed_focus_sessions.count, 0);
}

#[tokio::test]
async fn test_stats_with_sessions() {
    let ctx = setup().await;

    let now = Utc::now();

    // Create two work sessions
    ctx.create_manual_session(&CreateManualSessionDto {
        session_type: SessionTypeEnum::Work,
        started_at: now.timestamp(),
        ended_at: (now + Duration::minutes(25)).timestamp(),
        task_id: None,
        concentration_score: Some(4),
        notes: None,
    })
    .await;

    ctx.create_manual_session(&CreateManualSessionDto {
        session_type: SessionTypeEnum::Work,
        started_at: (now + Duration::minutes(30)).timestamp(),
        ended_at: (now + Duration::minutes(55)).timestamp(),
        task_id: None,
        concentration_score: Some(5),
        notes: None,
    })
    .await;

    // Create a break session
    ctx.create_manual_session(&CreateManualSessionDto {
        session_type: SessionTypeEnum::ShortBreak,
        started_at: (now + Duration::minutes(60)).timestamp(),
        ended_at: (now + Duration::minutes(65)).timestamp(),
        task_id: None,
        concentration_score: None,
        notes: None,
    })
    .await;

    let response = ctx
        .client
        .get(format!("{}/api/stats", ctx.base_url))
        .send()
        .await
        .expect("Failed to get stats");

    assert_eq!(response.status(), 200);
    let stats: GetStatsResponseDto = response.json().await.unwrap();

    assert_eq!(stats.completed_focus_sessions.count, 3);
}
