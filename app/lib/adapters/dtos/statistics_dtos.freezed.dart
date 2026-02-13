// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'statistics_dtos.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$GetStatsByPeriodDto {

 int get startDate; int? get endDate;
/// Create a copy of GetStatsByPeriodDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GetStatsByPeriodDtoCopyWith<GetStatsByPeriodDto> get copyWith => _$GetStatsByPeriodDtoCopyWithImpl<GetStatsByPeriodDto>(this as GetStatsByPeriodDto, _$identity);

  /// Serializes this GetStatsByPeriodDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GetStatsByPeriodDto&&(identical(other.startDate, startDate) || other.startDate == startDate)&&(identical(other.endDate, endDate) || other.endDate == endDate));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,startDate,endDate);

@override
String toString() {
  return 'GetStatsByPeriodDto(startDate: $startDate, endDate: $endDate)';
}


}

/// @nodoc
abstract mixin class $GetStatsByPeriodDtoCopyWith<$Res>  {
  factory $GetStatsByPeriodDtoCopyWith(GetStatsByPeriodDto value, $Res Function(GetStatsByPeriodDto) _then) = _$GetStatsByPeriodDtoCopyWithImpl;
@useResult
$Res call({
 int startDate, int? endDate
});




}
/// @nodoc
class _$GetStatsByPeriodDtoCopyWithImpl<$Res>
    implements $GetStatsByPeriodDtoCopyWith<$Res> {
  _$GetStatsByPeriodDtoCopyWithImpl(this._self, this._then);

  final GetStatsByPeriodDto _self;
  final $Res Function(GetStatsByPeriodDto) _then;

/// Create a copy of GetStatsByPeriodDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? startDate = null,Object? endDate = freezed,}) {
  return _then(_self.copyWith(
startDate: null == startDate ? _self.startDate : startDate // ignore: cast_nullable_to_non_nullable
as int,endDate: freezed == endDate ? _self.endDate : endDate // ignore: cast_nullable_to_non_nullable
as int?,
  ));
}

}


