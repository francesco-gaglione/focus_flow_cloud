use adapters::http::{
    category::create_category::CreateCategoryDto,
    dto::common::session_type_enum::SessionTypeEnum,
    session::create_manual_session::CreateManualSessionDto,
    stats::calculate_stats_by_period::{ConcentrationPeriodDto, GetStatsByPeriodResponseDto},
    task::create_task::CreateTaskDto,
};
use chrono::{Duration, Utc};
use common::setup;

mod common;

#[tokio::test]
async fn test_calculate_stats_scenarios() {
    let ctx = setup().await;

    // 1. Initial Empty Stats
    let now = Utc::now();
    let start_of_day = now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
    let end_of_day = now.date_naive().and_hms_opt(23, 59, 59).unwrap().and_utc();

    let response = ctx
        .client
        .get(format!("{}/api/stats/period", ctx.base_url))
        .query(&[
            ("startDate", start_of_day.timestamp().to_string()),
            ("endDate", end_of_day.timestamp().to_string()),
        ])
        .send()
        .await
        .expect("Failed to get stats");

    assert_eq!(response.status(), 200);
    let stats: GetStatsByPeriodResponseDto = response.json().await.unwrap();

    assert_eq!(stats.total_sessions, 0);
    assert_eq!(stats.total_focus_time, 0);
    assert!(stats.category_distribution.is_empty());

    // 2. Setup Data: Categories and Tasks
    let work_cat = ctx
        .create_category(&CreateCategoryDto {
            name: "Work".to_string(),
            color: Some("#FF0000".to_string()),
            description: None,
        })
        .await;

    let study_cat = ctx
        .create_category(&CreateCategoryDto {
            name: "Study".to_string(),
            color: Some("#00FF00".to_string()),
            description: None,
        })
        .await;

    let coding_task = ctx
        .create_task(&CreateTaskDto {
            name: "Coding".to_string(),
            category_id: Some(work_cat.category_id.clone()),
            description: None,
            scheduled_date: None,
        })
        .await;

    // 3. Create Sessions
    // Session 1: Work, Morning, 25 mins, Work Category, Coding Task, Score 5
    // Use a fixed morning time: 09:00 AM
    let morning_start = start_of_day + Duration::hours(9);
    ctx.create_manual_session(&CreateManualSessionDto {
        session_type: SessionTypeEnum::Work,
        started_at: morning_start.timestamp(),
        ended_at: (morning_start + Duration::minutes(25)).timestamp(),
        category_id: Some(work_cat.category_id.clone()),
        task_id: Some(coding_task.id.clone()),
        concentration_score: Some(5),
        notes: None,
    })
    .await;

    // Session 2: Work, Afternoon, 50 mins, Study Category, No Task, Score 3
    // Use a fixed afternoon time: 14:00 PM (2 PM)
    let afternoon_start = start_of_day + Duration::hours(14);
    ctx.create_manual_session(&CreateManualSessionDto {
        session_type: SessionTypeEnum::Work,
        started_at: afternoon_start.timestamp(),
        ended_at: (afternoon_start + Duration::minutes(50)).timestamp(),
        category_id: Some(study_cat.category_id.clone()),
        task_id: None,
        concentration_score: Some(3),
        notes: None,
    })
    .await;

    // Session 3: Break, Afternoon, 10 mins
    let break_start = afternoon_start + Duration::minutes(50);
    ctx.create_manual_session(&CreateManualSessionDto {
        session_type: SessionTypeEnum::ShortBreak,
        started_at: break_start.timestamp(),
        ended_at: (break_start + Duration::minutes(10)).timestamp(),
        category_id: None,
        task_id: None,
        concentration_score: None,
        notes: None,
    })
    .await;

    // 4. Verify Stats
    let response = ctx
        .client
        .get(format!("{}/api/stats/period", ctx.base_url))
        .query(&[
            ("startDate", start_of_day.timestamp().to_string()),
            ("endDate", end_of_day.timestamp().to_string()),
        ])
        .send()
        .await
        .expect("Failed to get stats");

    let stats: GetStatsByPeriodResponseDto = response.json().await.unwrap();

    // Totals
    assert_eq!(stats.total_sessions, 2); // 2 Work sessions
    assert_eq!(stats.total_breaks, 1); // 1 Break session
    assert_eq!(stats.total_focus_time, 25 * 60 + 50 * 60); // 75 mins in seconds
    assert_eq!(stats.total_break_time, 10 * 60);

    // Concentration
    // Morning: 5 (avg 5.0)
    // Afternoon: 3 (avg 3.0)
    // Most concentrated: Morning (5 >= 3)
    // Least concentrated: Afternoon (5 > 3)
    assert!(matches!(
        stats.most_concentrated_period,
        ConcentrationPeriodDto::Morning
    ));
    assert!(matches!(
        stats.less_concentrated_period,
        ConcentrationPeriodDto::Afternoon
    ));

    // Distribution:
    // Index 0 -> score 1
    // Index 1 -> score 2
    // Index 2 -> score 3
    // Index 3 -> score 4
    // Index 4 -> score 5
    assert_eq!(stats.concentration_distribution[2], 1); // Score 3
    assert_eq!(stats.concentration_distribution[4], 1); // Score 5
    assert_eq!(stats.concentration_distribution[0], 0);

    // Category Distribution
    // Work: 25 mins
    // Study: 50 mins
    // Sorted by total_focus_time desc -> Study first, then Work.
    assert_eq!(stats.category_distribution.len(), 2);
    assert_eq!(stats.category_distribution[0].category_name, "Study");
    assert_eq!(stats.category_distribution[0].total_focus_time, 50 * 60);
    // Percentage: 50 / 75 * 100 = 66.666...
    assert!((stats.category_distribution[0].percentage - 66.66).abs() < 0.1);

    assert_eq!(stats.category_distribution[1].category_name, "Work");
    assert_eq!(stats.category_distribution[1].total_focus_time, 25 * 60);
    // Percentage: 25 / 75 * 100 = 33.333...
    assert!((stats.category_distribution[1].percentage - 33.33).abs() < 0.1);

    // Task Distribution
    // Only sessions with task_id should appear.
    assert_eq!(stats.task_distribution.len(), 1);
    assert_eq!(stats.task_distribution[0].task_name, "Coding");
    assert_eq!(
        stats.task_distribution[0].category_name,
        Some("Work".to_string())
    );
    assert_eq!(stats.task_distribution[0].total_focus_time, 25 * 60);
    // Percentage: 25 / 25 * 100 = 100.0 (calculated relative to total time on tasks)
    assert!((stats.task_distribution[0].percentage - 100.0).abs() < 0.1);
}

