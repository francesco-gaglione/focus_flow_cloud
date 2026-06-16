use application::shared::use_cases::push_subscription::save_push_subscription::SavePushSubscriptionUseCase;
use application::shared::use_cases::reminder::get_pending_reminders::GetPendingRemindersUseCase;
use std::sync::Arc;

#[derive(Clone)]
pub struct SharedState {
    pub save_push_subscription_uc: Arc<SavePushSubscriptionUseCase>,
    pub get_pending_reminders_uc: Arc<GetPendingRemindersUseCase>,
}
