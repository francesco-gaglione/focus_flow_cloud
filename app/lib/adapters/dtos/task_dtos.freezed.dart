// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'task_dtos.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$CreateTaskDto {

 String get name; String? get description; String? get categoryId; int? get scheduledDate;
/// Create a copy of CreateTaskDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CreateTaskDtoCopyWith<CreateTaskDto> get copyWith => _$CreateTaskDtoCopyWithImpl<CreateTaskDto>(this as CreateTaskDto, _$identity);

  /// Serializes this CreateTaskDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CreateTaskDto&&(identical(other.name, name) || other.name == name)&&(identical(other.description, description) || other.description == description)&&(identical(other.categoryId, categoryId) || other.categoryId == categoryId)&&(identical(other.scheduledDate, scheduledDate) || other.scheduledDate == scheduledDate));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,name,description,categoryId,scheduledDate);

@override
String toString() {
  return 'CreateTaskDto(name: $name, description: $description, categoryId: $categoryId, scheduledDate: $scheduledDate)';
}


}

/// @nodoc
abstract mixin class $CreateTaskDtoCopyWith<$Res>  {
  factory $CreateTaskDtoCopyWith(CreateTaskDto value, $Res Function(CreateTaskDto) _then) = _$CreateTaskDtoCopyWithImpl;
@useResult
$Res call({
 String name, String? description, String? categoryId, int? scheduledDate
});




}
/// @nodoc
class _$CreateTaskDtoCopyWithImpl<$Res>
    implements $CreateTaskDtoCopyWith<$Res> {
  _$CreateTaskDtoCopyWithImpl(this._self, this._then);

  final CreateTaskDto _self;
  final $Res Function(CreateTaskDto) _then;

/// Create a copy of CreateTaskDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? name = null,Object? description = freezed,Object? categoryId = freezed,Object? scheduledDate = freezed,}) {
  return _then(_self.copyWith(
name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,description: freezed == description ? _self.description : description // ignore: cast_nullable_to_non_nullable
as String?,categoryId: freezed == categoryId ? _self.categoryId : categoryId // ignore: cast_nullable_to_non_nullable
as String?,scheduledDate: freezed == scheduledDate ? _self.scheduledDate : scheduledDate // ignore: cast_nullable_to_non_nullable
as int?,
  ));
}

}


