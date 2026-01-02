// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'ws_dtos.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_$UpdateConcentrationScoreImpl _$$UpdateConcentrationScoreImplFromJson(
  Map<String, dynamic> json,
) => _$UpdateConcentrationScoreImpl(
  concentrationScore: (json['concentrationScore'] as num).toInt(),
);

Map<String, dynamic> _$$UpdateConcentrationScoreImplToJson(
  _$UpdateConcentrationScoreImpl instance,
) => <String, dynamic>{'concentrationScore': instance.concentrationScore};

_$NoteUpdateImpl _$$NoteUpdateImplFromJson(Map<String, dynamic> json) =>
    _$NoteUpdateImpl(newNote: json['newNote'] as String);

Map<String, dynamic> _$$NoteUpdateImplToJson(_$NoteUpdateImpl instance) =>
    <String, dynamic>{'newNote': instance.newNote};

_$UpdatePomodoroContextImpl _$$UpdatePomodoroContextImplFromJson(
  Map<String, dynamic> json,
) => _$UpdatePomodoroContextImpl(
  categoryId: json['categoryId'] as String?,
  taskId: json['taskId'] as String?,
);

Map<String, dynamic> _$$UpdatePomodoroContextImplToJson(
  _$UpdatePomodoroContextImpl instance,
) => <String, dynamic>{
  'categoryId': instance.categoryId,
  'taskId': instance.taskId,
};

_$UpdateWorkContextImpl _$$UpdateWorkContextImplFromJson(
  Map<String, dynamic> json,
) => _$UpdateWorkContextImpl(
  categoryId: json['categoryId'] as String?,
  taskId: json['taskId'] as String?,
);

Map<String, dynamic> _$$UpdateWorkContextImplToJson(
  _$UpdateWorkContextImpl instance,
) => <String, dynamic>{
  'categoryId': instance.categoryId,
  'taskId': instance.taskId,
};

_$UpdateCurrentSessionImpl _$$UpdateCurrentSessionImplFromJson(
  Map<String, dynamic> json,
) => _$UpdateCurrentSessionImpl(
  sessionType: $enumDecode(_$SessionTypeEnumEnumMap, json['sessionType']),
  sessionStartTime: (json['sessionStartTime'] as num).toInt(),
  categoryId: json['categoryId'] as String?,
  taskId: json['taskId'] as String?,
  note: json['note'] as String?,
  concentrationScore: (json['concentrationScore'] as num?)?.toInt(),
);

