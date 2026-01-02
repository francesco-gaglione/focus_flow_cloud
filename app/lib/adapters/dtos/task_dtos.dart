// ignore_for_file: invalid_annotation_target
import 'package:freezed_annotation/freezed_annotation.dart';
import '../../domain/entities/task.dart';

part 'task_dtos.freezed.dart';
part 'task_dtos.g.dart';

// Request DTOs

@freezed
class CreateTaskDto with _$CreateTaskDto {
  const factory CreateTaskDto({
    required String name,
    String? description,
    String? categoryId,
    int? scheduledDate,
  }) = _CreateTaskDto;

  factory CreateTaskDto.fromJson(Map<String, dynamic> json) =>
      _$CreateTaskDtoFromJson(json);
}

@freezed
class UpdateTaskDto with _$UpdateTaskDto {
  const factory UpdateTaskDto({
    String? name,
    String? description,
    String? categoryId,
    int? scheduledDate,
    int? completedAt,
  }) = _UpdateTaskDto;

  factory UpdateTaskDto.fromJson(Map<String, dynamic> json) =>
      _$UpdateTaskDtoFromJson(json);
}

@freezed
class DeleteTasksDto with _$DeleteTasksDto {
  const factory DeleteTasksDto({required List<String> taskIds}) =
      _DeleteTasksDto;

  factory DeleteTasksDto.fromJson(Map<String, dynamic> json) =>
      _$DeleteTasksDtoFromJson(json);
}

// Response DTOs

@freezed
class TaskResponseDto with _$TaskResponseDto {
  const factory TaskResponseDto({
    required String id,
    required String name,
    String? description,
    String? categoryId,
    int? scheduledDate,
    int? completedAt,
  }) = _TaskResponseDto;

  factory TaskResponseDto.fromJson(Map<String, dynamic> json) =>
      _$TaskResponseDtoFromJson(json);

  factory TaskResponseDto.fromEntity(Task task) {
    return TaskResponseDto(
      id: task.id,
      name: task.name,
      description: task.description,
      categoryId: task.categoryId,
      scheduledDate: task.scheduledDate,
      completedAt: task.completedAt,
    );
  }
}

@freezed
class CreateTaskResponseDto with _$CreateTaskResponseDto {
  const factory CreateTaskResponseDto({required String id}) =
      _CreateTaskResponseDto;

  factory CreateTaskResponseDto.fromJson(Map<String, dynamic> json) =>
      _$CreateTaskResponseDtoFromJson(json);
}

@freezed
class UpdateTaskResponseDto with _$UpdateTaskResponseDto {
  const factory UpdateTaskResponseDto({required TaskResponseDto updatedTask}) =
      _UpdateTaskResponseDto;

  factory UpdateTaskResponseDto.fromJson(Map<String, dynamic> json) =>
      _$UpdateTaskResponseDtoFromJson(json);
}

@freezed
class DeleteTasksResponseDto with _$DeleteTasksResponseDto {
  const factory DeleteTasksResponseDto({
    @JsonKey(name: 'deleted_ids') required List<String> deletedIds,
  }) = _DeleteTasksResponseDto;

  factory DeleteTasksResponseDto.fromJson(Map<String, dynamic> json) =>
      _$DeleteTasksResponseDtoFromJson(json);
}

@freezed
class OrphanTasksResponseDto with _$OrphanTasksResponseDto {
  const factory OrphanTasksResponseDto({
    required List<TaskResponseDto> orphanTasks,
  }) = _OrphanTasksResponseDto;

  factory OrphanTasksResponseDto.fromJson(Map<String, dynamic> json) =>
      _$OrphanTasksResponseDtoFromJson(json);
}
