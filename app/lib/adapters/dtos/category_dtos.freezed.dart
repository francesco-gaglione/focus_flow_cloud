// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'category_dtos.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
  'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models',
);

CreateCategoryDto _$CreateCategoryDtoFromJson(Map<String, dynamic> json) {
  return _CreateCategoryDto.fromJson(json);
}

/// @nodoc
mixin _$CreateCategoryDto {
  String get name => throw _privateConstructorUsedError;
  String? get color => throw _privateConstructorUsedError;
  String? get description => throw _privateConstructorUsedError;

  /// Serializes this CreateCategoryDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of CreateCategoryDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $CreateCategoryDtoCopyWith<CreateCategoryDto> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CreateCategoryDtoCopyWith<$Res> {
  factory $CreateCategoryDtoCopyWith(
    CreateCategoryDto value,
    $Res Function(CreateCategoryDto) then,
  ) = _$CreateCategoryDtoCopyWithImpl<$Res, CreateCategoryDto>;
  @useResult
  $Res call({String name, String? color, String? description});
}

/// @nodoc
class _$CreateCategoryDtoCopyWithImpl<$Res, $Val extends CreateCategoryDto>
    implements $CreateCategoryDtoCopyWith<$Res> {
  _$CreateCategoryDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of CreateCategoryDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? name = null,
    Object? color = freezed,
    Object? description = freezed,
  }) {
    return _then(
      _value.copyWith(
            name:
                null == name
                    ? _value.name
                    : name // ignore: cast_nullable_to_non_nullable
                        as String,
            color:
                freezed == color
                    ? _value.color
                    : color // ignore: cast_nullable_to_non_nullable
                        as String?,
            description:
                freezed == description
                    ? _value.description
                    : description // ignore: cast_nullable_to_non_nullable
                        as String?,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$CreateCategoryDtoImplCopyWith<$Res>
    implements $CreateCategoryDtoCopyWith<$Res> {
  factory _$$CreateCategoryDtoImplCopyWith(
    _$CreateCategoryDtoImpl value,
    $Res Function(_$CreateCategoryDtoImpl) then,
  ) = __$$CreateCategoryDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String name, String? color, String? description});
}

/// @nodoc
class __$$CreateCategoryDtoImplCopyWithImpl<$Res>
    extends _$CreateCategoryDtoCopyWithImpl<$Res, _$CreateCategoryDtoImpl>
    implements _$$CreateCategoryDtoImplCopyWith<$Res> {
  __$$CreateCategoryDtoImplCopyWithImpl(
    _$CreateCategoryDtoImpl _value,
    $Res Function(_$CreateCategoryDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of CreateCategoryDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? name = null,
    Object? color = freezed,
    Object? description = freezed,
  }) {
    return _then(
      _$CreateCategoryDtoImpl(
        name:
            null == name
                ? _value.name
                : name // ignore: cast_nullable_to_non_nullable
                    as String,
        color:
            freezed == color
                ? _value.color
                : color // ignore: cast_nullable_to_non_nullable
                    as String?,
        description:
            freezed == description
                ? _value.description
                : description // ignore: cast_nullable_to_non_nullable
                    as String?,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$CreateCategoryDtoImpl implements _CreateCategoryDto {
  const _$CreateCategoryDtoImpl({
    required this.name,
    this.color,
    this.description,
  });

  factory _$CreateCategoryDtoImpl.fromJson(Map<String, dynamic> json) =>
      _$$CreateCategoryDtoImplFromJson(json);

  @override
  final String name;
  @override
  final String? color;
  @override
  final String? description;

  @override
  String toString() {
    return 'CreateCategoryDto(name: $name, color: $color, description: $description)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CreateCategoryDtoImpl &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.color, color) || other.color == color) &&
            (identical(other.description, description) ||
                other.description == description));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, name, color, description);

  /// Create a copy of CreateCategoryDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$CreateCategoryDtoImplCopyWith<_$CreateCategoryDtoImpl> get copyWith =>
      __$$CreateCategoryDtoImplCopyWithImpl<_$CreateCategoryDtoImpl>(
        this,
        _$identity,
      );

  @override
  Map<String, dynamic> toJson() {
    return _$$CreateCategoryDtoImplToJson(this);
  }
}

abstract class _CreateCategoryDto implements CreateCategoryDto {
  const factory _CreateCategoryDto({
    required final String name,
    final String? color,
    final String? description,
  }) = _$CreateCategoryDtoImpl;

  factory _CreateCategoryDto.fromJson(Map<String, dynamic> json) =
      _$CreateCategoryDtoImpl.fromJson;

  @override
  String get name;
  @override
  String? get color;
  @override
  String? get description;

  /// Create a copy of CreateCategoryDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$CreateCategoryDtoImplCopyWith<_$CreateCategoryDtoImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

UpdateCategoryDto _$UpdateCategoryDtoFromJson(Map<String, dynamic> json) {
  return _UpdateCategoryDto.fromJson(json);
}

/// @nodoc
mixin _$UpdateCategoryDto {
  String? get name => throw _privateConstructorUsedError;
  String? get color => throw _privateConstructorUsedError;
  String? get description => throw _privateConstructorUsedError;

  /// Serializes this UpdateCategoryDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of UpdateCategoryDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $UpdateCategoryDtoCopyWith<UpdateCategoryDto> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $UpdateCategoryDtoCopyWith<$Res> {
  factory $UpdateCategoryDtoCopyWith(
    UpdateCategoryDto value,
    $Res Function(UpdateCategoryDto) then,
  ) = _$UpdateCategoryDtoCopyWithImpl<$Res, UpdateCategoryDto>;
  @useResult
  $Res call({String? name, String? color, String? description});
}

/// @nodoc
class _$UpdateCategoryDtoCopyWithImpl<$Res, $Val extends UpdateCategoryDto>
    implements $UpdateCategoryDtoCopyWith<$Res> {
  _$UpdateCategoryDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of UpdateCategoryDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? name = freezed,
    Object? color = freezed,
    Object? description = freezed,
  }) {
    return _then(
      _value.copyWith(
            name:
                freezed == name
                    ? _value.name
                    : name // ignore: cast_nullable_to_non_nullable
                        as String?,
            color:
                freezed == color
                    ? _value.color
                    : color // ignore: cast_nullable_to_non_nullable
                        as String?,
            description:
                freezed == description
                    ? _value.description
                    : description // ignore: cast_nullable_to_non_nullable
                        as String?,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$UpdateCategoryDtoImplCopyWith<$Res>
    implements $UpdateCategoryDtoCopyWith<$Res> {
  factory _$$UpdateCategoryDtoImplCopyWith(
    _$UpdateCategoryDtoImpl value,
    $Res Function(_$UpdateCategoryDtoImpl) then,
  ) = __$$UpdateCategoryDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String? name, String? color, String? description});
}

/// @nodoc
class __$$UpdateCategoryDtoImplCopyWithImpl<$Res>
    extends _$UpdateCategoryDtoCopyWithImpl<$Res, _$UpdateCategoryDtoImpl>
    implements _$$UpdateCategoryDtoImplCopyWith<$Res> {
  __$$UpdateCategoryDtoImplCopyWithImpl(
    _$UpdateCategoryDtoImpl _value,
    $Res Function(_$UpdateCategoryDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of UpdateCategoryDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? name = freezed,
    Object? color = freezed,
    Object? description = freezed,
  }) {
    return _then(
      _$UpdateCategoryDtoImpl(
        name:
            freezed == name
                ? _value.name
                : name // ignore: cast_nullable_to_non_nullable
                    as String?,
        color:
            freezed == color
                ? _value.color
                : color // ignore: cast_nullable_to_non_nullable
                    as String?,
        description:
            freezed == description
                ? _value.description
                : description // ignore: cast_nullable_to_non_nullable
                    as String?,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$UpdateCategoryDtoImpl implements _UpdateCategoryDto {
  const _$UpdateCategoryDtoImpl({this.name, this.color, this.description});

  factory _$UpdateCategoryDtoImpl.fromJson(Map<String, dynamic> json) =>
      _$$UpdateCategoryDtoImplFromJson(json);

  @override
  final String? name;
  @override
  final String? color;
  @override
  final String? description;

  @override
  String toString() {
    return 'UpdateCategoryDto(name: $name, color: $color, description: $description)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$UpdateCategoryDtoImpl &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.color, color) || other.color == color) &&
            (identical(other.description, description) ||
                other.description == description));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, name, color, description);

  /// Create a copy of UpdateCategoryDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$UpdateCategoryDtoImplCopyWith<_$UpdateCategoryDtoImpl> get copyWith =>
      __$$UpdateCategoryDtoImplCopyWithImpl<_$UpdateCategoryDtoImpl>(
        this,
        _$identity,
      );

  @override
  Map<String, dynamic> toJson() {
    return _$$UpdateCategoryDtoImplToJson(this);
  }
}

abstract class _UpdateCategoryDto implements UpdateCategoryDto {
  const factory _UpdateCategoryDto({
    final String? name,
    final String? color,
    final String? description,
  }) = _$UpdateCategoryDtoImpl;

  factory _UpdateCategoryDto.fromJson(Map<String, dynamic> json) =
      _$UpdateCategoryDtoImpl.fromJson;

  @override
  String? get name;
  @override
  String? get color;
  @override
  String? get description;

  /// Create a copy of UpdateCategoryDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$UpdateCategoryDtoImplCopyWith<_$UpdateCategoryDtoImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

TaskDto _$TaskDtoFromJson(Map<String, dynamic> json) {
  return _TaskDto.fromJson(json);
}

/// @nodoc
mixin _$TaskDto {
  String get id => throw _privateConstructorUsedError;
  String get name => throw _privateConstructorUsedError;
  String? get description => throw _privateConstructorUsedError;
  String? get categoryId => throw _privateConstructorUsedError;
  int? get scheduledDate => throw _privateConstructorUsedError;
  int? get completedAt => throw _privateConstructorUsedError;

  /// Serializes this TaskDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of TaskDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $TaskDtoCopyWith<TaskDto> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $TaskDtoCopyWith<$Res> {
  factory $TaskDtoCopyWith(TaskDto value, $Res Function(TaskDto) then) =
      _$TaskDtoCopyWithImpl<$Res, TaskDto>;
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
class _$TaskDtoCopyWithImpl<$Res, $Val extends TaskDto>
    implements $TaskDtoCopyWith<$Res> {
  _$TaskDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of TaskDto
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
abstract class _$$TaskDtoImplCopyWith<$Res> implements $TaskDtoCopyWith<$Res> {
  factory _$$TaskDtoImplCopyWith(
    _$TaskDtoImpl value,
    $Res Function(_$TaskDtoImpl) then,
  ) = __$$TaskDtoImplCopyWithImpl<$Res>;
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
class __$$TaskDtoImplCopyWithImpl<$Res>
    extends _$TaskDtoCopyWithImpl<$Res, _$TaskDtoImpl>
    implements _$$TaskDtoImplCopyWith<$Res> {
  __$$TaskDtoImplCopyWithImpl(
    _$TaskDtoImpl _value,
    $Res Function(_$TaskDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of TaskDto
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
      _$TaskDtoImpl(
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
class _$TaskDtoImpl implements _TaskDto {
  const _$TaskDtoImpl({
    required this.id,
    required this.name,
    this.description,
    this.categoryId,
    this.scheduledDate,
    this.completedAt,
  });

  factory _$TaskDtoImpl.fromJson(Map<String, dynamic> json) =>
      _$$TaskDtoImplFromJson(json);

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
    return 'TaskDto(id: $id, name: $name, description: $description, categoryId: $categoryId, scheduledDate: $scheduledDate, completedAt: $completedAt)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$TaskDtoImpl &&
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

  /// Create a copy of TaskDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$TaskDtoImplCopyWith<_$TaskDtoImpl> get copyWith =>
      __$$TaskDtoImplCopyWithImpl<_$TaskDtoImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$TaskDtoImplToJson(this);
  }
}

abstract class _TaskDto implements TaskDto {
  const factory _TaskDto({
    required final String id,
    required final String name,
    final String? description,
    final String? categoryId,
    final int? scheduledDate,
    final int? completedAt,
  }) = _$TaskDtoImpl;

  factory _TaskDto.fromJson(Map<String, dynamic> json) = _$TaskDtoImpl.fromJson;

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

  /// Create a copy of TaskDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$TaskDtoImplCopyWith<_$TaskDtoImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

CategoryDto _$CategoryDtoFromJson(Map<String, dynamic> json) {
  return _CategoryDto.fromJson(json);
}

/// @nodoc
mixin _$CategoryDto {
  String get id => throw _privateConstructorUsedError;
  String get name => throw _privateConstructorUsedError;
  String get color => throw _privateConstructorUsedError;
  String? get description => throw _privateConstructorUsedError;
  List<TaskDto> get tasks => throw _privateConstructorUsedError;

  /// Serializes this CategoryDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of CategoryDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $CategoryDtoCopyWith<CategoryDto> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CategoryDtoCopyWith<$Res> {
  factory $CategoryDtoCopyWith(
    CategoryDto value,
    $Res Function(CategoryDto) then,
  ) = _$CategoryDtoCopyWithImpl<$Res, CategoryDto>;
  @useResult
  $Res call({
    String id,
    String name,
    String color,
    String? description,
    List<TaskDto> tasks,
  });
}

/// @nodoc
class _$CategoryDtoCopyWithImpl<$Res, $Val extends CategoryDto>
    implements $CategoryDtoCopyWith<$Res> {
  _$CategoryDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of CategoryDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? name = null,
    Object? color = null,
    Object? description = freezed,
    Object? tasks = null,
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
            color:
                null == color
                    ? _value.color
                    : color // ignore: cast_nullable_to_non_nullable
                        as String,
            description:
                freezed == description
                    ? _value.description
                    : description // ignore: cast_nullable_to_non_nullable
                        as String?,
            tasks:
                null == tasks
                    ? _value.tasks
                    : tasks // ignore: cast_nullable_to_non_nullable
                        as List<TaskDto>,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$CategoryDtoImplCopyWith<$Res>
    implements $CategoryDtoCopyWith<$Res> {
  factory _$$CategoryDtoImplCopyWith(
    _$CategoryDtoImpl value,
    $Res Function(_$CategoryDtoImpl) then,
  ) = __$$CategoryDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({
    String id,
    String name,
    String color,
    String? description,
    List<TaskDto> tasks,
  });
}

/// @nodoc
class __$$CategoryDtoImplCopyWithImpl<$Res>
    extends _$CategoryDtoCopyWithImpl<$Res, _$CategoryDtoImpl>
    implements _$$CategoryDtoImplCopyWith<$Res> {
  __$$CategoryDtoImplCopyWithImpl(
    _$CategoryDtoImpl _value,
    $Res Function(_$CategoryDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of CategoryDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? name = null,
    Object? color = null,
    Object? description = freezed,
    Object? tasks = null,
  }) {
    return _then(
      _$CategoryDtoImpl(
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
        color:
            null == color
                ? _value.color
                : color // ignore: cast_nullable_to_non_nullable
                    as String,
        description:
            freezed == description
                ? _value.description
                : description // ignore: cast_nullable_to_non_nullable
                    as String?,
        tasks:
            null == tasks
                ? _value._tasks
                : tasks // ignore: cast_nullable_to_non_nullable
                    as List<TaskDto>,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$CategoryDtoImpl implements _CategoryDto {
  const _$CategoryDtoImpl({
    required this.id,
    required this.name,
    required this.color,
    this.description,
    required final List<TaskDto> tasks,
  }) : _tasks = tasks;

  factory _$CategoryDtoImpl.fromJson(Map<String, dynamic> json) =>
      _$$CategoryDtoImplFromJson(json);

  @override
  final String id;
  @override
  final String name;
  @override
  final String color;
  @override
  final String? description;
  final List<TaskDto> _tasks;
  @override
  List<TaskDto> get tasks {
    if (_tasks is EqualUnmodifiableListView) return _tasks;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_tasks);
  }

  @override
  String toString() {
    return 'CategoryDto(id: $id, name: $name, color: $color, description: $description, tasks: $tasks)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CategoryDtoImpl &&
            (identical(other.id, id) || other.id == id) &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.color, color) || other.color == color) &&
            (identical(other.description, description) ||
                other.description == description) &&
            const DeepCollectionEquality().equals(other._tasks, _tasks));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
    runtimeType,
    id,
    name,
    color,
    description,
    const DeepCollectionEquality().hash(_tasks),
  );

  /// Create a copy of CategoryDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$CategoryDtoImplCopyWith<_$CategoryDtoImpl> get copyWith =>
      __$$CategoryDtoImplCopyWithImpl<_$CategoryDtoImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$CategoryDtoImplToJson(this);
  }
}

abstract class _CategoryDto implements CategoryDto {
  const factory _CategoryDto({
    required final String id,
    required final String name,
    required final String color,
    final String? description,
    required final List<TaskDto> tasks,
  }) = _$CategoryDtoImpl;

  factory _CategoryDto.fromJson(Map<String, dynamic> json) =
      _$CategoryDtoImpl.fromJson;

  @override
  String get id;
  @override
  String get name;
  @override
  String get color;
  @override
  String? get description;
  @override
  List<TaskDto> get tasks;

  /// Create a copy of CategoryDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$CategoryDtoImplCopyWith<_$CategoryDtoImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

GetCategoriesResponseDto _$GetCategoriesResponseDtoFromJson(
  Map<String, dynamic> json,
) {
  return _GetCategoriesResponseDto.fromJson(json);
}

/// @nodoc
mixin _$GetCategoriesResponseDto {
  List<CategoryDto> get categories => throw _privateConstructorUsedError;

  /// Serializes this GetCategoriesResponseDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of GetCategoriesResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $GetCategoriesResponseDtoCopyWith<GetCategoriesResponseDto> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $GetCategoriesResponseDtoCopyWith<$Res> {
  factory $GetCategoriesResponseDtoCopyWith(
    GetCategoriesResponseDto value,
    $Res Function(GetCategoriesResponseDto) then,
  ) = _$GetCategoriesResponseDtoCopyWithImpl<$Res, GetCategoriesResponseDto>;
  @useResult
  $Res call({List<CategoryDto> categories});
}

/// @nodoc
class _$GetCategoriesResponseDtoCopyWithImpl<
  $Res,
  $Val extends GetCategoriesResponseDto
>
    implements $GetCategoriesResponseDtoCopyWith<$Res> {
  _$GetCategoriesResponseDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of GetCategoriesResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? categories = null}) {
    return _then(
      _value.copyWith(
            categories:
                null == categories
                    ? _value.categories
                    : categories // ignore: cast_nullable_to_non_nullable
                        as List<CategoryDto>,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$GetCategoriesResponseDtoImplCopyWith<$Res>
    implements $GetCategoriesResponseDtoCopyWith<$Res> {
  factory _$$GetCategoriesResponseDtoImplCopyWith(
    _$GetCategoriesResponseDtoImpl value,
    $Res Function(_$GetCategoriesResponseDtoImpl) then,
  ) = __$$GetCategoriesResponseDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({List<CategoryDto> categories});
}

/// @nodoc
class __$$GetCategoriesResponseDtoImplCopyWithImpl<$Res>
    extends
        _$GetCategoriesResponseDtoCopyWithImpl<
          $Res,
          _$GetCategoriesResponseDtoImpl
        >
    implements _$$GetCategoriesResponseDtoImplCopyWith<$Res> {
  __$$GetCategoriesResponseDtoImplCopyWithImpl(
    _$GetCategoriesResponseDtoImpl _value,
    $Res Function(_$GetCategoriesResponseDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of GetCategoriesResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? categories = null}) {
    return _then(
      _$GetCategoriesResponseDtoImpl(
        categories:
            null == categories
                ? _value._categories
                : categories // ignore: cast_nullable_to_non_nullable
                    as List<CategoryDto>,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$GetCategoriesResponseDtoImpl implements _GetCategoriesResponseDto {
  const _$GetCategoriesResponseDtoImpl({
    required final List<CategoryDto> categories,
  }) : _categories = categories;

  factory _$GetCategoriesResponseDtoImpl.fromJson(Map<String, dynamic> json) =>
      _$$GetCategoriesResponseDtoImplFromJson(json);

  final List<CategoryDto> _categories;
  @override
  List<CategoryDto> get categories {
    if (_categories is EqualUnmodifiableListView) return _categories;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_categories);
  }

  @override
  String toString() {
    return 'GetCategoriesResponseDto(categories: $categories)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$GetCategoriesResponseDtoImpl &&
            const DeepCollectionEquality().equals(
              other._categories,
              _categories,
            ));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
    runtimeType,
    const DeepCollectionEquality().hash(_categories),
  );

  /// Create a copy of GetCategoriesResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$GetCategoriesResponseDtoImplCopyWith<_$GetCategoriesResponseDtoImpl>
  get copyWith => __$$GetCategoriesResponseDtoImplCopyWithImpl<
    _$GetCategoriesResponseDtoImpl
  >(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$GetCategoriesResponseDtoImplToJson(this);
  }
}

abstract class _GetCategoriesResponseDto implements GetCategoriesResponseDto {
  const factory _GetCategoriesResponseDto({
    required final List<CategoryDto> categories,
  }) = _$GetCategoriesResponseDtoImpl;

  factory _GetCategoriesResponseDto.fromJson(Map<String, dynamic> json) =
      _$GetCategoriesResponseDtoImpl.fromJson;

  @override
  List<CategoryDto> get categories;

  /// Create a copy of GetCategoriesResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$GetCategoriesResponseDtoImplCopyWith<_$GetCategoriesResponseDtoImpl>
  get copyWith => throw _privateConstructorUsedError;
}

CreateCategoryResponseDto _$CreateCategoryResponseDtoFromJson(
  Map<String, dynamic> json,
) {
  return _CreateCategoryResponseDto.fromJson(json);
}

/// @nodoc
mixin _$CreateCategoryResponseDto {
  bool get created => throw _privateConstructorUsedError;

  /// Serializes this CreateCategoryResponseDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of CreateCategoryResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $CreateCategoryResponseDtoCopyWith<CreateCategoryResponseDto> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CreateCategoryResponseDtoCopyWith<$Res> {
  factory $CreateCategoryResponseDtoCopyWith(
    CreateCategoryResponseDto value,
    $Res Function(CreateCategoryResponseDto) then,
  ) = _$CreateCategoryResponseDtoCopyWithImpl<$Res, CreateCategoryResponseDto>;
  @useResult
  $Res call({bool created});
}

/// @nodoc
class _$CreateCategoryResponseDtoCopyWithImpl<
  $Res,
  $Val extends CreateCategoryResponseDto
>
    implements $CreateCategoryResponseDtoCopyWith<$Res> {
  _$CreateCategoryResponseDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of CreateCategoryResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? created = null}) {
    return _then(
      _value.copyWith(
            created:
                null == created
                    ? _value.created
                    : created // ignore: cast_nullable_to_non_nullable
                        as bool,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$CreateCategoryResponseDtoImplCopyWith<$Res>
    implements $CreateCategoryResponseDtoCopyWith<$Res> {
  factory _$$CreateCategoryResponseDtoImplCopyWith(
    _$CreateCategoryResponseDtoImpl value,
    $Res Function(_$CreateCategoryResponseDtoImpl) then,
  ) = __$$CreateCategoryResponseDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({bool created});
}

/// @nodoc
class __$$CreateCategoryResponseDtoImplCopyWithImpl<$Res>
    extends
        _$CreateCategoryResponseDtoCopyWithImpl<
          $Res,
          _$CreateCategoryResponseDtoImpl
        >
    implements _$$CreateCategoryResponseDtoImplCopyWith<$Res> {
  __$$CreateCategoryResponseDtoImplCopyWithImpl(
    _$CreateCategoryResponseDtoImpl _value,
    $Res Function(_$CreateCategoryResponseDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of CreateCategoryResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? created = null}) {
    return _then(
      _$CreateCategoryResponseDtoImpl(
        created:
            null == created
                ? _value.created
                : created // ignore: cast_nullable_to_non_nullable
                    as bool,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$CreateCategoryResponseDtoImpl implements _CreateCategoryResponseDto {
  const _$CreateCategoryResponseDtoImpl({required this.created});

  factory _$CreateCategoryResponseDtoImpl.fromJson(Map<String, dynamic> json) =>
      _$$CreateCategoryResponseDtoImplFromJson(json);

  @override
  final bool created;

  @override
  String toString() {
    return 'CreateCategoryResponseDto(created: $created)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CreateCategoryResponseDtoImpl &&
            (identical(other.created, created) || other.created == created));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, created);

  /// Create a copy of CreateCategoryResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$CreateCategoryResponseDtoImplCopyWith<_$CreateCategoryResponseDtoImpl>
  get copyWith => __$$CreateCategoryResponseDtoImplCopyWithImpl<
    _$CreateCategoryResponseDtoImpl
  >(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$CreateCategoryResponseDtoImplToJson(this);
  }
}

abstract class _CreateCategoryResponseDto implements CreateCategoryResponseDto {
  const factory _CreateCategoryResponseDto({required final bool created}) =
      _$CreateCategoryResponseDtoImpl;

  factory _CreateCategoryResponseDto.fromJson(Map<String, dynamic> json) =
      _$CreateCategoryResponseDtoImpl.fromJson;

  @override
  bool get created;

  /// Create a copy of CreateCategoryResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$CreateCategoryResponseDtoImplCopyWith<_$CreateCategoryResponseDtoImpl>
  get copyWith => throw _privateConstructorUsedError;
}

UpdateCategoryResponseDto _$UpdateCategoryResponseDtoFromJson(
  Map<String, dynamic> json,
) {
  return _UpdateCategoryResponseDto.fromJson(json);
}

/// @nodoc
mixin _$UpdateCategoryResponseDto {
  CategoryDto get updatedCategory => throw _privateConstructorUsedError;

  /// Serializes this UpdateCategoryResponseDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of UpdateCategoryResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $UpdateCategoryResponseDtoCopyWith<UpdateCategoryResponseDto> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $UpdateCategoryResponseDtoCopyWith<$Res> {
  factory $UpdateCategoryResponseDtoCopyWith(
    UpdateCategoryResponseDto value,
    $Res Function(UpdateCategoryResponseDto) then,
  ) = _$UpdateCategoryResponseDtoCopyWithImpl<$Res, UpdateCategoryResponseDto>;
  @useResult
  $Res call({CategoryDto updatedCategory});

  $CategoryDtoCopyWith<$Res> get updatedCategory;
}

/// @nodoc
class _$UpdateCategoryResponseDtoCopyWithImpl<
  $Res,
  $Val extends UpdateCategoryResponseDto
>
    implements $UpdateCategoryResponseDtoCopyWith<$Res> {
  _$UpdateCategoryResponseDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of UpdateCategoryResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? updatedCategory = null}) {
    return _then(
      _value.copyWith(
            updatedCategory:
                null == updatedCategory
                    ? _value.updatedCategory
                    : updatedCategory // ignore: cast_nullable_to_non_nullable
                        as CategoryDto,
          )
          as $Val,
    );
  }

  /// Create a copy of UpdateCategoryResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $CategoryDtoCopyWith<$Res> get updatedCategory {
    return $CategoryDtoCopyWith<$Res>(_value.updatedCategory, (value) {
      return _then(_value.copyWith(updatedCategory: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$UpdateCategoryResponseDtoImplCopyWith<$Res>
    implements $UpdateCategoryResponseDtoCopyWith<$Res> {
  factory _$$UpdateCategoryResponseDtoImplCopyWith(
    _$UpdateCategoryResponseDtoImpl value,
    $Res Function(_$UpdateCategoryResponseDtoImpl) then,
  ) = __$$UpdateCategoryResponseDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({CategoryDto updatedCategory});

  @override
  $CategoryDtoCopyWith<$Res> get updatedCategory;
}

/// @nodoc
class __$$UpdateCategoryResponseDtoImplCopyWithImpl<$Res>
    extends
        _$UpdateCategoryResponseDtoCopyWithImpl<
          $Res,
          _$UpdateCategoryResponseDtoImpl
        >
    implements _$$UpdateCategoryResponseDtoImplCopyWith<$Res> {
  __$$UpdateCategoryResponseDtoImplCopyWithImpl(
    _$UpdateCategoryResponseDtoImpl _value,
    $Res Function(_$UpdateCategoryResponseDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of UpdateCategoryResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? updatedCategory = null}) {
    return _then(
      _$UpdateCategoryResponseDtoImpl(
        updatedCategory:
            null == updatedCategory
                ? _value.updatedCategory
                : updatedCategory // ignore: cast_nullable_to_non_nullable
                    as CategoryDto,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$UpdateCategoryResponseDtoImpl implements _UpdateCategoryResponseDto {
  const _$UpdateCategoryResponseDtoImpl({required this.updatedCategory});

  factory _$UpdateCategoryResponseDtoImpl.fromJson(Map<String, dynamic> json) =>
      _$$UpdateCategoryResponseDtoImplFromJson(json);

  @override
  final CategoryDto updatedCategory;

  @override
  String toString() {
    return 'UpdateCategoryResponseDto(updatedCategory: $updatedCategory)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$UpdateCategoryResponseDtoImpl &&
            (identical(other.updatedCategory, updatedCategory) ||
                other.updatedCategory == updatedCategory));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, updatedCategory);

  /// Create a copy of UpdateCategoryResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$UpdateCategoryResponseDtoImplCopyWith<_$UpdateCategoryResponseDtoImpl>
  get copyWith => __$$UpdateCategoryResponseDtoImplCopyWithImpl<
    _$UpdateCategoryResponseDtoImpl
  >(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$UpdateCategoryResponseDtoImplToJson(this);
  }
}

abstract class _UpdateCategoryResponseDto implements UpdateCategoryResponseDto {
  const factory _UpdateCategoryResponseDto({
    required final CategoryDto updatedCategory,
  }) = _$UpdateCategoryResponseDtoImpl;

  factory _UpdateCategoryResponseDto.fromJson(Map<String, dynamic> json) =
      _$UpdateCategoryResponseDtoImpl.fromJson;

  @override
  CategoryDto get updatedCategory;

  /// Create a copy of UpdateCategoryResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$UpdateCategoryResponseDtoImplCopyWith<_$UpdateCategoryResponseDtoImpl>
  get copyWith => throw _privateConstructorUsedError;
}

DeleteCategoriesResponseDto _$DeleteCategoriesResponseDtoFromJson(
  Map<String, dynamic> json,
) {
  return _DeleteCategoriesResponseDto.fromJson(json);
}

/// @nodoc
mixin _$DeleteCategoriesResponseDto {
  @JsonKey(name: 'deleted_ids')
  List<String> get deletedIds => throw _privateConstructorUsedError;

  /// Serializes this DeleteCategoriesResponseDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of DeleteCategoriesResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $DeleteCategoriesResponseDtoCopyWith<DeleteCategoriesResponseDto>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $DeleteCategoriesResponseDtoCopyWith<$Res> {
  factory $DeleteCategoriesResponseDtoCopyWith(
    DeleteCategoriesResponseDto value,
    $Res Function(DeleteCategoriesResponseDto) then,
  ) =
      _$DeleteCategoriesResponseDtoCopyWithImpl<
        $Res,
        DeleteCategoriesResponseDto
      >;
  @useResult
  $Res call({@JsonKey(name: 'deleted_ids') List<String> deletedIds});
}

/// @nodoc
class _$DeleteCategoriesResponseDtoCopyWithImpl<
  $Res,
  $Val extends DeleteCategoriesResponseDto
>
    implements $DeleteCategoriesResponseDtoCopyWith<$Res> {
  _$DeleteCategoriesResponseDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of DeleteCategoriesResponseDto
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
abstract class _$$DeleteCategoriesResponseDtoImplCopyWith<$Res>
    implements $DeleteCategoriesResponseDtoCopyWith<$Res> {
  factory _$$DeleteCategoriesResponseDtoImplCopyWith(
    _$DeleteCategoriesResponseDtoImpl value,
    $Res Function(_$DeleteCategoriesResponseDtoImpl) then,
  ) = __$$DeleteCategoriesResponseDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({@JsonKey(name: 'deleted_ids') List<String> deletedIds});
}

/// @nodoc
class __$$DeleteCategoriesResponseDtoImplCopyWithImpl<$Res>
    extends
        _$DeleteCategoriesResponseDtoCopyWithImpl<
          $Res,
          _$DeleteCategoriesResponseDtoImpl
        >
    implements _$$DeleteCategoriesResponseDtoImplCopyWith<$Res> {
  __$$DeleteCategoriesResponseDtoImplCopyWithImpl(
    _$DeleteCategoriesResponseDtoImpl _value,
    $Res Function(_$DeleteCategoriesResponseDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of DeleteCategoriesResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? deletedIds = null}) {
    return _then(
      _$DeleteCategoriesResponseDtoImpl(
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
class _$DeleteCategoriesResponseDtoImpl
    implements _DeleteCategoriesResponseDto {
  const _$DeleteCategoriesResponseDtoImpl({
    @JsonKey(name: 'deleted_ids') required final List<String> deletedIds,
  }) : _deletedIds = deletedIds;

  factory _$DeleteCategoriesResponseDtoImpl.fromJson(
    Map<String, dynamic> json,
  ) => _$$DeleteCategoriesResponseDtoImplFromJson(json);

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
    return 'DeleteCategoriesResponseDto(deletedIds: $deletedIds)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DeleteCategoriesResponseDtoImpl &&
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

  /// Create a copy of DeleteCategoriesResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$DeleteCategoriesResponseDtoImplCopyWith<_$DeleteCategoriesResponseDtoImpl>
  get copyWith => __$$DeleteCategoriesResponseDtoImplCopyWithImpl<
    _$DeleteCategoriesResponseDtoImpl
  >(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$DeleteCategoriesResponseDtoImplToJson(this);
  }
}

abstract class _DeleteCategoriesResponseDto
    implements DeleteCategoriesResponseDto {
  const factory _DeleteCategoriesResponseDto({
    @JsonKey(name: 'deleted_ids') required final List<String> deletedIds,
  }) = _$DeleteCategoriesResponseDtoImpl;

  factory _DeleteCategoriesResponseDto.fromJson(Map<String, dynamic> json) =
      _$DeleteCategoriesResponseDtoImpl.fromJson;

  @override
  @JsonKey(name: 'deleted_ids')
  List<String> get deletedIds;

  /// Create a copy of DeleteCategoriesResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$DeleteCategoriesResponseDtoImplCopyWith<_$DeleteCategoriesResponseDtoImpl>
  get copyWith => throw _privateConstructorUsedError;
}
