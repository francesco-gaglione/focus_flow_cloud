// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'ws_dtos.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_UpdateConcentrationScore _$UpdateConcentrationScoreFromJson(
  Map<String, dynamic> json,
) => _UpdateConcentrationScore(
  concentrationScore: (json['concentrationScore'] as num).toInt(),
);

Map<String, dynamic> _$UpdateConcentrationScoreToJson(
  _UpdateConcentrationScore instance,
) => <String, dynamic>{'concentrationScore': instance.concentrationScore};

_NoteUpdate _$NoteUpdateFromJson(Map<String, dynamic> json) =>
    _NoteUpdate(newNote: json['newNote'] as String);

Map<String, dynamic> _$NoteUpdateToJson(_NoteUpdate instance) =>
    <String, dynamic>{'newNote': instance.newNote};

_UpdatePomodoroContext _$UpdatePomodoroContextFromJson(
  Map<String, dynamic> json,
) => _UpdatePomodoroContext(
  categoryId: json['categoryId'] as String?,
  taskId: json['taskId'] as String?,
);

Map<String, dynamic> _$UpdatePomodoroContextToJson(
  _UpdatePomodoroContext instance,
) => <String, dynamic>{
  'categoryId': instance.categoryId,
  'taskId': instance.taskId,
};

_UpdateWorkContext _$UpdateWorkContextFromJson(Map<String, dynamic> json) =>
    _UpdateWorkContext(
      categoryId: json['categoryId'] as String?,
      taskId: json['taskId'] as String?,
    );

Map<String, dynamic> _$UpdateWorkContextToJson(_UpdateWorkContext instance) =>
    <String, dynamic>{
      'categoryId': instance.categoryId,
      'taskId': instance.taskId,
    };

_UpdateCurrentSession _$UpdateCurrentSessionFromJson(
  Map<String, dynamic> json,
) => _UpdateCurrentSession(
  sessionType: $enumDecode(_$SessionTypeEnumEnumMap, json['sessionType']),
  sessionStartTime: (json['sessionStartTime'] as num).toInt(),
  categoryId: json['categoryId'] as String?,
  taskId: json['taskId'] as String?,
  note: json['note'] as String?,
  concentrationScore: (json['concentrationScore'] as num?)?.toInt(),
);

Map<String, dynamic> _$UpdateCurrentSessionToJson(
  _UpdateCurrentSession instance,
) => <String, dynamic>{
  'sessionType': _$SessionTypeEnumEnumMap[instance.sessionType]!,
  'sessionStartTime': instance.sessionStartTime,
  'categoryId': instance.categoryId,
  'taskId': instance.taskId,
  'note': instance.note,
  'concentrationScore': instance.concentrationScore,
};

const _$SessionTypeEnumEnumMap = {
  SessionTypeEnum.work: 'Work',
  SessionTypeEnum.focus: 'focus',
  SessionTypeEnum.shortBreak: 'ShortBreak',
  SessionTypeEnum.longBreak: 'LongBreak',
};

_UpdatePomodoroState _$UpdatePomodoroStateFromJson(Map<String, dynamic> json) =>
    _UpdatePomodoroState(
      currentSession:
          json['currentSession'] == null
              ? null
              : UpdateCurrentSession.fromJson(
                json['currentSession'] as Map<String, dynamic>,
              ),
      workContext: UpdateWorkContext.fromJson(
        json['workContext'] as Map<String, dynamic>,
      ),
    );

Map<String, dynamic> _$UpdatePomodoroStateToJson(
  _UpdatePomodoroState instance,
) => <String, dynamic>{
  'currentSession': instance.currentSession,
  'workContext': instance.workContext,
};

