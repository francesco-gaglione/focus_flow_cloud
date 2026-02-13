// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'session_dtos.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$CreateManualSessionDto {

 String get sessionType; int get startedAt; int get endedAt; String? get taskId; String? get categoryId; int? get concentrationScore; String? get notes;
/// Create a copy of CreateManualSessionDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CreateManualSessionDtoCopyWith<CreateManualSessionDto> get copyWith => _$CreateManualSessionDtoCopyWithImpl<CreateManualSessionDto>(this as CreateManualSessionDto, _$identity);

  /// Serializes this CreateManualSessionDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CreateManualSessionDto&&(identical(other.sessionType, sessionType) || other.sessionType == sessionType)&&(identical(other.startedAt, startedAt) || other.startedAt == startedAt)&&(identical(other.endedAt, endedAt) || other.endedAt == endedAt)&&(identical(other.taskId, taskId) || other.taskId == taskId)&&(identical(other.categoryId, categoryId) || other.categoryId == categoryId)&&(identical(other.concentrationScore, concentrationScore) || other.concentrationScore == concentrationScore)&&(identical(other.notes, notes) || other.notes == notes));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,sessionType,startedAt,endedAt,taskId,categoryId,concentrationScore,notes);

@override
String toString() {
  return 'CreateManualSessionDto(sessionType: $sessionType, startedAt: $startedAt, endedAt: $endedAt, taskId: $taskId, categoryId: $categoryId, concentrationScore: $concentrationScore, notes: $notes)';
}


}

