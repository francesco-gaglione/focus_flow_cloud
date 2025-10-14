use tracing::debug;

use crate::adapters::http::app_state::AppState;

//TODO HO CAMBIATO IDEA:
// questo metodo deve essere quello che chiude completamente la sessione e quindi cancella anche tutto dallo stato. Quindi il flusso é
// - avvia la session
// - finisce i primi 25 minuti avvia una nuova sessione (ma di pausa, quindi la logica che sta qui sotto va spostata li)
// - fa vari avvii di sessioni (con logica controllata da be/ws)
// - quando clicca su interrompi allora chiama questo metodo qui sotto, che salva quello che deve salvare se c'é qualcosa da salvare e pulisce tutto lo stato
pub async fn end_session(state: &AppState) -> Result<(), String> {
    debug!("Starting session");

    let mut state = state.focus_session_state.write().await;

    match &state.current_session {
        Some(running_session) => {
            let cloned = running_session.clone();
            //TODO persist current_session state into the database

            state.current_session = None;
            state.consecutive_sessions = Vec::new();

            Ok(())
        }
        None => Err("No running sessions".to_string()),
    }
}
