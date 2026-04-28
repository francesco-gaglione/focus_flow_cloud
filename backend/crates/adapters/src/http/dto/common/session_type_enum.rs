pub use shared::focus_session::SessionTypeEnum;

use application::use_cases::pomodoro_state::fetch_user_pomodoro_state::UserSessionType;
use domain::entities::focus_session_type::FocusSessionType;

pub fn domain_to_enum(v: FocusSessionType) -> SessionTypeEnum {
    match v {
        FocusSessionType::Work => SessionTypeEnum::Work,
        FocusSessionType::ShortBreak => SessionTypeEnum::ShortBreak,
        FocusSessionType::LongBreak => SessionTypeEnum::LongBreak,
    }
}

pub fn enum_to_domain(v: SessionTypeEnum) -> FocusSessionType {
    match v {
        SessionTypeEnum::Work => FocusSessionType::Work,
        SessionTypeEnum::ShortBreak => FocusSessionType::ShortBreak,
        SessionTypeEnum::LongBreak => FocusSessionType::LongBreak,
    }
}

pub fn app_type_to_enum(v: UserSessionType) -> SessionTypeEnum {
    match v {
        UserSessionType::Work => SessionTypeEnum::Work,
        UserSessionType::ShortBreak => SessionTypeEnum::ShortBreak,
        UserSessionType::LongBreak => SessionTypeEnum::LongBreak,
    }
}
