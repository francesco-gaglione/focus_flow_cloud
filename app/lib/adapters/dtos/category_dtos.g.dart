// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'category_dtos.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_$CreateCategoryDtoImpl _$$CreateCategoryDtoImplFromJson(
  Map<String, dynamic> json,
) => _$CreateCategoryDtoImpl(
  name: json['name'] as String,
  color: json['color'] as String?,
  description: json['description'] as String?,
);

Map<String, dynamic> _$$CreateCategoryDtoImplToJson(
  _$CreateCategoryDtoImpl instance,
) => <String, dynamic>{
  'name': instance.name,
  'color': instance.color,
  'description': instance.description,
};

_$UpdateCategoryDtoImpl _$$UpdateCategoryDtoImplFromJson(
  Map<String, dynamic> json,
) => _$UpdateCategoryDtoImpl(
  name: json['name'] as String?,
  color: json['color'] as String?,
  description: json['description'] as String?,
);

Map<String, dynamic> _$$UpdateCategoryDtoImplToJson(
  _$UpdateCategoryDtoImpl instance,
) => <String, dynamic>{
  'name': instance.name,
  'color': instance.color,
  'description': instance.description,
};

_$TaskDtoImpl _$$TaskDtoImplFromJson(Map<String, dynamic> json) =>
    _$TaskDtoImpl(
      id: json['id'] as String,
      name: json['name'] as String,
      description: json['description'] as String?,
      categoryId: json['categoryId'] as String?,
      scheduledDate: (json['scheduledDate'] as num?)?.toInt(),
      completedAt: (json['completedAt'] as num?)?.toInt(),
    );

Map<String, dynamic> _$$TaskDtoImplToJson(_$TaskDtoImpl instance) =>
    <String, dynamic>{
      'id': instance.id,
      'name': instance.name,
      'description': instance.description,
      'categoryId': instance.categoryId,
      'scheduledDate': instance.scheduledDate,
      'completedAt': instance.completedAt,
    };

_$CategoryDtoImpl _$$CategoryDtoImplFromJson(Map<String, dynamic> json) =>
    _$CategoryDtoImpl(
      id: json['id'] as String,
      name: json['name'] as String,
      color: json['color'] as String,
      description: json['description'] as String?,
      tasks:
          (json['tasks'] as List<dynamic>)
              .map((e) => TaskDto.fromJson(e as Map<String, dynamic>))
              .toList(),
    );

Map<String, dynamic> _$$CategoryDtoImplToJson(_$CategoryDtoImpl instance) =>
    <String, dynamic>{
      'id': instance.id,
      'name': instance.name,
      'color': instance.color,
      'description': instance.description,
      'tasks': instance.tasks,
    };

_$GetCategoriesResponseDtoImpl _$$GetCategoriesResponseDtoImplFromJson(
  Map<String, dynamic> json,
) => _$GetCategoriesResponseDtoImpl(
  categories:
      (json['categories'] as List<dynamic>)
          .map((e) => CategoryDto.fromJson(e as Map<String, dynamic>))
          .toList(),
);

Map<String, dynamic> _$$GetCategoriesResponseDtoImplToJson(
  _$GetCategoriesResponseDtoImpl instance,
) => <String, dynamic>{'categories': instance.categories};

_$CreateCategoryResponseDtoImpl _$$CreateCategoryResponseDtoImplFromJson(
  Map<String, dynamic> json,
) => _$CreateCategoryResponseDtoImpl(created: json['created'] as bool);

Map<String, dynamic> _$$CreateCategoryResponseDtoImplToJson(
  _$CreateCategoryResponseDtoImpl instance,
) => <String, dynamic>{'created': instance.created};

_$UpdateCategoryResponseDtoImpl _$$UpdateCategoryResponseDtoImplFromJson(
  Map<String, dynamic> json,
) => _$UpdateCategoryResponseDtoImpl(
  updatedCategory: CategoryDto.fromJson(
    json['updatedCategory'] as Map<String, dynamic>,
  ),
);

Map<String, dynamic> _$$UpdateCategoryResponseDtoImplToJson(
  _$UpdateCategoryResponseDtoImpl instance,
) => <String, dynamic>{'updatedCategory': instance.updatedCategory};

_$DeleteCategoriesResponseDtoImpl _$$DeleteCategoriesResponseDtoImplFromJson(
  Map<String, dynamic> json,
) => _$DeleteCategoriesResponseDtoImpl(
  deletedIds:
      (json['deleted_ids'] as List<dynamic>).map((e) => e as String).toList(),
);

Map<String, dynamic> _$$DeleteCategoriesResponseDtoImplToJson(
  _$DeleteCategoriesResponseDtoImpl instance,
) => <String, dynamic>{'deleted_ids': instance.deletedIds};