/// Adds pattern-matching-related methods to [GetStatsByPeriodDto].
extension GetStatsByPeriodDtoPatterns on GetStatsByPeriodDto {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _GetStatsByPeriodDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _GetStatsByPeriodDto() when $default != null:
return $default(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _GetStatsByPeriodDto value)  $default,){
final _that = this;
switch (_that) {
case _GetStatsByPeriodDto():
return $default(_that);case _:
  throw StateError('Unexpected subclass');

}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _GetStatsByPeriodDto value)?  $default,){
final _that = this;
switch (_that) {
case _GetStatsByPeriodDto() when $default != null:
return $default(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( int startDate,  int? endDate)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _GetStatsByPeriodDto() when $default != null:
return $default(_that.startDate,_that.endDate);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( int startDate,  int? endDate)  $default,) {final _that = this;
switch (_that) {
case _GetStatsByPeriodDto():
return $default(_that.startDate,_that.endDate);case _:
  throw StateError('Unexpected subclass');

}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( int startDate,  int? endDate)?  $default,) {final _that = this;
switch (_that) {
case _GetStatsByPeriodDto() when $default != null:
return $default(_that.startDate,_that.endDate);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _GetStatsByPeriodDto implements GetStatsByPeriodDto {
  const _GetStatsByPeriodDto({required this.startDate, this.endDate});
  factory _GetStatsByPeriodDto.fromJson(Map<String, dynamic> json) => _$GetStatsByPeriodDtoFromJson(json);

@override final  int startDate;
@override final  int? endDate;

/// Create a copy of GetStatsByPeriodDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$GetStatsByPeriodDtoCopyWith<_GetStatsByPeriodDto> get copyWith => __$GetStatsByPeriodDtoCopyWithImpl<_GetStatsByPeriodDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$GetStatsByPeriodDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _GetStatsByPeriodDto&&(identical(other.startDate, startDate) || other.startDate == startDate)&&(identical(other.endDate, endDate) || other.endDate == endDate));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,startDate,endDate);

@override
String toString() {
  return 'GetStatsByPeriodDto(startDate: $startDate, endDate: $endDate)';
}


}

/// @nodoc
abstract mixin class _$GetStatsByPeriodDtoCopyWith<$Res> implements $GetStatsByPeriodDtoCopyWith<$Res> {
  factory _$GetStatsByPeriodDtoCopyWith(_GetStatsByPeriodDto value, $Res Function(_GetStatsByPeriodDto) _then) = __$GetStatsByPeriodDtoCopyWithImpl;
@override @useResult
$Res call({
 int startDate, int? endDate
});




}
/// @nodoc
class __$GetStatsByPeriodDtoCopyWithImpl<$Res>
    implements _$GetStatsByPeriodDtoCopyWith<$Res> {
  __$GetStatsByPeriodDtoCopyWithImpl(this._self, this._then);

  final _GetStatsByPeriodDto _self;
  final $Res Function(_GetStatsByPeriodDto) _then;

/// Create a copy of GetStatsByPeriodDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? startDate = null,Object? endDate = freezed,}) {
  return _then(_GetStatsByPeriodDto(
startDate: null == startDate ? _self.startDate : startDate // ignore: cast_nullable_to_non_nullable
as int,endDate: freezed == endDate ? _self.endDate : endDate // ignore: cast_nullable_to_non_nullable
as int?,
  ));
}


}


/// @nodoc
mixin _$CategoryDistributionDto {

 String get categoryId; String get categoryName; int get totalFocusTime; double get percentage; List<TaskDistributionDto> get taskDistribution;
/// Create a copy of CategoryDistributionDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CategoryDistributionDtoCopyWith<CategoryDistributionDto> get copyWith => _$CategoryDistributionDtoCopyWithImpl<CategoryDistributionDto>(this as CategoryDistributionDto, _$identity);

  /// Serializes this CategoryDistributionDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CategoryDistributionDto&&(identical(other.categoryId, categoryId) || other.categoryId == categoryId)&&(identical(other.categoryName, categoryName) || other.categoryName == categoryName)&&(identical(other.totalFocusTime, totalFocusTime) || other.totalFocusTime == totalFocusTime)&&(identical(other.percentage, percentage) || other.percentage == percentage)&&const DeepCollectionEquality().equals(other.taskDistribution, taskDistribution));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,categoryId,categoryName,totalFocusTime,percentage,const DeepCollectionEquality().hash(taskDistribution));

@override
String toString() {
  return 'CategoryDistributionDto(categoryId: $categoryId, categoryName: $categoryName, totalFocusTime: $totalFocusTime, percentage: $percentage, taskDistribution: $taskDistribution)';
}


}

/// @nodoc
abstract mixin class $CategoryDistributionDtoCopyWith<$Res>  {
  factory $CategoryDistributionDtoCopyWith(CategoryDistributionDto value, $Res Function(CategoryDistributionDto) _then) = _$CategoryDistributionDtoCopyWithImpl;
@useResult
$Res call({
 String categoryId, String categoryName, int totalFocusTime, double percentage, List<TaskDistributionDto> taskDistribution
});




}
/// @nodoc
class _$CategoryDistributionDtoCopyWithImpl<$Res>
    implements $CategoryDistributionDtoCopyWith<$Res> {
  _$CategoryDistributionDtoCopyWithImpl(this._self, this._then);

  final CategoryDistributionDto _self;
  final $Res Function(CategoryDistributionDto) _then;

/// Create a copy of CategoryDistributionDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? categoryId = null,Object? categoryName = null,Object? totalFocusTime = null,Object? percentage = null,Object? taskDistribution = null,}) {
  return _then(_self.copyWith(
categoryId: null == categoryId ? _self.categoryId : categoryId // ignore: cast_nullable_to_non_nullable
as String,categoryName: null == categoryName ? _self.categoryName : categoryName // ignore: cast_nullable_to_non_nullable
as String,totalFocusTime: null == totalFocusTime ? _self.totalFocusTime : totalFocusTime // ignore: cast_nullable_to_non_nullable
as int,percentage: null == percentage ? _self.percentage : percentage // ignore: cast_nullable_to_non_nullable
as double,taskDistribution: null == taskDistribution ? _self.taskDistribution : taskDistribution // ignore: cast_nullable_to_non_nullable
as List<TaskDistributionDto>,
  ));
}

}


/// Adds pattern-matching-related methods to [CategoryDistributionDto].
extension CategoryDistributionDtoPatterns on CategoryDistributionDto {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _CategoryDistributionDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _CategoryDistributionDto() when $default != null:
return $default(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _CategoryDistributionDto value)  $default,){
final _that = this;
switch (_that) {
case _CategoryDistributionDto():
return $default(_that);case _:
  throw StateError('Unexpected subclass');

}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _CategoryDistributionDto value)?  $default,){
final _that = this;
switch (_that) {
case _CategoryDistributionDto() when $default != null:
return $default(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( String categoryId,  String categoryName,  int totalFocusTime,  double percentage,  List<TaskDistributionDto> taskDistribution)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _CategoryDistributionDto() when $default != null:
return $default(_that.categoryId,_that.categoryName,_that.totalFocusTime,_that.percentage,_that.taskDistribution);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( String categoryId,  String categoryName,  int totalFocusTime,  double percentage,  List<TaskDistributionDto> taskDistribution)  $default,) {final _that = this;
switch (_that) {
case _CategoryDistributionDto():
return $default(_that.categoryId,_that.categoryName,_that.totalFocusTime,_that.percentage,_that.taskDistribution);case _:
  throw StateError('Unexpected subclass');

}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( String categoryId,  String categoryName,  int totalFocusTime,  double percentage,  List<TaskDistributionDto> taskDistribution)?  $default,) {final _that = this;
switch (_that) {
case _CategoryDistributionDto() when $default != null:
return $default(_that.categoryId,_that.categoryName,_that.totalFocusTime,_that.percentage,_that.taskDistribution);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _CategoryDistributionDto implements CategoryDistributionDto {
  const _CategoryDistributionDto({required this.categoryId, required this.categoryName, required this.totalFocusTime, required this.percentage, required final  List<TaskDistributionDto> taskDistribution}): _taskDistribution = taskDistribution;
  factory _CategoryDistributionDto.fromJson(Map<String, dynamic> json) => _$CategoryDistributionDtoFromJson(json);

@override final  String categoryId;
@override final  String categoryName;
@override final  int totalFocusTime;
@override final  double percentage;
 final  List<TaskDistributionDto> _taskDistribution;
@override List<TaskDistributionDto> get taskDistribution {
  if (_taskDistribution is EqualUnmodifiableListView) return _taskDistribution;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_taskDistribution);
}


/// Create a copy of CategoryDistributionDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$CategoryDistributionDtoCopyWith<_CategoryDistributionDto> get copyWith => __$CategoryDistributionDtoCopyWithImpl<_CategoryDistributionDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$CategoryDistributionDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _CategoryDistributionDto&&(identical(other.categoryId, categoryId) || other.categoryId == categoryId)&&(identical(other.categoryName, categoryName) || other.categoryName == categoryName)&&(identical(other.totalFocusTime, totalFocusTime) || other.totalFocusTime == totalFocusTime)&&(identical(other.percentage, percentage) || other.percentage == percentage)&&const DeepCollectionEquality().equals(other._taskDistribution, _taskDistribution));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,categoryId,categoryName,totalFocusTime,percentage,const DeepCollectionEquality().hash(_taskDistribution));

@override
String toString() {
  return 'CategoryDistributionDto(categoryId: $categoryId, categoryName: $categoryName, totalFocusTime: $totalFocusTime, percentage: $percentage, taskDistribution: $taskDistribution)';
}


}

/// @nodoc
abstract mixin class _$CategoryDistributionDtoCopyWith<$Res> implements $CategoryDistributionDtoCopyWith<$Res> {
  factory _$CategoryDistributionDtoCopyWith(_CategoryDistributionDto value, $Res Function(_CategoryDistributionDto) _then) = __$CategoryDistributionDtoCopyWithImpl;
@override @useResult
$Res call({
 String categoryId, String categoryName, int totalFocusTime, double percentage, List<TaskDistributionDto> taskDistribution
});




}
/// @nodoc
class __$CategoryDistributionDtoCopyWithImpl<$Res>
    implements _$CategoryDistributionDtoCopyWith<$Res> {
  __$CategoryDistributionDtoCopyWithImpl(this._self, this._then);

  final _CategoryDistributionDto _self;
  final $Res Function(_CategoryDistributionDto) _then;

/// Create a copy of CategoryDistributionDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? categoryId = null,Object? categoryName = null,Object? totalFocusTime = null,Object? percentage = null,Object? taskDistribution = null,}) {
  return _then(_CategoryDistributionDto(
categoryId: null == categoryId ? _self.categoryId : categoryId // ignore: cast_nullable_to_non_nullable
as String,categoryName: null == categoryName ? _self.categoryName : categoryName // ignore: cast_nullable_to_non_nullable
as String,totalFocusTime: null == totalFocusTime ? _self.totalFocusTime : totalFocusTime // ignore: cast_nullable_to_non_nullable
as int,percentage: null == percentage ? _self.percentage : percentage // ignore: cast_nullable_to_non_nullable
as double,taskDistribution: null == taskDistribution ? _self._taskDistribution : taskDistribution // ignore: cast_nullable_to_non_nullable
as List<TaskDistributionDto>,
  ));
}


}


/// @nodoc
mixin _$TaskDistributionDto {

 String get taskName; int get totalFocusTime; double get percentage;
/// Create a copy of TaskDistributionDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$TaskDistributionDtoCopyWith<TaskDistributionDto> get copyWith => _$TaskDistributionDtoCopyWithImpl<TaskDistributionDto>(this as TaskDistributionDto, _$identity);

  /// Serializes this TaskDistributionDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is TaskDistributionDto&&(identical(other.taskName, taskName) || other.taskName == taskName)&&(identical(other.totalFocusTime, totalFocusTime) || other.totalFocusTime == totalFocusTime)&&(identical(other.percentage, percentage) || other.percentage == percentage));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,taskName,totalFocusTime,percentage);

@override
String toString() {
  return 'TaskDistributionDto(taskName: $taskName, totalFocusTime: $totalFocusTime, percentage: $percentage)';
}


}

/// @nodoc
abstract mixin class $TaskDistributionDtoCopyWith<$Res>  {
  factory $TaskDistributionDtoCopyWith(TaskDistributionDto value, $Res Function(TaskDistributionDto) _then) = _$TaskDistributionDtoCopyWithImpl;
@useResult
$Res call({
 String taskName, int totalFocusTime, double percentage
});




}
/// @nodoc
class _$TaskDistributionDtoCopyWithImpl<$Res>
    implements $TaskDistributionDtoCopyWith<$Res> {
  _$TaskDistributionDtoCopyWithImpl(this._self, this._then);

  final TaskDistributionDto _self;
  final $Res Function(TaskDistributionDto) _then;

/// Create a copy of TaskDistributionDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? taskName = null,Object? totalFocusTime = null,Object? percentage = null,}) {
  return _then(_self.copyWith(
taskName: null == taskName ? _self.taskName : taskName // ignore: cast_nullable_to_non_nullable
as String,totalFocusTime: null == totalFocusTime ? _self.totalFocusTime : totalFocusTime // ignore: cast_nullable_to_non_nullable
as int,percentage: null == percentage ? _self.percentage : percentage // ignore: cast_nullable_to_non_nullable
as double,
  ));
}

}


/// Adds pattern-matching-related methods to [TaskDistributionDto].
extension TaskDistributionDtoPatterns on TaskDistributionDto {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _TaskDistributionDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _TaskDistributionDto() when $default != null:
return $default(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _TaskDistributionDto value)  $default,){
final _that = this;
switch (_that) {
case _TaskDistributionDto():
return $default(_that);case _:
  throw StateError('Unexpected subclass');

}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _TaskDistributionDto value)?  $default,){
final _that = this;
switch (_that) {
case _TaskDistributionDto() when $default != null:
return $default(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( String taskName,  int totalFocusTime,  double percentage)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _TaskDistributionDto() when $default != null:
return $default(_that.taskName,_that.totalFocusTime,_that.percentage);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( String taskName,  int totalFocusTime,  double percentage)  $default,) {final _that = this;
switch (_that) {
case _TaskDistributionDto():
return $default(_that.taskName,_that.totalFocusTime,_that.percentage);case _:
  throw StateError('Unexpected subclass');

}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( String taskName,  int totalFocusTime,  double percentage)?  $default,) {final _that = this;
switch (_that) {
case _TaskDistributionDto() when $default != null:
return $default(_that.taskName,_that.totalFocusTime,_that.percentage);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _TaskDistributionDto implements TaskDistributionDto {
  const _TaskDistributionDto({required this.taskName, required this.totalFocusTime, required this.percentage});
  factory _TaskDistributionDto.fromJson(Map<String, dynamic> json) => _$TaskDistributionDtoFromJson(json);

@override final  String taskName;
@override final  int totalFocusTime;
@override final  double percentage;

/// Create a copy of TaskDistributionDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$TaskDistributionDtoCopyWith<_TaskDistributionDto> get copyWith => __$TaskDistributionDtoCopyWithImpl<_TaskDistributionDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$TaskDistributionDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _TaskDistributionDto&&(identical(other.taskName, taskName) || other.taskName == taskName)&&(identical(other.totalFocusTime, totalFocusTime) || other.totalFocusTime == totalFocusTime)&&(identical(other.percentage, percentage) || other.percentage == percentage));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,taskName,totalFocusTime,percentage);

@override
String toString() {
  return 'TaskDistributionDto(taskName: $taskName, totalFocusTime: $totalFocusTime, percentage: $percentage)';
}


}

/// @nodoc
abstract mixin class _$TaskDistributionDtoCopyWith<$Res> implements $TaskDistributionDtoCopyWith<$Res> {
  factory _$TaskDistributionDtoCopyWith(_TaskDistributionDto value, $Res Function(_TaskDistributionDto) _then) = __$TaskDistributionDtoCopyWithImpl;
@override @useResult
$Res call({
 String taskName, int totalFocusTime, double percentage
});




}
/// @nodoc
class __$TaskDistributionDtoCopyWithImpl<$Res>
    implements _$TaskDistributionDtoCopyWith<$Res> {
  __$TaskDistributionDtoCopyWithImpl(this._self, this._then);

  final _TaskDistributionDto _self;
  final $Res Function(_TaskDistributionDto) _then;

/// Create a copy of TaskDistributionDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? taskName = null,Object? totalFocusTime = null,Object? percentage = null,}) {
  return _then(_TaskDistributionDto(
taskName: null == taskName ? _self.taskName : taskName // ignore: cast_nullable_to_non_nullable
as String,totalFocusTime: null == totalFocusTime ? _self.totalFocusTime : totalFocusTime // ignore: cast_nullable_to_non_nullable
as int,percentage: null == percentage ? _self.percentage : percentage // ignore: cast_nullable_to_non_nullable
as double,
  ));
}


}


/// @nodoc
mixin _$DailyActivityDistributionDto {

 String get categoryId; String get categoryName; int get totalFocusTime;
/// Create a copy of DailyActivityDistributionDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$DailyActivityDistributionDtoCopyWith<DailyActivityDistributionDto> get copyWith => _$DailyActivityDistributionDtoCopyWithImpl<DailyActivityDistributionDto>(this as DailyActivityDistributionDto, _$identity);

  /// Serializes this DailyActivityDistributionDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is DailyActivityDistributionDto&&(identical(other.categoryId, categoryId) || other.categoryId == categoryId)&&(identical(other.categoryName, categoryName) || other.categoryName == categoryName)&&(identical(other.totalFocusTime, totalFocusTime) || other.totalFocusTime == totalFocusTime));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,categoryId,categoryName,totalFocusTime);

@override
String toString() {
  return 'DailyActivityDistributionDto(categoryId: $categoryId, categoryName: $categoryName, totalFocusTime: $totalFocusTime)';
}


}

/// @nodoc
abstract mixin class $DailyActivityDistributionDtoCopyWith<$Res>  {
  factory $DailyActivityDistributionDtoCopyWith(DailyActivityDistributionDto value, $Res Function(DailyActivityDistributionDto) _then) = _$DailyActivityDistributionDtoCopyWithImpl;
@useResult
$Res call({
 String categoryId, String categoryName, int totalFocusTime
});




}
/// @nodoc
class _$DailyActivityDistributionDtoCopyWithImpl<$Res>
    implements $DailyActivityDistributionDtoCopyWith<$Res> {
  _$DailyActivityDistributionDtoCopyWithImpl(this._self, this._then);

  final DailyActivityDistributionDto _self;
  final $Res Function(DailyActivityDistributionDto) _then;

/// Create a copy of DailyActivityDistributionDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? categoryId = null,Object? categoryName = null,Object? totalFocusTime = null,}) {
  return _then(_self.copyWith(
categoryId: null == categoryId ? _self.categoryId : categoryId // ignore: cast_nullable_to_non_nullable
as String,categoryName: null == categoryName ? _self.categoryName : categoryName // ignore: cast_nullable_to_non_nullable
as String,totalFocusTime: null == totalFocusTime ? _self.totalFocusTime : totalFocusTime // ignore: cast_nullable_to_non_nullable
as int,
  ));
}

}


/// Adds pattern-matching-related methods to [DailyActivityDistributionDto].
extension DailyActivityDistributionDtoPatterns on DailyActivityDistributionDto {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _DailyActivityDistributionDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _DailyActivityDistributionDto() when $default != null:
return $default(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _DailyActivityDistributionDto value)  $default,){
final _that = this;
switch (_that) {
case _DailyActivityDistributionDto():
return $default(_that);case _:
  throw StateError('Unexpected subclass');

}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _DailyActivityDistributionDto value)?  $default,){
final _that = this;
switch (_that) {
case _DailyActivityDistributionDto() when $default != null:
return $default(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( String categoryId,  String categoryName,  int totalFocusTime)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _DailyActivityDistributionDto() when $default != null:
return $default(_that.categoryId,_that.categoryName,_that.totalFocusTime);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( String categoryId,  String categoryName,  int totalFocusTime)  $default,) {final _that = this;
switch (_that) {
case _DailyActivityDistributionDto():
return $default(_that.categoryId,_that.categoryName,_that.totalFocusTime);case _:
  throw StateError('Unexpected subclass');

}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( String categoryId,  String categoryName,  int totalFocusTime)?  $default,) {final _that = this;
switch (_that) {
case _DailyActivityDistributionDto() when $default != null:
return $default(_that.categoryId,_that.categoryName,_that.totalFocusTime);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _DailyActivityDistributionDto implements DailyActivityDistributionDto {
  const _DailyActivityDistributionDto({required this.categoryId, required this.categoryName, required this.totalFocusTime});
  factory _DailyActivityDistributionDto.fromJson(Map<String, dynamic> json) => _$DailyActivityDistributionDtoFromJson(json);

@override final  String categoryId;
@override final  String categoryName;
@override final  int totalFocusTime;

/// Create a copy of DailyActivityDistributionDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$DailyActivityDistributionDtoCopyWith<_DailyActivityDistributionDto> get copyWith => __$DailyActivityDistributionDtoCopyWithImpl<_DailyActivityDistributionDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$DailyActivityDistributionDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _DailyActivityDistributionDto&&(identical(other.categoryId, categoryId) || other.categoryId == categoryId)&&(identical(other.categoryName, categoryName) || other.categoryName == categoryName)&&(identical(other.totalFocusTime, totalFocusTime) || other.totalFocusTime == totalFocusTime));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,categoryId,categoryName,totalFocusTime);

@override
String toString() {
  return 'DailyActivityDistributionDto(categoryId: $categoryId, categoryName: $categoryName, totalFocusTime: $totalFocusTime)';
}


}

/// @nodoc
abstract mixin class _$DailyActivityDistributionDtoCopyWith<$Res> implements $DailyActivityDistributionDtoCopyWith<$Res> {
  factory _$DailyActivityDistributionDtoCopyWith(_DailyActivityDistributionDto value, $Res Function(_DailyActivityDistributionDto) _then) = __$DailyActivityDistributionDtoCopyWithImpl;
@override @useResult
$Res call({
 String categoryId, String categoryName, int totalFocusTime
});




}
/// @nodoc
class __$DailyActivityDistributionDtoCopyWithImpl<$Res>
    implements _$DailyActivityDistributionDtoCopyWith<$Res> {
  __$DailyActivityDistributionDtoCopyWithImpl(this._self, this._then);

  final _DailyActivityDistributionDto _self;
  final $Res Function(_DailyActivityDistributionDto) _then;

/// Create a copy of DailyActivityDistributionDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? categoryId = null,Object? categoryName = null,Object? totalFocusTime = null,}) {
  return _then(_DailyActivityDistributionDto(
categoryId: null == categoryId ? _self.categoryId : categoryId // ignore: cast_nullable_to_non_nullable
as String,categoryName: null == categoryName ? _self.categoryName : categoryName // ignore: cast_nullable_to_non_nullable
as String,totalFocusTime: null == totalFocusTime ? _self.totalFocusTime : totalFocusTime // ignore: cast_nullable_to_non_nullable
as int,
  ));
}


}


/// @nodoc
mixin _$DailyActivityDto {

 int get date; List<DailyActivityDistributionDto> get categoryDistribution;
/// Create a copy of DailyActivityDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$DailyActivityDtoCopyWith<DailyActivityDto> get copyWith => _$DailyActivityDtoCopyWithImpl<DailyActivityDto>(this as DailyActivityDto, _$identity);

  /// Serializes this DailyActivityDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is DailyActivityDto&&(identical(other.date, date) || other.date == date)&&const DeepCollectionEquality().equals(other.categoryDistribution, categoryDistribution));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,date,const DeepCollectionEquality().hash(categoryDistribution));

@override
String toString() {
  return 'DailyActivityDto(date: $date, categoryDistribution: $categoryDistribution)';
}


}

/// @nodoc
abstract mixin class $DailyActivityDtoCopyWith<$Res>  {
  factory $DailyActivityDtoCopyWith(DailyActivityDto value, $Res Function(DailyActivityDto) _then) = _$DailyActivityDtoCopyWithImpl;
@useResult
$Res call({
 int date, List<DailyActivityDistributionDto> categoryDistribution
});




}
/// @nodoc
class _$DailyActivityDtoCopyWithImpl<$Res>
    implements $DailyActivityDtoCopyWith<$Res> {
  _$DailyActivityDtoCopyWithImpl(this._self, this._then);

  final DailyActivityDto _self;
  final $Res Function(DailyActivityDto) _then;

/// Create a copy of DailyActivityDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? date = null,Object? categoryDistribution = null,}) {
  return _then(_self.copyWith(
date: null == date ? _self.date : date // ignore: cast_nullable_to_non_nullable
as int,categoryDistribution: null == categoryDistribution ? _self.categoryDistribution : categoryDistribution // ignore: cast_nullable_to_non_nullable
as List<DailyActivityDistributionDto>,
  ));
}

}


/// Adds pattern-matching-related methods to [DailyActivityDto].
extension DailyActivityDtoPatterns on DailyActivityDto {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _DailyActivityDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _DailyActivityDto() when $default != null:
return $default(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _DailyActivityDto value)  $default,){
final _that = this;
switch (_that) {
case _DailyActivityDto():
return $default(_that);case _:
  throw StateError('Unexpected subclass');

}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _DailyActivityDto value)?  $default,){
final _that = this;
switch (_that) {
case _DailyActivityDto() when $default != null:
return $default(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( int date,  List<DailyActivityDistributionDto> categoryDistribution)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _DailyActivityDto() when $default != null:
return $default(_that.date,_that.categoryDistribution);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( int date,  List<DailyActivityDistributionDto> categoryDistribution)  $default,) {final _that = this;
switch (_that) {
case _DailyActivityDto():
return $default(_that.date,_that.categoryDistribution);case _:
  throw StateError('Unexpected subclass');

}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( int date,  List<DailyActivityDistributionDto> categoryDistribution)?  $default,) {final _that = this;
switch (_that) {
case _DailyActivityDto() when $default != null:
return $default(_that.date,_that.categoryDistribution);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _DailyActivityDto implements DailyActivityDto {
  const _DailyActivityDto({required this.date, required final  List<DailyActivityDistributionDto> categoryDistribution}): _categoryDistribution = categoryDistribution;
  factory _DailyActivityDto.fromJson(Map<String, dynamic> json) => _$DailyActivityDtoFromJson(json);

@override final  int date;
 final  List<DailyActivityDistributionDto> _categoryDistribution;
@override List<DailyActivityDistributionDto> get categoryDistribution {
  if (_categoryDistribution is EqualUnmodifiableListView) return _categoryDistribution;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_categoryDistribution);
}


/// Create a copy of DailyActivityDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$DailyActivityDtoCopyWith<_DailyActivityDto> get copyWith => __$DailyActivityDtoCopyWithImpl<_DailyActivityDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$DailyActivityDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _DailyActivityDto&&(identical(other.date, date) || other.date == date)&&const DeepCollectionEquality().equals(other._categoryDistribution, _categoryDistribution));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,date,const DeepCollectionEquality().hash(_categoryDistribution));

@override
String toString() {
  return 'DailyActivityDto(date: $date, categoryDistribution: $categoryDistribution)';
}


}

/// @nodoc
abstract mixin class _$DailyActivityDtoCopyWith<$Res> implements $DailyActivityDtoCopyWith<$Res> {
  factory _$DailyActivityDtoCopyWith(_DailyActivityDto value, $Res Function(_DailyActivityDto) _then) = __$DailyActivityDtoCopyWithImpl;
@override @useResult
$Res call({
 int date, List<DailyActivityDistributionDto> categoryDistribution
});




}
/// @nodoc
class __$DailyActivityDtoCopyWithImpl<$Res>
    implements _$DailyActivityDtoCopyWith<$Res> {
  __$DailyActivityDtoCopyWithImpl(this._self, this._then);

  final _DailyActivityDto _self;
  final $Res Function(_DailyActivityDto) _then;

/// Create a copy of DailyActivityDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? date = null,Object? categoryDistribution = null,}) {
  return _then(_DailyActivityDto(
date: null == date ? _self.date : date // ignore: cast_nullable_to_non_nullable
as int,categoryDistribution: null == categoryDistribution ? _self._categoryDistribution : categoryDistribution // ignore: cast_nullable_to_non_nullable
as List<DailyActivityDistributionDto>,
  ));
}


}


/// @nodoc
mixin _$GetStatsByPeriodResponseDto {

 int get totalSessions; int get totalBreaks; int get totalFocusTime; int get totalBreakTime; double get focusPauseRatio; String get mostConcentratedPeriod; String get lessConcentratedPeriod; List<int> get concentrationDistribution; List<CategoryDistributionDto> get categoryDistribution; List<DailyActivityDto> get dailyActivity;
/// Create a copy of GetStatsByPeriodResponseDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GetStatsByPeriodResponseDtoCopyWith<GetStatsByPeriodResponseDto> get copyWith => _$GetStatsByPeriodResponseDtoCopyWithImpl<GetStatsByPeriodResponseDto>(this as GetStatsByPeriodResponseDto, _$identity);

  /// Serializes this GetStatsByPeriodResponseDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GetStatsByPeriodResponseDto&&(identical(other.totalSessions, totalSessions) || other.totalSessions == totalSessions)&&(identical(other.totalBreaks, totalBreaks) || other.totalBreaks == totalBreaks)&&(identical(other.totalFocusTime, totalFocusTime) || other.totalFocusTime == totalFocusTime)&&(identical(other.totalBreakTime, totalBreakTime) || other.totalBreakTime == totalBreakTime)&&(identical(other.focusPauseRatio, focusPauseRatio) || other.focusPauseRatio == focusPauseRatio)&&(identical(other.mostConcentratedPeriod, mostConcentratedPeriod) || other.mostConcentratedPeriod == mostConcentratedPeriod)&&(identical(other.lessConcentratedPeriod, lessConcentratedPeriod) || other.lessConcentratedPeriod == lessConcentratedPeriod)&&const DeepCollectionEquality().equals(other.concentrationDistribution, concentrationDistribution)&&const DeepCollectionEquality().equals(other.categoryDistribution, categoryDistribution)&&const DeepCollectionEquality().equals(other.dailyActivity, dailyActivity));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,totalSessions,totalBreaks,totalFocusTime,totalBreakTime,focusPauseRatio,mostConcentratedPeriod,lessConcentratedPeriod,const DeepCollectionEquality().hash(concentrationDistribution),const DeepCollectionEquality().hash(categoryDistribution),const DeepCollectionEquality().hash(dailyActivity));

@override
String toString() {
  return 'GetStatsByPeriodResponseDto(totalSessions: $totalSessions, totalBreaks: $totalBreaks, totalFocusTime: $totalFocusTime, totalBreakTime: $totalBreakTime, focusPauseRatio: $focusPauseRatio, mostConcentratedPeriod: $mostConcentratedPeriod, lessConcentratedPeriod: $lessConcentratedPeriod, concentrationDistribution: $concentrationDistribution, categoryDistribution: $categoryDistribution, dailyActivity: $dailyActivity)';
}


}

/// @nodoc
abstract mixin class $GetStatsByPeriodResponseDtoCopyWith<$Res>  {
  factory $GetStatsByPeriodResponseDtoCopyWith(GetStatsByPeriodResponseDto value, $Res Function(GetStatsByPeriodResponseDto) _then) = _$GetStatsByPeriodResponseDtoCopyWithImpl;
@useResult
$Res call({
 int totalSessions, int totalBreaks, int totalFocusTime, int totalBreakTime, double focusPauseRatio, String mostConcentratedPeriod, String lessConcentratedPeriod, List<int> concentrationDistribution, List<CategoryDistributionDto> categoryDistribution, List<DailyActivityDto> dailyActivity
});




}
/// @nodoc
class _$GetStatsByPeriodResponseDtoCopyWithImpl<$Res>
    implements $GetStatsByPeriodResponseDtoCopyWith<$Res> {
  _$GetStatsByPeriodResponseDtoCopyWithImpl(this._self, this._then);

  final GetStatsByPeriodResponseDto _self;
  final $Res Function(GetStatsByPeriodResponseDto) _then;

/// Create a copy of GetStatsByPeriodResponseDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? totalSessions = null,Object? totalBreaks = null,Object? totalFocusTime = null,Object? totalBreakTime = null,Object? focusPauseRatio = null,Object? mostConcentratedPeriod = null,Object? lessConcentratedPeriod = null,Object? concentrationDistribution = null,Object? categoryDistribution = null,Object? dailyActivity = null,}) {
  return _then(_self.copyWith(
totalSessions: null == totalSessions ? _self.totalSessions : totalSessions // ignore: cast_nullable_to_non_nullable
as int,totalBreaks: null == totalBreaks ? _self.totalBreaks : totalBreaks // ignore: cast_nullable_to_non_nullable
as int,totalFocusTime: null == totalFocusTime ? _self.totalFocusTime : totalFocusTime // ignore: cast_nullable_to_non_nullable
as int,totalBreakTime: null == totalBreakTime ? _self.totalBreakTime : totalBreakTime // ignore: cast_nullable_to_non_nullable
as int,focusPauseRatio: null == focusPauseRatio ? _self.focusPauseRatio : focusPauseRatio // ignore: cast_nullable_to_non_nullable
as double,mostConcentratedPeriod: null == mostConcentratedPeriod ? _self.mostConcentratedPeriod : mostConcentratedPeriod // ignore: cast_nullable_to_non_nullable
as String,lessConcentratedPeriod: null == lessConcentratedPeriod ? _self.lessConcentratedPeriod : lessConcentratedPeriod // ignore: cast_nullable_to_non_nullable
as String,concentrationDistribution: null == concentrationDistribution ? _self.concentrationDistribution : concentrationDistribution // ignore: cast_nullable_to_non_nullable
as List<int>,categoryDistribution: null == categoryDistribution ? _self.categoryDistribution : categoryDistribution // ignore: cast_nullable_to_non_nullable
as List<CategoryDistributionDto>,dailyActivity: null == dailyActivity ? _self.dailyActivity : dailyActivity // ignore: cast_nullable_to_non_nullable
as List<DailyActivityDto>,
  ));
}

}


/// Adds pattern-matching-related methods to [GetStatsByPeriodResponseDto].
extension GetStatsByPeriodResponseDtoPatterns on GetStatsByPeriodResponseDto {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _GetStatsByPeriodResponseDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _GetStatsByPeriodResponseDto() when $default != null:
return $default(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _GetStatsByPeriodResponseDto value)  $default,){
final _that = this;
switch (_that) {
case _GetStatsByPeriodResponseDto():
return $default(_that);case _:
  throw StateError('Unexpected subclass');

}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _GetStatsByPeriodResponseDto value)?  $default,){
final _that = this;
switch (_that) {
case _GetStatsByPeriodResponseDto() when $default != null:
return $default(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( int totalSessions,  int totalBreaks,  int totalFocusTime,  int totalBreakTime,  double focusPauseRatio,  String mostConcentratedPeriod,  String lessConcentratedPeriod,  List<int> concentrationDistribution,  List<CategoryDistributionDto> categoryDistribution,  List<DailyActivityDto> dailyActivity)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _GetStatsByPeriodResponseDto() when $default != null:
return $default(_that.totalSessions,_that.totalBreaks,_that.totalFocusTime,_that.totalBreakTime,_that.focusPauseRatio,_that.mostConcentratedPeriod,_that.lessConcentratedPeriod,_that.concentrationDistribution,_that.categoryDistribution,_that.dailyActivity);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( int totalSessions,  int totalBreaks,  int totalFocusTime,  int totalBreakTime,  double focusPauseRatio,  String mostConcentratedPeriod,  String lessConcentratedPeriod,  List<int> concentrationDistribution,  List<CategoryDistributionDto> categoryDistribution,  List<DailyActivityDto> dailyActivity)  $default,) {final _that = this;
switch (_that) {
case _GetStatsByPeriodResponseDto():
return $default(_that.totalSessions,_that.totalBreaks,_that.totalFocusTime,_that.totalBreakTime,_that.focusPauseRatio,_that.mostConcentratedPeriod,_that.lessConcentratedPeriod,_that.concentrationDistribution,_that.categoryDistribution,_that.dailyActivity);case _:
  throw StateError('Unexpected subclass');

}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( int totalSessions,  int totalBreaks,  int totalFocusTime,  int totalBreakTime,  double focusPauseRatio,  String mostConcentratedPeriod,  String lessConcentratedPeriod,  List<int> concentrationDistribution,  List<CategoryDistributionDto> categoryDistribution,  List<DailyActivityDto> dailyActivity)?  $default,) {final _that = this;
switch (_that) {
case _GetStatsByPeriodResponseDto() when $default != null:
return $default(_that.totalSessions,_that.totalBreaks,_that.totalFocusTime,_that.totalBreakTime,_that.focusPauseRatio,_that.mostConcentratedPeriod,_that.lessConcentratedPeriod,_that.concentrationDistribution,_that.categoryDistribution,_that.dailyActivity);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _GetStatsByPeriodResponseDto implements GetStatsByPeriodResponseDto {
  const _GetStatsByPeriodResponseDto({required this.totalSessions, required this.totalBreaks, required this.totalFocusTime, required this.totalBreakTime, required this.focusPauseRatio, required this.mostConcentratedPeriod, required this.lessConcentratedPeriod, required final  List<int> concentrationDistribution, required final  List<CategoryDistributionDto> categoryDistribution, required final  List<DailyActivityDto> dailyActivity}): _concentrationDistribution = concentrationDistribution,_categoryDistribution = categoryDistribution,_dailyActivity = dailyActivity;
  factory _GetStatsByPeriodResponseDto.fromJson(Map<String, dynamic> json) => _$GetStatsByPeriodResponseDtoFromJson(json);

@override final  int totalSessions;
@override final  int totalBreaks;
@override final  int totalFocusTime;
@override final  int totalBreakTime;
@override final  double focusPauseRatio;
@override final  String mostConcentratedPeriod;
@override final  String lessConcentratedPeriod;
 final  List<int> _concentrationDistribution;
@override List<int> get concentrationDistribution {
  if (_concentrationDistribution is EqualUnmodifiableListView) return _concentrationDistribution;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_concentrationDistribution);
}

 final  List<CategoryDistributionDto> _categoryDistribution;
@override List<CategoryDistributionDto> get categoryDistribution {
  if (_categoryDistribution is EqualUnmodifiableListView) return _categoryDistribution;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_categoryDistribution);
}

 final  List<DailyActivityDto> _dailyActivity;
@override List<DailyActivityDto> get dailyActivity {
  if (_dailyActivity is EqualUnmodifiableListView) return _dailyActivity;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_dailyActivity);
}


/// Create a copy of GetStatsByPeriodResponseDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$GetStatsByPeriodResponseDtoCopyWith<_GetStatsByPeriodResponseDto> get copyWith => __$GetStatsByPeriodResponseDtoCopyWithImpl<_GetStatsByPeriodResponseDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$GetStatsByPeriodResponseDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _GetStatsByPeriodResponseDto&&(identical(other.totalSessions, totalSessions) || other.totalSessions == totalSessions)&&(identical(other.totalBreaks, totalBreaks) || other.totalBreaks == totalBreaks)&&(identical(other.totalFocusTime, totalFocusTime) || other.totalFocusTime == totalFocusTime)&&(identical(other.totalBreakTime, totalBreakTime) || other.totalBreakTime == totalBreakTime)&&(identical(other.focusPauseRatio, focusPauseRatio) || other.focusPauseRatio == focusPauseRatio)&&(identical(other.mostConcentratedPeriod, mostConcentratedPeriod) || other.mostConcentratedPeriod == mostConcentratedPeriod)&&(identical(other.lessConcentratedPeriod, lessConcentratedPeriod) || other.lessConcentratedPeriod == lessConcentratedPeriod)&&const DeepCollectionEquality().equals(other._concentrationDistribution, _concentrationDistribution)&&const DeepCollectionEquality().equals(other._categoryDistribution, _categoryDistribution)&&const DeepCollectionEquality().equals(other._dailyActivity, _dailyActivity));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,totalSessions,totalBreaks,totalFocusTime,totalBreakTime,focusPauseRatio,mostConcentratedPeriod,lessConcentratedPeriod,const DeepCollectionEquality().hash(_concentrationDistribution),const DeepCollectionEquality().hash(_categoryDistribution),const DeepCollectionEquality().hash(_dailyActivity));

@override
String toString() {
  return 'GetStatsByPeriodResponseDto(totalSessions: $totalSessions, totalBreaks: $totalBreaks, totalFocusTime: $totalFocusTime, totalBreakTime: $totalBreakTime, focusPauseRatio: $focusPauseRatio, mostConcentratedPeriod: $mostConcentratedPeriod, lessConcentratedPeriod: $lessConcentratedPeriod, concentrationDistribution: $concentrationDistribution, categoryDistribution: $categoryDistribution, dailyActivity: $dailyActivity)';
}


}

/// @nodoc
abstract mixin class _$GetStatsByPeriodResponseDtoCopyWith<$Res> implements $GetStatsByPeriodResponseDtoCopyWith<$Res> {
  factory _$GetStatsByPeriodResponseDtoCopyWith(_GetStatsByPeriodResponseDto value, $Res Function(_GetStatsByPeriodResponseDto) _then) = __$GetStatsByPeriodResponseDtoCopyWithImpl;
@override @useResult
$Res call({
 int totalSessions, int totalBreaks, int totalFocusTime, int totalBreakTime, double focusPauseRatio, String mostConcentratedPeriod, String lessConcentratedPeriod, List<int> concentrationDistribution, List<CategoryDistributionDto> categoryDistribution, List<DailyActivityDto> dailyActivity
});




}
/// @nodoc
class __$GetStatsByPeriodResponseDtoCopyWithImpl<$Res>
    implements _$GetStatsByPeriodResponseDtoCopyWith<$Res> {
  __$GetStatsByPeriodResponseDtoCopyWithImpl(this._self, this._then);

  final _GetStatsByPeriodResponseDto _self;
  final $Res Function(_GetStatsByPeriodResponseDto) _then;

/// Create a copy of GetStatsByPeriodResponseDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? totalSessions = null,Object? totalBreaks = null,Object? totalFocusTime = null,Object? totalBreakTime = null,Object? focusPauseRatio = null,Object? mostConcentratedPeriod = null,Object? lessConcentratedPeriod = null,Object? concentrationDistribution = null,Object? categoryDistribution = null,Object? dailyActivity = null,}) {
  return _then(_GetStatsByPeriodResponseDto(
totalSessions: null == totalSessions ? _self.totalSessions : totalSessions // ignore: cast_nullable_to_non_nullable
as int,totalBreaks: null == totalBreaks ? _self.totalBreaks : totalBreaks // ignore: cast_nullable_to_non_nullable
as int,totalFocusTime: null == totalFocusTime ? _self.totalFocusTime : totalFocusTime // ignore: cast_nullable_to_non_nullable
as int,totalBreakTime: null == totalBreakTime ? _self.totalBreakTime : totalBreakTime // ignore: cast_nullable_to_non_nullable
as int,focusPauseRatio: null == focusPauseRatio ? _self.focusPauseRatio : focusPauseRatio // ignore: cast_nullable_to_non_nullable
as double,mostConcentratedPeriod: null == mostConcentratedPeriod ? _self.mostConcentratedPeriod : mostConcentratedPeriod // ignore: cast_nullable_to_non_nullable
as String,lessConcentratedPeriod: null == lessConcentratedPeriod ? _self.lessConcentratedPeriod : lessConcentratedPeriod // ignore: cast_nullable_to_non_nullable
as String,concentrationDistribution: null == concentrationDistribution ? _self._concentrationDistribution : concentrationDistribution // ignore: cast_nullable_to_non_nullable
as List<int>,categoryDistribution: null == categoryDistribution ? _self._categoryDistribution : categoryDistribution // ignore: cast_nullable_to_non_nullable
as List<CategoryDistributionDto>,dailyActivity: null == dailyActivity ? _self._dailyActivity : dailyActivity // ignore: cast_nullable_to_non_nullable
as List<DailyActivityDto>,
  ));
}


}

// dart format on