Map<String, dynamic> _$$UpdateCurrentSessionImplToJson(
  _$UpdateCurrentSessionImpl instance,
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

_$UpdatePomodoroStateImpl _$$UpdatePomodoroStateImplFromJson(
  Map<String, dynamic> json,
) => _$UpdatePomodoroStateImpl(
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

Map<String, dynamic> _$$UpdatePomodoroStateImplToJson(
  _$UpdatePomodoroStateImpl instance,
) => <String, dynamic>{
  'currentSession': instance.currentSession,
  'workContext': instance.workContext,
};

_$WsClientRequestImpl _$$WsClientRequestImplFromJson(
  Map<String, dynamic> json,
) => _$WsClientRequestImpl(
  requestId: json['requestId'] as String?,
  message: ClientMessage.fromJson(json['message'] as Map<String, dynamic>),
);

Map<String, dynamic> _$$WsClientRequestImplToJson(
  _$WsClientRequestImpl instance,
) => <String, dynamic>{
  'requestId': instance.requestId,
  'message': instance.message,
};

_$ClientMessageRequestSyncImpl _$$ClientMessageRequestSyncImplFromJson(
  Map<String, dynamic> json,
) => _$ClientMessageRequestSyncImpl($type: json['type'] as String?);

Map<String, dynamic> _$$ClientMessageRequestSyncImplToJson(
  _$ClientMessageRequestSyncImpl instance,
) => <String, dynamic>{'type': instance.$type};

_$ClientMessageStartEventImpl _$$ClientMessageStartEventImplFromJson(
  Map<String, dynamic> json,
) => _$ClientMessageStartEventImpl($type: json['type'] as String?);

Map<String, dynamic> _$$ClientMessageStartEventImplToJson(
  _$ClientMessageStartEventImpl instance,
) => <String, dynamic>{'type': instance.$type};

_$ClientMessageBreakEventImpl _$$ClientMessageBreakEventImplFromJson(
  Map<String, dynamic> json,
) => _$ClientMessageBreakEventImpl($type: json['type'] as String?);

Map<String, dynamic> _$$ClientMessageBreakEventImplToJson(
  _$ClientMessageBreakEventImpl instance,
) => <String, dynamic>{'type': instance.$type};

_$ClientMessageTerminateEventImpl _$$ClientMessageTerminateEventImplFromJson(
  Map<String, dynamic> json,
) => _$ClientMessageTerminateEventImpl($type: json['type'] as String?);

Map<String, dynamic> _$$ClientMessageTerminateEventImplToJson(
  _$ClientMessageTerminateEventImpl instance,
) => <String, dynamic>{'type': instance.$type};

_$ClientMessageUpdatePomodoroContextImpl
_$$ClientMessageUpdatePomodoroContextImplFromJson(Map<String, dynamic> json) =>
    _$ClientMessageUpdatePomodoroContextImpl(
      UpdatePomodoroContext.fromJson(json['payload'] as Map<String, dynamic>),
      $type: json['type'] as String?,
    );

Map<String, dynamic> _$$ClientMessageUpdatePomodoroContextImplToJson(
  _$ClientMessageUpdatePomodoroContextImpl instance,
) => <String, dynamic>{'payload': instance.payload, 'type': instance.$type};

_$ClientMessageUpdateNoteImpl _$$ClientMessageUpdateNoteImplFromJson(
  Map<String, dynamic> json,
) => _$ClientMessageUpdateNoteImpl(
  NoteUpdate.fromJson(json['payload'] as Map<String, dynamic>),
  $type: json['type'] as String?,
);

Map<String, dynamic> _$$ClientMessageUpdateNoteImplToJson(
  _$ClientMessageUpdateNoteImpl instance,
) => <String, dynamic>{'payload': instance.payload, 'type': instance.$type};

_$ClientMessageUpdateConcentrationScoreImpl
_$$ClientMessageUpdateConcentrationScoreImplFromJson(
  Map<String, dynamic> json,
) => _$ClientMessageUpdateConcentrationScoreImpl(
  UpdateConcentrationScore.fromJson(json['payload'] as Map<String, dynamic>),
  $type: json['type'] as String?,
);

Map<String, dynamic> _$$ClientMessageUpdateConcentrationScoreImplToJson(
  _$ClientMessageUpdateConcentrationScoreImpl instance,
) => <String, dynamic>{'payload': instance.payload, 'type': instance.$type};

_$ServerResponseSuccessImpl _$$ServerResponseSuccessImplFromJson(
  Map<String, dynamic> json,
) => _$ServerResponseSuccessImpl(
  message: json['message'] as String,
  requestId: json['requestId'] as String?,
  $type: json['type'] as String?,
);

Map<String, dynamic> _$$ServerResponseSuccessImplToJson(
  _$ServerResponseSuccessImpl instance,
) => <String, dynamic>{
  'message': instance.message,
  'requestId': instance.requestId,
  'type': instance.$type,
};

_$ServerResponseErrorImpl _$$ServerResponseErrorImplFromJson(
  Map<String, dynamic> json,
) => _$ServerResponseErrorImpl(
  code: json['code'] as String,
  message: json['message'] as String,
  requestId: json['requestId'] as String?,
  $type: json['type'] as String?,
);

Map<String, dynamic> _$$ServerResponseErrorImplToJson(
  _$ServerResponseErrorImpl instance,
) => <String, dynamic>{
  'code': instance.code,
  'message': instance.message,
  'requestId': instance.requestId,
  'type': instance.$type,
};

_$ServerResponseSyncDataImpl _$$ServerResponseSyncDataImplFromJson(
  Map<String, dynamic> json,
) => _$ServerResponseSyncDataImpl(
  UpdatePomodoroState.fromJson(json['payload'] as Map<String, dynamic>),
  $type: json['type'] as String?,
);

Map<String, dynamic> _$$ServerResponseSyncDataImplToJson(
  _$ServerResponseSyncDataImpl instance,
) => <String, dynamic>{'payload': instance.payload, 'type': instance.$type};

_$BroadcastEventPomodoroSessionUpdateImpl
_$$BroadcastEventPomodoroSessionUpdateImplFromJson(Map<String, dynamic> json) =>
    _$BroadcastEventPomodoroSessionUpdateImpl(
      UpdatePomodoroState.fromJson(json['payload'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$$BroadcastEventPomodoroSessionUpdateImplToJson(
  _$BroadcastEventPomodoroSessionUpdateImpl instance,
) => <String, dynamic>{'payload': instance.payload};
