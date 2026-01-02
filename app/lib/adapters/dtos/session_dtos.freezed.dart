// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'session_dtos.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
  'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models',
);

CreateManualSessionDto _$CreateManualSessionDtoFromJson(
  Map<String, dynamic> json,
) {
  return _CreateManualSessionDto.fromJson(json);
}

/// @nodoc
mixin _$CreateManualSessionDto {
  String get sessionType => throw _privateConstructorUsedError;
  int get startedAt => throw _privateConstructorUsedError;
  int get endedAt => throw _privateConstructorUsedError;
  String? get taskId => throw _privateConstructorUsedError;
  String? get categoryId => throw _privateConstructorUsedError;
  int? get concentrationScore => throw _privateConstructorUsedError;
  String? get notes => throw _privateConstructorUsedError;

  /// Serializes this CreateManualSessionDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of CreateManualSessionDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $CreateManualSessionDtoCopyWith<CreateManualSessionDto> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CreateManualSessionDtoCopyWith<$Res> {
  factory $CreateManualSessionDtoCopyWith(
    CreateManualSessionDto value,
    $Res Function(CreateManualSessionDto) then,
  ) = _$CreateManualSessionDtoCopyWithImpl<$Res, CreateManualSessionDto>;
  @useResult
  $Res call({
    String sessionType,
    int startedAt,
    int endedAt,
    String? taskId,
    String? categoryId,
    int? concentrationScore,
    String? notes,
  });
}

/// @nodoc
class _$CreateManualSessionDtoCopyWithImpl<
  $Res,
  $Val extends CreateManualSessionDto
