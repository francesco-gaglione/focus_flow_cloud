use adapters::shared::http::app_state::shared_state::SharedState;
use adapters::shared::persistence::PostgresPersistence;
use application::shared::use_cases::push_subscription::save_push_subscription::SavePushSubscriptionUseCase;
use application::shared::use_cases::reminder::get_pending_reminders::GetPendingRemindersUseCase;
use std::sync::Arc;

pub fn build_shared_state(postgres: Arc<PostgresPersistence>) -> SharedState {
    SharedState {
        save_push_subscription_uc: Arc::new(SavePushSubscriptionUseCase::new(postgres.clone())),
        get_pending_reminders_uc: Arc::new(GetPendingRemindersUseCase::new(postgres.clone())),
    }
}
