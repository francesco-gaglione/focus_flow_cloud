use tracing::debug;

use crate::adapters::http::{app_state::AppState, dto::ws_msg::note_update_ws::NoteUpdate};

//TODO HO CAMBIATO IDEA:
// questo metodo deve essere quello che chiude completamente la sessione e quindi cancella anche tutto dallo stato. Quindi il flusso é
// - avvia la session
// - finisce i primi 25 minuti avvia una nuova sessione (ma di pausa, quindi la logica che sta qui sotto va spostata li)
// - fa vari avvii di sessioni (con logica controllata da be/ws)
// - quando clicca su interrompi allora chiama questo metodo qui sotto, che salva quello che deve salvare se c'é qualcosa da salvare e pulisce tutto lo stato
pub async fn note_update(message: &NoteUpdate, state: &AppState) -> Result<(), String> {
    debug!("Starting session");

    let mut state = state.focus_session_state.write().await;

    match &mut state.current_session {
        Some(running_session) => {
            running_session.note = Some(message.new_note.clone());

            Ok(())
        }
        None => Err("No running sessions".to_string()),
    }
}
