import 'package:freezed_annotation/freezed_annotation.dart';

part 'ws_dtos.freezed.dart';
part 'ws_dtos.g.dart';

// -----------------------------------------------------------------------------
// Basic DTOs
// -----------------------------------------------------------------------------

/// Mapped from `common::session_type_enum::SessionTypeEnum`
enum SessionTypeEnum {
  @JsonValue('Work')
  work,
  @JsonValue('focus')
  focus,
  @JsonValue('ShortBreak')
  shortBreak,
  @JsonValue('LongBreak')
  longBreak,
}

/// Mapped from `update_concentration_score.rs`
@freezed
class UpdateConcentrationScore with _$UpdateConcentrationScore {
  const factory UpdateConcentrationScore({required int concentrationScore}) =
      _UpdateConcentrationScore;

  factory UpdateConcentrationScore.fromJson(Map<String, dynamic> json) =>
      _$UpdateConcentrationScoreFromJson(json);
}

/// Mapped from `note_update_ws.rs`
@freezed
class NoteUpdate with _$NoteUpdate {
  const factory NoteUpdate({required String newNote}) = _NoteUpdate;

  factory NoteUpdate.fromJson(Map<String, dynamic> json) =>
      _$NoteUpdateFromJson(json);
}

/// Mapped from `update_pomodoro_context.rs`
@freezed
class UpdatePomodoroContext with _$UpdatePomodoroContext {
  const factory UpdatePomodoroContext({String? categoryId, String? taskId}) =
      _UpdatePomodoroContext;

  factory UpdatePomodoroContext.fromJson(Map<String, dynamic> json) =>
      _$UpdatePomodoroContextFromJson(json);
}

// -----------------------------------------------------------------------------
// Pomodoro State DTOs (Mapped from `update_pomodoro_state.rs`)
// -----------------------------------------------------------------------------

@freezed
class UpdateWorkContext with _$UpdateWorkContext {
  const factory UpdateWorkContext({String? categoryId, String? taskId}) =
      _UpdateWorkContext;

  factory UpdateWorkContext.fromJson(Map<String, dynamic> json) =>
      _$UpdateWorkContextFromJson(json);
}

@freezed
class UpdateCurrentSession with _$UpdateCurrentSession {
  const factory UpdateCurrentSession({
    required SessionTypeEnum sessionType,
    required int sessionStartTime, // i64 maps to int in Dart
    String? categoryId,
    String? taskId,
    String? note,
    int? concentrationScore,
  }) = _UpdateCurrentSession;

  factory UpdateCurrentSession.fromJson(Map<String, dynamic> json) =>
      _$UpdateCurrentSessionFromJson(json);
}

@freezed
class UpdatePomodoroState with _$UpdatePomodoroState {
  const factory UpdatePomodoroState({
    UpdateCurrentSession? currentSession,
    required UpdateWorkContext workContext,
  }) = _UpdatePomodoroState;

  factory UpdatePomodoroState.fromJson(Map<String, dynamic> json) =>
      _$UpdatePomodoroStateFromJson(json);
}

// -----------------------------------------------------------------------------
// WebSocket Messages (Mapped from `ws_message.rs`)
// -----------------------------------------------------------------------------

/// Wrapper for client requests.
/// NOTE: Rust uses `#[serde(flatten)]`. If you need strict flattening behavior
/// (request_id at the same level as message fields), you might need a custom converter.
/// Standard mapping puts `message` as a nested object.
@freezed
class WsClientRequest with _$WsClientRequest {
  const factory WsClientRequest({
    String? requestId,
    required ClientMessage message,
  }) = _WsClientRequest;

  factory WsClientRequest.fromJson(Map<String, dynamic> json) =>
      _$WsClientRequestFromJson(json);
}

/// Client -> Server Messages
///
/// IMPORTANT: To match this Freezed configuration, ensure your Rust enum has:
/// `#[serde(tag = "type", content = "payload", rename_all = "camelCase")]`
/// or adjust the UnionKey/UnionValue logic below to match your Serde config.
@Freezed(unionKey: 'type', unionValueCase: FreezedUnionCase.none)
sealed class ClientMessage with _$ClientMessage {
  const factory ClientMessage.requestSync() = ClientMessageRequestSync;

  const factory ClientMessage.startEvent() = ClientMessageStartEvent;

  const factory ClientMessage.breakEvent() = ClientMessageBreakEvent;

  const factory ClientMessage.terminateEvent() = ClientMessageTerminateEvent;

  // Payload variants
  const factory ClientMessage.updatePomodoroContext(
    UpdatePomodoroContext payload,
  ) = ClientMessageUpdatePomodoroContext;

  const factory ClientMessage.updateNote(NoteUpdate payload) =
      ClientMessageUpdateNote;

  const factory ClientMessage.updateConcentrationScore(
    UpdateConcentrationScore payload,
  ) = ClientMessageUpdateConcentrationScore;

  factory ClientMessage.fromJson(Map<String, dynamic> json) =>
      _$ClientMessageFromJson(json);
}

/// Server -> Client Responses
@Freezed(unionKey: 'type', unionValueCase: FreezedUnionCase.none)
sealed class ServerResponse with _$ServerResponse {
  const factory ServerResponse.success({
    required String message,
    String? requestId,
  }) = ServerResponseSuccess;

  const factory ServerResponse.error({
    required String code,
    required String message,
    String? requestId,
  }) = ServerResponseError;

  const factory ServerResponse.syncData(UpdatePomodoroState payload) =
      ServerResponseSyncData;

  factory ServerResponse.fromJson(Map<String, dynamic> json) =>
      _$ServerResponseFromJson(json);
}

/// Broadcast Events
@Freezed(unionKey: 'type', unionValueCase: FreezedUnionCase.none)
sealed class BroadcastEvent with _$BroadcastEvent {
  const factory BroadcastEvent.pomodoroSessionUpdate(
    UpdatePomodoroState payload,
  ) = BroadcastEventPomodoroSessionUpdate;

  factory BroadcastEvent.fromJson(Map<String, dynamic> json) =>
      _$BroadcastEventFromJson(json);
}
