// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'task_dtos.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_CreateTaskDto _$CreateTaskDtoFromJson(Map<String, dynamic> json) =>
    _CreateTaskDto(
      name: json['name'] as String,
      description: json['description'] as String?,
      categoryId: json['categoryId'] as String?,
      scheduledDate: (json['scheduledDate'] as num?)?.toInt(),
    );

Map<String, dynamic> _$CreateTaskDtoToJson(_CreateTaskDto instance) =>
    <String, dynamic>{
      'name': instance.name,
      'description': instance.description,
      'categoryId': instance.categoryId,
      'scheduledDate': instance.scheduledDate,
    };

_UpdateTaskDto _$UpdateTaskDtoFromJson(Map<String, dynamic> json) =>
    _UpdateTaskDto(
      name: json['name'] as String?,
      description: json['description'] as String?,
      categoryId: json['categoryId'] as String?,
      scheduledDate: (json['scheduledDate'] as num?)?.toInt(),
      completedAt: (json['completedAt'] as num?)?.toInt(),
    );

Map<String, dynamic> _$UpdateTaskDtoToJson(_UpdateTaskDto instance) =>
    <String, dynamic>{
      'name': instance.name,
      'description': instance.description,
      'categoryId': instance.categoryId,
      'scheduledDate': instance.scheduledDate,
      'completedAt': instance.completedAt,
    };

_DeleteTasksDto _$DeleteTasksDtoFromJson(Map<String, dynamic> json) =>
    _DeleteTasksDto(
      taskIds:
          (json['taskIds'] as List<dynamic>).map((e) => e as String).toList(),
    );

Map<String, dynamic> _$DeleteTasksDtoToJson(_DeleteTasksDto instance) =>
    <String, dynamic>{'taskIds': instance.taskIds};

_TaskResponseDto _$TaskResponseDtoFromJson(Map<String, dynamic> json) =>
    _TaskResponseDto(
      id: json['id'] as String,
      name: json['name'] as String,
      description: json['description'] as String?,
      categoryId: json['categoryId'] as String?,
      scheduledDate: (json['scheduledDate'] as num?)?.toInt(),
      completedAt: (json['completedAt'] as num?)?.toInt(),
    );

Map<String, dynamic> _$TaskResponseDtoToJson(_TaskResponseDto instance) =>
    <String, dynamic>{
      'id': instance.id,
      'name': instance.name,
      'description': instance.description,
      'categoryId': instance.categoryId,
      'scheduledDate': instance.scheduledDate,
      'completedAt': instance.completedAt,
    };

_CreateTaskResponseDto _$CreateTaskResponseDtoFromJson(
  Map<String, dynamic> json,
) => _CreateTaskResponseDto(id: json['id'] as String);

Map<String, dynamic> _$CreateTaskResponseDtoToJson(
  _CreateTaskResponseDto instance,
) => <String, dynamic>{'id': instance.id};

_UpdateTaskResponseDto _$UpdateTaskResponseDtoFromJson(
  Map<String, dynamic> json,
) => _UpdateTaskResponseDto(
  updatedTask: TaskResponseDto.fromJson(
    json['updatedTask'] as Map<String, dynamic>,
  ),
);

Map<String, dynamic> _$UpdateTaskResponseDtoToJson(
  _UpdateTaskResponseDto instance,
) => <String, dynamic>{'updatedTask': instance.updatedTask};

_DeleteTasksResponseDto _$DeleteTasksResponseDtoFromJson(
  Map<String, dynamic> json,
) => _DeleteTasksResponseDto(
  deletedIds:
      (json['deleted_ids'] as List<dynamic>).map((e) => e as String).toList(),
);

Map<String, dynamic> _$DeleteTasksResponseDtoToJson(
  _DeleteTasksResponseDto instance,
) => <String, dynamic>{'deleted_ids': instance.deletedIds};

_OrphanTasksResponseDto _$OrphanTasksResponseDtoFromJson(
  Map<String, dynamic> json,
) => _OrphanTasksResponseDto(
  orphanTasks:
      (json['orphanTasks'] as List<dynamic>)
          .map((e) => TaskResponseDto.fromJson(e as Map<String, dynamic>))
          .toList(),
);

Map<String, dynamic> _$OrphanTasksResponseDtoToJson(
  _OrphanTasksResponseDto instance,
) => <String, dynamic>{'orphanTasks': instance.orphanTasks};