/// @nodoc
abstract mixin class $CreateManualSessionDtoCopyWith<$Res>  {
  factory $CreateManualSessionDtoCopyWith(CreateManualSessionDto value, $Res Function(CreateManualSessionDto) _then) = _$CreateManualSessionDtoCopyWithImpl;
@useResult
$Res call({
 String sessionType, int startedAt, int endedAt, String? taskId, String? categoryId, int? concentrationScore, String? notes
});




}
/// @nodoc
class _$CreateManualSessionDtoCopyWithImpl<$Res>
    implements $CreateManualSessionDtoCopyWith<$Res> {
  _$CreateManualSessionDtoCopyWithImpl(this._self, this._then);

  final CreateManualSessionDto _self;
  final $Res Function(CreateManualSessionDto) _then;

/// Create a copy of CreateManualSessionDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? sessionType = null,Object? startedAt = null,Object? endedAt = null,Object? taskId = freezed,Object? categoryId = freezed,Object? concentrationScore = freezed,Object? notes = freezed,}) {
  return _then(_self.copyWith(
sessionType: null == sessionType ? _self.sessionType : sessionType // ignore: cast_nullable_to_non_nullable
as String,startedAt: null == startedAt ? _self.startedAt : startedAt // ignore: cast_nullable_to_non_nullable
as int,endedAt: null == endedAt ? _self.endedAt : endedAt // ignore: cast_nullable_to_non_nullable
as int,taskId: freezed == taskId ? _self.taskId : taskId // ignore: cast_nullable_to_non_nullable
as String?,categoryId: freezed == categoryId ? _self.categoryId : categoryId // ignore: cast_nullable_to_non_nullable
as String?,concentrationScore: freezed == concentrationScore ? _self.concentrationScore : concentrationScore // ignore: cast_nullable_to_non_nullable
as int?,notes: freezed == notes ? _self.notes : notes // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}

}


/// Adds pattern-matching-related methods to [CreateManualSessionDto].
extension CreateManualSessionDtoPatterns on CreateManualSessionDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _CreateManualSessionDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _CreateManualSessionDto() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _CreateManualSessionDto value)  $default,){
final _that = this;
switch (_that) {
case _CreateManualSessionDto():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _CreateManualSessionDto value)?  $default,){
final _that = this;
switch (_that) {
case _CreateManualSessionDto() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( String sessionType,  int startedAt,  int endedAt,  String? taskId,  String? categoryId,  int? concentrationScore,  String? notes)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _CreateManualSessionDto() when $default != null:
return $default(_that.sessionType,_that.startedAt,_that.endedAt,_that.taskId,_that.categoryId,_that.concentrationScore,_that.notes);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( String sessionType,  int startedAt,  int endedAt,  String? taskId,  String? categoryId,  int? concentrationScore,  String? notes)  $default,) {final _that = this;
switch (_that) {
case _CreateManualSessionDto():
return $default(_that.sessionType,_that.startedAt,_that.endedAt,_that.taskId,_that.categoryId,_that.concentrationScore,_that.notes);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( String sessionType,  int startedAt,  int endedAt,  String? taskId,  String? categoryId,  int? concentrationScore,  String? notes)?  $default,) {final _that = this;
switch (_that) {
case _CreateManualSessionDto() when $default != null:
return $default(_that.sessionType,_that.startedAt,_that.endedAt,_that.taskId,_that.categoryId,_that.concentrationScore,_that.notes);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _CreateManualSessionDto extends CreateManualSessionDto {
  const _CreateManualSessionDto({required this.sessionType, required this.startedAt, required this.endedAt, this.taskId, this.categoryId, this.concentrationScore, this.notes}): super._();
  factory _CreateManualSessionDto.fromJson(Map<String, dynamic> json) => _$CreateManualSessionDtoFromJson(json);

@override final  String sessionType;
@override final  int startedAt;
@override final  int endedAt;
@override final  String? taskId;
@override final  String? categoryId;
@override final  int? concentrationScore;
@override final  String? notes;

/// Create a copy of CreateManualSessionDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$CreateManualSessionDtoCopyWith<_CreateManualSessionDto> get copyWith => __$CreateManualSessionDtoCopyWithImpl<_CreateManualSessionDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$CreateManualSessionDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _CreateManualSessionDto&&(identical(other.sessionType, sessionType) || other.sessionType == sessionType)&&(identical(other.startedAt, startedAt) || other.startedAt == startedAt)&&(identical(other.endedAt, endedAt) || other.endedAt == endedAt)&&(identical(other.taskId, taskId) || other.taskId == taskId)&&(identical(other.categoryId, categoryId) || other.categoryId == categoryId)&&(identical(other.concentrationScore, concentrationScore) || other.concentrationScore == concentrationScore)&&(identical(other.notes, notes) || other.notes == notes));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,sessionType,startedAt,endedAt,taskId,categoryId,concentrationScore,notes);

@override
String toString() {
  return 'CreateManualSessionDto(sessionType: $sessionType, startedAt: $startedAt, endedAt: $endedAt, taskId: $taskId, categoryId: $categoryId, concentrationScore: $concentrationScore, notes: $notes)';
}


}

/// @nodoc
abstract mixin class _$CreateManualSessionDtoCopyWith<$Res> implements $CreateManualSessionDtoCopyWith<$Res> {
  factory _$CreateManualSessionDtoCopyWith(_CreateManualSessionDto value, $Res Function(_CreateManualSessionDto) _then) = __$CreateManualSessionDtoCopyWithImpl;
@override @useResult
$Res call({
 String sessionType, int startedAt, int endedAt, String? taskId, String? categoryId, int? concentrationScore, String? notes
});




}
/// @nodoc
class __$CreateManualSessionDtoCopyWithImpl<$Res>
    implements _$CreateManualSessionDtoCopyWith<$Res> {
  __$CreateManualSessionDtoCopyWithImpl(this._self, this._then);

  final _CreateManualSessionDto _self;
  final $Res Function(_CreateManualSessionDto) _then;

/// Create a copy of CreateManualSessionDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? sessionType = null,Object? startedAt = null,Object? endedAt = null,Object? taskId = freezed,Object? categoryId = freezed,Object? concentrationScore = freezed,Object? notes = freezed,}) {
  return _then(_CreateManualSessionDto(
sessionType: null == sessionType ? _self.sessionType : sessionType // ignore: cast_nullable_to_non_nullable
as String,startedAt: null == startedAt ? _self.startedAt : startedAt // ignore: cast_nullable_to_non_nullable
as int,endedAt: null == endedAt ? _self.endedAt : endedAt // ignore: cast_nullable_to_non_nullable
as int,taskId: freezed == taskId ? _self.taskId : taskId // ignore: cast_nullable_to_non_nullable
as String?,categoryId: freezed == categoryId ? _self.categoryId : categoryId // ignore: cast_nullable_to_non_nullable
as String?,concentrationScore: freezed == concentrationScore ? _self.concentrationScore : concentrationScore // ignore: cast_nullable_to_non_nullable
as int?,notes: freezed == notes ? _self.notes : notes // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}


}


/// @nodoc
mixin _$GetSessionFiltersDto {

 int? get startDate; int? get endDate; List<String>? get categoryIds; String? get sessionType; int? get minConcentrationScore; int? get maxConcentrationScore;
/// Create a copy of GetSessionFiltersDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GetSessionFiltersDtoCopyWith<GetSessionFiltersDto> get copyWith => _$GetSessionFiltersDtoCopyWithImpl<GetSessionFiltersDto>(this as GetSessionFiltersDto, _$identity);

  /// Serializes this GetSessionFiltersDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GetSessionFiltersDto&&(identical(other.startDate, startDate) || other.startDate == startDate)&&(identical(other.endDate, endDate) || other.endDate == endDate)&&const DeepCollectionEquality().equals(other.categoryIds, categoryIds)&&(identical(other.sessionType, sessionType) || other.sessionType == sessionType)&&(identical(other.minConcentrationScore, minConcentrationScore) || other.minConcentrationScore == minConcentrationScore)&&(identical(other.maxConcentrationScore, maxConcentrationScore) || other.maxConcentrationScore == maxConcentrationScore));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,startDate,endDate,const DeepCollectionEquality().hash(categoryIds),sessionType,minConcentrationScore,maxConcentrationScore);

@override
String toString() {
  return 'GetSessionFiltersDto(startDate: $startDate, endDate: $endDate, categoryIds: $categoryIds, sessionType: $sessionType, minConcentrationScore: $minConcentrationScore, maxConcentrationScore: $maxConcentrationScore)';
}


}

/// @nodoc
abstract mixin class $GetSessionFiltersDtoCopyWith<$Res>  {
  factory $GetSessionFiltersDtoCopyWith(GetSessionFiltersDto value, $Res Function(GetSessionFiltersDto) _then) = _$GetSessionFiltersDtoCopyWithImpl;
@useResult
$Res call({
 int? startDate, int? endDate, List<String>? categoryIds, String? sessionType, int? minConcentrationScore, int? maxConcentrationScore
});




}
/// @nodoc
class _$GetSessionFiltersDtoCopyWithImpl<$Res>
    implements $GetSessionFiltersDtoCopyWith<$Res> {
  _$GetSessionFiltersDtoCopyWithImpl(this._self, this._then);

  final GetSessionFiltersDto _self;
  final $Res Function(GetSessionFiltersDto) _then;

/// Create a copy of GetSessionFiltersDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? startDate = freezed,Object? endDate = freezed,Object? categoryIds = freezed,Object? sessionType = freezed,Object? minConcentrationScore = freezed,Object? maxConcentrationScore = freezed,}) {
  return _then(_self.copyWith(
startDate: freezed == startDate ? _self.startDate : startDate // ignore: cast_nullable_to_non_nullable
as int?,endDate: freezed == endDate ? _self.endDate : endDate // ignore: cast_nullable_to_non_nullable
as int?,categoryIds: freezed == categoryIds ? _self.categoryIds : categoryIds // ignore: cast_nullable_to_non_nullable
as List<String>?,sessionType: freezed == sessionType ? _self.sessionType : sessionType // ignore: cast_nullable_to_non_nullable
as String?,minConcentrationScore: freezed == minConcentrationScore ? _self.minConcentrationScore : minConcentrationScore // ignore: cast_nullable_to_non_nullable
as int?,maxConcentrationScore: freezed == maxConcentrationScore ? _self.maxConcentrationScore : maxConcentrationScore // ignore: cast_nullable_to_non_nullable
as int?,
  ));
}

}


/// Adds pattern-matching-related methods to [GetSessionFiltersDto].
extension GetSessionFiltersDtoPatterns on GetSessionFiltersDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _GetSessionFiltersDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _GetSessionFiltersDto() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _GetSessionFiltersDto value)  $default,){
final _that = this;
switch (_that) {
case _GetSessionFiltersDto():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _GetSessionFiltersDto value)?  $default,){
final _that = this;
switch (_that) {
case _GetSessionFiltersDto() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( int? startDate,  int? endDate,  List<String>? categoryIds,  String? sessionType,  int? minConcentrationScore,  int? maxConcentrationScore)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _GetSessionFiltersDto() when $default != null:
return $default(_that.startDate,_that.endDate,_that.categoryIds,_that.sessionType,_that.minConcentrationScore,_that.maxConcentrationScore);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( int? startDate,  int? endDate,  List<String>? categoryIds,  String? sessionType,  int? minConcentrationScore,  int? maxConcentrationScore)  $default,) {final _that = this;
switch (_that) {
case _GetSessionFiltersDto():
return $default(_that.startDate,_that.endDate,_that.categoryIds,_that.sessionType,_that.minConcentrationScore,_that.maxConcentrationScore);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( int? startDate,  int? endDate,  List<String>? categoryIds,  String? sessionType,  int? minConcentrationScore,  int? maxConcentrationScore)?  $default,) {final _that = this;
switch (_that) {
case _GetSessionFiltersDto() when $default != null:
return $default(_that.startDate,_that.endDate,_that.categoryIds,_that.sessionType,_that.minConcentrationScore,_that.maxConcentrationScore);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _GetSessionFiltersDto extends GetSessionFiltersDto {
  const _GetSessionFiltersDto({this.startDate, this.endDate, final  List<String>? categoryIds, this.sessionType, this.minConcentrationScore, this.maxConcentrationScore}): _categoryIds = categoryIds,super._();
  factory _GetSessionFiltersDto.fromJson(Map<String, dynamic> json) => _$GetSessionFiltersDtoFromJson(json);

@override final  int? startDate;
@override final  int? endDate;
 final  List<String>? _categoryIds;
@override List<String>? get categoryIds {
  final value = _categoryIds;
  if (value == null) return null;
  if (_categoryIds is EqualUnmodifiableListView) return _categoryIds;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(value);
}

@override final  String? sessionType;
@override final  int? minConcentrationScore;
@override final  int? maxConcentrationScore;

/// Create a copy of GetSessionFiltersDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$GetSessionFiltersDtoCopyWith<_GetSessionFiltersDto> get copyWith => __$GetSessionFiltersDtoCopyWithImpl<_GetSessionFiltersDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$GetSessionFiltersDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _GetSessionFiltersDto&&(identical(other.startDate, startDate) || other.startDate == startDate)&&(identical(other.endDate, endDate) || other.endDate == endDate)&&const DeepCollectionEquality().equals(other._categoryIds, _categoryIds)&&(identical(other.sessionType, sessionType) || other.sessionType == sessionType)&&(identical(other.minConcentrationScore, minConcentrationScore) || other.minConcentrationScore == minConcentrationScore)&&(identical(other.maxConcentrationScore, maxConcentrationScore) || other.maxConcentrationScore == maxConcentrationScore));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,startDate,endDate,const DeepCollectionEquality().hash(_categoryIds),sessionType,minConcentrationScore,maxConcentrationScore);

@override
String toString() {
  return 'GetSessionFiltersDto(startDate: $startDate, endDate: $endDate, categoryIds: $categoryIds, sessionType: $sessionType, minConcentrationScore: $minConcentrationScore, maxConcentrationScore: $maxConcentrationScore)';
}


}

/// @nodoc
abstract mixin class _$GetSessionFiltersDtoCopyWith<$Res> implements $GetSessionFiltersDtoCopyWith<$Res> {
  factory _$GetSessionFiltersDtoCopyWith(_GetSessionFiltersDto value, $Res Function(_GetSessionFiltersDto) _then) = __$GetSessionFiltersDtoCopyWithImpl;
@override @useResult
$Res call({
 int? startDate, int? endDate, List<String>? categoryIds, String? sessionType, int? minConcentrationScore, int? maxConcentrationScore
});




}
/// @nodoc
class __$GetSessionFiltersDtoCopyWithImpl<$Res>
    implements _$GetSessionFiltersDtoCopyWith<$Res> {
  __$GetSessionFiltersDtoCopyWithImpl(this._self, this._then);

  final _GetSessionFiltersDto _self;
  final $Res Function(_GetSessionFiltersDto) _then;

/// Create a copy of GetSessionFiltersDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? startDate = freezed,Object? endDate = freezed,Object? categoryIds = freezed,Object? sessionType = freezed,Object? minConcentrationScore = freezed,Object? maxConcentrationScore = freezed,}) {
  return _then(_GetSessionFiltersDto(
startDate: freezed == startDate ? _self.startDate : startDate // ignore: cast_nullable_to_non_nullable
as int?,endDate: freezed == endDate ? _self.endDate : endDate // ignore: cast_nullable_to_non_nullable
as int?,categoryIds: freezed == categoryIds ? _self._categoryIds : categoryIds // ignore: cast_nullable_to_non_nullable
as List<String>?,sessionType: freezed == sessionType ? _self.sessionType : sessionType // ignore: cast_nullable_to_non_nullable
as String?,minConcentrationScore: freezed == minConcentrationScore ? _self.minConcentrationScore : minConcentrationScore // ignore: cast_nullable_to_non_nullable
as int?,maxConcentrationScore: freezed == maxConcentrationScore ? _self.maxConcentrationScore : maxConcentrationScore // ignore: cast_nullable_to_non_nullable
as int?,
  ));
}


}


/// @nodoc
mixin _$UpdateFocusSessionDto {

 String? get categoryId; String? get taskId; String? get notes; int? get concentrationScore; int? get startedAt; int? get endedAt; int? get actualDuration; String? get sessionType;
/// Create a copy of UpdateFocusSessionDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$UpdateFocusSessionDtoCopyWith<UpdateFocusSessionDto> get copyWith => _$UpdateFocusSessionDtoCopyWithImpl<UpdateFocusSessionDto>(this as UpdateFocusSessionDto, _$identity);

  /// Serializes this UpdateFocusSessionDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is UpdateFocusSessionDto&&(identical(other.categoryId, categoryId) || other.categoryId == categoryId)&&(identical(other.taskId, taskId) || other.taskId == taskId)&&(identical(other.notes, notes) || other.notes == notes)&&(identical(other.concentrationScore, concentrationScore) || other.concentrationScore == concentrationScore)&&(identical(other.startedAt, startedAt) || other.startedAt == startedAt)&&(identical(other.endedAt, endedAt) || other.endedAt == endedAt)&&(identical(other.actualDuration, actualDuration) || other.actualDuration == actualDuration)&&(identical(other.sessionType, sessionType) || other.sessionType == sessionType));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,categoryId,taskId,notes,concentrationScore,startedAt,endedAt,actualDuration,sessionType);

@override
String toString() {
  return 'UpdateFocusSessionDto(categoryId: $categoryId, taskId: $taskId, notes: $notes, concentrationScore: $concentrationScore, startedAt: $startedAt, endedAt: $endedAt, actualDuration: $actualDuration, sessionType: $sessionType)';
}


}

/// @nodoc
abstract mixin class $UpdateFocusSessionDtoCopyWith<$Res>  {
  factory $UpdateFocusSessionDtoCopyWith(UpdateFocusSessionDto value, $Res Function(UpdateFocusSessionDto) _then) = _$UpdateFocusSessionDtoCopyWithImpl;
@useResult
$Res call({
 String? categoryId, String? taskId, String? notes, int? concentrationScore, int? startedAt, int? endedAt, int? actualDuration, String? sessionType
});




}
/// @nodoc
class _$UpdateFocusSessionDtoCopyWithImpl<$Res>
    implements $UpdateFocusSessionDtoCopyWith<$Res> {
  _$UpdateFocusSessionDtoCopyWithImpl(this._self, this._then);

  final UpdateFocusSessionDto _self;
  final $Res Function(UpdateFocusSessionDto) _then;

/// Create a copy of UpdateFocusSessionDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? categoryId = freezed,Object? taskId = freezed,Object? notes = freezed,Object? concentrationScore = freezed,Object? startedAt = freezed,Object? endedAt = freezed,Object? actualDuration = freezed,Object? sessionType = freezed,}) {
  return _then(_self.copyWith(
categoryId: freezed == categoryId ? _self.categoryId : categoryId // ignore: cast_nullable_to_non_nullable
as String?,taskId: freezed == taskId ? _self.taskId : taskId // ignore: cast_nullable_to_non_nullable
as String?,notes: freezed == notes ? _self.notes : notes // ignore: cast_nullable_to_non_nullable
as String?,concentrationScore: freezed == concentrationScore ? _self.concentrationScore : concentrationScore // ignore: cast_nullable_to_non_nullable
as int?,startedAt: freezed == startedAt ? _self.startedAt : startedAt // ignore: cast_nullable_to_non_nullable
as int?,endedAt: freezed == endedAt ? _self.endedAt : endedAt // ignore: cast_nullable_to_non_nullable
as int?,actualDuration: freezed == actualDuration ? _self.actualDuration : actualDuration // ignore: cast_nullable_to_non_nullable
as int?,sessionType: freezed == sessionType ? _self.sessionType : sessionType // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}

}


/// Adds pattern-matching-related methods to [UpdateFocusSessionDto].
extension UpdateFocusSessionDtoPatterns on UpdateFocusSessionDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _UpdateFocusSessionDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _UpdateFocusSessionDto() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _UpdateFocusSessionDto value)  $default,){
final _that = this;
switch (_that) {
case _UpdateFocusSessionDto():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _UpdateFocusSessionDto value)?  $default,){
final _that = this;
switch (_that) {
case _UpdateFocusSessionDto() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( String? categoryId,  String? taskId,  String? notes,  int? concentrationScore,  int? startedAt,  int? endedAt,  int? actualDuration,  String? sessionType)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _UpdateFocusSessionDto() when $default != null:
return $default(_that.categoryId,_that.taskId,_that.notes,_that.concentrationScore,_that.startedAt,_that.endedAt,_that.actualDuration,_that.sessionType);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( String? categoryId,  String? taskId,  String? notes,  int? concentrationScore,  int? startedAt,  int? endedAt,  int? actualDuration,  String? sessionType)  $default,) {final _that = this;
switch (_that) {
case _UpdateFocusSessionDto():
return $default(_that.categoryId,_that.taskId,_that.notes,_that.concentrationScore,_that.startedAt,_that.endedAt,_that.actualDuration,_that.sessionType);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( String? categoryId,  String? taskId,  String? notes,  int? concentrationScore,  int? startedAt,  int? endedAt,  int? actualDuration,  String? sessionType)?  $default,) {final _that = this;
switch (_that) {
case _UpdateFocusSessionDto() when $default != null:
return $default(_that.categoryId,_that.taskId,_that.notes,_that.concentrationScore,_that.startedAt,_that.endedAt,_that.actualDuration,_that.sessionType);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _UpdateFocusSessionDto implements UpdateFocusSessionDto {
  const _UpdateFocusSessionDto({this.categoryId, this.taskId, this.notes, this.concentrationScore, this.startedAt, this.endedAt, this.actualDuration, this.sessionType});
  factory _UpdateFocusSessionDto.fromJson(Map<String, dynamic> json) => _$UpdateFocusSessionDtoFromJson(json);

@override final  String? categoryId;
@override final  String? taskId;
@override final  String? notes;
@override final  int? concentrationScore;
@override final  int? startedAt;
@override final  int? endedAt;
@override final  int? actualDuration;
@override final  String? sessionType;

/// Create a copy of UpdateFocusSessionDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$UpdateFocusSessionDtoCopyWith<_UpdateFocusSessionDto> get copyWith => __$UpdateFocusSessionDtoCopyWithImpl<_UpdateFocusSessionDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$UpdateFocusSessionDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _UpdateFocusSessionDto&&(identical(other.categoryId, categoryId) || other.categoryId == categoryId)&&(identical(other.taskId, taskId) || other.taskId == taskId)&&(identical(other.notes, notes) || other.notes == notes)&&(identical(other.concentrationScore, concentrationScore) || other.concentrationScore == concentrationScore)&&(identical(other.startedAt, startedAt) || other.startedAt == startedAt)&&(identical(other.endedAt, endedAt) || other.endedAt == endedAt)&&(identical(other.actualDuration, actualDuration) || other.actualDuration == actualDuration)&&(identical(other.sessionType, sessionType) || other.sessionType == sessionType));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,categoryId,taskId,notes,concentrationScore,startedAt,endedAt,actualDuration,sessionType);

@override
String toString() {
  return 'UpdateFocusSessionDto(categoryId: $categoryId, taskId: $taskId, notes: $notes, concentrationScore: $concentrationScore, startedAt: $startedAt, endedAt: $endedAt, actualDuration: $actualDuration, sessionType: $sessionType)';
}


}

/// @nodoc
abstract mixin class _$UpdateFocusSessionDtoCopyWith<$Res> implements $UpdateFocusSessionDtoCopyWith<$Res> {
  factory _$UpdateFocusSessionDtoCopyWith(_UpdateFocusSessionDto value, $Res Function(_UpdateFocusSessionDto) _then) = __$UpdateFocusSessionDtoCopyWithImpl;
@override @useResult
$Res call({
 String? categoryId, String? taskId, String? notes, int? concentrationScore, int? startedAt, int? endedAt, int? actualDuration, String? sessionType
});




}
/// @nodoc
class __$UpdateFocusSessionDtoCopyWithImpl<$Res>
    implements _$UpdateFocusSessionDtoCopyWith<$Res> {
  __$UpdateFocusSessionDtoCopyWithImpl(this._self, this._then);

  final _UpdateFocusSessionDto _self;
  final $Res Function(_UpdateFocusSessionDto) _then;

/// Create a copy of UpdateFocusSessionDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? categoryId = freezed,Object? taskId = freezed,Object? notes = freezed,Object? concentrationScore = freezed,Object? startedAt = freezed,Object? endedAt = freezed,Object? actualDuration = freezed,Object? sessionType = freezed,}) {
  return _then(_UpdateFocusSessionDto(
categoryId: freezed == categoryId ? _self.categoryId : categoryId // ignore: cast_nullable_to_non_nullable
as String?,taskId: freezed == taskId ? _self.taskId : taskId // ignore: cast_nullable_to_non_nullable
as String?,notes: freezed == notes ? _self.notes : notes // ignore: cast_nullable_to_non_nullable
as String?,concentrationScore: freezed == concentrationScore ? _self.concentrationScore : concentrationScore // ignore: cast_nullable_to_non_nullable
as int?,startedAt: freezed == startedAt ? _self.startedAt : startedAt // ignore: cast_nullable_to_non_nullable
as int?,endedAt: freezed == endedAt ? _self.endedAt : endedAt // ignore: cast_nullable_to_non_nullable
as int?,actualDuration: freezed == actualDuration ? _self.actualDuration : actualDuration // ignore: cast_nullable_to_non_nullable
as int?,sessionType: freezed == sessionType ? _self.sessionType : sessionType // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}


}


/// @nodoc
mixin _$FocusSessionDto {

 String get id; String get sessionType; int get startedAt; int? get endedAt; int? get actualDuration; String? get taskId; String? get categoryId; int? get concentrationScore; String? get notes; int get createdAt;
/// Create a copy of FocusSessionDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$FocusSessionDtoCopyWith<FocusSessionDto> get copyWith => _$FocusSessionDtoCopyWithImpl<FocusSessionDto>(this as FocusSessionDto, _$identity);

  /// Serializes this FocusSessionDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is FocusSessionDto&&(identical(other.id, id) || other.id == id)&&(identical(other.sessionType, sessionType) || other.sessionType == sessionType)&&(identical(other.startedAt, startedAt) || other.startedAt == startedAt)&&(identical(other.endedAt, endedAt) || other.endedAt == endedAt)&&(identical(other.actualDuration, actualDuration) || other.actualDuration == actualDuration)&&(identical(other.taskId, taskId) || other.taskId == taskId)&&(identical(other.categoryId, categoryId) || other.categoryId == categoryId)&&(identical(other.concentrationScore, concentrationScore) || other.concentrationScore == concentrationScore)&&(identical(other.notes, notes) || other.notes == notes)&&(identical(other.createdAt, createdAt) || other.createdAt == createdAt));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,id,sessionType,startedAt,endedAt,actualDuration,taskId,categoryId,concentrationScore,notes,createdAt);

@override
String toString() {
  return 'FocusSessionDto(id: $id, sessionType: $sessionType, startedAt: $startedAt, endedAt: $endedAt, actualDuration: $actualDuration, taskId: $taskId, categoryId: $categoryId, concentrationScore: $concentrationScore, notes: $notes, createdAt: $createdAt)';
}


}

/// @nodoc
abstract mixin class $FocusSessionDtoCopyWith<$Res>  {
  factory $FocusSessionDtoCopyWith(FocusSessionDto value, $Res Function(FocusSessionDto) _then) = _$FocusSessionDtoCopyWithImpl;
@useResult
$Res call({
 String id, String sessionType, int startedAt, int? endedAt, int? actualDuration, String? taskId, String? categoryId, int? concentrationScore, String? notes, int createdAt
});




}
/// @nodoc
class _$FocusSessionDtoCopyWithImpl<$Res>
    implements $FocusSessionDtoCopyWith<$Res> {
  _$FocusSessionDtoCopyWithImpl(this._self, this._then);

  final FocusSessionDto _self;
  final $Res Function(FocusSessionDto) _then;

/// Create a copy of FocusSessionDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? id = null,Object? sessionType = null,Object? startedAt = null,Object? endedAt = freezed,Object? actualDuration = freezed,Object? taskId = freezed,Object? categoryId = freezed,Object? concentrationScore = freezed,Object? notes = freezed,Object? createdAt = null,}) {
  return _then(_self.copyWith(
id: null == id ? _self.id : id // ignore: cast_nullable_to_non_nullable
as String,sessionType: null == sessionType ? _self.sessionType : sessionType // ignore: cast_nullable_to_non_nullable
as String,startedAt: null == startedAt ? _self.startedAt : startedAt // ignore: cast_nullable_to_non_nullable
as int,endedAt: freezed == endedAt ? _self.endedAt : endedAt // ignore: cast_nullable_to_non_nullable
as int?,actualDuration: freezed == actualDuration ? _self.actualDuration : actualDuration // ignore: cast_nullable_to_non_nullable
as int?,taskId: freezed == taskId ? _self.taskId : taskId // ignore: cast_nullable_to_non_nullable
as String?,categoryId: freezed == categoryId ? _self.categoryId : categoryId // ignore: cast_nullable_to_non_nullable
as String?,concentrationScore: freezed == concentrationScore ? _self.concentrationScore : concentrationScore // ignore: cast_nullable_to_non_nullable
as int?,notes: freezed == notes ? _self.notes : notes // ignore: cast_nullable_to_non_nullable
as String?,createdAt: null == createdAt ? _self.createdAt : createdAt // ignore: cast_nullable_to_non_nullable
as int,
  ));
}

}


/// Adds pattern-matching-related methods to [FocusSessionDto].
extension FocusSessionDtoPatterns on FocusSessionDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _FocusSessionDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _FocusSessionDto() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _FocusSessionDto value)  $default,){
final _that = this;
switch (_that) {
case _FocusSessionDto():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _FocusSessionDto value)?  $default,){
final _that = this;
switch (_that) {
case _FocusSessionDto() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( String id,  String sessionType,  int startedAt,  int? endedAt,  int? actualDuration,  String? taskId,  String? categoryId,  int? concentrationScore,  String? notes,  int createdAt)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _FocusSessionDto() when $default != null:
return $default(_that.id,_that.sessionType,_that.startedAt,_that.endedAt,_that.actualDuration,_that.taskId,_that.categoryId,_that.concentrationScore,_that.notes,_that.createdAt);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( String id,  String sessionType,  int startedAt,  int? endedAt,  int? actualDuration,  String? taskId,  String? categoryId,  int? concentrationScore,  String? notes,  int createdAt)  $default,) {final _that = this;
switch (_that) {
case _FocusSessionDto():
return $default(_that.id,_that.sessionType,_that.startedAt,_that.endedAt,_that.actualDuration,_that.taskId,_that.categoryId,_that.concentrationScore,_that.notes,_that.createdAt);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( String id,  String sessionType,  int startedAt,  int? endedAt,  int? actualDuration,  String? taskId,  String? categoryId,  int? concentrationScore,  String? notes,  int createdAt)?  $default,) {final _that = this;
switch (_that) {
case _FocusSessionDto() when $default != null:
return $default(_that.id,_that.sessionType,_that.startedAt,_that.endedAt,_that.actualDuration,_that.taskId,_that.categoryId,_that.concentrationScore,_that.notes,_that.createdAt);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _FocusSessionDto implements FocusSessionDto {
  const _FocusSessionDto({required this.id, required this.sessionType, required this.startedAt, this.endedAt, this.actualDuration, this.taskId, this.categoryId, this.concentrationScore, this.notes, required this.createdAt});
  factory _FocusSessionDto.fromJson(Map<String, dynamic> json) => _$FocusSessionDtoFromJson(json);

@override final  String id;
@override final  String sessionType;
@override final  int startedAt;
@override final  int? endedAt;
@override final  int? actualDuration;
@override final  String? taskId;
@override final  String? categoryId;
@override final  int? concentrationScore;
@override final  String? notes;
@override final  int createdAt;

/// Create a copy of FocusSessionDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$FocusSessionDtoCopyWith<_FocusSessionDto> get copyWith => __$FocusSessionDtoCopyWithImpl<_FocusSessionDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$FocusSessionDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _FocusSessionDto&&(identical(other.id, id) || other.id == id)&&(identical(other.sessionType, sessionType) || other.sessionType == sessionType)&&(identical(other.startedAt, startedAt) || other.startedAt == startedAt)&&(identical(other.endedAt, endedAt) || other.endedAt == endedAt)&&(identical(other.actualDuration, actualDuration) || other.actualDuration == actualDuration)&&(identical(other.taskId, taskId) || other.taskId == taskId)&&(identical(other.categoryId, categoryId) || other.categoryId == categoryId)&&(identical(other.concentrationScore, concentrationScore) || other.concentrationScore == concentrationScore)&&(identical(other.notes, notes) || other.notes == notes)&&(identical(other.createdAt, createdAt) || other.createdAt == createdAt));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,id,sessionType,startedAt,endedAt,actualDuration,taskId,categoryId,concentrationScore,notes,createdAt);

@override
String toString() {
  return 'FocusSessionDto(id: $id, sessionType: $sessionType, startedAt: $startedAt, endedAt: $endedAt, actualDuration: $actualDuration, taskId: $taskId, categoryId: $categoryId, concentrationScore: $concentrationScore, notes: $notes, createdAt: $createdAt)';
}


}

/// @nodoc
abstract mixin class _$FocusSessionDtoCopyWith<$Res> implements $FocusSessionDtoCopyWith<$Res> {
  factory _$FocusSessionDtoCopyWith(_FocusSessionDto value, $Res Function(_FocusSessionDto) _then) = __$FocusSessionDtoCopyWithImpl;
@override @useResult
$Res call({
 String id, String sessionType, int startedAt, int? endedAt, int? actualDuration, String? taskId, String? categoryId, int? concentrationScore, String? notes, int createdAt
});




}
/// @nodoc
class __$FocusSessionDtoCopyWithImpl<$Res>
    implements _$FocusSessionDtoCopyWith<$Res> {
  __$FocusSessionDtoCopyWithImpl(this._self, this._then);

  final _FocusSessionDto _self;
  final $Res Function(_FocusSessionDto) _then;

/// Create a copy of FocusSessionDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? id = null,Object? sessionType = null,Object? startedAt = null,Object? endedAt = freezed,Object? actualDuration = freezed,Object? taskId = freezed,Object? categoryId = freezed,Object? concentrationScore = freezed,Object? notes = freezed,Object? createdAt = null,}) {
  return _then(_FocusSessionDto(
id: null == id ? _self.id : id // ignore: cast_nullable_to_non_nullable
as String,sessionType: null == sessionType ? _self.sessionType : sessionType // ignore: cast_nullable_to_non_nullable
as String,startedAt: null == startedAt ? _self.startedAt : startedAt // ignore: cast_nullable_to_non_nullable
as int,endedAt: freezed == endedAt ? _self.endedAt : endedAt // ignore: cast_nullable_to_non_nullable
as int?,actualDuration: freezed == actualDuration ? _self.actualDuration : actualDuration // ignore: cast_nullable_to_non_nullable
as int?,taskId: freezed == taskId ? _self.taskId : taskId // ignore: cast_nullable_to_non_nullable
as String?,categoryId: freezed == categoryId ? _self.categoryId : categoryId // ignore: cast_nullable_to_non_nullable
as String?,concentrationScore: freezed == concentrationScore ? _self.concentrationScore : concentrationScore // ignore: cast_nullable_to_non_nullable
as int?,notes: freezed == notes ? _self.notes : notes // ignore: cast_nullable_to_non_nullable
as String?,createdAt: null == createdAt ? _self.createdAt : createdAt // ignore: cast_nullable_to_non_nullable
as int,
  ));
}


}


/// @nodoc
mixin _$CreateManualSessionResponseDto {

 String get id;
/// Create a copy of CreateManualSessionResponseDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CreateManualSessionResponseDtoCopyWith<CreateManualSessionResponseDto> get copyWith => _$CreateManualSessionResponseDtoCopyWithImpl<CreateManualSessionResponseDto>(this as CreateManualSessionResponseDto, _$identity);

  /// Serializes this CreateManualSessionResponseDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CreateManualSessionResponseDto&&(identical(other.id, id) || other.id == id));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,id);

@override
String toString() {
  return 'CreateManualSessionResponseDto(id: $id)';
}


}

/// @nodoc
abstract mixin class $CreateManualSessionResponseDtoCopyWith<$Res>  {
  factory $CreateManualSessionResponseDtoCopyWith(CreateManualSessionResponseDto value, $Res Function(CreateManualSessionResponseDto) _then) = _$CreateManualSessionResponseDtoCopyWithImpl;
@useResult
$Res call({
 String id
});




}
/// @nodoc
class _$CreateManualSessionResponseDtoCopyWithImpl<$Res>
    implements $CreateManualSessionResponseDtoCopyWith<$Res> {
  _$CreateManualSessionResponseDtoCopyWithImpl(this._self, this._then);

  final CreateManualSessionResponseDto _self;
  final $Res Function(CreateManualSessionResponseDto) _then;

/// Create a copy of CreateManualSessionResponseDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? id = null,}) {
  return _then(_self.copyWith(
id: null == id ? _self.id : id // ignore: cast_nullable_to_non_nullable
as String,
  ));
}

}


/// Adds pattern-matching-related methods to [CreateManualSessionResponseDto].
extension CreateManualSessionResponseDtoPatterns on CreateManualSessionResponseDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _CreateManualSessionResponseDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _CreateManualSessionResponseDto() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _CreateManualSessionResponseDto value)  $default,){
final _that = this;
switch (_that) {
case _CreateManualSessionResponseDto():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _CreateManualSessionResponseDto value)?  $default,){
final _that = this;
switch (_that) {
case _CreateManualSessionResponseDto() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( String id)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _CreateManualSessionResponseDto() when $default != null:
return $default(_that.id);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( String id)  $default,) {final _that = this;
switch (_that) {
case _CreateManualSessionResponseDto():
return $default(_that.id);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( String id)?  $default,) {final _that = this;
switch (_that) {
case _CreateManualSessionResponseDto() when $default != null:
return $default(_that.id);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _CreateManualSessionResponseDto implements CreateManualSessionResponseDto {
  const _CreateManualSessionResponseDto({required this.id});
  factory _CreateManualSessionResponseDto.fromJson(Map<String, dynamic> json) => _$CreateManualSessionResponseDtoFromJson(json);

@override final  String id;

/// Create a copy of CreateManualSessionResponseDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$CreateManualSessionResponseDtoCopyWith<_CreateManualSessionResponseDto> get copyWith => __$CreateManualSessionResponseDtoCopyWithImpl<_CreateManualSessionResponseDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$CreateManualSessionResponseDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _CreateManualSessionResponseDto&&(identical(other.id, id) || other.id == id));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,id);

@override
String toString() {
  return 'CreateManualSessionResponseDto(id: $id)';
}


}

/// @nodoc
abstract mixin class _$CreateManualSessionResponseDtoCopyWith<$Res> implements $CreateManualSessionResponseDtoCopyWith<$Res> {
  factory _$CreateManualSessionResponseDtoCopyWith(_CreateManualSessionResponseDto value, $Res Function(_CreateManualSessionResponseDto) _then) = __$CreateManualSessionResponseDtoCopyWithImpl;
@override @useResult
$Res call({
 String id
});




}
/// @nodoc
class __$CreateManualSessionResponseDtoCopyWithImpl<$Res>
    implements _$CreateManualSessionResponseDtoCopyWith<$Res> {
  __$CreateManualSessionResponseDtoCopyWithImpl(this._self, this._then);

  final _CreateManualSessionResponseDto _self;
  final $Res Function(_CreateManualSessionResponseDto) _then;

/// Create a copy of CreateManualSessionResponseDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? id = null,}) {
  return _then(_CreateManualSessionResponseDto(
id: null == id ? _self.id : id // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}


/// @nodoc
mixin _$GetSessionFiltersResponseDto {

 List<FocusSessionDto> get focusSessions;
/// Create a copy of GetSessionFiltersResponseDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GetSessionFiltersResponseDtoCopyWith<GetSessionFiltersResponseDto> get copyWith => _$GetSessionFiltersResponseDtoCopyWithImpl<GetSessionFiltersResponseDto>(this as GetSessionFiltersResponseDto, _$identity);

  /// Serializes this GetSessionFiltersResponseDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GetSessionFiltersResponseDto&&const DeepCollectionEquality().equals(other.focusSessions, focusSessions));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(focusSessions));

@override
String toString() {
  return 'GetSessionFiltersResponseDto(focusSessions: $focusSessions)';
}


}

/// @nodoc
abstract mixin class $GetSessionFiltersResponseDtoCopyWith<$Res>  {
  factory $GetSessionFiltersResponseDtoCopyWith(GetSessionFiltersResponseDto value, $Res Function(GetSessionFiltersResponseDto) _then) = _$GetSessionFiltersResponseDtoCopyWithImpl;
@useResult
$Res call({
 List<FocusSessionDto> focusSessions
});




}
/// @nodoc
class _$GetSessionFiltersResponseDtoCopyWithImpl<$Res>
    implements $GetSessionFiltersResponseDtoCopyWith<$Res> {
  _$GetSessionFiltersResponseDtoCopyWithImpl(this._self, this._then);

  final GetSessionFiltersResponseDto _self;
  final $Res Function(GetSessionFiltersResponseDto) _then;

/// Create a copy of GetSessionFiltersResponseDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? focusSessions = null,}) {
  return _then(_self.copyWith(
focusSessions: null == focusSessions ? _self.focusSessions : focusSessions // ignore: cast_nullable_to_non_nullable
as List<FocusSessionDto>,
  ));
}

}


/// Adds pattern-matching-related methods to [GetSessionFiltersResponseDto].
extension GetSessionFiltersResponseDtoPatterns on GetSessionFiltersResponseDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _GetSessionFiltersResponseDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _GetSessionFiltersResponseDto() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _GetSessionFiltersResponseDto value)  $default,){
final _that = this;
switch (_that) {
case _GetSessionFiltersResponseDto():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _GetSessionFiltersResponseDto value)?  $default,){
final _that = this;
switch (_that) {
case _GetSessionFiltersResponseDto() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( List<FocusSessionDto> focusSessions)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _GetSessionFiltersResponseDto() when $default != null:
return $default(_that.focusSessions);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( List<FocusSessionDto> focusSessions)  $default,) {final _that = this;
switch (_that) {
case _GetSessionFiltersResponseDto():
return $default(_that.focusSessions);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( List<FocusSessionDto> focusSessions)?  $default,) {final _that = this;
switch (_that) {
case _GetSessionFiltersResponseDto() when $default != null:
return $default(_that.focusSessions);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _GetSessionFiltersResponseDto implements GetSessionFiltersResponseDto {
  const _GetSessionFiltersResponseDto({required final  List<FocusSessionDto> focusSessions}): _focusSessions = focusSessions;
  factory _GetSessionFiltersResponseDto.fromJson(Map<String, dynamic> json) => _$GetSessionFiltersResponseDtoFromJson(json);

 final  List<FocusSessionDto> _focusSessions;
@override List<FocusSessionDto> get focusSessions {
  if (_focusSessions is EqualUnmodifiableListView) return _focusSessions;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_focusSessions);
}


/// Create a copy of GetSessionFiltersResponseDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$GetSessionFiltersResponseDtoCopyWith<_GetSessionFiltersResponseDto> get copyWith => __$GetSessionFiltersResponseDtoCopyWithImpl<_GetSessionFiltersResponseDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$GetSessionFiltersResponseDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _GetSessionFiltersResponseDto&&const DeepCollectionEquality().equals(other._focusSessions, _focusSessions));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(_focusSessions));

@override
String toString() {
  return 'GetSessionFiltersResponseDto(focusSessions: $focusSessions)';
}


}

/// @nodoc
abstract mixin class _$GetSessionFiltersResponseDtoCopyWith<$Res> implements $GetSessionFiltersResponseDtoCopyWith<$Res> {
  factory _$GetSessionFiltersResponseDtoCopyWith(_GetSessionFiltersResponseDto value, $Res Function(_GetSessionFiltersResponseDto) _then) = __$GetSessionFiltersResponseDtoCopyWithImpl;
@override @useResult
$Res call({
 List<FocusSessionDto> focusSessions
});




}
/// @nodoc
class __$GetSessionFiltersResponseDtoCopyWithImpl<$Res>
    implements _$GetSessionFiltersResponseDtoCopyWith<$Res> {
  __$GetSessionFiltersResponseDtoCopyWithImpl(this._self, this._then);

  final _GetSessionFiltersResponseDto _self;
  final $Res Function(_GetSessionFiltersResponseDto) _then;

/// Create a copy of GetSessionFiltersResponseDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? focusSessions = null,}) {
  return _then(_GetSessionFiltersResponseDto(
focusSessions: null == focusSessions ? _self._focusSessions : focusSessions // ignore: cast_nullable_to_non_nullable
as List<FocusSessionDto>,
  ));
}


}

// dart format on