/// Adds pattern-matching-related methods to [CreateTaskDto].
extension CreateTaskDtoPatterns on CreateTaskDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _CreateTaskDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _CreateTaskDto() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _CreateTaskDto value)  $default,){
final _that = this;
switch (_that) {
case _CreateTaskDto():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _CreateTaskDto value)?  $default,){
final _that = this;
switch (_that) {
case _CreateTaskDto() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( String name,  String? description,  String? categoryId,  int? scheduledDate)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _CreateTaskDto() when $default != null:
return $default(_that.name,_that.description,_that.categoryId,_that.scheduledDate);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( String name,  String? description,  String? categoryId,  int? scheduledDate)  $default,) {final _that = this;
switch (_that) {
case _CreateTaskDto():
return $default(_that.name,_that.description,_that.categoryId,_that.scheduledDate);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( String name,  String? description,  String? categoryId,  int? scheduledDate)?  $default,) {final _that = this;
switch (_that) {
case _CreateTaskDto() when $default != null:
return $default(_that.name,_that.description,_that.categoryId,_that.scheduledDate);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _CreateTaskDto implements CreateTaskDto {
  const _CreateTaskDto({required this.name, this.description, this.categoryId, this.scheduledDate});
  factory _CreateTaskDto.fromJson(Map<String, dynamic> json) => _$CreateTaskDtoFromJson(json);

@override final  String name;
@override final  String? description;
@override final  String? categoryId;
@override final  int? scheduledDate;

/// Create a copy of CreateTaskDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$CreateTaskDtoCopyWith<_CreateTaskDto> get copyWith => __$CreateTaskDtoCopyWithImpl<_CreateTaskDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$CreateTaskDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _CreateTaskDto&&(identical(other.name, name) || other.name == name)&&(identical(other.description, description) || other.description == description)&&(identical(other.categoryId, categoryId) || other.categoryId == categoryId)&&(identical(other.scheduledDate, scheduledDate) || other.scheduledDate == scheduledDate));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,name,description,categoryId,scheduledDate);

@override
String toString() {
  return 'CreateTaskDto(name: $name, description: $description, categoryId: $categoryId, scheduledDate: $scheduledDate)';
}


}

/// @nodoc
abstract mixin class _$CreateTaskDtoCopyWith<$Res> implements $CreateTaskDtoCopyWith<$Res> {
  factory _$CreateTaskDtoCopyWith(_CreateTaskDto value, $Res Function(_CreateTaskDto) _then) = __$CreateTaskDtoCopyWithImpl;
@override @useResult
$Res call({
 String name, String? description, String? categoryId, int? scheduledDate
});




}
/// @nodoc
class __$CreateTaskDtoCopyWithImpl<$Res>
    implements _$CreateTaskDtoCopyWith<$Res> {
  __$CreateTaskDtoCopyWithImpl(this._self, this._then);

  final _CreateTaskDto _self;
  final $Res Function(_CreateTaskDto) _then;

/// Create a copy of CreateTaskDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? name = null,Object? description = freezed,Object? categoryId = freezed,Object? scheduledDate = freezed,}) {
  return _then(_CreateTaskDto(
name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,description: freezed == description ? _self.description : description // ignore: cast_nullable_to_non_nullable
as String?,categoryId: freezed == categoryId ? _self.categoryId : categoryId // ignore: cast_nullable_to_non_nullable
as String?,scheduledDate: freezed == scheduledDate ? _self.scheduledDate : scheduledDate // ignore: cast_nullable_to_non_nullable
as int?,
  ));
}


}


/// @nodoc
mixin _$UpdateTaskDto {

 String? get name; String? get description; String? get categoryId; int? get scheduledDate; int? get completedAt;
/// Create a copy of UpdateTaskDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$UpdateTaskDtoCopyWith<UpdateTaskDto> get copyWith => _$UpdateTaskDtoCopyWithImpl<UpdateTaskDto>(this as UpdateTaskDto, _$identity);

  /// Serializes this UpdateTaskDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is UpdateTaskDto&&(identical(other.name, name) || other.name == name)&&(identical(other.description, description) || other.description == description)&&(identical(other.categoryId, categoryId) || other.categoryId == categoryId)&&(identical(other.scheduledDate, scheduledDate) || other.scheduledDate == scheduledDate)&&(identical(other.completedAt, completedAt) || other.completedAt == completedAt));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,name,description,categoryId,scheduledDate,completedAt);

@override
String toString() {
  return 'UpdateTaskDto(name: $name, description: $description, categoryId: $categoryId, scheduledDate: $scheduledDate, completedAt: $completedAt)';
}


}

/// @nodoc
abstract mixin class $UpdateTaskDtoCopyWith<$Res>  {
  factory $UpdateTaskDtoCopyWith(UpdateTaskDto value, $Res Function(UpdateTaskDto) _then) = _$UpdateTaskDtoCopyWithImpl;
@useResult
$Res call({
 String? name, String? description, String? categoryId, int? scheduledDate, int? completedAt
});




}
/// @nodoc
class _$UpdateTaskDtoCopyWithImpl<$Res>
    implements $UpdateTaskDtoCopyWith<$Res> {
  _$UpdateTaskDtoCopyWithImpl(this._self, this._then);

  final UpdateTaskDto _self;
  final $Res Function(UpdateTaskDto) _then;

/// Create a copy of UpdateTaskDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? name = freezed,Object? description = freezed,Object? categoryId = freezed,Object? scheduledDate = freezed,Object? completedAt = freezed,}) {
  return _then(_self.copyWith(
name: freezed == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String?,description: freezed == description ? _self.description : description // ignore: cast_nullable_to_non_nullable
as String?,categoryId: freezed == categoryId ? _self.categoryId : categoryId // ignore: cast_nullable_to_non_nullable
as String?,scheduledDate: freezed == scheduledDate ? _self.scheduledDate : scheduledDate // ignore: cast_nullable_to_non_nullable
as int?,completedAt: freezed == completedAt ? _self.completedAt : completedAt // ignore: cast_nullable_to_non_nullable
as int?,
  ));
}

}


/// Adds pattern-matching-related methods to [UpdateTaskDto].
extension UpdateTaskDtoPatterns on UpdateTaskDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _UpdateTaskDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _UpdateTaskDto() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _UpdateTaskDto value)  $default,){
final _that = this;
switch (_that) {
case _UpdateTaskDto():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _UpdateTaskDto value)?  $default,){
final _that = this;
switch (_that) {
case _UpdateTaskDto() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( String? name,  String? description,  String? categoryId,  int? scheduledDate,  int? completedAt)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _UpdateTaskDto() when $default != null:
return $default(_that.name,_that.description,_that.categoryId,_that.scheduledDate,_that.completedAt);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( String? name,  String? description,  String? categoryId,  int? scheduledDate,  int? completedAt)  $default,) {final _that = this;
switch (_that) {
case _UpdateTaskDto():
return $default(_that.name,_that.description,_that.categoryId,_that.scheduledDate,_that.completedAt);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( String? name,  String? description,  String? categoryId,  int? scheduledDate,  int? completedAt)?  $default,) {final _that = this;
switch (_that) {
case _UpdateTaskDto() when $default != null:
return $default(_that.name,_that.description,_that.categoryId,_that.scheduledDate,_that.completedAt);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _UpdateTaskDto implements UpdateTaskDto {
  const _UpdateTaskDto({this.name, this.description, this.categoryId, this.scheduledDate, this.completedAt});
  factory _UpdateTaskDto.fromJson(Map<String, dynamic> json) => _$UpdateTaskDtoFromJson(json);

@override final  String? name;
@override final  String? description;
@override final  String? categoryId;
@override final  int? scheduledDate;
@override final  int? completedAt;

/// Create a copy of UpdateTaskDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$UpdateTaskDtoCopyWith<_UpdateTaskDto> get copyWith => __$UpdateTaskDtoCopyWithImpl<_UpdateTaskDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$UpdateTaskDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _UpdateTaskDto&&(identical(other.name, name) || other.name == name)&&(identical(other.description, description) || other.description == description)&&(identical(other.categoryId, categoryId) || other.categoryId == categoryId)&&(identical(other.scheduledDate, scheduledDate) || other.scheduledDate == scheduledDate)&&(identical(other.completedAt, completedAt) || other.completedAt == completedAt));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,name,description,categoryId,scheduledDate,completedAt);

@override
String toString() {
  return 'UpdateTaskDto(name: $name, description: $description, categoryId: $categoryId, scheduledDate: $scheduledDate, completedAt: $completedAt)';
}


}

/// @nodoc
abstract mixin class _$UpdateTaskDtoCopyWith<$Res> implements $UpdateTaskDtoCopyWith<$Res> {
  factory _$UpdateTaskDtoCopyWith(_UpdateTaskDto value, $Res Function(_UpdateTaskDto) _then) = __$UpdateTaskDtoCopyWithImpl;
@override @useResult
$Res call({
 String? name, String? description, String? categoryId, int? scheduledDate, int? completedAt
});




}
/// @nodoc
class __$UpdateTaskDtoCopyWithImpl<$Res>
    implements _$UpdateTaskDtoCopyWith<$Res> {
  __$UpdateTaskDtoCopyWithImpl(this._self, this._then);

  final _UpdateTaskDto _self;
  final $Res Function(_UpdateTaskDto) _then;

/// Create a copy of UpdateTaskDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? name = freezed,Object? description = freezed,Object? categoryId = freezed,Object? scheduledDate = freezed,Object? completedAt = freezed,}) {
  return _then(_UpdateTaskDto(
name: freezed == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String?,description: freezed == description ? _self.description : description // ignore: cast_nullable_to_non_nullable
as String?,categoryId: freezed == categoryId ? _self.categoryId : categoryId // ignore: cast_nullable_to_non_nullable
as String?,scheduledDate: freezed == scheduledDate ? _self.scheduledDate : scheduledDate // ignore: cast_nullable_to_non_nullable
as int?,completedAt: freezed == completedAt ? _self.completedAt : completedAt // ignore: cast_nullable_to_non_nullable
as int?,
  ));
}


}


/// @nodoc
mixin _$DeleteTasksDto {

 List<String> get taskIds;
/// Create a copy of DeleteTasksDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$DeleteTasksDtoCopyWith<DeleteTasksDto> get copyWith => _$DeleteTasksDtoCopyWithImpl<DeleteTasksDto>(this as DeleteTasksDto, _$identity);

  /// Serializes this DeleteTasksDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is DeleteTasksDto&&const DeepCollectionEquality().equals(other.taskIds, taskIds));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(taskIds));

@override
String toString() {
  return 'DeleteTasksDto(taskIds: $taskIds)';
}


}

/// @nodoc
abstract mixin class $DeleteTasksDtoCopyWith<$Res>  {
  factory $DeleteTasksDtoCopyWith(DeleteTasksDto value, $Res Function(DeleteTasksDto) _then) = _$DeleteTasksDtoCopyWithImpl;
@useResult
$Res call({
 List<String> taskIds
});




}
/// @nodoc
class _$DeleteTasksDtoCopyWithImpl<$Res>
    implements $DeleteTasksDtoCopyWith<$Res> {
  _$DeleteTasksDtoCopyWithImpl(this._self, this._then);

  final DeleteTasksDto _self;
  final $Res Function(DeleteTasksDto) _then;

/// Create a copy of DeleteTasksDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? taskIds = null,}) {
  return _then(_self.copyWith(
taskIds: null == taskIds ? _self.taskIds : taskIds // ignore: cast_nullable_to_non_nullable
as List<String>,
  ));
}

}


/// Adds pattern-matching-related methods to [DeleteTasksDto].
extension DeleteTasksDtoPatterns on DeleteTasksDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _DeleteTasksDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _DeleteTasksDto() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _DeleteTasksDto value)  $default,){
final _that = this;
switch (_that) {
case _DeleteTasksDto():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _DeleteTasksDto value)?  $default,){
final _that = this;
switch (_that) {
case _DeleteTasksDto() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( List<String> taskIds)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _DeleteTasksDto() when $default != null:
return $default(_that.taskIds);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( List<String> taskIds)  $default,) {final _that = this;
switch (_that) {
case _DeleteTasksDto():
return $default(_that.taskIds);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( List<String> taskIds)?  $default,) {final _that = this;
switch (_that) {
case _DeleteTasksDto() when $default != null:
return $default(_that.taskIds);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _DeleteTasksDto implements DeleteTasksDto {
  const _DeleteTasksDto({required final  List<String> taskIds}): _taskIds = taskIds;
  factory _DeleteTasksDto.fromJson(Map<String, dynamic> json) => _$DeleteTasksDtoFromJson(json);

 final  List<String> _taskIds;
@override List<String> get taskIds {
  if (_taskIds is EqualUnmodifiableListView) return _taskIds;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_taskIds);
}


/// Create a copy of DeleteTasksDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$DeleteTasksDtoCopyWith<_DeleteTasksDto> get copyWith => __$DeleteTasksDtoCopyWithImpl<_DeleteTasksDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$DeleteTasksDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _DeleteTasksDto&&const DeepCollectionEquality().equals(other._taskIds, _taskIds));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(_taskIds));

@override
String toString() {
  return 'DeleteTasksDto(taskIds: $taskIds)';
}


}

/// @nodoc
abstract mixin class _$DeleteTasksDtoCopyWith<$Res> implements $DeleteTasksDtoCopyWith<$Res> {
  factory _$DeleteTasksDtoCopyWith(_DeleteTasksDto value, $Res Function(_DeleteTasksDto) _then) = __$DeleteTasksDtoCopyWithImpl;
@override @useResult
$Res call({
 List<String> taskIds
});




}
/// @nodoc
class __$DeleteTasksDtoCopyWithImpl<$Res>
    implements _$DeleteTasksDtoCopyWith<$Res> {
  __$DeleteTasksDtoCopyWithImpl(this._self, this._then);

  final _DeleteTasksDto _self;
  final $Res Function(_DeleteTasksDto) _then;

/// Create a copy of DeleteTasksDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? taskIds = null,}) {
  return _then(_DeleteTasksDto(
taskIds: null == taskIds ? _self._taskIds : taskIds // ignore: cast_nullable_to_non_nullable
as List<String>,
  ));
}


}


/// @nodoc
mixin _$TaskResponseDto {

 String get id; String get name; String? get description; String? get categoryId; int? get scheduledDate; int? get completedAt;
/// Create a copy of TaskResponseDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$TaskResponseDtoCopyWith<TaskResponseDto> get copyWith => _$TaskResponseDtoCopyWithImpl<TaskResponseDto>(this as TaskResponseDto, _$identity);

  /// Serializes this TaskResponseDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is TaskResponseDto&&(identical(other.id, id) || other.id == id)&&(identical(other.name, name) || other.name == name)&&(identical(other.description, description) || other.description == description)&&(identical(other.categoryId, categoryId) || other.categoryId == categoryId)&&(identical(other.scheduledDate, scheduledDate) || other.scheduledDate == scheduledDate)&&(identical(other.completedAt, completedAt) || other.completedAt == completedAt));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,id,name,description,categoryId,scheduledDate,completedAt);

@override
String toString() {
  return 'TaskResponseDto(id: $id, name: $name, description: $description, categoryId: $categoryId, scheduledDate: $scheduledDate, completedAt: $completedAt)';
}


}

/// @nodoc
abstract mixin class $TaskResponseDtoCopyWith<$Res>  {
  factory $TaskResponseDtoCopyWith(TaskResponseDto value, $Res Function(TaskResponseDto) _then) = _$TaskResponseDtoCopyWithImpl;
@useResult
$Res call({
 String id, String name, String? description, String? categoryId, int? scheduledDate, int? completedAt
});




}
/// @nodoc
class _$TaskResponseDtoCopyWithImpl<$Res>
    implements $TaskResponseDtoCopyWith<$Res> {
  _$TaskResponseDtoCopyWithImpl(this._self, this._then);

  final TaskResponseDto _self;
  final $Res Function(TaskResponseDto) _then;

/// Create a copy of TaskResponseDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? id = null,Object? name = null,Object? description = freezed,Object? categoryId = freezed,Object? scheduledDate = freezed,Object? completedAt = freezed,}) {
  return _then(_self.copyWith(
id: null == id ? _self.id : id // ignore: cast_nullable_to_non_nullable
as String,name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,description: freezed == description ? _self.description : description // ignore: cast_nullable_to_non_nullable
as String?,categoryId: freezed == categoryId ? _self.categoryId : categoryId // ignore: cast_nullable_to_non_nullable
as String?,scheduledDate: freezed == scheduledDate ? _self.scheduledDate : scheduledDate // ignore: cast_nullable_to_non_nullable
as int?,completedAt: freezed == completedAt ? _self.completedAt : completedAt // ignore: cast_nullable_to_non_nullable
as int?,
  ));
}

}


/// Adds pattern-matching-related methods to [TaskResponseDto].
extension TaskResponseDtoPatterns on TaskResponseDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _TaskResponseDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _TaskResponseDto() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _TaskResponseDto value)  $default,){
final _that = this;
switch (_that) {
case _TaskResponseDto():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _TaskResponseDto value)?  $default,){
final _that = this;
switch (_that) {
case _TaskResponseDto() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( String id,  String name,  String? description,  String? categoryId,  int? scheduledDate,  int? completedAt)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _TaskResponseDto() when $default != null:
return $default(_that.id,_that.name,_that.description,_that.categoryId,_that.scheduledDate,_that.completedAt);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( String id,  String name,  String? description,  String? categoryId,  int? scheduledDate,  int? completedAt)  $default,) {final _that = this;
switch (_that) {
case _TaskResponseDto():
return $default(_that.id,_that.name,_that.description,_that.categoryId,_that.scheduledDate,_that.completedAt);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( String id,  String name,  String? description,  String? categoryId,  int? scheduledDate,  int? completedAt)?  $default,) {final _that = this;
switch (_that) {
case _TaskResponseDto() when $default != null:
return $default(_that.id,_that.name,_that.description,_that.categoryId,_that.scheduledDate,_that.completedAt);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _TaskResponseDto implements TaskResponseDto {
  const _TaskResponseDto({required this.id, required this.name, this.description, this.categoryId, this.scheduledDate, this.completedAt});
  factory _TaskResponseDto.fromJson(Map<String, dynamic> json) => _$TaskResponseDtoFromJson(json);

@override final  String id;
@override final  String name;
@override final  String? description;
@override final  String? categoryId;
@override final  int? scheduledDate;
@override final  int? completedAt;

/// Create a copy of TaskResponseDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$TaskResponseDtoCopyWith<_TaskResponseDto> get copyWith => __$TaskResponseDtoCopyWithImpl<_TaskResponseDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$TaskResponseDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _TaskResponseDto&&(identical(other.id, id) || other.id == id)&&(identical(other.name, name) || other.name == name)&&(identical(other.description, description) || other.description == description)&&(identical(other.categoryId, categoryId) || other.categoryId == categoryId)&&(identical(other.scheduledDate, scheduledDate) || other.scheduledDate == scheduledDate)&&(identical(other.completedAt, completedAt) || other.completedAt == completedAt));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,id,name,description,categoryId,scheduledDate,completedAt);

@override
String toString() {
  return 'TaskResponseDto(id: $id, name: $name, description: $description, categoryId: $categoryId, scheduledDate: $scheduledDate, completedAt: $completedAt)';
}


}

/// @nodoc
abstract mixin class _$TaskResponseDtoCopyWith<$Res> implements $TaskResponseDtoCopyWith<$Res> {
  factory _$TaskResponseDtoCopyWith(_TaskResponseDto value, $Res Function(_TaskResponseDto) _then) = __$TaskResponseDtoCopyWithImpl;
@override @useResult
$Res call({
 String id, String name, String? description, String? categoryId, int? scheduledDate, int? completedAt
});




}
/// @nodoc
class __$TaskResponseDtoCopyWithImpl<$Res>
    implements _$TaskResponseDtoCopyWith<$Res> {
  __$TaskResponseDtoCopyWithImpl(this._self, this._then);

  final _TaskResponseDto _self;
  final $Res Function(_TaskResponseDto) _then;

/// Create a copy of TaskResponseDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? id = null,Object? name = null,Object? description = freezed,Object? categoryId = freezed,Object? scheduledDate = freezed,Object? completedAt = freezed,}) {
  return _then(_TaskResponseDto(
id: null == id ? _self.id : id // ignore: cast_nullable_to_non_nullable
as String,name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,description: freezed == description ? _self.description : description // ignore: cast_nullable_to_non_nullable
as String?,categoryId: freezed == categoryId ? _self.categoryId : categoryId // ignore: cast_nullable_to_non_nullable
as String?,scheduledDate: freezed == scheduledDate ? _self.scheduledDate : scheduledDate // ignore: cast_nullable_to_non_nullable
as int?,completedAt: freezed == completedAt ? _self.completedAt : completedAt // ignore: cast_nullable_to_non_nullable
as int?,
  ));
}


}


/// @nodoc
mixin _$CreateTaskResponseDto {

 String get id;
/// Create a copy of CreateTaskResponseDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CreateTaskResponseDtoCopyWith<CreateTaskResponseDto> get copyWith => _$CreateTaskResponseDtoCopyWithImpl<CreateTaskResponseDto>(this as CreateTaskResponseDto, _$identity);

  /// Serializes this CreateTaskResponseDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CreateTaskResponseDto&&(identical(other.id, id) || other.id == id));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,id);

@override
String toString() {
  return 'CreateTaskResponseDto(id: $id)';
}


}

/// @nodoc
abstract mixin class $CreateTaskResponseDtoCopyWith<$Res>  {
  factory $CreateTaskResponseDtoCopyWith(CreateTaskResponseDto value, $Res Function(CreateTaskResponseDto) _then) = _$CreateTaskResponseDtoCopyWithImpl;
@useResult
$Res call({
 String id
});




}
/// @nodoc
class _$CreateTaskResponseDtoCopyWithImpl<$Res>
    implements $CreateTaskResponseDtoCopyWith<$Res> {
  _$CreateTaskResponseDtoCopyWithImpl(this._self, this._then);

  final CreateTaskResponseDto _self;
  final $Res Function(CreateTaskResponseDto) _then;

/// Create a copy of CreateTaskResponseDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? id = null,}) {
  return _then(_self.copyWith(
id: null == id ? _self.id : id // ignore: cast_nullable_to_non_nullable
as String,
  ));
}

}


/// Adds pattern-matching-related methods to [CreateTaskResponseDto].
extension CreateTaskResponseDtoPatterns on CreateTaskResponseDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _CreateTaskResponseDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _CreateTaskResponseDto() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _CreateTaskResponseDto value)  $default,){
final _that = this;
switch (_that) {
case _CreateTaskResponseDto():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _CreateTaskResponseDto value)?  $default,){
final _that = this;
switch (_that) {
case _CreateTaskResponseDto() when $default != null:
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
case _CreateTaskResponseDto() when $default != null:
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
case _CreateTaskResponseDto():
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
case _CreateTaskResponseDto() when $default != null:
return $default(_that.id);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _CreateTaskResponseDto implements CreateTaskResponseDto {
  const _CreateTaskResponseDto({required this.id});
  factory _CreateTaskResponseDto.fromJson(Map<String, dynamic> json) => _$CreateTaskResponseDtoFromJson(json);

@override final  String id;

/// Create a copy of CreateTaskResponseDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$CreateTaskResponseDtoCopyWith<_CreateTaskResponseDto> get copyWith => __$CreateTaskResponseDtoCopyWithImpl<_CreateTaskResponseDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$CreateTaskResponseDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _CreateTaskResponseDto&&(identical(other.id, id) || other.id == id));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,id);

@override
String toString() {
  return 'CreateTaskResponseDto(id: $id)';
}


}

/// @nodoc
abstract mixin class _$CreateTaskResponseDtoCopyWith<$Res> implements $CreateTaskResponseDtoCopyWith<$Res> {
  factory _$CreateTaskResponseDtoCopyWith(_CreateTaskResponseDto value, $Res Function(_CreateTaskResponseDto) _then) = __$CreateTaskResponseDtoCopyWithImpl;
@override @useResult
$Res call({
 String id
});




}
/// @nodoc
class __$CreateTaskResponseDtoCopyWithImpl<$Res>
    implements _$CreateTaskResponseDtoCopyWith<$Res> {
  __$CreateTaskResponseDtoCopyWithImpl(this._self, this._then);

  final _CreateTaskResponseDto _self;
  final $Res Function(_CreateTaskResponseDto) _then;

/// Create a copy of CreateTaskResponseDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? id = null,}) {
  return _then(_CreateTaskResponseDto(
id: null == id ? _self.id : id // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}


/// @nodoc
mixin _$UpdateTaskResponseDto {

 TaskResponseDto get updatedTask;
/// Create a copy of UpdateTaskResponseDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$UpdateTaskResponseDtoCopyWith<UpdateTaskResponseDto> get copyWith => _$UpdateTaskResponseDtoCopyWithImpl<UpdateTaskResponseDto>(this as UpdateTaskResponseDto, _$identity);

  /// Serializes this UpdateTaskResponseDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is UpdateTaskResponseDto&&(identical(other.updatedTask, updatedTask) || other.updatedTask == updatedTask));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,updatedTask);

@override
String toString() {
  return 'UpdateTaskResponseDto(updatedTask: $updatedTask)';
}


}

/// @nodoc
abstract mixin class $UpdateTaskResponseDtoCopyWith<$Res>  {
  factory $UpdateTaskResponseDtoCopyWith(UpdateTaskResponseDto value, $Res Function(UpdateTaskResponseDto) _then) = _$UpdateTaskResponseDtoCopyWithImpl;
@useResult
$Res call({
 TaskResponseDto updatedTask
});


$TaskResponseDtoCopyWith<$Res> get updatedTask;

}
/// @nodoc
class _$UpdateTaskResponseDtoCopyWithImpl<$Res>
    implements $UpdateTaskResponseDtoCopyWith<$Res> {
  _$UpdateTaskResponseDtoCopyWithImpl(this._self, this._then);

  final UpdateTaskResponseDto _self;
  final $Res Function(UpdateTaskResponseDto) _then;

/// Create a copy of UpdateTaskResponseDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? updatedTask = null,}) {
  return _then(_self.copyWith(
updatedTask: null == updatedTask ? _self.updatedTask : updatedTask // ignore: cast_nullable_to_non_nullable
as TaskResponseDto,
  ));
}
/// Create a copy of UpdateTaskResponseDto
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$TaskResponseDtoCopyWith<$Res> get updatedTask {
  
  return $TaskResponseDtoCopyWith<$Res>(_self.updatedTask, (value) {
    return _then(_self.copyWith(updatedTask: value));
  });
}
}


/// Adds pattern-matching-related methods to [UpdateTaskResponseDto].
extension UpdateTaskResponseDtoPatterns on UpdateTaskResponseDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _UpdateTaskResponseDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _UpdateTaskResponseDto() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _UpdateTaskResponseDto value)  $default,){
final _that = this;
switch (_that) {
case _UpdateTaskResponseDto():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _UpdateTaskResponseDto value)?  $default,){
final _that = this;
switch (_that) {
case _UpdateTaskResponseDto() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( TaskResponseDto updatedTask)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _UpdateTaskResponseDto() when $default != null:
return $default(_that.updatedTask);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( TaskResponseDto updatedTask)  $default,) {final _that = this;
switch (_that) {
case _UpdateTaskResponseDto():
return $default(_that.updatedTask);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( TaskResponseDto updatedTask)?  $default,) {final _that = this;
switch (_that) {
case _UpdateTaskResponseDto() when $default != null:
return $default(_that.updatedTask);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _UpdateTaskResponseDto implements UpdateTaskResponseDto {
  const _UpdateTaskResponseDto({required this.updatedTask});
  factory _UpdateTaskResponseDto.fromJson(Map<String, dynamic> json) => _$UpdateTaskResponseDtoFromJson(json);

@override final  TaskResponseDto updatedTask;

/// Create a copy of UpdateTaskResponseDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$UpdateTaskResponseDtoCopyWith<_UpdateTaskResponseDto> get copyWith => __$UpdateTaskResponseDtoCopyWithImpl<_UpdateTaskResponseDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$UpdateTaskResponseDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _UpdateTaskResponseDto&&(identical(other.updatedTask, updatedTask) || other.updatedTask == updatedTask));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,updatedTask);

@override
String toString() {
  return 'UpdateTaskResponseDto(updatedTask: $updatedTask)';
}


}

/// @nodoc
abstract mixin class _$UpdateTaskResponseDtoCopyWith<$Res> implements $UpdateTaskResponseDtoCopyWith<$Res> {
  factory _$UpdateTaskResponseDtoCopyWith(_UpdateTaskResponseDto value, $Res Function(_UpdateTaskResponseDto) _then) = __$UpdateTaskResponseDtoCopyWithImpl;
@override @useResult
$Res call({
 TaskResponseDto updatedTask
});


@override $TaskResponseDtoCopyWith<$Res> get updatedTask;

}
/// @nodoc
class __$UpdateTaskResponseDtoCopyWithImpl<$Res>
    implements _$UpdateTaskResponseDtoCopyWith<$Res> {
  __$UpdateTaskResponseDtoCopyWithImpl(this._self, this._then);

  final _UpdateTaskResponseDto _self;
  final $Res Function(_UpdateTaskResponseDto) _then;

/// Create a copy of UpdateTaskResponseDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? updatedTask = null,}) {
  return _then(_UpdateTaskResponseDto(
updatedTask: null == updatedTask ? _self.updatedTask : updatedTask // ignore: cast_nullable_to_non_nullable
as TaskResponseDto,
  ));
}

/// Create a copy of UpdateTaskResponseDto
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$TaskResponseDtoCopyWith<$Res> get updatedTask {
  
  return $TaskResponseDtoCopyWith<$Res>(_self.updatedTask, (value) {
    return _then(_self.copyWith(updatedTask: value));
  });
}
}


/// @nodoc
mixin _$DeleteTasksResponseDto {

@JsonKey(name: 'deleted_ids') List<String> get deletedIds;
/// Create a copy of DeleteTasksResponseDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$DeleteTasksResponseDtoCopyWith<DeleteTasksResponseDto> get copyWith => _$DeleteTasksResponseDtoCopyWithImpl<DeleteTasksResponseDto>(this as DeleteTasksResponseDto, _$identity);

  /// Serializes this DeleteTasksResponseDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is DeleteTasksResponseDto&&const DeepCollectionEquality().equals(other.deletedIds, deletedIds));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(deletedIds));

@override
String toString() {
  return 'DeleteTasksResponseDto(deletedIds: $deletedIds)';
}


}

/// @nodoc
abstract mixin class $DeleteTasksResponseDtoCopyWith<$Res>  {
  factory $DeleteTasksResponseDtoCopyWith(DeleteTasksResponseDto value, $Res Function(DeleteTasksResponseDto) _then) = _$DeleteTasksResponseDtoCopyWithImpl;
@useResult
$Res call({
@JsonKey(name: 'deleted_ids') List<String> deletedIds
});




}
/// @nodoc
class _$DeleteTasksResponseDtoCopyWithImpl<$Res>
    implements $DeleteTasksResponseDtoCopyWith<$Res> {
  _$DeleteTasksResponseDtoCopyWithImpl(this._self, this._then);

  final DeleteTasksResponseDto _self;
  final $Res Function(DeleteTasksResponseDto) _then;

/// Create a copy of DeleteTasksResponseDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? deletedIds = null,}) {
  return _then(_self.copyWith(
deletedIds: null == deletedIds ? _self.deletedIds : deletedIds // ignore: cast_nullable_to_non_nullable
as List<String>,
  ));
}

}


/// Adds pattern-matching-related methods to [DeleteTasksResponseDto].
extension DeleteTasksResponseDtoPatterns on DeleteTasksResponseDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _DeleteTasksResponseDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _DeleteTasksResponseDto() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _DeleteTasksResponseDto value)  $default,){
final _that = this;
switch (_that) {
case _DeleteTasksResponseDto():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _DeleteTasksResponseDto value)?  $default,){
final _that = this;
switch (_that) {
case _DeleteTasksResponseDto() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function(@JsonKey(name: 'deleted_ids')  List<String> deletedIds)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _DeleteTasksResponseDto() when $default != null:
return $default(_that.deletedIds);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function(@JsonKey(name: 'deleted_ids')  List<String> deletedIds)  $default,) {final _that = this;
switch (_that) {
case _DeleteTasksResponseDto():
return $default(_that.deletedIds);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function(@JsonKey(name: 'deleted_ids')  List<String> deletedIds)?  $default,) {final _that = this;
switch (_that) {
case _DeleteTasksResponseDto() when $default != null:
return $default(_that.deletedIds);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _DeleteTasksResponseDto implements DeleteTasksResponseDto {
  const _DeleteTasksResponseDto({@JsonKey(name: 'deleted_ids') required final  List<String> deletedIds}): _deletedIds = deletedIds;
  factory _DeleteTasksResponseDto.fromJson(Map<String, dynamic> json) => _$DeleteTasksResponseDtoFromJson(json);

 final  List<String> _deletedIds;
@override@JsonKey(name: 'deleted_ids') List<String> get deletedIds {
  if (_deletedIds is EqualUnmodifiableListView) return _deletedIds;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_deletedIds);
}


/// Create a copy of DeleteTasksResponseDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$DeleteTasksResponseDtoCopyWith<_DeleteTasksResponseDto> get copyWith => __$DeleteTasksResponseDtoCopyWithImpl<_DeleteTasksResponseDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$DeleteTasksResponseDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _DeleteTasksResponseDto&&const DeepCollectionEquality().equals(other._deletedIds, _deletedIds));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(_deletedIds));

@override
String toString() {
  return 'DeleteTasksResponseDto(deletedIds: $deletedIds)';
}


}

/// @nodoc
abstract mixin class _$DeleteTasksResponseDtoCopyWith<$Res> implements $DeleteTasksResponseDtoCopyWith<$Res> {
  factory _$DeleteTasksResponseDtoCopyWith(_DeleteTasksResponseDto value, $Res Function(_DeleteTasksResponseDto) _then) = __$DeleteTasksResponseDtoCopyWithImpl;
@override @useResult
$Res call({
@JsonKey(name: 'deleted_ids') List<String> deletedIds
});




}
/// @nodoc
class __$DeleteTasksResponseDtoCopyWithImpl<$Res>
    implements _$DeleteTasksResponseDtoCopyWith<$Res> {
  __$DeleteTasksResponseDtoCopyWithImpl(this._self, this._then);

  final _DeleteTasksResponseDto _self;
  final $Res Function(_DeleteTasksResponseDto) _then;

/// Create a copy of DeleteTasksResponseDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? deletedIds = null,}) {
  return _then(_DeleteTasksResponseDto(
deletedIds: null == deletedIds ? _self._deletedIds : deletedIds // ignore: cast_nullable_to_non_nullable
as List<String>,
  ));
}


}


/// @nodoc
mixin _$OrphanTasksResponseDto {

 List<TaskResponseDto> get orphanTasks;
/// Create a copy of OrphanTasksResponseDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$OrphanTasksResponseDtoCopyWith<OrphanTasksResponseDto> get copyWith => _$OrphanTasksResponseDtoCopyWithImpl<OrphanTasksResponseDto>(this as OrphanTasksResponseDto, _$identity);

  /// Serializes this OrphanTasksResponseDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is OrphanTasksResponseDto&&const DeepCollectionEquality().equals(other.orphanTasks, orphanTasks));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(orphanTasks));

@override
String toString() {
  return 'OrphanTasksResponseDto(orphanTasks: $orphanTasks)';
}


}

/// @nodoc
abstract mixin class $OrphanTasksResponseDtoCopyWith<$Res>  {
  factory $OrphanTasksResponseDtoCopyWith(OrphanTasksResponseDto value, $Res Function(OrphanTasksResponseDto) _then) = _$OrphanTasksResponseDtoCopyWithImpl;
@useResult
$Res call({
 List<TaskResponseDto> orphanTasks
});




}
/// @nodoc
class _$OrphanTasksResponseDtoCopyWithImpl<$Res>
    implements $OrphanTasksResponseDtoCopyWith<$Res> {
  _$OrphanTasksResponseDtoCopyWithImpl(this._self, this._then);

  final OrphanTasksResponseDto _self;
  final $Res Function(OrphanTasksResponseDto) _then;

/// Create a copy of OrphanTasksResponseDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? orphanTasks = null,}) {
  return _then(_self.copyWith(
orphanTasks: null == orphanTasks ? _self.orphanTasks : orphanTasks // ignore: cast_nullable_to_non_nullable
as List<TaskResponseDto>,
  ));
}

}


/// Adds pattern-matching-related methods to [OrphanTasksResponseDto].
extension OrphanTasksResponseDtoPatterns on OrphanTasksResponseDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _OrphanTasksResponseDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _OrphanTasksResponseDto() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _OrphanTasksResponseDto value)  $default,){
final _that = this;
switch (_that) {
case _OrphanTasksResponseDto():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _OrphanTasksResponseDto value)?  $default,){
final _that = this;
switch (_that) {
case _OrphanTasksResponseDto() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( List<TaskResponseDto> orphanTasks)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _OrphanTasksResponseDto() when $default != null:
return $default(_that.orphanTasks);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( List<TaskResponseDto> orphanTasks)  $default,) {final _that = this;
switch (_that) {
case _OrphanTasksResponseDto():
return $default(_that.orphanTasks);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( List<TaskResponseDto> orphanTasks)?  $default,) {final _that = this;
switch (_that) {
case _OrphanTasksResponseDto() when $default != null:
return $default(_that.orphanTasks);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _OrphanTasksResponseDto implements OrphanTasksResponseDto {
  const _OrphanTasksResponseDto({required final  List<TaskResponseDto> orphanTasks}): _orphanTasks = orphanTasks;
  factory _OrphanTasksResponseDto.fromJson(Map<String, dynamic> json) => _$OrphanTasksResponseDtoFromJson(json);

 final  List<TaskResponseDto> _orphanTasks;
@override List<TaskResponseDto> get orphanTasks {
  if (_orphanTasks is EqualUnmodifiableListView) return _orphanTasks;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_orphanTasks);
}


/// Create a copy of OrphanTasksResponseDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$OrphanTasksResponseDtoCopyWith<_OrphanTasksResponseDto> get copyWith => __$OrphanTasksResponseDtoCopyWithImpl<_OrphanTasksResponseDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$OrphanTasksResponseDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _OrphanTasksResponseDto&&const DeepCollectionEquality().equals(other._orphanTasks, _orphanTasks));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(_orphanTasks));

@override
String toString() {
  return 'OrphanTasksResponseDto(orphanTasks: $orphanTasks)';
}


}

/// @nodoc
abstract mixin class _$OrphanTasksResponseDtoCopyWith<$Res> implements $OrphanTasksResponseDtoCopyWith<$Res> {
  factory _$OrphanTasksResponseDtoCopyWith(_OrphanTasksResponseDto value, $Res Function(_OrphanTasksResponseDto) _then) = __$OrphanTasksResponseDtoCopyWithImpl;
@override @useResult
$Res call({
 List<TaskResponseDto> orphanTasks
});




}
/// @nodoc
class __$OrphanTasksResponseDtoCopyWithImpl<$Res>
    implements _$OrphanTasksResponseDtoCopyWith<$Res> {
  __$OrphanTasksResponseDtoCopyWithImpl(this._self, this._then);

  final _OrphanTasksResponseDto _self;
  final $Res Function(_OrphanTasksResponseDto) _then;

/// Create a copy of OrphanTasksResponseDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? orphanTasks = null,}) {
  return _then(_OrphanTasksResponseDto(
orphanTasks: null == orphanTasks ? _self._orphanTasks : orphanTasks // ignore: cast_nullable_to_non_nullable
as List<TaskResponseDto>,
  ));
}


}

// dart format on
