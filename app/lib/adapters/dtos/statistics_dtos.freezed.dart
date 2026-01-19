// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'statistics_dtos.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
  'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models',
);

GetStatsByPeriodDto _$GetStatsByPeriodDtoFromJson(Map<String, dynamic> json) {
  return _GetStatsByPeriodDto.fromJson(json);
}

/// @nodoc
mixin _$GetStatsByPeriodDto {
  int get startDate => throw _privateConstructorUsedError;
  int? get endDate => throw _privateConstructorUsedError;

  /// Serializes this GetStatsByPeriodDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of GetStatsByPeriodDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $GetStatsByPeriodDtoCopyWith<GetStatsByPeriodDto> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $GetStatsByPeriodDtoCopyWith<$Res> {
  factory $GetStatsByPeriodDtoCopyWith(
    GetStatsByPeriodDto value,
    $Res Function(GetStatsByPeriodDto) then,
  ) = _$GetStatsByPeriodDtoCopyWithImpl<$Res, GetStatsByPeriodDto>;
  @useResult
  $Res call({int startDate, int? endDate});
}

/// @nodoc
class _$GetStatsByPeriodDtoCopyWithImpl<$Res, $Val extends GetStatsByPeriodDto>
    implements $GetStatsByPeriodDtoCopyWith<$Res> {
  _$GetStatsByPeriodDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of GetStatsByPeriodDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? startDate = null, Object? endDate = freezed}) {
    return _then(
      _value.copyWith(
            startDate:
                null == startDate
                    ? _value.startDate
                    : startDate // ignore: cast_nullable_to_non_nullable
                        as int,
            endDate:
                freezed == endDate
                    ? _value.endDate
                    : endDate // ignore: cast_nullable_to_non_nullable
                        as int?,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$GetStatsByPeriodDtoImplCopyWith<$Res>
    implements $GetStatsByPeriodDtoCopyWith<$Res> {
  factory _$$GetStatsByPeriodDtoImplCopyWith(
    _$GetStatsByPeriodDtoImpl value,
    $Res Function(_$GetStatsByPeriodDtoImpl) then,
  ) = __$$GetStatsByPeriodDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int startDate, int? endDate});
}

/// @nodoc
class __$$GetStatsByPeriodDtoImplCopyWithImpl<$Res>
    extends _$GetStatsByPeriodDtoCopyWithImpl<$Res, _$GetStatsByPeriodDtoImpl>
    implements _$$GetStatsByPeriodDtoImplCopyWith<$Res> {
  __$$GetStatsByPeriodDtoImplCopyWithImpl(
    _$GetStatsByPeriodDtoImpl _value,
    $Res Function(_$GetStatsByPeriodDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of GetStatsByPeriodDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? startDate = null, Object? endDate = freezed}) {
    return _then(
      _$GetStatsByPeriodDtoImpl(
        startDate:
            null == startDate
                ? _value.startDate
                : startDate // ignore: cast_nullable_to_non_nullable
                    as int,
        endDate:
            freezed == endDate
                ? _value.endDate
                : endDate // ignore: cast_nullable_to_non_nullable
                    as int?,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$GetStatsByPeriodDtoImpl implements _GetStatsByPeriodDto {
  const _$GetStatsByPeriodDtoImpl({required this.startDate, this.endDate});

  factory _$GetStatsByPeriodDtoImpl.fromJson(Map<String, dynamic> json) =>
      _$$GetStatsByPeriodDtoImplFromJson(json);

  @override
  final int startDate;
  @override
  final int? endDate;

  @override
  String toString() {
    return 'GetStatsByPeriodDto(startDate: $startDate, endDate: $endDate)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$GetStatsByPeriodDtoImpl &&
            (identical(other.startDate, startDate) ||
                other.startDate == startDate) &&
            (identical(other.endDate, endDate) || other.endDate == endDate));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, startDate, endDate);

  /// Create a copy of GetStatsByPeriodDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$GetStatsByPeriodDtoImplCopyWith<_$GetStatsByPeriodDtoImpl> get copyWith =>
      __$$GetStatsByPeriodDtoImplCopyWithImpl<_$GetStatsByPeriodDtoImpl>(
        this,
        _$identity,
      );

  @override
  Map<String, dynamic> toJson() {
    return _$$GetStatsByPeriodDtoImplToJson(this);
  }
}

abstract class _GetStatsByPeriodDto implements GetStatsByPeriodDto {
  const factory _GetStatsByPeriodDto({
    required final int startDate,
    final int? endDate,
  }) = _$GetStatsByPeriodDtoImpl;

  factory _GetStatsByPeriodDto.fromJson(Map<String, dynamic> json) =
      _$GetStatsByPeriodDtoImpl.fromJson;

  @override
  int get startDate;
  @override
  int? get endDate;

  /// Create a copy of GetStatsByPeriodDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$GetStatsByPeriodDtoImplCopyWith<_$GetStatsByPeriodDtoImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

CategoryDistributionDto _$CategoryDistributionDtoFromJson(
  Map<String, dynamic> json,
) {
  return _CategoryDistributionDto.fromJson(json);
}

/// @nodoc
mixin _$CategoryDistributionDto {
  String get categoryId => throw _privateConstructorUsedError;
  String get categoryName => throw _privateConstructorUsedError;
  int get totalFocusTime => throw _privateConstructorUsedError;
  double get percentage => throw _privateConstructorUsedError;
  List<TaskDistributionDto> get taskDistribution =>
      throw _privateConstructorUsedError;

  /// Serializes this CategoryDistributionDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of CategoryDistributionDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $CategoryDistributionDtoCopyWith<CategoryDistributionDto> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CategoryDistributionDtoCopyWith<$Res> {
  factory $CategoryDistributionDtoCopyWith(
    CategoryDistributionDto value,
    $Res Function(CategoryDistributionDto) then,
  ) = _$CategoryDistributionDtoCopyWithImpl<$Res, CategoryDistributionDto>;
  @useResult
  $Res call({
    String categoryId,
    String categoryName,
    int totalFocusTime,
    double percentage,
    List<TaskDistributionDto> taskDistribution,
  });
}

/// @nodoc
class _$CategoryDistributionDtoCopyWithImpl<
  $Res,
  $Val extends CategoryDistributionDto
>
    implements $CategoryDistributionDtoCopyWith<$Res> {
  _$CategoryDistributionDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of CategoryDistributionDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? categoryId = null,
    Object? categoryName = null,
    Object? totalFocusTime = null,
    Object? percentage = null,
    Object? taskDistribution = null,
  }) {
    return _then(
      _value.copyWith(
            categoryId:
                null == categoryId
                    ? _value.categoryId
                    : categoryId // ignore: cast_nullable_to_non_nullable
                        as String,
            categoryName:
                null == categoryName
                    ? _value.categoryName
                    : categoryName // ignore: cast_nullable_to_non_nullable
                        as String,
            totalFocusTime:
                null == totalFocusTime
                    ? _value.totalFocusTime
                    : totalFocusTime // ignore: cast_nullable_to_non_nullable
                        as int,
            percentage:
                null == percentage
                    ? _value.percentage
                    : percentage // ignore: cast_nullable_to_non_nullable
                        as double,
            taskDistribution:
                null == taskDistribution
                    ? _value.taskDistribution
                    : taskDistribution // ignore: cast_nullable_to_non_nullable
                        as List<TaskDistributionDto>,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$CategoryDistributionDtoImplCopyWith<$Res>
    implements $CategoryDistributionDtoCopyWith<$Res> {
  factory _$$CategoryDistributionDtoImplCopyWith(
    _$CategoryDistributionDtoImpl value,
    $Res Function(_$CategoryDistributionDtoImpl) then,
  ) = __$$CategoryDistributionDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({
    String categoryId,
    String categoryName,
    int totalFocusTime,
    double percentage,
    List<TaskDistributionDto> taskDistribution,
  });
}

/// @nodoc
class __$$CategoryDistributionDtoImplCopyWithImpl<$Res>
    extends
        _$CategoryDistributionDtoCopyWithImpl<
          $Res,
          _$CategoryDistributionDtoImpl
        >
    implements _$$CategoryDistributionDtoImplCopyWith<$Res> {
  __$$CategoryDistributionDtoImplCopyWithImpl(
    _$CategoryDistributionDtoImpl _value,
    $Res Function(_$CategoryDistributionDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of CategoryDistributionDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? categoryId = null,
    Object? categoryName = null,
    Object? totalFocusTime = null,
    Object? percentage = null,
    Object? taskDistribution = null,
  }) {
    return _then(
      _$CategoryDistributionDtoImpl(
        categoryId:
            null == categoryId
                ? _value.categoryId
                : categoryId // ignore: cast_nullable_to_non_nullable
                    as String,
        categoryName:
            null == categoryName
                ? _value.categoryName
                : categoryName // ignore: cast_nullable_to_non_nullable
                    as String,
        totalFocusTime:
            null == totalFocusTime
                ? _value.totalFocusTime
                : totalFocusTime // ignore: cast_nullable_to_non_nullable
                    as int,
        percentage:
            null == percentage
                ? _value.percentage
                : percentage // ignore: cast_nullable_to_non_nullable
                    as double,
        taskDistribution:
            null == taskDistribution
                ? _value._taskDistribution
                : taskDistribution // ignore: cast_nullable_to_non_nullable
                    as List<TaskDistributionDto>,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$CategoryDistributionDtoImpl implements _CategoryDistributionDto {
  const _$CategoryDistributionDtoImpl({
    required this.categoryId,
    required this.categoryName,
    required this.totalFocusTime,
    required this.percentage,
    required final List<TaskDistributionDto> taskDistribution,
  }) : _taskDistribution = taskDistribution;

  factory _$CategoryDistributionDtoImpl.fromJson(Map<String, dynamic> json) =>
      _$$CategoryDistributionDtoImplFromJson(json);

  @override
  final String categoryId;
  @override
  final String categoryName;
  @override
  final int totalFocusTime;
  @override
  final double percentage;
  final List<TaskDistributionDto> _taskDistribution;
  @override
  List<TaskDistributionDto> get taskDistribution {
    if (_taskDistribution is EqualUnmodifiableListView)
      return _taskDistribution;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_taskDistribution);
  }

  @override
  String toString() {
    return 'CategoryDistributionDto(categoryId: $categoryId, categoryName: $categoryName, totalFocusTime: $totalFocusTime, percentage: $percentage, taskDistribution: $taskDistribution)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CategoryDistributionDtoImpl &&
            (identical(other.categoryId, categoryId) ||
                other.categoryId == categoryId) &&
            (identical(other.categoryName, categoryName) ||
                other.categoryName == categoryName) &&
            (identical(other.totalFocusTime, totalFocusTime) ||
                other.totalFocusTime == totalFocusTime) &&
            (identical(other.percentage, percentage) ||
                other.percentage == percentage) &&
            const DeepCollectionEquality().equals(
              other._taskDistribution,
              _taskDistribution,
            ));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
    runtimeType,
    categoryId,
    categoryName,
    totalFocusTime,
    percentage,
    const DeepCollectionEquality().hash(_taskDistribution),
  );

  /// Create a copy of CategoryDistributionDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$CategoryDistributionDtoImplCopyWith<_$CategoryDistributionDtoImpl>
  get copyWith => __$$CategoryDistributionDtoImplCopyWithImpl<
    _$CategoryDistributionDtoImpl
  >(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$CategoryDistributionDtoImplToJson(this);
  }
}

abstract class _CategoryDistributionDto implements CategoryDistributionDto {
  const factory _CategoryDistributionDto({
    required final String categoryId,
    required final String categoryName,
    required final int totalFocusTime,
    required final double percentage,
    required final List<TaskDistributionDto> taskDistribution,
  }) = _$CategoryDistributionDtoImpl;

  factory _CategoryDistributionDto.fromJson(Map<String, dynamic> json) =
      _$CategoryDistributionDtoImpl.fromJson;

  @override
  String get categoryId;
  @override
  String get categoryName;
  @override
  int get totalFocusTime;
  @override
  double get percentage;
  @override
  List<TaskDistributionDto> get taskDistribution;

  /// Create a copy of CategoryDistributionDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$CategoryDistributionDtoImplCopyWith<_$CategoryDistributionDtoImpl>
  get copyWith => throw _privateConstructorUsedError;
}

TaskDistributionDto _$TaskDistributionDtoFromJson(Map<String, dynamic> json) {
  return _TaskDistributionDto.fromJson(json);
}

/// @nodoc
mixin _$TaskDistributionDto {
  String get taskName => throw _privateConstructorUsedError;
  int get totalFocusTime => throw _privateConstructorUsedError;
  double get percentage => throw _privateConstructorUsedError;

  /// Serializes this TaskDistributionDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of TaskDistributionDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $TaskDistributionDtoCopyWith<TaskDistributionDto> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $TaskDistributionDtoCopyWith<$Res> {
  factory $TaskDistributionDtoCopyWith(
    TaskDistributionDto value,
    $Res Function(TaskDistributionDto) then,
  ) = _$TaskDistributionDtoCopyWithImpl<$Res, TaskDistributionDto>;
  @useResult
  $Res call({String taskName, int totalFocusTime, double percentage});
}

/// @nodoc
class _$TaskDistributionDtoCopyWithImpl<$Res, $Val extends TaskDistributionDto>
    implements $TaskDistributionDtoCopyWith<$Res> {
  _$TaskDistributionDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of TaskDistributionDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? taskName = null,
    Object? totalFocusTime = null,
    Object? percentage = null,
  }) {
    return _then(
      _value.copyWith(
            taskName:
                null == taskName
                    ? _value.taskName
                    : taskName // ignore: cast_nullable_to_non_nullable
                        as String,
            totalFocusTime:
                null == totalFocusTime
                    ? _value.totalFocusTime
                    : totalFocusTime // ignore: cast_nullable_to_non_nullable
                        as int,
            percentage:
                null == percentage
                    ? _value.percentage
                    : percentage // ignore: cast_nullable_to_non_nullable
                        as double,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$TaskDistributionDtoImplCopyWith<$Res>
    implements $TaskDistributionDtoCopyWith<$Res> {
  factory _$$TaskDistributionDtoImplCopyWith(
    _$TaskDistributionDtoImpl value,
    $Res Function(_$TaskDistributionDtoImpl) then,
  ) = __$$TaskDistributionDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String taskName, int totalFocusTime, double percentage});
}

/// @nodoc
class __$$TaskDistributionDtoImplCopyWithImpl<$Res>
    extends _$TaskDistributionDtoCopyWithImpl<$Res, _$TaskDistributionDtoImpl>
    implements _$$TaskDistributionDtoImplCopyWith<$Res> {
  __$$TaskDistributionDtoImplCopyWithImpl(
    _$TaskDistributionDtoImpl _value,
    $Res Function(_$TaskDistributionDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of TaskDistributionDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? taskName = null,
    Object? totalFocusTime = null,
    Object? percentage = null,
  }) {
    return _then(
      _$TaskDistributionDtoImpl(
        taskName:
            null == taskName
                ? _value.taskName
                : taskName // ignore: cast_nullable_to_non_nullable
                    as String,
        totalFocusTime:
            null == totalFocusTime
                ? _value.totalFocusTime
                : totalFocusTime // ignore: cast_nullable_to_non_nullable
                    as int,
        percentage:
            null == percentage
                ? _value.percentage
                : percentage // ignore: cast_nullable_to_non_nullable
                    as double,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$TaskDistributionDtoImpl implements _TaskDistributionDto {
  const _$TaskDistributionDtoImpl({
    required this.taskName,
    required this.totalFocusTime,
    required this.percentage,
  });

  factory _$TaskDistributionDtoImpl.fromJson(Map<String, dynamic> json) =>
      _$$TaskDistributionDtoImplFromJson(json);

  @override
  final String taskName;
  @override
  final int totalFocusTime;
  @override
  final double percentage;

  @override
  String toString() {
    return 'TaskDistributionDto(taskName: $taskName, totalFocusTime: $totalFocusTime, percentage: $percentage)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$TaskDistributionDtoImpl &&
            (identical(other.taskName, taskName) ||
                other.taskName == taskName) &&
            (identical(other.totalFocusTime, totalFocusTime) ||
                other.totalFocusTime == totalFocusTime) &&
            (identical(other.percentage, percentage) ||
                other.percentage == percentage));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode =>
      Object.hash(runtimeType, taskName, totalFocusTime, percentage);

  /// Create a copy of TaskDistributionDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$TaskDistributionDtoImplCopyWith<_$TaskDistributionDtoImpl> get copyWith =>
      __$$TaskDistributionDtoImplCopyWithImpl<_$TaskDistributionDtoImpl>(
        this,
        _$identity,
      );

  @override
  Map<String, dynamic> toJson() {
    return _$$TaskDistributionDtoImplToJson(this);
  }
}

abstract class _TaskDistributionDto implements TaskDistributionDto {
  const factory _TaskDistributionDto({
    required final String taskName,
    required final int totalFocusTime,
    required final double percentage,
  }) = _$TaskDistributionDtoImpl;

  factory _TaskDistributionDto.fromJson(Map<String, dynamic> json) =
      _$TaskDistributionDtoImpl.fromJson;

  @override
  String get taskName;
  @override
  int get totalFocusTime;
  @override
  double get percentage;

  /// Create a copy of TaskDistributionDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$TaskDistributionDtoImplCopyWith<_$TaskDistributionDtoImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

DailyActivityDistributionDto _$DailyActivityDistributionDtoFromJson(
  Map<String, dynamic> json,
) {
  return _DailyActivityDistributionDto.fromJson(json);
}

/// @nodoc
mixin _$DailyActivityDistributionDto {
  String get categoryId => throw _privateConstructorUsedError;
  String get categoryName => throw _privateConstructorUsedError;
  int get totalFocusTime => throw _privateConstructorUsedError;

  /// Serializes this DailyActivityDistributionDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of DailyActivityDistributionDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $DailyActivityDistributionDtoCopyWith<DailyActivityDistributionDto>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $DailyActivityDistributionDtoCopyWith<$Res> {
  factory $DailyActivityDistributionDtoCopyWith(
    DailyActivityDistributionDto value,
    $Res Function(DailyActivityDistributionDto) then,
  ) =
      _$DailyActivityDistributionDtoCopyWithImpl<
        $Res,
        DailyActivityDistributionDto
      >;
  @useResult
  $Res call({String categoryId, String categoryName, int totalFocusTime});
}

/// @nodoc
class _$DailyActivityDistributionDtoCopyWithImpl<
  $Res,
  $Val extends DailyActivityDistributionDto
>
    implements $DailyActivityDistributionDtoCopyWith<$Res> {
  _$DailyActivityDistributionDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of DailyActivityDistributionDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? categoryId = null,
    Object? categoryName = null,
    Object? totalFocusTime = null,
  }) {
    return _then(
      _value.copyWith(
            categoryId:
                null == categoryId
                    ? _value.categoryId
                    : categoryId // ignore: cast_nullable_to_non_nullable
                        as String,
            categoryName:
                null == categoryName
                    ? _value.categoryName
                    : categoryName // ignore: cast_nullable_to_non_nullable
                        as String,
            totalFocusTime:
                null == totalFocusTime
                    ? _value.totalFocusTime
                    : totalFocusTime // ignore: cast_nullable_to_non_nullable
                        as int,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$DailyActivityDistributionDtoImplCopyWith<$Res>
    implements $DailyActivityDistributionDtoCopyWith<$Res> {
  factory _$$DailyActivityDistributionDtoImplCopyWith(
    _$DailyActivityDistributionDtoImpl value,
    $Res Function(_$DailyActivityDistributionDtoImpl) then,
  ) = __$$DailyActivityDistributionDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String categoryId, String categoryName, int totalFocusTime});
}

/// @nodoc
class __$$DailyActivityDistributionDtoImplCopyWithImpl<$Res>
    extends
        _$DailyActivityDistributionDtoCopyWithImpl<
          $Res,
          _$DailyActivityDistributionDtoImpl
        >
    implements _$$DailyActivityDistributionDtoImplCopyWith<$Res> {
  __$$DailyActivityDistributionDtoImplCopyWithImpl(
    _$DailyActivityDistributionDtoImpl _value,
    $Res Function(_$DailyActivityDistributionDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of DailyActivityDistributionDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? categoryId = null,
    Object? categoryName = null,
    Object? totalFocusTime = null,
  }) {
    return _then(
      _$DailyActivityDistributionDtoImpl(
        categoryId:
            null == categoryId
                ? _value.categoryId
                : categoryId // ignore: cast_nullable_to_non_nullable
                    as String,
        categoryName:
            null == categoryName
                ? _value.categoryName
                : categoryName // ignore: cast_nullable_to_non_nullable
                    as String,
        totalFocusTime:
            null == totalFocusTime
                ? _value.totalFocusTime
                : totalFocusTime // ignore: cast_nullable_to_non_nullable
                    as int,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$DailyActivityDistributionDtoImpl
    implements _DailyActivityDistributionDto {
  const _$DailyActivityDistributionDtoImpl({
    required this.categoryId,
    required this.categoryName,
    required this.totalFocusTime,
  });

  factory _$DailyActivityDistributionDtoImpl.fromJson(
    Map<String, dynamic> json,
  ) => _$$DailyActivityDistributionDtoImplFromJson(json);

  @override
  final String categoryId;
  @override
  final String categoryName;
  @override
  final int totalFocusTime;

  @override
  String toString() {
    return 'DailyActivityDistributionDto(categoryId: $categoryId, categoryName: $categoryName, totalFocusTime: $totalFocusTime)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DailyActivityDistributionDtoImpl &&
            (identical(other.categoryId, categoryId) ||
                other.categoryId == categoryId) &&
            (identical(other.categoryName, categoryName) ||
                other.categoryName == categoryName) &&
            (identical(other.totalFocusTime, totalFocusTime) ||
                other.totalFocusTime == totalFocusTime));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode =>
      Object.hash(runtimeType, categoryId, categoryName, totalFocusTime);

  /// Create a copy of DailyActivityDistributionDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$DailyActivityDistributionDtoImplCopyWith<
    _$DailyActivityDistributionDtoImpl
  >
  get copyWith => __$$DailyActivityDistributionDtoImplCopyWithImpl<
    _$DailyActivityDistributionDtoImpl
  >(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$DailyActivityDistributionDtoImplToJson(this);
  }
}

abstract class _DailyActivityDistributionDto
    implements DailyActivityDistributionDto {
  const factory _DailyActivityDistributionDto({
    required final String categoryId,
    required final String categoryName,
    required final int totalFocusTime,
  }) = _$DailyActivityDistributionDtoImpl;

  factory _DailyActivityDistributionDto.fromJson(Map<String, dynamic> json) =
      _$DailyActivityDistributionDtoImpl.fromJson;

  @override
  String get categoryId;
  @override
  String get categoryName;
  @override
  int get totalFocusTime;

  /// Create a copy of DailyActivityDistributionDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$DailyActivityDistributionDtoImplCopyWith<
    _$DailyActivityDistributionDtoImpl
  >
  get copyWith => throw _privateConstructorUsedError;
}

DailyActivityDto _$DailyActivityDtoFromJson(Map<String, dynamic> json) {
  return _DailyActivityDto.fromJson(json);
}

/// @nodoc
mixin _$DailyActivityDto {
  int get date => throw _privateConstructorUsedError;
  List<DailyActivityDistributionDto> get categoryDistribution =>
      throw _privateConstructorUsedError;

  /// Serializes this DailyActivityDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of DailyActivityDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $DailyActivityDtoCopyWith<DailyActivityDto> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $DailyActivityDtoCopyWith<$Res> {
  factory $DailyActivityDtoCopyWith(
    DailyActivityDto value,
    $Res Function(DailyActivityDto) then,
  ) = _$DailyActivityDtoCopyWithImpl<$Res, DailyActivityDto>;
  @useResult
  $Res call({
    int date,
    List<DailyActivityDistributionDto> categoryDistribution,
  });
}

/// @nodoc
class _$DailyActivityDtoCopyWithImpl<$Res, $Val extends DailyActivityDto>
    implements $DailyActivityDtoCopyWith<$Res> {
  _$DailyActivityDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of DailyActivityDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? date = null, Object? categoryDistribution = null}) {
    return _then(
      _value.copyWith(
            date:
                null == date
                    ? _value.date
                    : date // ignore: cast_nullable_to_non_nullable
                        as int,
            categoryDistribution:
                null == categoryDistribution
                    ? _value.categoryDistribution
                    : categoryDistribution // ignore: cast_nullable_to_non_nullable
                        as List<DailyActivityDistributionDto>,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$DailyActivityDtoImplCopyWith<$Res>
    implements $DailyActivityDtoCopyWith<$Res> {
  factory _$$DailyActivityDtoImplCopyWith(
    _$DailyActivityDtoImpl value,
    $Res Function(_$DailyActivityDtoImpl) then,
  ) = __$$DailyActivityDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({
    int date,
    List<DailyActivityDistributionDto> categoryDistribution,
  });
}

/// @nodoc
class __$$DailyActivityDtoImplCopyWithImpl<$Res>
    extends _$DailyActivityDtoCopyWithImpl<$Res, _$DailyActivityDtoImpl>
    implements _$$DailyActivityDtoImplCopyWith<$Res> {
  __$$DailyActivityDtoImplCopyWithImpl(
    _$DailyActivityDtoImpl _value,
    $Res Function(_$DailyActivityDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of DailyActivityDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? date = null, Object? categoryDistribution = null}) {
    return _then(
      _$DailyActivityDtoImpl(
        date:
            null == date
                ? _value.date
                : date // ignore: cast_nullable_to_non_nullable
                    as int,
        categoryDistribution:
            null == categoryDistribution
                ? _value._categoryDistribution
                : categoryDistribution // ignore: cast_nullable_to_non_nullable
                    as List<DailyActivityDistributionDto>,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$DailyActivityDtoImpl implements _DailyActivityDto {
  const _$DailyActivityDtoImpl({
    required this.date,
    required final List<DailyActivityDistributionDto> categoryDistribution,
  }) : _categoryDistribution = categoryDistribution;

  factory _$DailyActivityDtoImpl.fromJson(Map<String, dynamic> json) =>
      _$$DailyActivityDtoImplFromJson(json);

  @override
  final int date;
  final List<DailyActivityDistributionDto> _categoryDistribution;
  @override
  List<DailyActivityDistributionDto> get categoryDistribution {
    if (_categoryDistribution is EqualUnmodifiableListView)
      return _categoryDistribution;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_categoryDistribution);
  }

  @override
  String toString() {
    return 'DailyActivityDto(date: $date, categoryDistribution: $categoryDistribution)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DailyActivityDtoImpl &&
            (identical(other.date, date) || other.date == date) &&
            const DeepCollectionEquality().equals(
              other._categoryDistribution,
              _categoryDistribution,
            ));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
    runtimeType,
    date,
    const DeepCollectionEquality().hash(_categoryDistribution),
  );

  /// Create a copy of DailyActivityDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$DailyActivityDtoImplCopyWith<_$DailyActivityDtoImpl> get copyWith =>
      __$$DailyActivityDtoImplCopyWithImpl<_$DailyActivityDtoImpl>(
        this,
        _$identity,
      );

  @override
  Map<String, dynamic> toJson() {
    return _$$DailyActivityDtoImplToJson(this);
  }
}

abstract class _DailyActivityDto implements DailyActivityDto {
  const factory _DailyActivityDto({
    required final int date,
    required final List<DailyActivityDistributionDto> categoryDistribution,
  }) = _$DailyActivityDtoImpl;

  factory _DailyActivityDto.fromJson(Map<String, dynamic> json) =
      _$DailyActivityDtoImpl.fromJson;

  @override
  int get date;
  @override
  List<DailyActivityDistributionDto> get categoryDistribution;

  /// Create a copy of DailyActivityDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$DailyActivityDtoImplCopyWith<_$DailyActivityDtoImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

GetStatsByPeriodResponseDto _$GetStatsByPeriodResponseDtoFromJson(
  Map<String, dynamic> json,
) {
  return _GetStatsByPeriodResponseDto.fromJson(json);
}

/// @nodoc
mixin _$GetStatsByPeriodResponseDto {
  int get totalSessions => throw _privateConstructorUsedError;
  int get totalBreaks => throw _privateConstructorUsedError;
  int get totalFocusTime => throw _privateConstructorUsedError;
  int get totalBreakTime => throw _privateConstructorUsedError;
  String get mostConcentratedPeriod => throw _privateConstructorUsedError;
  String get lessConcentratedPeriod => throw _privateConstructorUsedError;
  List<int> get concentrationDistribution => throw _privateConstructorUsedError;
  List<CategoryDistributionDto> get categoryDistribution =>
      throw _privateConstructorUsedError;
  List<DailyActivityDto> get dailyActivity =>
      throw _privateConstructorUsedError;

  /// Serializes this GetStatsByPeriodResponseDto to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of GetStatsByPeriodResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $GetStatsByPeriodResponseDtoCopyWith<GetStatsByPeriodResponseDto>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $GetStatsByPeriodResponseDtoCopyWith<$Res> {
  factory $GetStatsByPeriodResponseDtoCopyWith(
    GetStatsByPeriodResponseDto value,
    $Res Function(GetStatsByPeriodResponseDto) then,
  ) =
      _$GetStatsByPeriodResponseDtoCopyWithImpl<
        $Res,
        GetStatsByPeriodResponseDto
      >;
  @useResult
  $Res call({
    int totalSessions,
    int totalBreaks,
    int totalFocusTime,
    int totalBreakTime,
    String mostConcentratedPeriod,
    String lessConcentratedPeriod,
    List<int> concentrationDistribution,
    List<CategoryDistributionDto> categoryDistribution,
    List<DailyActivityDto> dailyActivity,
  });
}

/// @nodoc
class _$GetStatsByPeriodResponseDtoCopyWithImpl<
  $Res,
  $Val extends GetStatsByPeriodResponseDto
>
    implements $GetStatsByPeriodResponseDtoCopyWith<$Res> {
  _$GetStatsByPeriodResponseDtoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of GetStatsByPeriodResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? totalSessions = null,
    Object? totalBreaks = null,
    Object? totalFocusTime = null,
    Object? totalBreakTime = null,
    Object? mostConcentratedPeriod = null,
    Object? lessConcentratedPeriod = null,
    Object? concentrationDistribution = null,
    Object? categoryDistribution = null,
    Object? dailyActivity = null,
  }) {
    return _then(
      _value.copyWith(
            totalSessions:
                null == totalSessions
                    ? _value.totalSessions
                    : totalSessions // ignore: cast_nullable_to_non_nullable
                        as int,
            totalBreaks:
                null == totalBreaks
                    ? _value.totalBreaks
                    : totalBreaks // ignore: cast_nullable_to_non_nullable
                        as int,
            totalFocusTime:
                null == totalFocusTime
                    ? _value.totalFocusTime
                    : totalFocusTime // ignore: cast_nullable_to_non_nullable
                        as int,
            totalBreakTime:
                null == totalBreakTime
                    ? _value.totalBreakTime
                    : totalBreakTime // ignore: cast_nullable_to_non_nullable
                        as int,
            mostConcentratedPeriod:
                null == mostConcentratedPeriod
                    ? _value.mostConcentratedPeriod
                    : mostConcentratedPeriod // ignore: cast_nullable_to_non_nullable
                        as String,
            lessConcentratedPeriod:
                null == lessConcentratedPeriod
                    ? _value.lessConcentratedPeriod
                    : lessConcentratedPeriod // ignore: cast_nullable_to_non_nullable
                        as String,
            concentrationDistribution:
                null == concentrationDistribution
                    ? _value.concentrationDistribution
                    : concentrationDistribution // ignore: cast_nullable_to_non_nullable
                        as List<int>,
            categoryDistribution:
                null == categoryDistribution
                    ? _value.categoryDistribution
                    : categoryDistribution // ignore: cast_nullable_to_non_nullable
                        as List<CategoryDistributionDto>,
            dailyActivity:
                null == dailyActivity
                    ? _value.dailyActivity
                    : dailyActivity // ignore: cast_nullable_to_non_nullable
                        as List<DailyActivityDto>,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$GetStatsByPeriodResponseDtoImplCopyWith<$Res>
    implements $GetStatsByPeriodResponseDtoCopyWith<$Res> {
  factory _$$GetStatsByPeriodResponseDtoImplCopyWith(
    _$GetStatsByPeriodResponseDtoImpl value,
    $Res Function(_$GetStatsByPeriodResponseDtoImpl) then,
  ) = __$$GetStatsByPeriodResponseDtoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({
    int totalSessions,
    int totalBreaks,
    int totalFocusTime,
    int totalBreakTime,
    String mostConcentratedPeriod,
    String lessConcentratedPeriod,
    List<int> concentrationDistribution,
    List<CategoryDistributionDto> categoryDistribution,
    List<DailyActivityDto> dailyActivity,
  });
}

/// @nodoc
class __$$GetStatsByPeriodResponseDtoImplCopyWithImpl<$Res>
    extends
        _$GetStatsByPeriodResponseDtoCopyWithImpl<
          $Res,
          _$GetStatsByPeriodResponseDtoImpl
        >
    implements _$$GetStatsByPeriodResponseDtoImplCopyWith<$Res> {
  __$$GetStatsByPeriodResponseDtoImplCopyWithImpl(
    _$GetStatsByPeriodResponseDtoImpl _value,
    $Res Function(_$GetStatsByPeriodResponseDtoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of GetStatsByPeriodResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? totalSessions = null,
    Object? totalBreaks = null,
    Object? totalFocusTime = null,
    Object? totalBreakTime = null,
    Object? mostConcentratedPeriod = null,
    Object? lessConcentratedPeriod = null,
    Object? concentrationDistribution = null,
    Object? categoryDistribution = null,
    Object? dailyActivity = null,
  }) {
    return _then(
      _$GetStatsByPeriodResponseDtoImpl(
        totalSessions:
            null == totalSessions
                ? _value.totalSessions
                : totalSessions // ignore: cast_nullable_to_non_nullable
                    as int,
        totalBreaks:
            null == totalBreaks
                ? _value.totalBreaks
                : totalBreaks // ignore: cast_nullable_to_non_nullable
                    as int,
        totalFocusTime:
            null == totalFocusTime
                ? _value.totalFocusTime
                : totalFocusTime // ignore: cast_nullable_to_non_nullable
                    as int,
        totalBreakTime:
            null == totalBreakTime
                ? _value.totalBreakTime
                : totalBreakTime // ignore: cast_nullable_to_non_nullable
                    as int,
        mostConcentratedPeriod:
            null == mostConcentratedPeriod
                ? _value.mostConcentratedPeriod
                : mostConcentratedPeriod // ignore: cast_nullable_to_non_nullable
                    as String,
        lessConcentratedPeriod:
            null == lessConcentratedPeriod
                ? _value.lessConcentratedPeriod
                : lessConcentratedPeriod // ignore: cast_nullable_to_non_nullable
                    as String,
        concentrationDistribution:
            null == concentrationDistribution
                ? _value._concentrationDistribution
                : concentrationDistribution // ignore: cast_nullable_to_non_nullable
                    as List<int>,
        categoryDistribution:
            null == categoryDistribution
                ? _value._categoryDistribution
                : categoryDistribution // ignore: cast_nullable_to_non_nullable
                    as List<CategoryDistributionDto>,
        dailyActivity:
            null == dailyActivity
                ? _value._dailyActivity
                : dailyActivity // ignore: cast_nullable_to_non_nullable
                    as List<DailyActivityDto>,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$GetStatsByPeriodResponseDtoImpl
    implements _GetStatsByPeriodResponseDto {
  const _$GetStatsByPeriodResponseDtoImpl({
    required this.totalSessions,
    required this.totalBreaks,
    required this.totalFocusTime,
    required this.totalBreakTime,
    required this.mostConcentratedPeriod,
    required this.lessConcentratedPeriod,
    required final List<int> concentrationDistribution,
    required final List<CategoryDistributionDto> categoryDistribution,
    required final List<DailyActivityDto> dailyActivity,
  }) : _concentrationDistribution = concentrationDistribution,
       _categoryDistribution = categoryDistribution,
       _dailyActivity = dailyActivity;

  factory _$GetStatsByPeriodResponseDtoImpl.fromJson(
    Map<String, dynamic> json,
  ) => _$$GetStatsByPeriodResponseDtoImplFromJson(json);

  @override
  final int totalSessions;
  @override
  final int totalBreaks;
  @override
  final int totalFocusTime;
  @override
  final int totalBreakTime;
  @override
  final String mostConcentratedPeriod;
  @override
  final String lessConcentratedPeriod;
  final List<int> _concentrationDistribution;
  @override
  List<int> get concentrationDistribution {
    if (_concentrationDistribution is EqualUnmodifiableListView)
      return _concentrationDistribution;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_concentrationDistribution);
  }

  final List<CategoryDistributionDto> _categoryDistribution;
  @override
  List<CategoryDistributionDto> get categoryDistribution {
    if (_categoryDistribution is EqualUnmodifiableListView)
      return _categoryDistribution;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_categoryDistribution);
  }

  final List<DailyActivityDto> _dailyActivity;
  @override
  List<DailyActivityDto> get dailyActivity {
    if (_dailyActivity is EqualUnmodifiableListView) return _dailyActivity;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_dailyActivity);
  }

  @override
  String toString() {
    return 'GetStatsByPeriodResponseDto(totalSessions: $totalSessions, totalBreaks: $totalBreaks, totalFocusTime: $totalFocusTime, totalBreakTime: $totalBreakTime, mostConcentratedPeriod: $mostConcentratedPeriod, lessConcentratedPeriod: $lessConcentratedPeriod, concentrationDistribution: $concentrationDistribution, categoryDistribution: $categoryDistribution, dailyActivity: $dailyActivity)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$GetStatsByPeriodResponseDtoImpl &&
            (identical(other.totalSessions, totalSessions) ||
                other.totalSessions == totalSessions) &&
            (identical(other.totalBreaks, totalBreaks) ||
                other.totalBreaks == totalBreaks) &&
            (identical(other.totalFocusTime, totalFocusTime) ||
                other.totalFocusTime == totalFocusTime) &&
            (identical(other.totalBreakTime, totalBreakTime) ||
                other.totalBreakTime == totalBreakTime) &&
            (identical(other.mostConcentratedPeriod, mostConcentratedPeriod) ||
                other.mostConcentratedPeriod == mostConcentratedPeriod) &&
            (identical(other.lessConcentratedPeriod, lessConcentratedPeriod) ||
                other.lessConcentratedPeriod == lessConcentratedPeriod) &&
            const DeepCollectionEquality().equals(
              other._concentrationDistribution,
              _concentrationDistribution,
            ) &&
            const DeepCollectionEquality().equals(
              other._categoryDistribution,
              _categoryDistribution,
            ) &&
            const DeepCollectionEquality().equals(
              other._dailyActivity,
              _dailyActivity,
            ));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
    runtimeType,
    totalSessions,
    totalBreaks,
    totalFocusTime,
    totalBreakTime,
    mostConcentratedPeriod,
    lessConcentratedPeriod,
    const DeepCollectionEquality().hash(_concentrationDistribution),
    const DeepCollectionEquality().hash(_categoryDistribution),
    const DeepCollectionEquality().hash(_dailyActivity),
  );

  /// Create a copy of GetStatsByPeriodResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$GetStatsByPeriodResponseDtoImplCopyWith<_$GetStatsByPeriodResponseDtoImpl>
  get copyWith => __$$GetStatsByPeriodResponseDtoImplCopyWithImpl<
    _$GetStatsByPeriodResponseDtoImpl
  >(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$GetStatsByPeriodResponseDtoImplToJson(this);
  }
}

abstract class _GetStatsByPeriodResponseDto
    implements GetStatsByPeriodResponseDto {
  const factory _GetStatsByPeriodResponseDto({
    required final int totalSessions,
    required final int totalBreaks,
    required final int totalFocusTime,
    required final int totalBreakTime,
    required final String mostConcentratedPeriod,
    required final String lessConcentratedPeriod,
    required final List<int> concentrationDistribution,
    required final List<CategoryDistributionDto> categoryDistribution,
    required final List<DailyActivityDto> dailyActivity,
  }) = _$GetStatsByPeriodResponseDtoImpl;

  factory _GetStatsByPeriodResponseDto.fromJson(Map<String, dynamic> json) =
      _$GetStatsByPeriodResponseDtoImpl.fromJson;

  @override
  int get totalSessions;
  @override
  int get totalBreaks;
  @override
  int get totalFocusTime;
  @override
  int get totalBreakTime;
  @override
  String get mostConcentratedPeriod;
  @override
  String get lessConcentratedPeriod;
  @override
  List<int> get concentrationDistribution;
  @override
  List<CategoryDistributionDto> get categoryDistribution;
  @override
  List<DailyActivityDto> get dailyActivity;

  /// Create a copy of GetStatsByPeriodResponseDto
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$GetStatsByPeriodResponseDtoImplCopyWith<_$GetStatsByPeriodResponseDtoImpl>
  get copyWith => throw _privateConstructorUsedError;
}
