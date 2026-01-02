// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'task_dtos.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_$CreateTaskDtoImpl _$$CreateTaskDtoImplFromJson(Map<String, dynamic> json) =>
    _$CreateTaskDtoImpl(
      name: json['name'] as String,
      description: json['description'] as String?,
      categoryId: json['categoryId'] as String?,
      scheduledDate: (json['scheduledDate'] as num?)?.toInt(),
    );

Map<String, dynamic> _$$CreateTaskDtoImplToJson(_$CreateTaskDtoImpl instance) =>
    <String, dynamic>{
      'name': instance.name,
      'description': instance.description,
      'categoryId': instance.categoryId,
      'scheduledDate': instance.scheduledDate,
    };

_$UpdateTaskDtoImpl _$$UpdateTaskDtoImplFromJson(Map<String, dynamic> json) =>
    _$UpdateTaskDtoImpl(
      name: json['name'] as String?,
      description: json['description'] as String?,
      categoryId: json['categoryId'] as String?,
      scheduledDate: (json['scheduledDate'] as num?)?.toInt(),
      completedAt: (json['completedAt'] as num?)?.toInt(),
    );

Map<String, dynamic> _$$UpdateTaskDtoImplToJson(_$UpdateTaskDtoImpl instance) =>
    <String, dynamic>{
      'name': instance.name,
      'description': instance.description,
      'categoryId': instance.categoryId,
      'scheduledDate': instance.scheduledDate,
      'completedAt': instance.completedAt,
    };

_$DeleteTasksDtoImpl _$$DeleteTasksDtoImplFromJson(Map<String, dynamic> json) =>
    _$DeleteTasksDtoImpl(
      taskIds:
          (json['taskIds'] as List<dynamic>).map((e) => e as String).toList(),
    );

Map<String, dynamic> _$$DeleteTasksDtoImplToJson(
  _$DeleteTasksDtoImpl instance,
) => <String, dynamic>{'taskIds': instance.taskIds};

_$TaskResponseDtoImpl _$$TaskResponseDtoImplFromJson(
  Map<String, dynamic> json,
) => _$TaskResponseDtoImpl(
  id: json['id'] as String,
  name: json['name'] as String,
  description: json['description'] as String?,
  categoryId: json['categoryId'] as String?,
  scheduledDate: (json['scheduledDate'] as num?)?.toInt(),
  completedAt: (json['completedAt'] as num?)?.toInt(),
);

Map<String, dynamic> _$$TaskResponseDtoImplToJson(
  _$TaskResponseDtoImpl instance,
) => <String, dynamic>{
  'id': instance.id,
  'name': instance.name,
  'description': instance.description,
  'categoryId': instance.categoryId,
  'scheduledDate': instance.scheduledDate,
  'completedAt': instance.completedAt,
};

_$CreateTaskResponseDtoImpl _$$CreateTaskResponseDtoImplFromJson(
  Map<String, dynamic> json,
) => _$CreateTaskResponseDtoImpl(id: json['id'] as String);

Map<String, dynamic> _$$CreateTaskResponseDtoImplToJson(
  _$CreateTaskResponseDtoImpl instance,
) => <String, dynamic>{'id': instance.id};

_$UpdateTaskResponseDtoImpl _$$UpdateTaskResponseDtoImplFromJson(
  Map<String, dynamic> json,
) => _$UpdateTaskResponseDtoImpl(
  updatedTask: TaskResponseDto.fromJson(
    json['updatedTask'] as Map<String, dynamic>,
  ),
);

Map<String, dynamic> _$$UpdateTaskResponseDtoImplToJson(
  _$UpdateTaskResponseDtoImpl instance,
) => <String, dynamic>{'updatedTask': instance.updatedTask};

_$DeleteTasksResponseDtoImpl _$$DeleteTasksResponseDtoImplFromJson(
  Map<String, dynamic> json,
) => _$DeleteTasksResponseDtoImpl(
  deletedIds:
      (json['deleted_ids'] as List<dynamic>).map((e) => e as String).toList(),
);

Map<String, dynamic> _$$DeleteTasksResponseDtoImplToJson(
  _$DeleteTasksResponseDtoImpl instance,
) => <String, dynamic>{'deleted_ids': instance.deletedIds};

_$OrphanTasksResponseDtoImpl _$$OrphanTasksResponseDtoImplFromJson(
  Map<String, dynamic> json,
) => _$OrphanTasksResponseDtoImpl(
  orphanTasks:
      (json['orphanTasks'] as List<dynamic>)
          .map((e) => TaskResponseDto.fromJson(e as Map<String, dynamic>))
          .toList(),
);

Map<String, dynamic> _$$OrphanTasksResponseDtoImplToJson(
  _$OrphanTasksResponseDtoImpl instance,
) => <String, dynamic>{'orphanTasks': instance.orphanTasks};