#[tokio::test]
async fn test_multi_day_stats_activity() {
    let ctx = setup().await;
    let now = Utc::now();

    // Day 1
    let day1_start = now - Duration::days(2);
    let work_cat = ctx
        .create_category(&CreateCategoryDto {
            name: "Work".to_string(),
            color: Some("#FF0000".to_string()),
            description: None,
        })
        .await;

    ctx.create_manual_session(&CreateManualSessionDto {
        session_type: SessionTypeEnum::Work,
        started_at: day1_start.timestamp(),
        ended_at: (day1_start + Duration::minutes(60)).timestamp(),
        category_id: Some(work_cat.category_id.clone()),
        task_id: None,
        concentration_score: Some(4),
        notes: None,
    })
    .await;

    // Day 2
    let day2_start = now - Duration::days(1);
    ctx.create_manual_session(&CreateManualSessionDto {
        session_type: SessionTypeEnum::Work,
        started_at: day2_start.timestamp(),
        ended_at: (day2_start + Duration::minutes(30)).timestamp(),
        category_id: Some(work_cat.category_id.clone()),
        task_id: None,
        concentration_score: Some(5),
        notes: None,
    })
    .await;

    // Query Period covering both days
    // Ensure we cover enough range
    let start_date = now - Duration::days(3);
    let end_date = now;

    let response = ctx
        .client
        .get(format!("{}/api/stats/period", ctx.base_url))
        .query(&[
            ("startDate", start_date.timestamp().to_string()),
            ("endDate", end_date.timestamp().to_string()),
        ])
        .send()
        .await
        .expect("Failed to get stats");

    let stats: GetStatsByPeriodResponseDto = response.json().await.unwrap();

    assert_eq!(stats.total_sessions, 2);
    assert_eq!(stats.total_focus_time, 90 * 60);

    // Daily Activity
    // Should have 2 entries
    assert_eq!(stats.daily_activity.len(), 2);

    let da1 = &stats.daily_activity[0];
    let da2 = &stats.daily_activity[1];

    // Verify dates
    let day1_date_ts = day1_start
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp();
    assert_eq!(da1.date, day1_date_ts);
    assert_eq!(da1.category_distribution[0].total_focus_time, 60 * 60);

    let day2_date_ts = day2_start
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp();
    assert_eq!(da2.date, day2_date_ts);
    assert_eq!(da2.category_distribution[0].total_focus_time, 30 * 60);
}

#[tokio::test]
async fn test_stats_filtering() {
    let ctx = setup().await;
    let now = Utc::now();

    // Session 1: Today
    ctx.create_manual_session(&CreateManualSessionDto {
        session_type: SessionTypeEnum::Work,
        started_at: now.timestamp(),
        ended_at: (now + Duration::minutes(30)).timestamp(),
        category_id: None,
        task_id: None,
        concentration_score: None,
        notes: None,
    })
    .await;

    // Session 2: Last Week (outside range)
    let last_week = now - Duration::days(7);
    ctx.create_manual_session(&CreateManualSessionDto {
        session_type: SessionTypeEnum::Work,
        started_at: last_week.timestamp(),
        ended_at: (last_week + Duration::minutes(30)).timestamp(),
        category_id: None,
        task_id: None,
        concentration_score: None,
        notes: None,
    })
    .await;

    // Query Today
    let start_of_day = now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
    let end_of_day = now.date_naive().and_hms_opt(23, 59, 59).unwrap().and_utc();

    let response = ctx
        .client
        .get(format!("{}/api/stats/period", ctx.base_url))
        .query(&[
            ("startDate", start_of_day.timestamp().to_string()),
            ("endDate", end_of_day.timestamp().to_string()),
        ])
        .send()
        .await
        .expect("Failed to get stats");

    let stats: GetStatsByPeriodResponseDto = response.json().await.unwrap();
    assert_eq!(stats.total_sessions, 1);
    assert_eq!(stats.total_focus_time, 30 * 60);
}
