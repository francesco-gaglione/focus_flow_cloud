// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'session_dtos.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_CreateManualSessionDto _$CreateManualSessionDtoFromJson(
  Map<String, dynamic> json,
) => _CreateManualSessionDto(
  sessionType: json['sessionType'] as String,
  startedAt: (json['startedAt'] as num).toInt(),
  endedAt: (json['endedAt'] as num).toInt(),
  taskId: json['taskId'] as String?,
  categoryId: json['categoryId'] as String?,
  concentrationScore: (json['concentrationScore'] as num?)?.toInt(),
  notes: json['notes'] as String?,
);

Map<String, dynamic> _$CreateManualSessionDtoToJson(
  _CreateManualSessionDto instance,
) => <String, dynamic>{
  'sessionType': instance.sessionType,
  'startedAt': instance.startedAt,
  'endedAt': instance.endedAt,
  'taskId': instance.taskId,
  'categoryId': instance.categoryId,
  'concentrationScore': instance.concentrationScore,
  'notes': instance.notes,
};

_GetSessionFiltersDto _$GetSessionFiltersDtoFromJson(
  Map<String, dynamic> json,
) => _GetSessionFiltersDto(
  startDate: (json['startDate'] as num?)?.toInt(),
  endDate: (json['endDate'] as num?)?.toInt(),
  categoryIds:
      (json['categoryIds'] as List<dynamic>?)?.map((e) => e as String).toList(),
  sessionType: json['sessionType'] as String?,
  minConcentrationScore: (json['minConcentrationScore'] as num?)?.toInt(),
  maxConcentrationScore: (json['maxConcentrationScore'] as num?)?.toInt(),
);

Map<String, dynamic> _$GetSessionFiltersDtoToJson(
  _GetSessionFiltersDto instance,
) => <String, dynamic>{
  'startDate': instance.startDate,
  'endDate': instance.endDate,
  'categoryIds': instance.categoryIds,
  'sessionType': instance.sessionType,
  'minConcentrationScore': instance.minConcentrationScore,
  'maxConcentrationScore': instance.maxConcentrationScore,
};

_UpdateFocusSessionDto _$UpdateFocusSessionDtoFromJson(
  Map<String, dynamic> json,
) => _UpdateFocusSessionDto(
  categoryId: json['categoryId'] as String?,
  taskId: json['taskId'] as String?,
  notes: json['notes'] as String?,
  concentrationScore: (json['concentrationScore'] as num?)?.toInt(),
  startedAt: (json['startedAt'] as num?)?.toInt(),
  endedAt: (json['endedAt'] as num?)?.toInt(),
  actualDuration: (json['actualDuration'] as num?)?.toInt(),
  sessionType: json['sessionType'] as String?,
);

Map<String, dynamic> _$UpdateFocusSessionDtoToJson(
  _UpdateFocusSessionDto instance,
) => <String, dynamic>{
  'categoryId': instance.categoryId,
  'taskId': instance.taskId,
  'notes': instance.notes,
  'concentrationScore': instance.concentrationScore,
  'startedAt': instance.startedAt,
  'endedAt': instance.endedAt,
  'actualDuration': instance.actualDuration,
  'sessionType': instance.sessionType,
};

_FocusSessionDto _$FocusSessionDtoFromJson(Map<String, dynamic> json) =>
    _FocusSessionDto(
      id: json['id'] as String,
      sessionType: json['sessionType'] as String,
      startedAt: (json['startedAt'] as num).toInt(),
      endedAt: (json['endedAt'] as num?)?.toInt(),
      actualDuration: (json['actualDuration'] as num?)?.toInt(),
      taskId: json['taskId'] as String?,
      categoryId: json['categoryId'] as String?,
      concentrationScore: (json['concentrationScore'] as num?)?.toInt(),
      notes: json['notes'] as String?,
      createdAt: (json['createdAt'] as num).toInt(),
    );

Map<String, dynamic> _$FocusSessionDtoToJson(_FocusSessionDto instance) =>
    <String, dynamic>{
      'id': instance.id,
      'sessionType': instance.sessionType,
      'startedAt': instance.startedAt,
      'endedAt': instance.endedAt,
      'actualDuration': instance.actualDuration,
      'taskId': instance.taskId,
      'categoryId': instance.categoryId,
      'concentrationScore': instance.concentrationScore,
      'notes': instance.notes,
      'createdAt': instance.createdAt,
    };

_CreateManualSessionResponseDto _$CreateManualSessionResponseDtoFromJson(
  Map<String, dynamic> json,
) => _CreateManualSessionResponseDto(id: json['id'] as String);

Map<String, dynamic> _$CreateManualSessionResponseDtoToJson(
  _CreateManualSessionResponseDto instance,
) => <String, dynamic>{'id': instance.id};

_GetSessionFiltersResponseDto _$GetSessionFiltersResponseDtoFromJson(
  Map<String, dynamic> json,
) => _GetSessionFiltersResponseDto(
  focusSessions:
      (json['focusSessions'] as List<dynamic>)
          .map((e) => FocusSessionDto.fromJson(e as Map<String, dynamic>))
          .toList(),
);

Map<String, dynamic> _$GetSessionFiltersResponseDtoToJson(
  _GetSessionFiltersResponseDto instance,
) => <String, dynamic>{'focusSessions': instance.focusSessions};
