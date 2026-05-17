use std::time::Duration;

use chrono::Utc;
use dioxus::prelude::*;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use crate::{
    components::button::{Button, ButtonVariant},
    state::{app_state::AppState, auth_state::AuthState},
};

#[derive(Debug, Clone, Deserialize, PartialEq)]
enum WsSessionType {
    Work,
    ShortBreak,
    LongBreak,
}

impl WsSessionType {
    fn label(&self) -> &'static str {
        match self {
            WsSessionType::Work => "Focus",
            WsSessionType::ShortBreak => "Short Break",
            WsSessionType::LongBreak => "Long Break",
        }
    }
    fn target_secs(&self) -> i64 {
        match self {
            WsSessionType::Work => 25 * 60,
            WsSessionType::ShortBreak => 5 * 60,
            WsSessionType::LongBreak => 15 * 60,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WsCurrentSession {
    session_type: WsSessionType,
    session_start_time: i64,
    note: Option<String>,
    concentration_score: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct PomodoroWsState {
    current_session: Option<WsCurrentSession>,
    task_id: Option<String>,
}

enum WsCmd {
    Start,
    TakeBreak,
    Terminate,
    UpdateNote(String),
    UpdateScore(i32),
}

fn fmt_time(secs: i64) -> String {
    let secs = secs.max(0);
    format!("{:02}:{:02}", secs / 60, secs % 60)
}

const CIRCUMFERENCE: f64 = 552.92; // 2π * 88

// Outbound WS message payloads

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WsContextPayload {
    task_id: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WsNotePayload {
    new_note: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WsScorePayload {
    concentration_score: i32,
}

fn ws_msg(val: serde_json::Value) -> Message {
    Message::Text(val.to_string())
}

#[component]
pub fn Pomodoro() -> Element {
    let auth_state = use_context::<Signal<AuthState>>();
    let app_state_ctx = use_context::<Signal<AppState>>();
    let mut selected_task = use_context::<Signal<Option<(String, String)>>>();

    let mut pomo_state: Signal<Option<PomodoroWsState>> = use_signal(|| None);
    let mut ws_connected = use_signal(|| false);
    let mut ws_error: Signal<Option<String>> = use_signal(|| None);
    let mut ws_tx: Signal<Option<tokio::sync::mpsc::UnboundedSender<WsCmd>>> = use_signal(|| None);
    let mut note_input = use_signal(String::new);
    let mut tick = use_signal(|| 0u64);

    // 1-second tick → re-render for live timer
    let _resource = use_resource(move || async move {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            *tick.write() += 1;
        }
    });

    // WS connection — run once on mount via guard signal
    let mut ws_initialized = use_signal(|| false);
    use_effect(move || {
        if *ws_initialized.read() {
            return;
        }
        ws_initialized.set(true);

        let token = auth_state.read().auth_token().map(|t| t.to_string());
        let server_url = app_state_ctx
            .read()
            .server_url()
            .unwrap_or("http://192.168.1.135:8080") //TODO improve
            .to_string();
        let preset_task = selected_task.read().clone();

        if let Some(token) = token {
            let ws_url = server_url
                .replacen("https://", "wss://", 1)
                .replacen("http://", "ws://", 1);
            let ws_url = format!("{}/ws/session?token={}", ws_url, token);

            info!("[pomodoro] connecting to {}", ws_url);

            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<WsCmd>();
            ws_tx.set(Some(tx));

            spawn(async move {
                match connect_async(&ws_url).await {
                    Ok((stream, _)) => {
                        info!("[pomodoro] ws connected to {}", ws_url);
                        ws_connected.set(true);
                        let (mut write, mut read) = stream.split();

                        let _ = write
                            .send(ws_msg(serde_json::json!({"requestSync": null})))
                            .await;
                        info!("[pomodoro] sent requestSync");

                        if let Some((ref task_id, ref task_title)) = preset_task {
                            info!(
                                "[pomodoro] setting preset task context: {} ({})",
                                task_title, task_id
                            );
                            let payload = WsContextPayload {
                                task_id: Some(task_id.clone()),
                            };
                            let _ = write
                                .send(ws_msg(
                                    serde_json::json!({"updatePomodoroContext": payload}),
                                ))
                                .await;
                        }

                        loop {
                            tokio::select! {
                                cmd = rx.recv() => match cmd {
                                    Some(WsCmd::Start) => {
                                        info!("[pomodoro] -> startEvent");
                                        let _ = write.send(ws_msg(serde_json::json!({"startEvent": null}))).await;
                                    }
                                    Some(WsCmd::TakeBreak) => {
                                        info!("[pomodoro] -> breakEvent");
                                        let _ = write.send(ws_msg(serde_json::json!({"breakEvent": null}))).await;
                                    }
                                    Some(WsCmd::Terminate) => {
                                        info!("[pomodoro] -> terminateEvent");
                                        let _ = write.send(ws_msg(serde_json::json!({"terminateEvent": null}))).await;
                                        *selected_task.write() = None;
                                    }
                                    Some(WsCmd::UpdateNote(ref note)) => {
                                        info!("[pomodoro] -> updateNote: \"{}\"", note);
                                        let payload = WsNotePayload { new_note: note.clone() };
                                        let _ = write.send(ws_msg(serde_json::json!({"updateNote": payload}))).await;
                                    }
                                    Some(WsCmd::UpdateScore(score)) => {
                                        info!("[pomodoro] -> updateConcentrationScore: {}", score);
                                        let payload = WsScorePayload { concentration_score: score };
                                        let _ = write.send(ws_msg(serde_json::json!({"updateConcentrationScore": payload}))).await;
                                    }
                                    None => {
                                        info!("[pomodoro] command channel closed, disconnecting");
                                        break;
                                    },
                                },
                                msg = read.next() => match msg {
                                    Some(Ok(Message::Text(ref text))) => {
                                        info!("[pomodoro] <- raw: {}", text);
                                        if let Ok(val) = serde_json::from_str::<serde_json::Value>(text) {
                                            let state_val = val.get("syncData")
                                                .or_else(|| val.get("pomodoroSessionUpdate"));
                                            if let Some(sv) = state_val {
                                                match serde_json::from_value::<PomodoroWsState>(sv.clone()) {
                                                    Ok(s) => {
                                                        let session_info = s.current_session.as_ref()
                                                            .map(|c| format!("{:?} started_at={}", c.session_type, c.session_start_time))
                                                            .unwrap_or_else(|| "no active session".to_string());
                                                        info!("[pomodoro] state update: {}", session_info);
                                                        pomo_state.set(Some(s));
                                                    }
                                                    Err(e) => {
                                                        error!("[pomodoro] failed to parse PomodoroWsState: {} — raw: {}", e, sv);
                                                    }
                                                }
                                            } else {
                                                info!("[pomodoro] <- non-state message: {}", val);
                                            }
                                        } else {
                                            error!("[pomodoro] failed to parse JSON: {}", text);
                                        }
                                    }
                                    Some(Ok(Message::Close(frame))) => {
                                        info!("[pomodoro] ws closed by server: {:?}", frame);
                                        break;
                                    }
                                    None => {
                                        info!("[pomodoro] ws stream ended");
                                        break;
                                    }
                                    Some(Err(e)) => {
                                        error!("[pomodoro] ws error: {}", e);
                                        ws_error.set(Some(e.to_string()));
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                        }

                        info!("[pomodoro] ws loop exited, disconnected");
                        ws_connected.set(false);
                    }
                    Err(e) => {
                        error!("[pomodoro] ws connect failed ({}): {}", ws_url, e);
                        ws_error.set(Some(format!("{}", e)));
                    }
                }
            });
        } else {
            error!("[pomodoro] no auth token, cannot connect");
            ws_error.set(Some("Not authenticated".into()));
        }
    });

    // Derived state for rendering
    let _ = *tick.read(); // subscribe to ticks
    let state = pomo_state.read().clone();
    let session = state
        .as_ref()
        .and_then(|s| s.current_session.as_ref())
        .cloned();
    let task_title = selected_task.read().as_ref().map(|(_, t)| t.clone());
    let connected = *ws_connected.read();
    let error = ws_error.read().clone();

    let has_session = session.is_some();
    let is_work = session
        .as_ref()
        .map(|s| s.session_type == WsSessionType::Work)
        .unwrap_or(false);
    let session_label = session
        .as_ref()
        .map(|s| s.session_type.label())
        .unwrap_or("Focus");
    let existing_score = session.as_ref().and_then(|s| s.concentration_score);
    let existing_note = session
        .as_ref()
        .and_then(|s| s.note.clone())
        .unwrap_or_default();

    let (remaining, progress) = if let Some(ref sess) = session {
        let now = Utc::now().timestamp();
        let elapsed = (now - sess.session_start_time).max(0);
        let target = sess.session_type.target_secs();
        let remaining = (target - elapsed).max(0);
        let progress = 1.0 - (remaining as f64 / target as f64).min(1.0);
        (remaining, progress)
    } else {
        (25 * 60, 0.0)
    };

    let dash_offset = format!("{:.1}", CIRCUMFERENCE * (1.0 - progress));

    rsx! {
        div { class: "pomo-page",
            // Connection + task context bar
            div { class: "pomo-header",
                if let Some(title) = &task_title {
                    div { class: "pomo-task-chip",
                        div { class: "pomo-task-dot" }
                        span { "{title}" }
                    }
                } else {
                    div { class: "pomo-task-chip pomo-task-chip-empty",
                        span { "No task selected" }
                    }
                }
                div {
                    class: if connected { "pomo-conn-dot connected" } else { "pomo-conn-dot" },
                    title: if connected { "Connected" } else { "Disconnected" }
                }
            }

            // Session type label
            div { class: "pomo-session-type",
                span { class: "pomo-session-label", "{session_label}" }
            }

            // Timer ring
            div { class: "pomo-ring-wrap",
                svg {
                    class: "pomo-ring",
                    view_box: "0 0 200 200",
                    width: "200",
                    height: "200",
                    circle {
                        class: "pomo-ring-track",
                        cx: "100", cy: "100", r: "88",
                        fill: "none",
                    }
                    circle {
                        class: "pomo-ring-progress",
                        cx: "100", cy: "100", r: "88",
                        fill: "none",
                        stroke_dasharray: "{CIRCUMFERENCE}",
                        stroke_dashoffset: "{dash_offset}",
                        transform: "rotate(-90 100 100)",
                    }
                }
                div { class: "pomo-time-display",
                    span { class: "pomo-time", "{fmt_time(remaining)}" }
                    if has_session {
                        span { class: "pomo-time-sub", "remaining" }
                    } else {
                        span { class: "pomo-time-sub", "ready" }
                    }
                }
            }

            // Controls
            div { class: "pomo-controls",
                if !is_work {
                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: move |_| {
                            if let Some(tx) = ws_tx.read().as_ref() {
                                let _ = tx.send(WsCmd::Start);
                            }
                        },
                        svg { view_box: "0 0 16 16", width: "18", height: "18",
                            polygon { points: "5 3 13 8 5 13", fill: "currentColor" }
                        }
                        if has_session { "Resume" } else { "Start" }
                    }
                }
                if is_work {
                    Button {
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| {
                            if let Some(tx) = ws_tx.read().as_ref() {
                                let _ = tx.send(WsCmd::TakeBreak);
                            }
                        },
                        svg { view_box: "0 0 16 16", width: "18", height: "18",
                            rect { x: "3", y: "4", width: "3", height: "8", fill: "currentColor" }
                            rect { x: "10", y: "4", width: "3", height: "8", fill: "currentColor" }
                        }
                        "Break"
                    }
                }
                if has_session {
                    Button {
                        variant: ButtonVariant::Destructive,
                        onclick: move |_| {
                            if let Some(tx) = ws_tx.read().as_ref() {
                                let _ = tx.send(WsCmd::Terminate);
                            }
                        },
                        svg { view_box: "0 0 16 16", width: "18", height: "18",
                            rect { x: "3", y: "3", width: "10", height: "10", rx: "1", fill: "currentColor" }
                        }
                        "Stop"
                    }
                }
            }

            // Note + score (only during work session)
            if has_session && is_work {
                div { class: "pomo-extras",
                    // Concentration score
                    div { class: "pomo-score-row",
                        span { class: "pomo-extras-label", "Focus score" }
                        div { class: "pomo-stars",
                            for i in 1..=5 {
                                button {
                                    class: if existing_score.map(|s| i <= s).unwrap_or(false) {
                                        "pomo-star active"
                                    } else {
                                        "pomo-star"
                                    },
                                    onclick: move |_| {
                                        if let Some(tx) = ws_tx.read().as_ref() {
                                            let _ = tx.send(WsCmd::UpdateScore(i));
                                        }
                                    },
                                    "★"
                                }
                            }
                        }
                    }
                    // Note input
                    div { class: "pomo-note-row",
                        span { class: "pomo-extras-label", "Note" }
                        div { class: "pomo-note-input-wrap",
                            input {
                                class: "pomo-note-input",
                                placeholder: if existing_note.is_empty() { "Add a note…" } else { existing_note.as_str() },
                                value: "{note_input}",
                                oninput: move |e| note_input.set(e.value()),
                                onkeydown: move |e| {
                                    if e.key() == Key::Enter {
                                        e.prevent_default();
                                        let val = note_input.read().trim().to_string();
                                        if !val.is_empty() {
                                            if let Some(tx) = ws_tx.read().as_ref() {
                                                let _ = tx.send(WsCmd::UpdateNote(val.clone()));
                                            }
                                            note_input.set(String::new());
                                        }
                                    }
                                },
                            }
                            button {
                                class: "pomo-note-send",
                                onclick: move |_| {
                                    let val = note_input.read().trim().to_string();
                                    if !val.is_empty() {
                                        if let Some(tx) = ws_tx.read().as_ref() {
                                            let _ = tx.send(WsCmd::UpdateNote(val.clone()));
                                        }
                                        note_input.set(String::new());
                                    }
                                },
                                svg { view_box: "0 0 16 16", width: "14", height: "14",
                                    line { x1: "2", y1: "8", x2: "14", y2: "8", stroke: "currentColor", stroke_width: "1.6", stroke_linecap: "round" }
                                    polyline { points: "9 3 14 8 9 13", stroke: "currentColor", stroke_width: "1.6", fill: "none", stroke_linecap: "round", stroke_linejoin: "round" }
                                }
                            }
                        }
                    }
                }
            }

            if let Some(ref err) = error {
                div { class: "pomo-error", "Connection error: {err}" }
            }
        }
    }
}
