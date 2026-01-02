// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'task_dtos.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
  'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models',
);

CreateTaskDto _$CreateTaskDtoFromJson(Map<String, dynamic> json) {
  return _CreateTaskDto.fromJson(json);
}

/// @nodoc
mixin _$CreateTaskDto {
  String get name => throw _privateConstructorUsedError;
  String? get description => throw _privateConstructorUsedError;
  String? get categoryId => throw _privateConstructorUsedError;
  int? get scheduledDate => throw _privateConstructorUsedError;

  /// Serializes this CreateTaskDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of CreateTaskDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $CreateTaskDtoCopyWith<CreateTaskDto> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CreateTaskDtoCopyWith<$Res> {
  factory $CreateTaskDtoCopyWith(
    CreateTaskDto value,
    $Res Function(CreateTaskDto) then,
  ) = _$CreateTaskDtoCopyWithImpl<$Res, CreateTaskDto>;
  @useResult
  $Res call({
    String name,
    String? description,
    String? categoryId,
    int? scheduledDate,
  });
}

/// @nodoc
class _$CreateTaskDtoCopyWithImpl<$Res, $Val extends CreateTaskDto>
    implements $CreateTaskDtoCopyWith<$Res> {
  _$CreateTaskDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of CreateTaskDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? name = null,
    Object? description = freezed,
    Object? categoryId = freezed,
    Object? scheduledDate = freezed,
  }) {
    return _then(
      _value.copyWith(
            name:
                null == name
                    ? _value.name
                    : name // ignore: cast_nullable_to_non_nullable
                        as String,
            description:
                freezed == description
                    ? _value.description
                    : description // ignore: cast_nullable_to_non_nullable
                        as String?,
            categoryId:
                freezed == categoryId
                    ? _value.categoryId
                    : categoryId // ignore: cast_nullable_to_non_nullable
                        as String?,
            scheduledDate:
                freezed == scheduledDate
                    ? _value.scheduledDate
                    : scheduledDate // ignore: cast_nullable_to_non_nullable
                        as int?,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$CreateTaskDtoImplCopyWith<$Res>
    implements $CreateTaskDtoCopyWith<$Res> {
  factory _$$CreateTaskDtoImplCopyWith(
    _$CreateTaskDtoImpl value,
    $Res Function(_$CreateTaskDtoImpl) then,
  ) = __$$CreateTaskDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({
    String name,
    String? description,
    String? categoryId,
    int? scheduledDate,
  });
}

/// @nodoc
class __$$CreateTaskDtoImplCopyWithImpl<$Res>
    extends _$CreateTaskDtoCopyWithImpl<$Res, _$CreateTaskDtoImpl>
    implements _$$CreateTaskDtoImplCopyWith<$Res> {
  __$$CreateTaskDtoImplCopyWithImpl(
    _$CreateTaskDtoImpl _value,
    $Res Function(_$CreateTaskDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of CreateTaskDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? name = null,
    Object? description = freezed,
    Object? categoryId = freezed,
    Object? scheduledDate = freezed,
  }) {
    return _then(
      _$CreateTaskDtoImpl(
        name:
            null == name
                ? _value.name
                : name // ignore: cast_nullable_to_non_nullable
                    as String,
        description:
            freezed == description
                ? _value.description
                : description // ignore: cast_nullable_to_non_nullable
                    as String?,
        categoryId:
            freezed == categoryId
                ? _value.categoryId
                : categoryId // ignore: cast_nullable_to_non_nullable
                    as String?,
        scheduledDate:
            freezed == scheduledDate
                ? _value.scheduledDate
                : scheduledDate // ignore: cast_nullable_to_non_nullable
                    as int?,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$CreateTaskDtoImpl implements _CreateTaskDto {
  const _$CreateTaskDtoImpl({
    required this.name,
    this.description,
    this.categoryId,
    this.scheduledDate,
  });

  factory _$CreateTaskDtoImpl.fromJson(Map<String, dynamic> json) =>
      _$$CreateTaskDtoImplFromJson(json);

  @override
  final String name;
  @override
  final String? description;
  @override
  final String? categoryId;
  @override
  final int? scheduledDate;

  @override
  String toString() {
    return 'CreateTaskDto(name: $name, description: $description, categoryId: $categoryId, scheduledDate: $scheduledDate)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CreateTaskDtoImpl &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.description, description) ||
                other.description == description) &&
            (identical(other.categoryId, categoryId) ||
                other.categoryId == categoryId) &&
            (identical(other.scheduledDate, scheduledDate) ||
                other.scheduledDate == scheduledDate));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode =>
      Object.hash(runtimeType, name, description, categoryId, scheduledDate);

  /// Create a copy of CreateTaskDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$CreateTaskDtoImplCopyWith<_$CreateTaskDtoImpl> get copyWith =>
      __$$CreateTaskDtoImplCopyWithImpl<_$CreateTaskDtoImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$CreateTaskDtoImplToJson(this);
  }
}

abstract class _CreateTaskDto implements CreateTaskDto {
  const factory _CreateTaskDto({
    required final String name,
    final String? description,
    final String? categoryId,
    final int? scheduledDate,
  }) = _$CreateTaskDtoImpl;

  factory _CreateTaskDto.fromJson(Map<String, dynamic> json) =
      _$CreateTaskDtoImpl.fromJson;

  @override
  String get name;
  @override
  String? get description;
  @override
  String? get categoryId;
  @override
  int? get scheduledDate;

  /// Create a copy of CreateTaskDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$CreateTaskDtoImplCopyWith<_$CreateTaskDtoImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

UpdateTaskDto _$UpdateTaskDtoFromJson(Map<String, dynamic> json) {
  return _UpdateTaskDto.fromJson(json);
}

/// @nodoc
mixin _$UpdateTaskDto {
  String? get name => throw _privateConstructorUsedError;
  String? get description => throw _privateConstructorUsedError;
  String? get categoryId => throw _privateConstructorUsedError;
  int? get scheduledDate => throw _privateConstructorUsedError;
  int? get completedAt => throw _privateConstructorUsedError;

  /// Serializes this UpdateTaskDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of UpdateTaskDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $UpdateTaskDtoCopyWith<UpdateTaskDto> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $UpdateTaskDtoCopyWith<$Res> {
  factory $UpdateTaskDtoCopyWith(
    UpdateTaskDto value,
    $Res Function(UpdateTaskDto) then,
  ) = _$UpdateTaskDtoCopyWithImpl<$Res, UpdateTaskDto>;
  @useResult
  $Res call({
    String? name,
    String? description,
    String? categoryId,
    int? scheduledDate,
    int? completedAt,
  });
}

/// @nodoc
class _$UpdateTaskDtoCopyWithImpl<$Res, $Val extends UpdateTaskDto>
    implements $UpdateTaskDtoCopyWith<$Res> {
  _$UpdateTaskDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of UpdateTaskDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? name = freezed,
    Object? description = freezed,
    Object? categoryId = freezed,
    Object? scheduledDate = freezed,
    Object? completedAt = freezed,
  }) {
    return _then(
      _value.copyWith(
            name:
                freezed == name
                    ? _value.name
                    : name // ignore: cast_nullable_to_non_nullable
                        as String?,
            description:
                freezed == description
                    ? _value.description
                    : description // ignore: cast_nullable_to_non_nullable
                        as String?,
            categoryId:
                freezed == categoryId
                    ? _value.categoryId
                    : categoryId // ignore: cast_nullable_to_non_nullable
                        as String?,
            scheduledDate:
                freezed == scheduledDate
                    ? _value.scheduledDate
                    : scheduledDate // ignore: cast_nullable_to_non_nullable
                        as int?,
            completedAt:
                freezed == completedAt
                    ? _value.completedAt
                    : completedAt // ignore: cast_nullable_to_non_nullable
                        as int?,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$UpdateTaskDtoImplCopyWith<$Res>
    implements $UpdateTaskDtoCopyWith<$Res> {
  factory _$$UpdateTaskDtoImplCopyWith(
    _$UpdateTaskDtoImpl value,
    $Res Function(_$UpdateTaskDtoImpl) then,
  ) = __$$UpdateTaskDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({
    String? name,
    String? description,
    String? categoryId,
    int? scheduledDate,
    int? completedAt,
  });
}

/// @nodoc
class __$$UpdateTaskDtoImplCopyWithImpl<$Res>
    extends _$UpdateTaskDtoCopyWithImpl<$Res, _$UpdateTaskDtoImpl>
    implements _$$UpdateTaskDtoImplCopyWith<$Res> {
  __$$UpdateTaskDtoImplCopyWithImpl(
    _$UpdateTaskDtoImpl _value,
    $Res Function(_$UpdateTaskDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of UpdateTaskDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? name = freezed,
    Object? description = freezed,
    Object? categoryId = freezed,
    Object? scheduledDate = freezed,
    Object? completedAt = freezed,
  }) {
    return _then(
      _$UpdateTaskDtoImpl(
        name:
            freezed == name
                ? _value.name
                : name // ignore: cast_nullable_to_non_nullable
                    as String?,
        description:
            freezed == description
                ? _value.description
                : description // ignore: cast_nullable_to_non_nullable
                    as String?,
        categoryId:
            freezed == categoryId
                ? _value.categoryId
                : categoryId // ignore: cast_nullable_to_non_nullable
                    as String?,
        scheduledDate:
            freezed == scheduledDate
                ? _value.scheduledDate
                : scheduledDate // ignore: cast_nullable_to_non_nullable
                    as int?,
        completedAt:
            freezed == completedAt
                ? _value.completedAt
                : completedAt // ignore: cast_nullable_to_non_nullable
                    as int?,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$UpdateTaskDtoImpl implements _UpdateTaskDto {
  const _$UpdateTaskDtoImpl({
    this.name,
    this.description,
    this.categoryId,
    this.scheduledDate,
    this.completedAt,
  });

  factory _$UpdateTaskDtoImpl.fromJson(Map<String, dynamic> json) =>
      _$$UpdateTaskDtoImplFromJson(json);

  @override
  final String? name;
  @override
  final String? description;
  @override
  final String? categoryId;
  @override
  final int? scheduledDate;
  @override
  final int? completedAt;

  @override
  String toString() {
    return 'UpdateTaskDto(name: $name, description: $description, categoryId: $categoryId, scheduledDate: $scheduledDate, completedAt: $completedAt)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$UpdateTaskDtoImpl &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.description, description) ||
                other.description == description) &&
            (identical(other.categoryId, categoryId) ||
                other.categoryId == categoryId) &&
            (identical(other.scheduledDate, scheduledDate) ||
                other.scheduledDate == scheduledDate) &&
            (identical(other.completedAt, completedAt) ||
                other.completedAt == completedAt));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
    runtimeType,
    name,
    description,
    categoryId,
    scheduledDate,
    completedAt,
  );

  /// Create a copy of UpdateTaskDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$UpdateTaskDtoImplCopyWith<_$UpdateTaskDtoImpl> get copyWith =>
      __$$UpdateTaskDtoImplCopyWithImpl<_$UpdateTaskDtoImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$UpdateTaskDtoImplToJson(this);
  }
}

abstract class _UpdateTaskDto implements UpdateTaskDto {
  const factory _UpdateTaskDto({
    final String? name,
    final String? description,
    final String? categoryId,
    final int? scheduledDate,
    final int? completedAt,
  }) = _$UpdateTaskDtoImpl;

  factory _UpdateTaskDto.fromJson(Map<String, dynamic> json) =
      _$UpdateTaskDtoImpl.fromJson;

  @override
  String? get name;
  @override
  String? get description;
  @override
  String? get categoryId;
  @override
  int? get scheduledDate;
  @override
  int? get completedAt;

  /// Create a copy of UpdateTaskDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$UpdateTaskDtoImplCopyWith<_$UpdateTaskDtoImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

DeleteTasksDto _$DeleteTasksDtoFromJson(Map<String, dynamic> json) {
  return _DeleteTasksDto.fromJson(json);
}

/// @nodoc
mixin _$DeleteTasksDto {
  List<String> get taskIds => throw _privateConstructorUsedError;

  /// Serializes this DeleteTasksDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of DeleteTasksDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $DeleteTasksDtoCopyWith<DeleteTasksDto> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $DeleteTasksDtoCopyWith<$Res> {
  factory $DeleteTasksDtoCopyWith(
    DeleteTasksDto value,
    $Res Function(DeleteTasksDto) then,
  ) = _$DeleteTasksDtoCopyWithImpl<$Res, DeleteTasksDto>;
  @useResult
  $Res call({List<String> taskIds});
}

/// @nodoc
class _$DeleteTasksDtoCopyWithImpl<$Res, $Val extends DeleteTasksDto>
    implements $DeleteTasksDtoCopyWith<$Res> {
  _$DeleteTasksDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of DeleteTasksDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? taskIds = null}) {
    return _then(
      _value.copyWith(
            taskIds:
                null == taskIds
                    ? _value.taskIds
                    : taskIds // ignore: cast_nullable_to_non_nullable
                        as List<String>,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$DeleteTasksDtoImplCopyWith<$Res>
    implements $DeleteTasksDtoCopyWith<$Res> {
  factory _$$DeleteTasksDtoImplCopyWith(
    _$DeleteTasksDtoImpl value,
    $Res Function(_$DeleteTasksDtoImpl) then,
  ) = __$$DeleteTasksDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({List<String> taskIds});
}

/// @nodoc
class __$$DeleteTasksDtoImplCopyWithImpl<$Res>
    extends _$DeleteTasksDtoCopyWithImpl<$Res, _$DeleteTasksDtoImpl>
    implements _$$DeleteTasksDtoImplCopyWith<$Res> {
  __$$DeleteTasksDtoImplCopyWithImpl(
    _$DeleteTasksDtoImpl _value,
    $Res Function(_$DeleteTasksDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of DeleteTasksDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? taskIds = null}) {
    return _then(
      _$DeleteTasksDtoImpl(
        taskIds:
            null == taskIds
                ? _value._taskIds
                : taskIds // ignore: cast_nullable_to_non_nullable
                    as List<String>,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$DeleteTasksDtoImpl implements _DeleteTasksDto {
  const _$DeleteTasksDtoImpl({required final List<String> taskIds})
    : _taskIds = taskIds;

  factory _$DeleteTasksDtoImpl.fromJson(Map<String, dynamic> json) =>
      _$$DeleteTasksDtoImplFromJson(json);

  final List<String> _taskIds;
  @override
  List<String> get taskIds {
    if (_taskIds is EqualUnmodifiableListView) return _taskIds;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_taskIds);
  }

  @override
  String toString() {
    return 'DeleteTasksDto(taskIds: $taskIds)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DeleteTasksDtoImpl &&
            const DeepCollectionEquality().equals(other._taskIds, _taskIds));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(_taskIds));

  /// Create a copy of DeleteTasksDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$DeleteTasksDtoImplCopyWith<_$DeleteTasksDtoImpl> get copyWith =>
      __$$DeleteTasksDtoImplCopyWithImpl<_$DeleteTasksDtoImpl>(
        this,
        _$identity,
      );

  @override
  Map<String, dynamic> toJson() {
    return _$$DeleteTasksDtoImplToJson(this);
  }
}

abstract class _DeleteTasksDto implements DeleteTasksDto {
  const factory _DeleteTasksDto({required final List<String> taskIds}) =
      _$DeleteTasksDtoImpl;

  factory _DeleteTasksDto.fromJson(Map<String, dynamic> json) =
      _$DeleteTasksDtoImpl.fromJson;

  @override
  List<String> get taskIds;

  /// Create a copy of DeleteTasksDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$DeleteTasksDtoImplCopyWith<_$DeleteTasksDtoImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

TaskResponseDto _$TaskResponseDtoFromJson(Map<String, dynamic> json) {
  return _TaskResponseDto.fromJson(json);
}

/// @nodoc
mixin _$TaskResponseDto {
  String get id => throw _privateConstructorUsedError;
  String get name => throw _privateConstructorUsedError;
  String? get description => throw _privateConstructorUsedError;
  String? get categoryId => throw _privateConstructorUsedError;
  int? get scheduledDate => throw _privateConstructorUsedError;
  int? get completedAt => throw _privateConstructorUsedError;

  /// Serializes this TaskResponseDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of TaskResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $TaskResponseDtoCopyWith<TaskResponseDto> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $TaskResponseDtoCopyWith<$Res> {
  factory $TaskResponseDtoCopyWith(
    TaskResponseDto value,
    $Res Function(TaskResponseDto) then,
  ) = _$TaskResponseDtoCopyWithImpl<$Res, TaskResponseDto>;
  @useResult
  $Res call({
    String id,
    String name,
    String? description,
    String? categoryId,
    int? scheduledDate,
    int? completedAt,
  });
}

/// @nodoc
class _$TaskResponseDtoCopyWithImpl<$Res, $Val extends TaskResponseDto>
    implements $TaskResponseDtoCopyWith<$Res> {
  _$TaskResponseDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of TaskResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? name = null,
    Object? description = freezed,
    Object? categoryId = freezed,
    Object? scheduledDate = freezed,
    Object? completedAt = freezed,
  }) {
    return _then(
      _value.copyWith(
            id:
                null == id
                    ? _value.id
                    : id // ignore: cast_nullable_to_non_nullable
                        as String,
            name:
                null == name
                    ? _value.name
                    : name // ignore: cast_nullable_to_non_nullable
                        as String,
            description:
                freezed == description
                    ? _value.description
                    : description // ignore: cast_nullable_to_non_nullable
                        as String?,
            categoryId:
                freezed == categoryId
                    ? _value.categoryId
                    : categoryId // ignore: cast_nullable_to_non_nullable
                        as String?,
            scheduledDate:
                freezed == scheduledDate
                    ? _value.scheduledDate
                    : scheduledDate // ignore: cast_nullable_to_non_nullable
                        as int?,
            completedAt:
                freezed == completedAt
                    ? _value.completedAt
                    : completedAt // ignore: cast_nullable_to_non_nullable
                        as int?,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$TaskResponseDtoImplCopyWith<$Res>
    implements $TaskResponseDtoCopyWith<$Res> {
  factory _$$TaskResponseDtoImplCopyWith(
    _$TaskResponseDtoImpl value,
    $Res Function(_$TaskResponseDtoImpl) then,
  ) = __$$TaskResponseDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({
    String id,
    String name,
    String? description,
    String? categoryId,
    int? scheduledDate,
    int? completedAt,
  });
}

/// @nodoc
class __$$TaskResponseDtoImplCopyWithImpl<$Res>
    extends _$TaskResponseDtoCopyWithImpl<$Res, _$TaskResponseDtoImpl>
    implements _$$TaskResponseDtoImplCopyWith<$Res> {
  __$$TaskResponseDtoImplCopyWithImpl(
    _$TaskResponseDtoImpl _value,
    $Res Function(_$TaskResponseDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of TaskResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? name = null,
    Object? description = freezed,
    Object? categoryId = freezed,
    Object? scheduledDate = freezed,
    Object? completedAt = freezed,
  }) {
    return _then(
      _$TaskResponseDtoImpl(
        id:
            null == id
                ? _value.id
                : id // ignore: cast_nullable_to_non_nullable
                    as String,
        name:
            null == name
                ? _value.name
                : name // ignore: cast_nullable_to_non_nullable
                    as String,
        description:
            freezed == description
                ? _value.description
                : description // ignore: cast_nullable_to_non_nullable
                    as String?,
        categoryId:
            freezed == categoryId
                ? _value.categoryId
                : categoryId // ignore: cast_nullable_to_non_nullable
                    as String?,
        scheduledDate:
            freezed == scheduledDate
                ? _value.scheduledDate
                : scheduledDate // ignore: cast_nullable_to_non_nullable
                    as int?,
        completedAt:
            freezed == completedAt
                ? _value.completedAt
                : completedAt // ignore: cast_nullable_to_non_nullable
                    as int?,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$TaskResponseDtoImpl implements _TaskResponseDto {
  const _$TaskResponseDtoImpl({
    required this.id,
    required this.name,
    this.description,
    this.categoryId,
    this.scheduledDate,
    this.completedAt,
  });

  factory _$TaskResponseDtoImpl.fromJson(Map<String, dynamic> json) =>
      _$$TaskResponseDtoImplFromJson(json);

  @override
  final String id;
  @override
  final String name;
  @override
  final String? description;
  @override
  final String? categoryId;
  @override
  final int? scheduledDate;
  @override
  final int? completedAt;

  @override
  String toString() {
    return 'TaskResponseDto(id: $id, name: $name, description: $description, categoryId: $categoryId, scheduledDate: $scheduledDate, completedAt: $completedAt)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$TaskResponseDtoImpl &&
            (identical(other.id, id) || other.id == id) &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.description, description) ||
                other.description == description) &&
            (identical(other.categoryId, categoryId) ||
                other.categoryId == categoryId) &&
            (identical(other.scheduledDate, scheduledDate) ||
                other.scheduledDate == scheduledDate) &&
            (identical(other.completedAt, completedAt) ||
                other.completedAt == completedAt));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
    runtimeType,
    id,
    name,
    description,
    categoryId,
    scheduledDate,
    completedAt,
  );

  /// Create a copy of TaskResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$TaskResponseDtoImplCopyWith<_$TaskResponseDtoImpl> get copyWith =>
      __$$TaskResponseDtoImplCopyWithImpl<_$TaskResponseDtoImpl>(
        this,
        _$identity,
      );

  @override
  Map<String, dynamic> toJson() {
    return _$$TaskResponseDtoImplToJson(this);
  }
}

abstract class _TaskResponseDto implements TaskResponseDto {
  const factory _TaskResponseDto({
    required final String id,
    required final String name,
    final String? description,
    final String? categoryId,
    final int? scheduledDate,
    final int? completedAt,
  }) = _$TaskResponseDtoImpl;

  factory _TaskResponseDto.fromJson(Map<String, dynamic> json) =
      _$TaskResponseDtoImpl.fromJson;

  @override
  String get id;
  @override
  String get name;
  @override
  String? get description;
  @override
  String? get categoryId;
  @override
  int? get scheduledDate;
  @override
  int? get completedAt;

  /// Create a copy of TaskResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$TaskResponseDtoImplCopyWith<_$TaskResponseDtoImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

CreateTaskResponseDto _$CreateTaskResponseDtoFromJson(
  Map<String, dynamic> json,
) {
  return _CreateTaskResponseDto.fromJson(json);
}

/// @nodoc
mixin _$CreateTaskResponseDto {
  String get id => throw _privateConstructorUsedError;

  /// Serializes this CreateTaskResponseDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of CreateTaskResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $CreateTaskResponseDtoCopyWith<CreateTaskResponseDto> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CreateTaskResponseDtoCopyWith<$Res> {
  factory $CreateTaskResponseDtoCopyWith(
    CreateTaskResponseDto value,
    $Res Function(CreateTaskResponseDto) then,
  ) = _$CreateTaskResponseDtoCopyWithImpl<$Res, CreateTaskResponseDto>;
  @useResult
  $Res call({String id});
}

/// @nodoc
class _$CreateTaskResponseDtoCopyWithImpl<
  $Res,
  $Val extends CreateTaskResponseDto
>
    implements $CreateTaskResponseDtoCopyWith<$Res> {
  _$CreateTaskResponseDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of CreateTaskResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? id = null}) {
    return _then(
      _value.copyWith(
            id:
                null == id
                    ? _value.id
                    : id // ignore: cast_nullable_to_non_nullable
                        as String,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$CreateTaskResponseDtoImplCopyWith<$Res>
    implements $CreateTaskResponseDtoCopyWith<$Res> {
  factory _$$CreateTaskResponseDtoImplCopyWith(
    _$CreateTaskResponseDtoImpl value,
    $Res Function(_$CreateTaskResponseDtoImpl) then,
  ) = __$$CreateTaskResponseDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String id});
}

/// @nodoc
class __$$CreateTaskResponseDtoImplCopyWithImpl<$Res>
    extends
        _$CreateTaskResponseDtoCopyWithImpl<$Res, _$CreateTaskResponseDtoImpl>
    implements _$$CreateTaskResponseDtoImplCopyWith<$Res> {
  __$$CreateTaskResponseDtoImplCopyWithImpl(
    _$CreateTaskResponseDtoImpl _value,
    $Res Function(_$CreateTaskResponseDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of CreateTaskResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? id = null}) {
    return _then(
      _$CreateTaskResponseDtoImpl(
        id:
            null == id
                ? _value.id
                : id // ignore: cast_nullable_to_non_nullable
                    as String,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$CreateTaskResponseDtoImpl implements _CreateTaskResponseDto {
  const _$CreateTaskResponseDtoImpl({required this.id});

  factory _$CreateTaskResponseDtoImpl.fromJson(Map<String, dynamic> json) =>
      _$$CreateTaskResponseDtoImplFromJson(json);

  @override
  final String id;

  @override
  String toString() {
    return 'CreateTaskResponseDto(id: $id)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CreateTaskResponseDtoImpl &&
            (identical(other.id, id) || other.id == id));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, id);

  /// Create a copy of CreateTaskResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$CreateTaskResponseDtoImplCopyWith<_$CreateTaskResponseDtoImpl>
  get copyWith =>
      __$$CreateTaskResponseDtoImplCopyWithImpl<_$CreateTaskResponseDtoImpl>(
        this,
        _$identity,
      );

  @override
  Map<String, dynamic> toJson() {
    return _$$CreateTaskResponseDtoImplToJson(this);
  }
}

abstract class _CreateTaskResponseDto implements CreateTaskResponseDto {
  const factory _CreateTaskResponseDto({required final String id}) =
      _$CreateTaskResponseDtoImpl;

  factory _CreateTaskResponseDto.fromJson(Map<String, dynamic> json) =
      _$CreateTaskResponseDtoImpl.fromJson;

  @override
  String get id;

  /// Create a copy of CreateTaskResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$CreateTaskResponseDtoImplCopyWith<_$CreateTaskResponseDtoImpl>
  get copyWith => throw _privateConstructorUsedError;
}

UpdateTaskResponseDto _$UpdateTaskResponseDtoFromJson(
  Map<String, dynamic> json,
) {
  return _UpdateTaskResponseDto.fromJson(json);
}

/// @nodoc
mixin _$UpdateTaskResponseDto {
  TaskResponseDto get updatedTask => throw _privateConstructorUsedError;

  /// Serializes this UpdateTaskResponseDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of UpdateTaskResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $UpdateTaskResponseDtoCopyWith<UpdateTaskResponseDto> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $UpdateTaskResponseDtoCopyWith<$Res> {
  factory $UpdateTaskResponseDtoCopyWith(
    UpdateTaskResponseDto value,
    $Res Function(UpdateTaskResponseDto) then,
  ) = _$UpdateTaskResponseDtoCopyWithImpl<$Res, UpdateTaskResponseDto>;
  @useResult
  $Res call({TaskResponseDto updatedTask});

  $TaskResponseDtoCopyWith<$Res> get updatedTask;
}

/// @nodoc
class _$UpdateTaskResponseDtoCopyWithImpl<
  $Res,
  $Val extends UpdateTaskResponseDto
>
    implements $UpdateTaskResponseDtoCopyWith<$Res> {
  _$UpdateTaskResponseDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of UpdateTaskResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? updatedTask = null}) {
    return _then(
      _value.copyWith(
            updatedTask:
                null == updatedTask
                    ? _value.updatedTask
                    : updatedTask // ignore: cast_nullable_to_non_nullable
                        as TaskResponseDto,
          )
          as $Val,
    );
  }

  /// Create a copy of UpdateTaskResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $TaskResponseDtoCopyWith<$Res> get updatedTask {
    return $TaskResponseDtoCopyWith<$Res>(_value.updatedTask, (value) {
      return _then(_value.copyWith(updatedTask: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$UpdateTaskResponseDtoImplCopyWith<$Res>
    implements $UpdateTaskResponseDtoCopyWith<$Res> {
  factory _$$UpdateTaskResponseDtoImplCopyWith(
    _$UpdateTaskResponseDtoImpl value,
    $Res Function(_$UpdateTaskResponseDtoImpl) then,
  ) = __$$UpdateTaskResponseDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({TaskResponseDto updatedTask});

  @override
  $TaskResponseDtoCopyWith<$Res> get updatedTask;
}

/// @nodoc
class __$$UpdateTaskResponseDtoImplCopyWithImpl<$Res>
    extends
        _$UpdateTaskResponseDtoCopyWithImpl<$Res, _$UpdateTaskResponseDtoImpl>
    implements _$$UpdateTaskResponseDtoImplCopyWith<$Res> {
  __$$UpdateTaskResponseDtoImplCopyWithImpl(
    _$UpdateTaskResponseDtoImpl _value,
    $Res Function(_$UpdateTaskResponseDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of UpdateTaskResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? updatedTask = null}) {
    return _then(
      _$UpdateTaskResponseDtoImpl(
        updatedTask:
            null == updatedTask
                ? _value.updatedTask
                : updatedTask // ignore: cast_nullable_to_non_nullable
                    as TaskResponseDto,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$UpdateTaskResponseDtoImpl implements _UpdateTaskResponseDto {
  const _$UpdateTaskResponseDtoImpl({required this.updatedTask});

  factory _$UpdateTaskResponseDtoImpl.fromJson(Map<String, dynamic> json) =>
      _$$UpdateTaskResponseDtoImplFromJson(json);

  @override
  final TaskResponseDto updatedTask;

  @override
  String toString() {
    return 'UpdateTaskResponseDto(updatedTask: $updatedTask)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$UpdateTaskResponseDtoImpl &&
            (identical(other.updatedTask, updatedTask) ||
                other.updatedTask == updatedTask));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, updatedTask);

  /// Create a copy of UpdateTaskResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$UpdateTaskResponseDtoImplCopyWith<_$UpdateTaskResponseDtoImpl>
  get copyWith =>
      __$$UpdateTaskResponseDtoImplCopyWithImpl<_$UpdateTaskResponseDtoImpl>(
        this,
        _$identity,
      );

  @override
  Map<String, dynamic> toJson() {
    return _$$UpdateTaskResponseDtoImplToJson(this);
  }
}

abstract class _UpdateTaskResponseDto implements UpdateTaskResponseDto {
  const factory _UpdateTaskResponseDto({
    required final TaskResponseDto updatedTask,
  }) = _$UpdateTaskResponseDtoImpl;

  factory _UpdateTaskResponseDto.fromJson(Map<String, dynamic> json) =
      _$UpdateTaskResponseDtoImpl.fromJson;

  @override
  TaskResponseDto get updatedTask;

  /// Create a copy of UpdateTaskResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$UpdateTaskResponseDtoImplCopyWith<_$UpdateTaskResponseDtoImpl>
  get copyWith => throw _privateConstructorUsedError;
}

DeleteTasksResponseDto _$DeleteTasksResponseDtoFromJson(
  Map<String, dynamic> json,
) {
  return _DeleteTasksResponseDto.fromJson(json);
}

/// @nodoc
mixin _$DeleteTasksResponseDto {
  @JsonKey(name: 'deleted_ids')
  List<String> get deletedIds => throw _privateConstructorUsedError;

  /// Serializes this DeleteTasksResponseDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of DeleteTasksResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $DeleteTasksResponseDtoCopyWith<DeleteTasksResponseDto> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $DeleteTasksResponseDtoCopyWith<$Res> {
  factory $DeleteTasksResponseDtoCopyWith(
    DeleteTasksResponseDto value,
    $Res Function(DeleteTasksResponseDto) then,
  ) = _$DeleteTasksResponseDtoCopyWithImpl<$Res, DeleteTasksResponseDto>;
  @useResult
  $Res call({@JsonKey(name: 'deleted_ids') List<String> deletedIds});
}

/// @nodoc
class _$DeleteTasksResponseDtoCopyWithImpl<
  $Res,
  $Val extends DeleteTasksResponseDto
>
    implements $DeleteTasksResponseDtoCopyWith<$Res> {
  _$DeleteTasksResponseDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of DeleteTasksResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? deletedIds = null}) {
    return _then(
      _value.copyWith(
            deletedIds:
                null == deletedIds
                    ? _value.deletedIds
                    : deletedIds // ignore: cast_nullable_to_non_nullable
                        as List<String>,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$DeleteTasksResponseDtoImplCopyWith<$Res>
    implements $DeleteTasksResponseDtoCopyWith<$Res> {
  factory _$$DeleteTasksResponseDtoImplCopyWith(
    _$DeleteTasksResponseDtoImpl value,
    $Res Function(_$DeleteTasksResponseDtoImpl) then,
  ) = __$$DeleteTasksResponseDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({@JsonKey(name: 'deleted_ids') List<String> deletedIds});
}

/// @nodoc
class __$$DeleteTasksResponseDtoImplCopyWithImpl<$Res>
    extends
        _$DeleteTasksResponseDtoCopyWithImpl<$Res, _$DeleteTasksResponseDtoImpl>
    implements _$$DeleteTasksResponseDtoImplCopyWith<$Res> {
  __$$DeleteTasksResponseDtoImplCopyWithImpl(
    _$DeleteTasksResponseDtoImpl _value,
    $Res Function(_$DeleteTasksResponseDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of DeleteTasksResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? deletedIds = null}) {
    return _then(
      _$DeleteTasksResponseDtoImpl(
        deletedIds:
            null == deletedIds
                ? _value._deletedIds
                : deletedIds // ignore: cast_nullable_to_non_nullable
                    as List<String>,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$DeleteTasksResponseDtoImpl implements _DeleteTasksResponseDto {
  const _$DeleteTasksResponseDtoImpl({
    @JsonKey(name: 'deleted_ids') required final List<String> deletedIds,
  }) : _deletedIds = deletedIds;

  factory _$DeleteTasksResponseDtoImpl.fromJson(Map<String, dynamic> json) =>
      _$$DeleteTasksResponseDtoImplFromJson(json);

  final List<String> _deletedIds;
  @override
  @JsonKey(name: 'deleted_ids')
  List<String> get deletedIds {
    if (_deletedIds is EqualUnmodifiableListView) return _deletedIds;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_deletedIds);
  }

  @override
  String toString() {
    return 'DeleteTasksResponseDto(deletedIds: $deletedIds)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DeleteTasksResponseDtoImpl &&
            const DeepCollectionEquality().equals(
              other._deletedIds,
              _deletedIds,
            ));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
    runtimeType,
    const DeepCollectionEquality().hash(_deletedIds),
  );

  /// Create a copy of DeleteTasksResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$DeleteTasksResponseDtoImplCopyWith<_$DeleteTasksResponseDtoImpl>
  get copyWith =>
      __$$DeleteTasksResponseDtoImplCopyWithImpl<_$DeleteTasksResponseDtoImpl>(
        this,
        _$identity,
      );

  @override
  Map<String, dynamic> toJson() {
    return _$$DeleteTasksResponseDtoImplToJson(this);
  }
}

abstract class _DeleteTasksResponseDto implements DeleteTasksResponseDto {
  const factory _DeleteTasksResponseDto({
    @JsonKey(name: 'deleted_ids') required final List<String> deletedIds,
  }) = _$DeleteTasksResponseDtoImpl;

  factory _DeleteTasksResponseDto.fromJson(Map<String, dynamic> json) =
      _$DeleteTasksResponseDtoImpl.fromJson;

  @override
  @JsonKey(name: 'deleted_ids')
  List<String> get deletedIds;

  /// Create a copy of DeleteTasksResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$DeleteTasksResponseDtoImplCopyWith<_$DeleteTasksResponseDtoImpl>
  get copyWith => throw _privateConstructorUsedError;
}

OrphanTasksResponseDto _$OrphanTasksResponseDtoFromJson(
  Map<String, dynamic> json,
) {
  return _OrphanTasksResponseDto.fromJson(json);
}

/// @nodoc
mixin _$OrphanTasksResponseDto {
  List<TaskResponseDto> get orphanTasks => throw _privateConstructorUsedError;

  /// Serializes this OrphanTasksResponseDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of OrphanTasksResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $OrphanTasksResponseDtoCopyWith<OrphanTasksResponseDto> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $OrphanTasksResponseDtoCopyWith<$Res> {
  factory $OrphanTasksResponseDtoCopyWith(
    OrphanTasksResponseDto value,
    $Res Function(OrphanTasksResponseDto) then,
  ) = _$OrphanTasksResponseDtoCopyWithImpl<$Res, OrphanTasksResponseDto>;
  @useResult
  $Res call({List<TaskResponseDto> orphanTasks});
}

/// @nodoc
class _$OrphanTasksResponseDtoCopyWithImpl<
  $Res,
  $Val extends OrphanTasksResponseDto
>
    implements $OrphanTasksResponseDtoCopyWith<$Res> {
  _$OrphanTasksResponseDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of OrphanTasksResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? orphanTasks = null}) {
    return _then(
      _value.copyWith(
            orphanTasks:
                null == orphanTasks
                    ? _value.orphanTasks
                    : orphanTasks // ignore: cast_nullable_to_non_nullable
                        as List<TaskResponseDto>,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$OrphanTasksResponseDtoImplCopyWith<$Res>
    implements $OrphanTasksResponseDtoCopyWith<$Res> {
  factory _$$OrphanTasksResponseDtoImplCopyWith(
    _$OrphanTasksResponseDtoImpl value,
    $Res Function(_$OrphanTasksResponseDtoImpl) then,
  ) = __$$OrphanTasksResponseDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({List<TaskResponseDto> orphanTasks});
}

/// @nodoc
class __$$OrphanTasksResponseDtoImplCopyWithImpl<$Res>
    extends
        _$OrphanTasksResponseDtoCopyWithImpl<$Res, _$OrphanTasksResponseDtoImpl>
    implements _$$OrphanTasksResponseDtoImplCopyWith<$Res> {
  __$$OrphanTasksResponseDtoImplCopyWithImpl(
    _$OrphanTasksResponseDtoImpl _value,
    $Res Function(_$OrphanTasksResponseDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of OrphanTasksResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? orphanTasks = null}) {
    return _then(
      _$OrphanTasksResponseDtoImpl(
        orphanTasks:
            null == orphanTasks
                ? _value._orphanTasks
                : orphanTasks // ignore: cast_nullable_to_non_nullable
                    as List<TaskResponseDto>,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$OrphanTasksResponseDtoImpl implements _OrphanTasksResponseDto {
  const _$OrphanTasksResponseDtoImpl({
    required final List<TaskResponseDto> orphanTasks,
  }) : _orphanTasks = orphanTasks;

  factory _$OrphanTasksResponseDtoImpl.fromJson(Map<String, dynamic> json) =>
      _$$OrphanTasksResponseDtoImplFromJson(json);

  final List<TaskResponseDto> _orphanTasks;
  @override
  List<TaskResponseDto> get orphanTasks {
    if (_orphanTasks is EqualUnmodifiableListView) return _orphanTasks;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_orphanTasks);
  }

  @override
  String toString() {
    return 'OrphanTasksResponseDto(orphanTasks: $orphanTasks)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$OrphanTasksResponseDtoImpl &&
            const DeepCollectionEquality().equals(
              other._orphanTasks,
              _orphanTasks,
            ));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
    runtimeType,
    const DeepCollectionEquality().hash(_orphanTasks),
  );

  /// Create a copy of OrphanTasksResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$OrphanTasksResponseDtoImplCopyWith<_$OrphanTasksResponseDtoImpl>
  get copyWith =>
      __$$OrphanTasksResponseDtoImplCopyWithImpl<_$OrphanTasksResponseDtoImpl>(
        this,
        _$identity,
      );

  @override
  Map<String, dynamic> toJson() {
    return _$$OrphanTasksResponseDtoImplToJson(this);
  }
}

abstract class _OrphanTasksResponseDto implements OrphanTasksResponseDto {
  const factory _OrphanTasksResponseDto({
    required final List<TaskResponseDto> orphanTasks,
  }) = _$OrphanTasksResponseDtoImpl;

  factory _OrphanTasksResponseDto.fromJson(Map<String, dynamic> json) =
      _$OrphanTasksResponseDtoImpl.fromJson;

  @override
  List<TaskResponseDto> get orphanTasks;

  /// Create a copy of OrphanTasksResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$OrphanTasksResponseDtoImplCopyWith<_$OrphanTasksResponseDtoImpl>
  get copyWith => throw _privateConstructorUsedError;
}