>
    implements $CreateManualSessionDtoCopyWith<$Res> {
  _$CreateManualSessionDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of CreateManualSessionDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? sessionType = null,
    Object? startedAt = null,
    Object? endedAt = null,
    Object? taskId = freezed,
    Object? categoryId = freezed,
    Object? concentrationScore = freezed,
    Object? notes = freezed,
  }) {
    return _then(
      _value.copyWith(
            sessionType:
                null == sessionType
                    ? _value.sessionType
                    : sessionType // ignore: cast_nullable_to_non_nullable
                        as String,
            startedAt:
                null == startedAt
                    ? _value.startedAt
                    : startedAt // ignore: cast_nullable_to_non_nullable
                        as int,
            endedAt:
                null == endedAt
                    ? _value.endedAt
                    : endedAt // ignore: cast_nullable_to_non_nullable
                        as int,
            taskId:
                freezed == taskId
                    ? _value.taskId
                    : taskId // ignore: cast_nullable_to_non_nullable
                        as String?,
            categoryId:
                freezed == categoryId
                    ? _value.categoryId
                    : categoryId // ignore: cast_nullable_to_non_nullable
                        as String?,
            concentrationScore:
                freezed == concentrationScore
                    ? _value.concentrationScore
                    : concentrationScore // ignore: cast_nullable_to_non_nullable
                        as int?,
            notes:
                freezed == notes
                    ? _value.notes
                    : notes // ignore: cast_nullable_to_non_nullable
                        as String?,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$CreateManualSessionDtoImplCopyWith<$Res>
    implements $CreateManualSessionDtoCopyWith<$Res> {
  factory _$$CreateManualSessionDtoImplCopyWith(
    _$CreateManualSessionDtoImpl value,
    $Res Function(_$CreateManualSessionDtoImpl) then,
  ) = __$$CreateManualSessionDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({
    String sessionType,
    int startedAt,
    int endedAt,
    String? taskId,
    String? categoryId,
    int? concentrationScore,
    String? notes,
  });
}

/// @nodoc
class __$$CreateManualSessionDtoImplCopyWithImpl<$Res>
    extends
        _$CreateManualSessionDtoCopyWithImpl<$Res, _$CreateManualSessionDtoImpl>
    implements _$$CreateManualSessionDtoImplCopyWith<$Res> {
  __$$CreateManualSessionDtoImplCopyWithImpl(
    _$CreateManualSessionDtoImpl _value,
    $Res Function(_$CreateManualSessionDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of CreateManualSessionDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? sessionType = null,
    Object? startedAt = null,
    Object? endedAt = null,
    Object? taskId = freezed,
    Object? categoryId = freezed,
    Object? concentrationScore = freezed,
    Object? notes = freezed,
  }) {
    return _then(
      _$CreateManualSessionDtoImpl(
        sessionType:
            null == sessionType
                ? _value.sessionType
                : sessionType // ignore: cast_nullable_to_non_nullable
                    as String,
        startedAt:
            null == startedAt
                ? _value.startedAt
                : startedAt // ignore: cast_nullable_to_non_nullable
                    as int,
        endedAt:
            null == endedAt
                ? _value.endedAt
                : endedAt // ignore: cast_nullable_to_non_nullable
                    as int,
        taskId:
            freezed == taskId
                ? _value.taskId
                : taskId // ignore: cast_nullable_to_non_nullable
                    as String?,
        categoryId:
            freezed == categoryId
                ? _value.categoryId
                : categoryId // ignore: cast_nullable_to_non_nullable
                    as String?,
        concentrationScore:
            freezed == concentrationScore
                ? _value.concentrationScore
                : concentrationScore // ignore: cast_nullable_to_non_nullable
                    as int?,
        notes:
            freezed == notes
                ? _value.notes
                : notes // ignore: cast_nullable_to_non_nullable
                    as String?,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$CreateManualSessionDtoImpl extends _CreateManualSessionDto {
  const _$CreateManualSessionDtoImpl({
    required this.sessionType,
    required this.startedAt,
    required this.endedAt,
    this.taskId,
    this.categoryId,
    this.concentrationScore,
    this.notes,
  }) : super._();

  factory _$CreateManualSessionDtoImpl.fromJson(Map<String, dynamic> json) =>
      _$$CreateManualSessionDtoImplFromJson(json);

  @override
  final String sessionType;
  @override
  final int startedAt;
  @override
  final int endedAt;
  @override
  final String? taskId;
  @override
  final String? categoryId;
  @override
  final int? concentrationScore;
  @override
  final String? notes;

  @override
  String toString() {
    return 'CreateManualSessionDto(sessionType: $sessionType, startedAt: $startedAt, endedAt: $endedAt, taskId: $taskId, categoryId: $categoryId, concentrationScore: $concentrationScore, notes: $notes)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CreateManualSessionDtoImpl &&
            (identical(other.sessionType, sessionType) ||
                other.sessionType == sessionType) &&
            (identical(other.startedAt, startedAt) ||
                other.startedAt == startedAt) &&
            (identical(other.endedAt, endedAt) || other.endedAt == endedAt) &&
            (identical(other.taskId, taskId) || other.taskId == taskId) &&
            (identical(other.categoryId, categoryId) ||
                other.categoryId == categoryId) &&
            (identical(other.concentrationScore, concentrationScore) ||
                other.concentrationScore == concentrationScore) &&
            (identical(other.notes, notes) || other.notes == notes));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
    runtimeType,
    sessionType,
    startedAt,
    endedAt,
    taskId,
    categoryId,
    concentrationScore,
    notes,
  );

  /// Create a copy of CreateManualSessionDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$CreateManualSessionDtoImplCopyWith<_$CreateManualSessionDtoImpl>
  get copyWith =>
      __$$CreateManualSessionDtoImplCopyWithImpl<_$CreateManualSessionDtoImpl>(
        this,
        _$identity,
      );

  @override
  Map<String, dynamic> toJson() {
    return _$$CreateManualSessionDtoImplToJson(this);
  }
}

abstract class _CreateManualSessionDto extends CreateManualSessionDto {
  const factory _CreateManualSessionDto({
    required final String sessionType,
    required final int startedAt,
    required final int endedAt,
    final String? taskId,
    final String? categoryId,
    final int? concentrationScore,
    final String? notes,
  }) = _$CreateManualSessionDtoImpl;
  const _CreateManualSessionDto._() : super._();

  factory _CreateManualSessionDto.fromJson(Map<String, dynamic> json) =
      _$CreateManualSessionDtoImpl.fromJson;

  @override
  String get sessionType;
  @override
  int get startedAt;
  @override
  int get endedAt;
  @override
  String? get taskId;
  @override
  String? get categoryId;
  @override
  int? get concentrationScore;
  @override
  String? get notes;

  /// Create a copy of CreateManualSessionDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$CreateManualSessionDtoImplCopyWith<_$CreateManualSessionDtoImpl>
  get copyWith => throw _privateConstructorUsedError;
}

GetSessionFiltersDto _$GetSessionFiltersDtoFromJson(Map<String, dynamic> json) {
  return _GetSessionFiltersDto.fromJson(json);
}

/// @nodoc
mixin _$GetSessionFiltersDto {
  int? get startDate => throw _privateConstructorUsedError;
  int? get endDate => throw _privateConstructorUsedError;
  List<String>? get categoryIds => throw _privateConstructorUsedError;
  String? get sessionType => throw _privateConstructorUsedError;
  int? get minConcentrationScore => throw _privateConstructorUsedError;
  int? get maxConcentrationScore => throw _privateConstructorUsedError;

  /// Serializes this GetSessionFiltersDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of GetSessionFiltersDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $GetSessionFiltersDtoCopyWith<GetSessionFiltersDto> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $GetSessionFiltersDtoCopyWith<$Res> {
  factory $GetSessionFiltersDtoCopyWith(
    GetSessionFiltersDto value,
    $Res Function(GetSessionFiltersDto) then,
  ) = _$GetSessionFiltersDtoCopyWithImpl<$Res, GetSessionFiltersDto>;
  @useResult
  $Res call({
    int? startDate,
    int? endDate,
    List<String>? categoryIds,
    String? sessionType,
    int? minConcentrationScore,
    int? maxConcentrationScore,
  });
}

/// @nodoc
class _$GetSessionFiltersDtoCopyWithImpl<
  $Res,
  $Val extends GetSessionFiltersDto
>
    implements $GetSessionFiltersDtoCopyWith<$Res> {
  _$GetSessionFiltersDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of GetSessionFiltersDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? startDate = freezed,
    Object? endDate = freezed,
    Object? categoryIds = freezed,
    Object? sessionType = freezed,
    Object? minConcentrationScore = freezed,
    Object? maxConcentrationScore = freezed,
  }) {
    return _then(
      _value.copyWith(
            startDate:
                freezed == startDate
                    ? _value.startDate
                    : startDate // ignore: cast_nullable_to_non_nullable
                        as int?,
            endDate:
                freezed == endDate
                    ? _value.endDate
                    : endDate // ignore: cast_nullable_to_non_nullable
                        as int?,
            categoryIds:
                freezed == categoryIds
                    ? _value.categoryIds
                    : categoryIds // ignore: cast_nullable_to_non_nullable
                        as List<String>?,
            sessionType:
                freezed == sessionType
                    ? _value.sessionType
                    : sessionType // ignore: cast_nullable_to_non_nullable
                        as String?,
            minConcentrationScore:
                freezed == minConcentrationScore
                    ? _value.minConcentrationScore
                    : minConcentrationScore // ignore: cast_nullable_to_non_nullable
                        as int?,
            maxConcentrationScore:
                freezed == maxConcentrationScore
                    ? _value.maxConcentrationScore
                    : maxConcentrationScore // ignore: cast_nullable_to_non_nullable
                        as int?,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$GetSessionFiltersDtoImplCopyWith<$Res>
    implements $GetSessionFiltersDtoCopyWith<$Res> {
  factory _$$GetSessionFiltersDtoImplCopyWith(
    _$GetSessionFiltersDtoImpl value,
    $Res Function(_$GetSessionFiltersDtoImpl) then,
  ) = __$$GetSessionFiltersDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({
    int? startDate,
    int? endDate,
    List<String>? categoryIds,
    String? sessionType,
    int? minConcentrationScore,
    int? maxConcentrationScore,
  });
}

/// @nodoc
class __$$GetSessionFiltersDtoImplCopyWithImpl<$Res>
    extends _$GetSessionFiltersDtoCopyWithImpl<$Res, _$GetSessionFiltersDtoImpl>
    implements _$$GetSessionFiltersDtoImplCopyWith<$Res> {
  __$$GetSessionFiltersDtoImplCopyWithImpl(
    _$GetSessionFiltersDtoImpl _value,
    $Res Function(_$GetSessionFiltersDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of GetSessionFiltersDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? startDate = freezed,
    Object? endDate = freezed,
    Object? categoryIds = freezed,
    Object? sessionType = freezed,
    Object? minConcentrationScore = freezed,
    Object? maxConcentrationScore = freezed,
  }) {
    return _then(
      _$GetSessionFiltersDtoImpl(
        startDate:
            freezed == startDate
                ? _value.startDate
                : startDate // ignore: cast_nullable_to_non_nullable
                    as int?,
        endDate:
            freezed == endDate
                ? _value.endDate
                : endDate // ignore: cast_nullable_to_non_nullable
                    as int?,
        categoryIds:
            freezed == categoryIds
                ? _value._categoryIds
                : categoryIds // ignore: cast_nullable_to_non_nullable
                    as List<String>?,
        sessionType:
            freezed == sessionType
                ? _value.sessionType
                : sessionType // ignore: cast_nullable_to_non_nullable
                    as String?,
        minConcentrationScore:
            freezed == minConcentrationScore
                ? _value.minConcentrationScore
                : minConcentrationScore // ignore: cast_nullable_to_non_nullable
                    as int?,
        maxConcentrationScore:
            freezed == maxConcentrationScore
                ? _value.maxConcentrationScore
                : maxConcentrationScore // ignore: cast_nullable_to_non_nullable
                    as int?,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$GetSessionFiltersDtoImpl extends _GetSessionFiltersDto {
  const _$GetSessionFiltersDtoImpl({
    this.startDate,
    this.endDate,
    final List<String>? categoryIds,
    this.sessionType,
    this.minConcentrationScore,
    this.maxConcentrationScore,
  }) : _categoryIds = categoryIds,
       super._();

  factory _$GetSessionFiltersDtoImpl.fromJson(Map<String, dynamic> json) =>
      _$$GetSessionFiltersDtoImplFromJson(json);

  @override
  final int? startDate;
  @override
  final int? endDate;
  final List<String>? _categoryIds;
  @override
  List<String>? get categoryIds {
    final value = _categoryIds;
    if (value == null) return null;
    if (_categoryIds is EqualUnmodifiableListView) return _categoryIds;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(value);
  }

  @override
  final String? sessionType;
  @override
  final int? minConcentrationScore;
  @override
  final int? maxConcentrationScore;

  @override
  String toString() {
    return 'GetSessionFiltersDto(startDate: $startDate, endDate: $endDate, categoryIds: $categoryIds, sessionType: $sessionType, minConcentrationScore: $minConcentrationScore, maxConcentrationScore: $maxConcentrationScore)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$GetSessionFiltersDtoImpl &&
            (identical(other.startDate, startDate) ||
                other.startDate == startDate) &&
            (identical(other.endDate, endDate) || other.endDate == endDate) &&
            const DeepCollectionEquality().equals(
              other._categoryIds,
              _categoryIds,
            ) &&
            (identical(other.sessionType, sessionType) ||
                other.sessionType == sessionType) &&
            (identical(other.minConcentrationScore, minConcentrationScore) ||
                other.minConcentrationScore == minConcentrationScore) &&
            (identical(other.maxConcentrationScore, maxConcentrationScore) ||
                other.maxConcentrationScore == maxConcentrationScore));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
    runtimeType,
    startDate,
    endDate,
    const DeepCollectionEquality().hash(_categoryIds),
    sessionType,
    minConcentrationScore,
    maxConcentrationScore,
  );

  /// Create a copy of GetSessionFiltersDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$GetSessionFiltersDtoImplCopyWith<_$GetSessionFiltersDtoImpl>
  get copyWith =>
      __$$GetSessionFiltersDtoImplCopyWithImpl<_$GetSessionFiltersDtoImpl>(
        this,
        _$identity,
      );

  @override
  Map<String, dynamic> toJson() {
    return _$$GetSessionFiltersDtoImplToJson(this);
  }
}

abstract class _GetSessionFiltersDto extends GetSessionFiltersDto {
  const factory _GetSessionFiltersDto({
    final int? startDate,
    final int? endDate,
    final List<String>? categoryIds,
    final String? sessionType,
    final int? minConcentrationScore,
    final int? maxConcentrationScore,
  }) = _$GetSessionFiltersDtoImpl;
  const _GetSessionFiltersDto._() : super._();

  factory _GetSessionFiltersDto.fromJson(Map<String, dynamic> json) =
      _$GetSessionFiltersDtoImpl.fromJson;

  @override
  int? get startDate;
  @override
  int? get endDate;
  @override
  List<String>? get categoryIds;
  @override
  String? get sessionType;
  @override
  int? get minConcentrationScore;
  @override
  int? get maxConcentrationScore;

  /// Create a copy of GetSessionFiltersDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$GetSessionFiltersDtoImplCopyWith<_$GetSessionFiltersDtoImpl>
  get copyWith => throw _privateConstructorUsedError;
}

UpdateFocusSessionDto _$UpdateFocusSessionDtoFromJson(
  Map<String, dynamic> json,
) {
  return _UpdateFocusSessionDto.fromJson(json);
}

/// @nodoc
mixin _$UpdateFocusSessionDto {
  String? get categoryId => throw _privateConstructorUsedError;
  String? get taskId => throw _privateConstructorUsedError;
  String? get notes => throw _privateConstructorUsedError;
  int? get concentrationScore => throw _privateConstructorUsedError;
  int? get startedAt => throw _privateConstructorUsedError;
  int? get endedAt => throw _privateConstructorUsedError;
  int? get actualDuration => throw _privateConstructorUsedError;
  String? get sessionType => throw _privateConstructorUsedError;

  /// Serializes this UpdateFocusSessionDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of UpdateFocusSessionDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $UpdateFocusSessionDtoCopyWith<UpdateFocusSessionDto> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $UpdateFocusSessionDtoCopyWith<$Res> {
  factory $UpdateFocusSessionDtoCopyWith(
    UpdateFocusSessionDto value,
    $Res Function(UpdateFocusSessionDto) then,
  ) = _$UpdateFocusSessionDtoCopyWithImpl<$Res, UpdateFocusSessionDto>;
  @useResult
  $Res call({
    String? categoryId,
    String? taskId,
    String? notes,
    int? concentrationScore,
    int? startedAt,
    int? endedAt,
    int? actualDuration,
    String? sessionType,
  });
}

/// @nodoc
class _$UpdateFocusSessionDtoCopyWithImpl<
  $Res,
  $Val extends UpdateFocusSessionDto
>
    implements $UpdateFocusSessionDtoCopyWith<$Res> {
  _$UpdateFocusSessionDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of UpdateFocusSessionDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? categoryId = freezed,
    Object? taskId = freezed,
    Object? notes = freezed,
    Object? concentrationScore = freezed,
    Object? startedAt = freezed,
    Object? endedAt = freezed,
    Object? actualDuration = freezed,
    Object? sessionType = freezed,
  }) {
    return _then(
      _value.copyWith(
            categoryId:
                freezed == categoryId
                    ? _value.categoryId
                    : categoryId // ignore: cast_nullable_to_non_nullable
                        as String?,
            taskId:
                freezed == taskId
                    ? _value.taskId
                    : taskId // ignore: cast_nullable_to_non_nullable
                        as String?,
            notes:
                freezed == notes
                    ? _value.notes
                    : notes // ignore: cast_nullable_to_non_nullable
                        as String?,
            concentrationScore:
                freezed == concentrationScore
                    ? _value.concentrationScore
                    : concentrationScore // ignore: cast_nullable_to_non_nullable
                        as int?,
            startedAt:
                freezed == startedAt
                    ? _value.startedAt
                    : startedAt // ignore: cast_nullable_to_non_nullable
                        as int?,
            endedAt:
                freezed == endedAt
                    ? _value.endedAt
                    : endedAt // ignore: cast_nullable_to_non_nullable
                        as int?,
            actualDuration:
                freezed == actualDuration
                    ? _value.actualDuration
                    : actualDuration // ignore: cast_nullable_to_non_nullable
                        as int?,
            sessionType:
                freezed == sessionType
                    ? _value.sessionType
                    : sessionType // ignore: cast_nullable_to_non_nullable
                        as String?,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$UpdateFocusSessionDtoImplCopyWith<$Res>
    implements $UpdateFocusSessionDtoCopyWith<$Res> {
  factory _$$UpdateFocusSessionDtoImplCopyWith(
    _$UpdateFocusSessionDtoImpl value,
    $Res Function(_$UpdateFocusSessionDtoImpl) then,
  ) = __$$UpdateFocusSessionDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({
    String? categoryId,
    String? taskId,
    String? notes,
    int? concentrationScore,
    int? startedAt,
    int? endedAt,
    int? actualDuration,
    String? sessionType,
  });
}

/// @nodoc
class __$$UpdateFocusSessionDtoImplCopyWithImpl<$Res>
    extends
        _$UpdateFocusSessionDtoCopyWithImpl<$Res, _$UpdateFocusSessionDtoImpl>
    implements _$$UpdateFocusSessionDtoImplCopyWith<$Res> {
  __$$UpdateFocusSessionDtoImplCopyWithImpl(
    _$UpdateFocusSessionDtoImpl _value,
    $Res Function(_$UpdateFocusSessionDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of UpdateFocusSessionDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? categoryId = freezed,
    Object? taskId = freezed,
    Object? notes = freezed,
    Object? concentrationScore = freezed,
    Object? startedAt = freezed,
    Object? endedAt = freezed,
    Object? actualDuration = freezed,
    Object? sessionType = freezed,
  }) {
    return _then(
      _$UpdateFocusSessionDtoImpl(
        categoryId:
            freezed == categoryId
                ? _value.categoryId
                : categoryId // ignore: cast_nullable_to_non_nullable
                    as String?,
        taskId:
            freezed == taskId
                ? _value.taskId
                : taskId // ignore: cast_nullable_to_non_nullable
                    as String?,
        notes:
            freezed == notes
                ? _value.notes
                : notes // ignore: cast_nullable_to_non_nullable
                    as String?,
        concentrationScore:
            freezed == concentrationScore
                ? _value.concentrationScore
                : concentrationScore // ignore: cast_nullable_to_non_nullable
                    as int?,
        startedAt:
            freezed == startedAt
                ? _value.startedAt
                : startedAt // ignore: cast_nullable_to_non_nullable
                    as int?,
        endedAt:
            freezed == endedAt
                ? _value.endedAt
                : endedAt // ignore: cast_nullable_to_non_nullable
                    as int?,
        actualDuration:
            freezed == actualDuration
                ? _value.actualDuration
                : actualDuration // ignore: cast_nullable_to_non_nullable
                    as int?,
        sessionType:
            freezed == sessionType
                ? _value.sessionType
                : sessionType // ignore: cast_nullable_to_non_nullable
                    as String?,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$UpdateFocusSessionDtoImpl implements _UpdateFocusSessionDto {
  const _$UpdateFocusSessionDtoImpl({
    this.categoryId,
    this.taskId,
    this.notes,
    this.concentrationScore,
    this.startedAt,
    this.endedAt,
    this.actualDuration,
    this.sessionType,
  });

  factory _$UpdateFocusSessionDtoImpl.fromJson(Map<String, dynamic> json) =>
      _$$UpdateFocusSessionDtoImplFromJson(json);

  @override
  final String? categoryId;
  @override
  final String? taskId;
  @override
  final String? notes;
  @override
  final int? concentrationScore;
  @override
  final int? startedAt;
  @override
  final int? endedAt;
  @override
  final int? actualDuration;
  @override
  final String? sessionType;

  @override
  String toString() {
    return 'UpdateFocusSessionDto(categoryId: $categoryId, taskId: $taskId, notes: $notes, concentrationScore: $concentrationScore, startedAt: $startedAt, endedAt: $endedAt, actualDuration: $actualDuration, sessionType: $sessionType)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$UpdateFocusSessionDtoImpl &&
            (identical(other.categoryId, categoryId) ||
                other.categoryId == categoryId) &&
            (identical(other.taskId, taskId) || other.taskId == taskId) &&
            (identical(other.notes, notes) || other.notes == notes) &&
            (identical(other.concentrationScore, concentrationScore) ||
                other.concentrationScore == concentrationScore) &&
            (identical(other.startedAt, startedAt) ||
                other.startedAt == startedAt) &&
            (identical(other.endedAt, endedAt) || other.endedAt == endedAt) &&
            (identical(other.actualDuration, actualDuration) ||
                other.actualDuration == actualDuration) &&
            (identical(other.sessionType, sessionType) ||
                other.sessionType == sessionType));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
    runtimeType,
    categoryId,
    taskId,
    notes,
    concentrationScore,
    startedAt,
    endedAt,
    actualDuration,
    sessionType,
  );

  /// Create a copy of UpdateFocusSessionDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$UpdateFocusSessionDtoImplCopyWith<_$UpdateFocusSessionDtoImpl>
  get copyWith =>
      __$$UpdateFocusSessionDtoImplCopyWithImpl<_$UpdateFocusSessionDtoImpl>(
        this,
        _$identity,
      );

  @override
  Map<String, dynamic> toJson() {
    return _$$UpdateFocusSessionDtoImplToJson(this);
  }
}

abstract class _UpdateFocusSessionDto implements UpdateFocusSessionDto {
  const factory _UpdateFocusSessionDto({
    final String? categoryId,
    final String? taskId,
    final String? notes,
    final int? concentrationScore,
    final int? startedAt,
    final int? endedAt,
    final int? actualDuration,
    final String? sessionType,
  }) = _$UpdateFocusSessionDtoImpl;

  factory _UpdateFocusSessionDto.fromJson(Map<String, dynamic> json) =
      _$UpdateFocusSessionDtoImpl.fromJson;

  @override
  String? get categoryId;
  @override
  String? get taskId;
  @override
  String? get notes;
  @override
  int? get concentrationScore;
  @override
  int? get startedAt;
  @override
  int? get endedAt;
  @override
  int? get actualDuration;
  @override
  String? get sessionType;

  /// Create a copy of UpdateFocusSessionDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$UpdateFocusSessionDtoImplCopyWith<_$UpdateFocusSessionDtoImpl>
  get copyWith => throw _privateConstructorUsedError;
}

FocusSessionDto _$FocusSessionDtoFromJson(Map<String, dynamic> json) {
  return _FocusSessionDto.fromJson(json);
}

/// @nodoc
mixin _$FocusSessionDto {
  String get id => throw _privateConstructorUsedError;
  String get sessionType => throw _privateConstructorUsedError;
  int get startedAt => throw _privateConstructorUsedError;
  int? get endedAt => throw _privateConstructorUsedError;
  int? get actualDuration => throw _privateConstructorUsedError;
  String? get taskId => throw _privateConstructorUsedError;
  String? get categoryId => throw _privateConstructorUsedError;
  int? get concentrationScore => throw _privateConstructorUsedError;
  String? get notes => throw _privateConstructorUsedError;
  int get createdAt => throw _privateConstructorUsedError;

  /// Serializes this FocusSessionDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of FocusSessionDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $FocusSessionDtoCopyWith<FocusSessionDto> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $FocusSessionDtoCopyWith<$Res> {
  factory $FocusSessionDtoCopyWith(
    FocusSessionDto value,
    $Res Function(FocusSessionDto) then,
  ) = _$FocusSessionDtoCopyWithImpl<$Res, FocusSessionDto>;
  @useResult
  $Res call({
    String id,
    String sessionType,
    int startedAt,
    int? endedAt,
    int? actualDuration,
    String? taskId,
    String? categoryId,
    int? concentrationScore,
    String? notes,
    int createdAt,
  });
}

/// @nodoc
class _$FocusSessionDtoCopyWithImpl<$Res, $Val extends FocusSessionDto>
    implements $FocusSessionDtoCopyWith<$Res> {
  _$FocusSessionDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of FocusSessionDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? sessionType = null,
    Object? startedAt = null,
    Object? endedAt = freezed,
    Object? actualDuration = freezed,
    Object? taskId = freezed,
    Object? categoryId = freezed,
    Object? concentrationScore = freezed,
    Object? notes = freezed,
    Object? createdAt = null,
  }) {
    return _then(
      _value.copyWith(
            id:
                null == id
                    ? _value.id
                    : id // ignore: cast_nullable_to_non_nullable
                        as String,
            sessionType:
                null == sessionType
                    ? _value.sessionType
                    : sessionType // ignore: cast_nullable_to_non_nullable
                        as String,
            startedAt:
                null == startedAt
                    ? _value.startedAt
                    : startedAt // ignore: cast_nullable_to_non_nullable
                        as int,
            endedAt:
                freezed == endedAt
                    ? _value.endedAt
                    : endedAt // ignore: cast_nullable_to_non_nullable
                        as int?,
            actualDuration:
                freezed == actualDuration
                    ? _value.actualDuration
                    : actualDuration // ignore: cast_nullable_to_non_nullable
                        as int?,
            taskId:
                freezed == taskId
                    ? _value.taskId
                    : taskId // ignore: cast_nullable_to_non_nullable
                        as String?,
            categoryId:
                freezed == categoryId
                    ? _value.categoryId
                    : categoryId // ignore: cast_nullable_to_non_nullable
                        as String?,
            concentrationScore:
                freezed == concentrationScore
                    ? _value.concentrationScore
                    : concentrationScore // ignore: cast_nullable_to_non_nullable
                        as int?,
            notes:
                freezed == notes
                    ? _value.notes
                    : notes // ignore: cast_nullable_to_non_nullable
                        as String?,
            createdAt:
                null == createdAt
                    ? _value.createdAt
                    : createdAt // ignore: cast_nullable_to_non_nullable
                        as int,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$FocusSessionDtoImplCopyWith<$Res>
    implements $FocusSessionDtoCopyWith<$Res> {
  factory _$$FocusSessionDtoImplCopyWith(
    _$FocusSessionDtoImpl value,
    $Res Function(_$FocusSessionDtoImpl) then,
  ) = __$$FocusSessionDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({
    String id,
    String sessionType,
    int startedAt,
    int? endedAt,
    int? actualDuration,
    String? taskId,
    String? categoryId,
    int? concentrationScore,
    String? notes,
    int createdAt,
  });
}

/// @nodoc
class __$$FocusSessionDtoImplCopyWithImpl<$Res>
    extends _$FocusSessionDtoCopyWithImpl<$Res, _$FocusSessionDtoImpl>
    implements _$$FocusSessionDtoImplCopyWith<$Res> {
  __$$FocusSessionDtoImplCopyWithImpl(
    _$FocusSessionDtoImpl _value,
    $Res Function(_$FocusSessionDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of FocusSessionDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? sessionType = null,
    Object? startedAt = null,
    Object? endedAt = freezed,
    Object? actualDuration = freezed,
    Object? taskId = freezed,
    Object? categoryId = freezed,
    Object? concentrationScore = freezed,
    Object? notes = freezed,
    Object? createdAt = null,
  }) {
    return _then(
      _$FocusSessionDtoImpl(
        id:
            null == id
                ? _value.id
                : id // ignore: cast_nullable_to_non_nullable
                    as String,
        sessionType:
            null == sessionType
                ? _value.sessionType
                : sessionType // ignore: cast_nullable_to_non_nullable
                    as String,
        startedAt:
            null == startedAt
                ? _value.startedAt
                : startedAt // ignore: cast_nullable_to_non_nullable
                    as int,
        endedAt:
            freezed == endedAt
                ? _value.endedAt
                : endedAt // ignore: cast_nullable_to_non_nullable
                    as int?,
        actualDuration:
            freezed == actualDuration
                ? _value.actualDuration
                : actualDuration // ignore: cast_nullable_to_non_nullable
                    as int?,
        taskId:
            freezed == taskId
                ? _value.taskId
                : taskId // ignore: cast_nullable_to_non_nullable
                    as String?,
        categoryId:
            freezed == categoryId
                ? _value.categoryId
                : categoryId // ignore: cast_nullable_to_non_nullable
                    as String?,
        concentrationScore:
            freezed == concentrationScore
                ? _value.concentrationScore
                : concentrationScore // ignore: cast_nullable_to_non_nullable
                    as int?,
        notes:
            freezed == notes
                ? _value.notes
                : notes // ignore: cast_nullable_to_non_nullable
                    as String?,
        createdAt:
            null == createdAt
                ? _value.createdAt
                : createdAt // ignore: cast_nullable_to_non_nullable
                    as int,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$FocusSessionDtoImpl implements _FocusSessionDto {
  const _$FocusSessionDtoImpl({
    required this.id,
    required this.sessionType,
    required this.startedAt,
    this.endedAt,
    this.actualDuration,
    this.taskId,
    this.categoryId,
    this.concentrationScore,
    this.notes,
    required this.createdAt,
  });

  factory _$FocusSessionDtoImpl.fromJson(Map<String, dynamic> json) =>
      _$$FocusSessionDtoImplFromJson(json);

  @override
  final String id;
  @override
  final String sessionType;
  @override
  final int startedAt;
  @override
  final int? endedAt;
  @override
  final int? actualDuration;
  @override
  final String? taskId;
  @override
  final String? categoryId;
  @override
  final int? concentrationScore;
  @override
  final String? notes;
  @override
  final int createdAt;

  @override
  String toString() {
    return 'FocusSessionDto(id: $id, sessionType: $sessionType, startedAt: $startedAt, endedAt: $endedAt, actualDuration: $actualDuration, taskId: $taskId, categoryId: $categoryId, concentrationScore: $concentrationScore, notes: $notes, createdAt: $createdAt)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$FocusSessionDtoImpl &&
            (identical(other.id, id) || other.id == id) &&
            (identical(other.sessionType, sessionType) ||
                other.sessionType == sessionType) &&
            (identical(other.startedAt, startedAt) ||
                other.startedAt == startedAt) &&
            (identical(other.endedAt, endedAt) || other.endedAt == endedAt) &&
            (identical(other.actualDuration, actualDuration) ||
                other.actualDuration == actualDuration) &&
            (identical(other.taskId, taskId) || other.taskId == taskId) &&
            (identical(other.categoryId, categoryId) ||
                other.categoryId == categoryId) &&
            (identical(other.concentrationScore, concentrationScore) ||
                other.concentrationScore == concentrationScore) &&
            (identical(other.notes, notes) || other.notes == notes) &&
            (identical(other.createdAt, createdAt) ||
                other.createdAt == createdAt));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
    runtimeType,
    id,
    sessionType,
    startedAt,
    endedAt,
    actualDuration,
    taskId,
    categoryId,
    concentrationScore,
    notes,
    createdAt,
  );

  /// Create a copy of FocusSessionDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$FocusSessionDtoImplCopyWith<_$FocusSessionDtoImpl> get copyWith =>
      __$$FocusSessionDtoImplCopyWithImpl<_$FocusSessionDtoImpl>(
        this,
        _$identity,
      );

  @override
  Map<String, dynamic> toJson() {
    return _$$FocusSessionDtoImplToJson(this);
  }
}

abstract class _FocusSessionDto implements FocusSessionDto {
  const factory _FocusSessionDto({
    required final String id,
    required final String sessionType,
    required final int startedAt,
    final int? endedAt,
    final int? actualDuration,
    final String? taskId,
    final String? categoryId,
    final int? concentrationScore,
    final String? notes,
    required final int createdAt,
  }) = _$FocusSessionDtoImpl;

  factory _FocusSessionDto.fromJson(Map<String, dynamic> json) =
      _$FocusSessionDtoImpl.fromJson;

  @override
  String get id;
  @override
  String get sessionType;
  @override
  int get startedAt;
  @override
  int? get endedAt;
  @override
  int? get actualDuration;
  @override
  String? get taskId;
  @override
  String? get categoryId;
  @override
  int? get concentrationScore;
  @override
  String? get notes;
  @override
  int get createdAt;

  /// Create a copy of FocusSessionDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$FocusSessionDtoImplCopyWith<_$FocusSessionDtoImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

CreateManualSessionResponseDto _$CreateManualSessionResponseDtoFromJson(
  Map<String, dynamic> json,
) {
  return _CreateManualSessionResponseDto.fromJson(json);
}

/// @nodoc
mixin _$CreateManualSessionResponseDto {
  String get id => throw _privateConstructorUsedError;

  /// Serializes this CreateManualSessionResponseDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of CreateManualSessionResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $CreateManualSessionResponseDtoCopyWith<CreateManualSessionResponseDto>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CreateManualSessionResponseDtoCopyWith<$Res> {
  factory $CreateManualSessionResponseDtoCopyWith(
    CreateManualSessionResponseDto value,
    $Res Function(CreateManualSessionResponseDto) then,
  ) =
      _$CreateManualSessionResponseDtoCopyWithImpl<
        $Res,
        CreateManualSessionResponseDto
      >;
  @useResult
  $Res call({String id});
}

/// @nodoc
class _$CreateManualSessionResponseDtoCopyWithImpl<
  $Res,
  $Val extends CreateManualSessionResponseDto
>
    implements $CreateManualSessionResponseDtoCopyWith<$Res> {
  _$CreateManualSessionResponseDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of CreateManualSessionResponseDto
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
abstract class _$$CreateManualSessionResponseDtoImplCopyWith<$Res>
    implements $CreateManualSessionResponseDtoCopyWith<$Res> {
  factory _$$CreateManualSessionResponseDtoImplCopyWith(
    _$CreateManualSessionResponseDtoImpl value,
    $Res Function(_$CreateManualSessionResponseDtoImpl) then,
  ) = __$$CreateManualSessionResponseDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String id});
}

/// @nodoc
class __$$CreateManualSessionResponseDtoImplCopyWithImpl<$Res>
    extends
        _$CreateManualSessionResponseDtoCopyWithImpl<
          $Res,
          _$CreateManualSessionResponseDtoImpl
        >
    implements _$$CreateManualSessionResponseDtoImplCopyWith<$Res> {
  __$$CreateManualSessionResponseDtoImplCopyWithImpl(
    _$CreateManualSessionResponseDtoImpl _value,
    $Res Function(_$CreateManualSessionResponseDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of CreateManualSessionResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? id = null}) {
    return _then(
      _$CreateManualSessionResponseDtoImpl(
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
class _$CreateManualSessionResponseDtoImpl
    implements _CreateManualSessionResponseDto {
  const _$CreateManualSessionResponseDtoImpl({required this.id});

  factory _$CreateManualSessionResponseDtoImpl.fromJson(
    Map<String, dynamic> json,
  ) => _$$CreateManualSessionResponseDtoImplFromJson(json);

  @override
  final String id;

  @override
  String toString() {
    return 'CreateManualSessionResponseDto(id: $id)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CreateManualSessionResponseDtoImpl &&
            (identical(other.id, id) || other.id == id));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, id);

  /// Create a copy of CreateManualSessionResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$CreateManualSessionResponseDtoImplCopyWith<
    _$CreateManualSessionResponseDtoImpl
  >
  get copyWith => __$$CreateManualSessionResponseDtoImplCopyWithImpl<
    _$CreateManualSessionResponseDtoImpl
  >(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$CreateManualSessionResponseDtoImplToJson(this);
  }
}

abstract class _CreateManualSessionResponseDto
    implements CreateManualSessionResponseDto {
  const factory _CreateManualSessionResponseDto({required final String id}) =
      _$CreateManualSessionResponseDtoImpl;

  factory _CreateManualSessionResponseDto.fromJson(Map<String, dynamic> json) =
      _$CreateManualSessionResponseDtoImpl.fromJson;

  @override
  String get id;

  /// Create a copy of CreateManualSessionResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$CreateManualSessionResponseDtoImplCopyWith<
    _$CreateManualSessionResponseDtoImpl
  >
  get copyWith => throw _privateConstructorUsedError;
}

GetSessionFiltersResponseDto _$GetSessionFiltersResponseDtoFromJson(
  Map<String, dynamic> json,
) {
  return _GetSessionFiltersResponseDto.fromJson(json);
}

/// @nodoc
mixin _$GetSessionFiltersResponseDto {
  List<FocusSessionDto> get focusSessions => throw _privateConstructorUsedError;

  /// Serializes this GetSessionFiltersResponseDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of GetSessionFiltersResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $GetSessionFiltersResponseDtoCopyWith<GetSessionFiltersResponseDto>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $GetSessionFiltersResponseDtoCopyWith<$Res> {
  factory $GetSessionFiltersResponseDtoCopyWith(
    GetSessionFiltersResponseDto value,
    $Res Function(GetSessionFiltersResponseDto) then,
  ) =
      _$GetSessionFiltersResponseDtoCopyWithImpl<
        $Res,
        GetSessionFiltersResponseDto
      >;
  @useResult
  $Res call({List<FocusSessionDto> focusSessions});
}

/// @nodoc
class _$GetSessionFiltersResponseDtoCopyWithImpl<
  $Res,
  $Val extends GetSessionFiltersResponseDto
>
    implements $GetSessionFiltersResponseDtoCopyWith<$Res> {
  _$GetSessionFiltersResponseDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of GetSessionFiltersResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? focusSessions = null}) {
    return _then(
      _value.copyWith(
            focusSessions:
                null == focusSessions
                    ? _value.focusSessions
                    : focusSessions // ignore: cast_nullable_to_non_nullable
                        as List<FocusSessionDto>,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$GetSessionFiltersResponseDtoImplCopyWith<$Res>
    implements $GetSessionFiltersResponseDtoCopyWith<$Res> {
  factory _$$GetSessionFiltersResponseDtoImplCopyWith(
    _$GetSessionFiltersResponseDtoImpl value,
    $Res Function(_$GetSessionFiltersResponseDtoImpl) then,
  ) = __$$GetSessionFiltersResponseDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({List<FocusSessionDto> focusSessions});
}

/// @nodoc
class __$$GetSessionFiltersResponseDtoImplCopyWithImpl<$Res>
    extends
        _$GetSessionFiltersResponseDtoCopyWithImpl<
          $Res,
          _$GetSessionFiltersResponseDtoImpl
        >
    implements _$$GetSessionFiltersResponseDtoImplCopyWith<$Res> {
  __$$GetSessionFiltersResponseDtoImplCopyWithImpl(
    _$GetSessionFiltersResponseDtoImpl _value,
    $Res Function(_$GetSessionFiltersResponseDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of GetSessionFiltersResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? focusSessions = null}) {
    return _then(
      _$GetSessionFiltersResponseDtoImpl(
        focusSessions:
            null == focusSessions
                ? _value._focusSessions
                : focusSessions // ignore: cast_nullable_to_non_nullable
                    as List<FocusSessionDto>,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$GetSessionFiltersResponseDtoImpl
    implements _GetSessionFiltersResponseDto {
  const _$GetSessionFiltersResponseDtoImpl({
    required final List<FocusSessionDto> focusSessions,
  }) : _focusSessions = focusSessions;

  factory _$GetSessionFiltersResponseDtoImpl.fromJson(
    Map<String, dynamic> json,
  ) => _$$GetSessionFiltersResponseDtoImplFromJson(json);

  final List<FocusSessionDto> _focusSessions;
  @override
  List<FocusSessionDto> get focusSessions {
    if (_focusSessions is EqualUnmodifiableListView) return _focusSessions;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_focusSessions);
  }

  @override
  String toString() {
    return 'GetSessionFiltersResponseDto(focusSessions: $focusSessions)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$GetSessionFiltersResponseDtoImpl &&
            const DeepCollectionEquality().equals(
              other._focusSessions,
              _focusSessions,
            ));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
    runtimeType,
    const DeepCollectionEquality().hash(_focusSessions),
  );

  /// Create a copy of GetSessionFiltersResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$GetSessionFiltersResponseDtoImplCopyWith<
    _$GetSessionFiltersResponseDtoImpl
  >
  get copyWith => __$$GetSessionFiltersResponseDtoImplCopyWithImpl<
    _$GetSessionFiltersResponseDtoImpl
  >(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$GetSessionFiltersResponseDtoImplToJson(this);
  }
}

abstract class _GetSessionFiltersResponseDto
    implements GetSessionFiltersResponseDto {
  const factory _GetSessionFiltersResponseDto({
    required final List<FocusSessionDto> focusSessions,
  }) = _$GetSessionFiltersResponseDtoImpl;

  factory _GetSessionFiltersResponseDto.fromJson(Map<String, dynamic> json) =
      _$GetSessionFiltersResponseDtoImpl.fromJson;

  @override
  List<FocusSessionDto> get focusSessions;

  /// Create a copy of GetSessionFiltersResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$GetSessionFiltersResponseDtoImplCopyWith<
    _$GetSessionFiltersResponseDtoImpl
  >
  get copyWith => throw _privateConstructorUsedError;
}
