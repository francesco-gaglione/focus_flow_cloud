// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'category_dtos.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_CreateCategoryDto _$CreateCategoryDtoFromJson(Map<String, dynamic> json) =>
    _CreateCategoryDto(
      name: json['name'] as String,
      color: json['color'] as String?,
      description: json['description'] as String?,
    );

Map<String, dynamic> _$CreateCategoryDtoToJson(_CreateCategoryDto instance) =>
    <String, dynamic>{
      'name': instance.name,
      'color': instance.color,
      'description': instance.description,
    };

_UpdateCategoryDto _$UpdateCategoryDtoFromJson(Map<String, dynamic> json) =>
    _UpdateCategoryDto(
      name: json['name'] as String?,
      color: json['color'] as String?,
      description: json['description'] as String?,
    );

Map<String, dynamic> _$UpdateCategoryDtoToJson(_UpdateCategoryDto instance) =>
    <String, dynamic>{
      'name': instance.name,
      'color': instance.color,
      'description': instance.description,
    };

_TaskDto _$TaskDtoFromJson(Map<String, dynamic> json) => _TaskDto(
  id: json['id'] as String,
  name: json['name'] as String,
  description: json['description'] as String?,
  categoryId: json['categoryId'] as String?,
  scheduledDate: (json['scheduledDate'] as num?)?.toInt(),
  completedAt: (json['completedAt'] as num?)?.toInt(),
);

Map<String, dynamic> _$TaskDtoToJson(_TaskDto instance) => <String, dynamic>{
  'id': instance.id,
  'name': instance.name,
  'description': instance.description,
  'categoryId': instance.categoryId,
  'scheduledDate': instance.scheduledDate,
  'completedAt': instance.completedAt,
};

_CategoryDto _$CategoryDtoFromJson(Map<String, dynamic> json) => _CategoryDto(
  id: json['id'] as String,
  name: json['name'] as String,
  color: json['color'] as String,
  description: json['description'] as String?,
  tasks:
      (json['tasks'] as List<dynamic>)
          .map((e) => TaskDto.fromJson(e as Map<String, dynamic>))
          .toList(),
);

Map<String, dynamic> _$CategoryDtoToJson(_CategoryDto instance) =>
    <String, dynamic>{
      'id': instance.id,
      'name': instance.name,
      'color': instance.color,
      'description': instance.description,
      'tasks': instance.tasks,
    };

_GetCategoriesResponseDto _$GetCategoriesResponseDtoFromJson(
  Map<String, dynamic> json,
) => _GetCategoriesResponseDto(
  categories:
      (json['categories'] as List<dynamic>)
          .map((e) => CategoryDto.fromJson(e as Map<String, dynamic>))
          .toList(),
);

Map<String, dynamic> _$GetCategoriesResponseDtoToJson(
  _GetCategoriesResponseDto instance,
) => <String, dynamic>{'categories': instance.categories};

_CreateCategoryResponseDto _$CreateCategoryResponseDtoFromJson(
  Map<String, dynamic> json,
) => _CreateCategoryResponseDto(created: json['created'] as bool);

Map<String, dynamic> _$CreateCategoryResponseDtoToJson(
  _CreateCategoryResponseDto instance,
) => <String, dynamic>{'created': instance.created};

_UpdateCategoryResponseDto _$UpdateCategoryResponseDtoFromJson(
  Map<String, dynamic> json,
) => _UpdateCategoryResponseDto(
  updatedCategory: CategoryDto.fromJson(
    json['updatedCategory'] as Map<String, dynamic>,
  ),
);

Map<String, dynamic> _$UpdateCategoryResponseDtoToJson(
  _UpdateCategoryResponseDto instance,
) => <String, dynamic>{'updatedCategory': instance.updatedCategory};

_DeleteCategoriesResponseDto _$DeleteCategoriesResponseDtoFromJson(
  Map<String, dynamic> json,
) => _DeleteCategoriesResponseDto(
  deletedIds:
      (json['deleted_ids'] as List<dynamic>).map((e) => e as String).toList(),
);

Map<String, dynamic> _$DeleteCategoriesResponseDtoToJson(
  _DeleteCategoriesResponseDto instance,
) => <String, dynamic>{'deleted_ids': instance.deletedIds};