_WsClientRequest _$WsClientRequestFromJson(Map<String, dynamic> json) =>
    _WsClientRequest(
      requestId: json['requestId'] as String?,
      message: ClientMessage.fromJson(json['message'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$WsClientRequestToJson(_WsClientRequest instance) =>
    <String, dynamic>{
      'requestId': instance.requestId,
      'message': instance.message,
    };

ClientMessageRequestSync _$ClientMessageRequestSyncFromJson(
  Map<String, dynamic> json,
) => ClientMessageRequestSync($type: json['type'] as String?);

Map<String, dynamic> _$ClientMessageRequestSyncToJson(
  ClientMessageRequestSync instance,
) => <String, dynamic>{'type': instance.$type};

ClientMessageStartEvent _$ClientMessageStartEventFromJson(
  Map<String, dynamic> json,
) => ClientMessageStartEvent($type: json['type'] as String?);

Map<String, dynamic> _$ClientMessageStartEventToJson(
  ClientMessageStartEvent instance,
) => <String, dynamic>{'type': instance.$type};

ClientMessageBreakEvent _$ClientMessageBreakEventFromJson(
  Map<String, dynamic> json,
) => ClientMessageBreakEvent($type: json['type'] as String?);

Map<String, dynamic> _$ClientMessageBreakEventToJson(
  ClientMessageBreakEvent instance,
) => <String, dynamic>{'type': instance.$type};

ClientMessageTerminateEvent _$ClientMessageTerminateEventFromJson(
  Map<String, dynamic> json,
) => ClientMessageTerminateEvent($type: json['type'] as String?);

Map<String, dynamic> _$ClientMessageTerminateEventToJson(
  ClientMessageTerminateEvent instance,
) => <String, dynamic>{'type': instance.$type};

ClientMessageUpdatePomodoroContext _$ClientMessageUpdatePomodoroContextFromJson(
  Map<String, dynamic> json,
) => ClientMessageUpdatePomodoroContext(
  UpdatePomodoroContext.fromJson(json['payload'] as Map<String, dynamic>),
  $type: json['type'] as String?,
);

Map<String, dynamic> _$ClientMessageUpdatePomodoroContextToJson(
  ClientMessageUpdatePomodoroContext instance,
) => <String, dynamic>{'payload': instance.payload, 'type': instance.$type};

ClientMessageUpdateNote _$ClientMessageUpdateNoteFromJson(
  Map<String, dynamic> json,
) => ClientMessageUpdateNote(
  NoteUpdate.fromJson(json['payload'] as Map<String, dynamic>),
  $type: json['type'] as String?,
);

Map<String, dynamic> _$ClientMessageUpdateNoteToJson(
  ClientMessageUpdateNote instance,
) => <String, dynamic>{'payload': instance.payload, 'type': instance.$type};

ClientMessageUpdateConcentrationScore
_$ClientMessageUpdateConcentrationScoreFromJson(Map<String, dynamic> json) =>
    ClientMessageUpdateConcentrationScore(
      UpdateConcentrationScore.fromJson(
        json['payload'] as Map<String, dynamic>,
      ),
      $type: json['type'] as String?,
    );

Map<String, dynamic> _$ClientMessageUpdateConcentrationScoreToJson(
  ClientMessageUpdateConcentrationScore instance,
) => <String, dynamic>{'payload': instance.payload, 'type': instance.$type};

ServerResponseSuccess _$ServerResponseSuccessFromJson(
  Map<String, dynamic> json,
) => ServerResponseSuccess(
  message: json['message'] as String,
  requestId: json['requestId'] as String?,
  $type: json['type'] as String?,
);

Map<String, dynamic> _$ServerResponseSuccessToJson(
  ServerResponseSuccess instance,
) => <String, dynamic>{
  'message': instance.message,
  'requestId': instance.requestId,
  'type': instance.$type,
};

ServerResponseError _$ServerResponseErrorFromJson(Map<String, dynamic> json) =>
    ServerResponseError(
      code: json['code'] as String,
      message: json['message'] as String,
      requestId: json['requestId'] as String?,
      $type: json['type'] as String?,
    );

Map<String, dynamic> _$ServerResponseErrorToJson(
  ServerResponseError instance,
) => <String, dynamic>{
  'code': instance.code,
  'message': instance.message,
  'requestId': instance.requestId,
  'type': instance.$type,
};

ServerResponseSyncData _$ServerResponseSyncDataFromJson(
  Map<String, dynamic> json,
) => ServerResponseSyncData(
  UpdatePomodoroState.fromJson(json['payload'] as Map<String, dynamic>),
  $type: json['type'] as String?,
);

Map<String, dynamic> _$ServerResponseSyncDataToJson(
  ServerResponseSyncData instance,
) => <String, dynamic>{'payload': instance.payload, 'type': instance.$type};

BroadcastEventPomodoroSessionUpdate
_$BroadcastEventPomodoroSessionUpdateFromJson(Map<String, dynamic> json) =>
    BroadcastEventPomodoroSessionUpdate(
      UpdatePomodoroState.fromJson(json['payload'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$BroadcastEventPomodoroSessionUpdateToJson(
  BroadcastEventPomodoroSessionUpdate instance,
) => <String, dynamic>{'payload': instance.payload};
