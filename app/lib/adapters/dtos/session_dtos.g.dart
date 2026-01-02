// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'session_dtos.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_$CreateManualSessionDtoImpl _$$CreateManualSessionDtoImplFromJson(
  Map<String, dynamic> json,
) => _$CreateManualSessionDtoImpl(
  sessionType: json['sessionType'] as String,
  startedAt: (json['startedAt'] as num).toInt(),
  endedAt: (json['endedAt'] as num).toInt(),
  taskId: json['taskId'] as String?,
  categoryId: json['categoryId'] as String?,
  concentrationScore: (json['concentrationScore'] as num?)?.toInt(),
  notes: json['notes'] as String?,
);

Map<String, dynamic> _$$CreateManualSessionDtoImplToJson(
  _$CreateManualSessionDtoImpl instance,
) => <String, dynamic>{
  'sessionType': instance.sessionType,
  'startedAt': instance.startedAt,
  'endedAt': instance.endedAt,
  'taskId': instance.taskId,
  'categoryId': instance.categoryId,
  'concentrationScore': instance.concentrationScore,
  'notes': instance.notes,
};

_$GetSessionFiltersDtoImpl _$$GetSessionFiltersDtoImplFromJson(
  Map<String, dynamic> json,
) => _$GetSessionFiltersDtoImpl(
  startDate: (json['startDate'] as num?)?.toInt(),
  endDate: (json['endDate'] as num?)?.toInt(),
  categoryIds:
      (json['categoryIds'] as List<dynamic>?)?.map((e) => e as String).toList(),
  sessionType: json['sessionType'] as String?,
  minConcentrationScore: (json['minConcentrationScore'] as num?)?.toInt(),
  maxConcentrationScore: (json['maxConcentrationScore'] as num?)?.toInt(),
);

Map<String, dynamic> _$$GetSessionFiltersDtoImplToJson(
  _$GetSessionFiltersDtoImpl instance,
) => <String, dynamic>{
  'startDate': instance.startDate,
  'endDate': instance.endDate,
  'categoryIds': instance.categoryIds,
  'sessionType': instance.sessionType,
  'minConcentrationScore': instance.minConcentrationScore,
  'maxConcentrationScore': instance.maxConcentrationScore,
};

_$UpdateFocusSessionDtoImpl _$$UpdateFocusSessionDtoImplFromJson(
  Map<String, dynamic> json,
) => _$UpdateFocusSessionDtoImpl(
  categoryId: json['categoryId'] as String?,
  taskId: json['taskId'] as String?,
  notes: json['notes'] as String?,
  concentrationScore: (json['concentrationScore'] as num?)?.toInt(),
  startedAt: (json['startedAt'] as num?)?.toInt(),
  endedAt: (json['endedAt'] as num?)?.toInt(),
  actualDuration: (json['actualDuration'] as num?)?.toInt(),
  sessionType: json['sessionType'] as String?,
);

Map<String, dynamic> _$$UpdateFocusSessionDtoImplToJson(
  _$UpdateFocusSessionDtoImpl instance,
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

_$FocusSessionDtoImpl _$$FocusSessionDtoImplFromJson(
  Map<String, dynamic> json,
) => _$FocusSessionDtoImpl(
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

Map<String, dynamic> _$$FocusSessionDtoImplToJson(
  _$FocusSessionDtoImpl instance,
) => <String, dynamic>{
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

_$CreateManualSessionResponseDtoImpl
_$$CreateManualSessionResponseDtoImplFromJson(Map<String, dynamic> json) =>
    _$CreateManualSessionResponseDtoImpl(id: json['id'] as String);

Map<String, dynamic> _$$CreateManualSessionResponseDtoImplToJson(
  _$CreateManualSessionResponseDtoImpl instance,
) => <String, dynamic>{'id': instance.id};

_$GetSessionFiltersResponseDtoImpl _$$GetSessionFiltersResponseDtoImplFromJson(
  Map<String, dynamic> json,
) => _$GetSessionFiltersResponseDtoImpl(
  focusSessions:
      (json['focusSessions'] as List<dynamic>)
          .map((e) => FocusSessionDto.fromJson(e as Map<String, dynamic>))
          .toList(),
);

Map<String, dynamic> _$$GetSessionFiltersResponseDtoImplToJson(
  _$GetSessionFiltersResponseDtoImpl instance,
) => <String, dynamic>{'focusSessions': instance.focusSessions};
