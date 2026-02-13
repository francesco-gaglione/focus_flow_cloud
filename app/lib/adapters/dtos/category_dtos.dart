// ignore_for_file: invalid_annotation_target
import 'package:freezed_annotation/freezed_annotation.dart';
import '../../domain/entities/category.dart';
import '../../domain/entities/task.dart';

part 'category_dtos.freezed.dart';
part 'category_dtos.g.dart';

// Request DTOs

@freezed
abstract class CreateCategoryDto with _$CreateCategoryDto {
  const factory CreateCategoryDto({
    required String name,
    String? color,
    String? description,
  }) = _CreateCategoryDto;

  factory CreateCategoryDto.fromJson(Map<String, dynamic> json) =>
      _$CreateCategoryDtoFromJson(json);
}

@freezed
abstract class UpdateCategoryDto with _$UpdateCategoryDto {
  const factory UpdateCategoryDto({
    String? name,
    String? color,
    String? description,
  }) = _UpdateCategoryDto;

  factory UpdateCategoryDto.fromJson(Map<String, dynamic> json) =>
      _$UpdateCategoryDtoFromJson(json);
}

// Response DTOs

@freezed
abstract class TaskDto with _$TaskDto {
  const factory TaskDto({
    required String id,
    required String name,
    String? description,
    String? categoryId,
    int? scheduledDate,
    int? completedAt,
  }) = _TaskDto;

  factory TaskDto.fromJson(Map<String, dynamic> json) =>
      _$TaskDtoFromJson(json);

  factory TaskDto.fromEntity(Task task) {
    return TaskDto(
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
abstract class CategoryDto with _$CategoryDto {
  const factory CategoryDto({
    required String id,
    required String name,
    required String color,
    String? description,
    required List<TaskDto> tasks,
  }) = _CategoryDto;

  factory CategoryDto.fromJson(Map<String, dynamic> json) =>
      _$CategoryDtoFromJson(json);

  factory CategoryDto.fromEntity(Category category, List<Task> tasks) {
    return CategoryDto(
      id: category.id,
      name: category.name,
      color: category.color,
      description: category.description,
      tasks: tasks.map((task) => TaskDto.fromEntity(task)).toList(),
    );
  }
}

@freezed
abstract class GetCategoriesResponseDto with _$GetCategoriesResponseDto {
  const factory GetCategoriesResponseDto({
    required List<CategoryDto> categories,
  }) = _GetCategoriesResponseDto;

  factory GetCategoriesResponseDto.fromJson(Map<String, dynamic> json) =>
      _$GetCategoriesResponseDtoFromJson(json);
}

@freezed
abstract class CreateCategoryResponseDto with _$CreateCategoryResponseDto {
  const factory CreateCategoryResponseDto({required bool created}) =
      _CreateCategoryResponseDto;

  factory CreateCategoryResponseDto.fromJson(Map<String, dynamic> json) =>
      _$CreateCategoryResponseDtoFromJson(json);
}

@freezed
abstract class UpdateCategoryResponseDto with _$UpdateCategoryResponseDto {
  const factory UpdateCategoryResponseDto({
    required CategoryDto updatedCategory,
  }) = _UpdateCategoryResponseDto;

  factory UpdateCategoryResponseDto.fromJson(Map<String, dynamic> json) =>
      _$UpdateCategoryResponseDtoFromJson(json);
}

@freezed
abstract class DeleteCategoriesResponseDto with _$DeleteCategoriesResponseDto {
  const factory DeleteCategoriesResponseDto({
    @JsonKey(name: 'deleted_ids') required List<String> deletedIds,
  }) = _DeleteCategoriesResponseDto;

  factory DeleteCategoriesResponseDto.fromJson(Map<String, dynamic> json) =>
      _$DeleteCategoriesResponseDtoFromJson(json);
}
