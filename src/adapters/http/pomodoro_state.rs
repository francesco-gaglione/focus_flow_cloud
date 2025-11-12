use crate::adapters::http::dto::common::session_type_enum::SessionTypeEnum;

#[derive(Debug, Default, Clone)]
pub struct PomodoroState {
    context: WorkContext,
    current_session: Option<FocusSessionState>,
    consecutive_sessions: Vec<FocusSessionState>,
}

#[derive(Debug, Default, Clone)]
pub struct WorkContext {
    category_id: Option<String>,
    task_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FocusSessionState {
    session_type: SessionTypeEnum,
    start_date: i64,
    end_date: Option<i64>,
    category_id: Option<String>,
    task_id: Option<String>,
    note: Option<String>,
    concentration_score: Option<i32>,
}

const WORK_SESSIONS_BEFORE_LONG_BREAK: usize = 4;

impl PomodoroState {
    pub fn current_session(&self) -> Option<&FocusSessionState> {
        self.current_session.as_ref()
    }

    pub fn current_work_context(&self) -> WorkContext {
        self.context.clone()
    }

    pub fn update_work_context(&mut self, category_id: Option<String>, task_id: Option<String>) {
        self.context.update(category_id, task_id);
    }

    pub fn current_session_type(&self) -> Option<SessionTypeEnum> {
        self.current_session
            .as_ref()
            .map(|session| session.session_type.clone())
    }

    pub fn update_current_session_note(&mut self, new_note: String) -> Result<(), String> {
        if let Some(session) = self.current_session.as_mut() {
            session.update_note(new_note);
            Ok(())
        } else {
            Err("No running sessions".to_string())
        }
    }

    pub fn update_current_session_concentration_score(
        &mut self,
        new_score: i32,
    ) -> Result<(), String> {
        if let Some(session) = self.current_session.as_mut() {
            session.update_concentration_score(new_score);
            Ok(())
        } else {
            Err("No running sessions".to_string())
        }
    }

    pub fn close_current_session(&mut self, end_time: i64) -> Result<(), String> {
        if let Some(mut session) = self.current_session.take() {
            session.end_session(end_time);
            self.consecutive_sessions.push(session);
            self.current_session = None;
            Ok(())
        } else {
            Err("No active sessions, cannot close".to_string())
        }
    }

    pub fn last_session(&self) -> Option<FocusSessionState> {
        self.consecutive_sessions.last().cloned()
    }

    pub fn start_new_session(
        &mut self,
        session_type: SessionTypeEnum,
        start_time: i64,
        category_id: Option<String>,
        task_id: Option<String>,
    ) -> Result<(), String> {
        let new_session = FocusSessionState::new(session_type, start_time, category_id, task_id);
        self.current_session = Some(new_session);
        Ok(())
    }

    /// Calculates the next session type based on Pomodoro technique rules:
    /// - After completing a Work session: ShortBreak or LongBreak
    /// - After completing a Break session: Work
    /// - LongBreak is suggested after 4 completed Work sessions
    pub fn calculate_next_session_type(&self) -> SessionTypeEnum {
        if self.consecutive_sessions.is_empty() {
            tracing::warn!("No previous session found, returning a work session fallback");
            return SessionTypeEnum::Work;
        }

        let last_session_type = match self.consecutive_sessions.last() {
            Some(last_session) => last_session.session_type(),
            None => {
                tracing::warn!("No previous session found, returning a work session fallback");
                return SessionTypeEnum::Work;
            }
        };

        match last_session_type {
            SessionTypeEnum::Work => {
                let completed_work_sessions = self
                    .consecutive_sessions
                    .iter()
                    .filter(|s| s.session_type == SessionTypeEnum::Work)
                    .count();

                let next_type = if completed_work_sessions % WORK_SESSIONS_BEFORE_LONG_BREAK == 0 {
                    SessionTypeEnum::LongBreak
                } else {
                    SessionTypeEnum::ShortBreak
                };

                tracing::debug!(
                    "After work session #{}, suggesting: {:?}",
                    completed_work_sessions,
                    next_type
                );

                next_type
            }
            SessionTypeEnum::ShortBreak | SessionTypeEnum::LongBreak => {
                tracing::debug!("After break, suggesting work session");
                SessionTypeEnum::Work
            }
        }
    }
}

impl FocusSessionState {
    pub fn new(
        session_type: SessionTypeEnum,
        start_time: i64,
        category_id: Option<String>,
        task_id: Option<String>,
    ) -> Self {
        Self {
            session_type,
            start_date: start_time,
            end_date: None,
            category_id,
            task_id,
            note: None,
            concentration_score: None,
        }
    }

    pub fn session_type(&self) -> &SessionTypeEnum {
        &self.session_type
    }

    pub fn task_id(&self) -> Option<&String> {
        self.task_id.as_ref()
    }

    pub fn category_id(&self) -> Option<&String> {
        self.category_id.as_ref()
    }

    pub fn start_date(&self) -> i64 {
        self.start_date
    }

    pub fn end_date(&self) -> Option<i64> {
        self.end_date
    }

    pub fn concentration_score(&self) -> Option<i32> {
        self.concentration_score
    }

    pub fn update_concentration_score(&mut self, new_score: i32) {
        self.concentration_score = Some(new_score);
    }

    pub fn note(&self) -> Option<String> {
        self.note.clone()
    }

    pub fn update_note(&mut self, new_note: String) {
        self.note = Some(new_note);
    }

    pub fn actual_duration(&self) -> Option<i64> {
        self.end_date.map(|end| end - self.start_date)
    }

    pub fn is_work_session(&self) -> bool {
        self.session_type == SessionTypeEnum::Work
    }

    pub fn end_session(&mut self, end_time: i64) {
        self.end_date = Some(end_time);
    }
}

impl WorkContext {
    pub fn new(category_id: Option<String>, task_id: Option<String>) -> Self {
        Self {
            category_id,
            task_id,
        }
    }

    pub fn category_id(&self) -> Option<&String> {
        self.category_id.as_ref()
    }

    pub fn task_id(&self) -> Option<&String> {
        self.task_id.as_ref()
    }

    pub fn update(&mut self, category_id: Option<String>, task_id: Option<String>) {
        self.category_id = category_id;
        self.task_id = task_id;
    }
}
