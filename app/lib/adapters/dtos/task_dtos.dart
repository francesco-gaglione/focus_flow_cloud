// ignore_for_file: invalid_annotation_target
import 'package:freezed_annotation/freezed_annotation.dart';
import '../../domain/entities/task.dart';

part 'task_dtos.freezed.dart';
part 'task_dtos.g.dart';

// Request DTOs

@freezed
abstract class CreateTaskDto with _$CreateTaskDto {
  const factory CreateTaskDto({
    required String name,
    String? description,
    String? categoryId,
    int? scheduledDate,
    int? scheduledEndDate,
  }) = _CreateTaskDto;

  factory CreateTaskDto.fromJson(Map<String, dynamic> json) =>
      _$CreateTaskDtoFromJson(json);
}

@freezed
abstract class UpdateTaskDto with _$UpdateTaskDto {
  const factory UpdateTaskDto({
    String? name,
    String? description,
    String? categoryId,
    int? scheduledDate,
    int? scheduledEndDate,
    int? completedAt,
  }) = _UpdateTaskDto;

  factory UpdateTaskDto.fromJson(Map<String, dynamic> json) =>
      _$UpdateTaskDtoFromJson(json);
}

@freezed
abstract class DeleteTasksDto with _$DeleteTasksDto {
  const factory DeleteTasksDto({required List<String> taskIds}) =
      _DeleteTasksDto;

  factory DeleteTasksDto.fromJson(Map<String, dynamic> json) =>
      _$DeleteTasksDtoFromJson(json);
}

// Response DTOs

@freezed
abstract class TaskResponseDto with _$TaskResponseDto {
  const factory TaskResponseDto({
    required String id,
    required String name,
    String? description,
    String? categoryId,
    int? scheduledDate,
    int? scheduledEndDate,
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
      scheduledEndDate: task.scheduledEndDate,
      completedAt: task.completedAt,
    );
  }
}

@freezed
abstract class CreateTaskResponseDto with _$CreateTaskResponseDto {
  const factory CreateTaskResponseDto({required String id}) =
      _CreateTaskResponseDto;

  factory CreateTaskResponseDto.fromJson(Map<String, dynamic> json) =>
      _$CreateTaskResponseDtoFromJson(json);
}

@freezed
abstract class UpdateTaskResponseDto with _$UpdateTaskResponseDto {
  const factory UpdateTaskResponseDto({required bool success}) =
      _UpdateTaskResponseDto;

  factory UpdateTaskResponseDto.fromJson(Map<String, dynamic> json) =>
      _$UpdateTaskResponseDtoFromJson(json);
}

@freezed
abstract class DeleteTasksResponseDto with _$DeleteTasksResponseDto {
  const factory DeleteTasksResponseDto({
    @JsonKey(name: 'deleted_ids') required List<String> deletedIds,
  }) = _DeleteTasksResponseDto;

  factory DeleteTasksResponseDto.fromJson(Map<String, dynamic> json) =>
      _$DeleteTasksResponseDtoFromJson(json);
}

@freezed
abstract class OrphanTasksResponseDto with _$OrphanTasksResponseDto {
  const factory OrphanTasksResponseDto({
    required List<TaskResponseDto> orphanTasks,
  }) = _OrphanTasksResponseDto;

  factory OrphanTasksResponseDto.fromJson(Map<String, dynamic> json) =>
      _$OrphanTasksResponseDtoFromJson(json);
}

@freezed
abstract class ScheduledTasksResponseDto with _$ScheduledTasksResponseDto {
  const factory ScheduledTasksResponseDto({
    required List<TaskResponseDto> tasks,
  }) = _ScheduledTasksResponseDto;

  factory ScheduledTasksResponseDto.fromJson(Map<String, dynamic> json) =>
      _$ScheduledTasksResponseDtoFromJson(json);
}
