// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'category_dtos.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$CreateCategoryDto {

 String get name; String? get color; String? get description;
/// Create a copy of CreateCategoryDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CreateCategoryDtoCopyWith<CreateCategoryDto> get copyWith => _$CreateCategoryDtoCopyWithImpl<CreateCategoryDto>(this as CreateCategoryDto, _$identity);

  /// Serializes this CreateCategoryDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CreateCategoryDto&&(identical(other.name, name) || other.name == name)&&(identical(other.color, color) || other.color == color)&&(identical(other.description, description) || other.description == description));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,name,color,description);

@override
String toString() {
  return 'CreateCategoryDto(name: $name, color: $color, description: $description)';
}


}

/// @nodoc
abstract mixin class $CreateCategoryDtoCopyWith<$Res>  {
  factory $CreateCategoryDtoCopyWith(CreateCategoryDto value, $Res Function(CreateCategoryDto) _then) = _$CreateCategoryDtoCopyWithImpl;
@useResult
$Res call({
 String name, String? color, String? description
});




}
/// @nodoc
class _$CreateCategoryDtoCopyWithImpl<$Res>
    implements $CreateCategoryDtoCopyWith<$Res> {
  _$CreateCategoryDtoCopyWithImpl(this._self, this._then);

  final CreateCategoryDto _self;
  final $Res Function(CreateCategoryDto) _then;

/// Create a copy of CreateCategoryDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? name = null,Object? color = freezed,Object? description = freezed,}) {
  return _then(_self.copyWith(
name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,color: freezed == color ? _self.color : color // ignore: cast_nullable_to_non_nullable
as String?,description: freezed == description ? _self.description : description // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}

}


/// Adds pattern-matching-related methods to [CreateCategoryDto].
extension CreateCategoryDtoPatterns on CreateCategoryDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _CreateCategoryDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _CreateCategoryDto() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _CreateCategoryDto value)  $default,){
final _that = this;
switch (_that) {
case _CreateCategoryDto():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _CreateCategoryDto value)?  $default,){
final _that = this;
switch (_that) {
case _CreateCategoryDto() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( String name,  String? color,  String? description)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _CreateCategoryDto() when $default != null:
return $default(_that.name,_that.color,_that.description);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( String name,  String? color,  String? description)  $default,) {final _that = this;
switch (_that) {
case _CreateCategoryDto():
return $default(_that.name,_that.color,_that.description);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( String name,  String? color,  String? description)?  $default,) {final _that = this;
switch (_that) {
case _CreateCategoryDto() when $default != null:
return $default(_that.name,_that.color,_that.description);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _CreateCategoryDto implements CreateCategoryDto {
  const _CreateCategoryDto({required this.name, this.color, this.description});
  factory _CreateCategoryDto.fromJson(Map<String, dynamic> json) => _$CreateCategoryDtoFromJson(json);

@override final  String name;
@override final  String? color;
@override final  String? description;

/// Create a copy of CreateCategoryDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$CreateCategoryDtoCopyWith<_CreateCategoryDto> get copyWith => __$CreateCategoryDtoCopyWithImpl<_CreateCategoryDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$CreateCategoryDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _CreateCategoryDto&&(identical(other.name, name) || other.name == name)&&(identical(other.color, color) || other.color == color)&&(identical(other.description, description) || other.description == description));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,name,color,description);

@override
String toString() {
  return 'CreateCategoryDto(name: $name, color: $color, description: $description)';
}


}

/// @nodoc
abstract mixin class _$CreateCategoryDtoCopyWith<$Res> implements $CreateCategoryDtoCopyWith<$Res> {
  factory _$CreateCategoryDtoCopyWith(_CreateCategoryDto value, $Res Function(_CreateCategoryDto) _then) = __$CreateCategoryDtoCopyWithImpl;
@override @useResult
$Res call({
 String name, String? color, String? description
});




}
/// @nodoc
class __$CreateCategoryDtoCopyWithImpl<$Res>
    implements _$CreateCategoryDtoCopyWith<$Res> {
  __$CreateCategoryDtoCopyWithImpl(this._self, this._then);

  final _CreateCategoryDto _self;
  final $Res Function(_CreateCategoryDto) _then;

/// Create a copy of CreateCategoryDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? name = null,Object? color = freezed,Object? description = freezed,}) {
  return _then(_CreateCategoryDto(
name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,color: freezed == color ? _self.color : color // ignore: cast_nullable_to_non_nullable
as String?,description: freezed == description ? _self.description : description // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}


}


/// @nodoc
mixin _$UpdateCategoryDto {

 String? get name; String? get color; String? get description;
/// Create a copy of UpdateCategoryDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$UpdateCategoryDtoCopyWith<UpdateCategoryDto> get copyWith => _$UpdateCategoryDtoCopyWithImpl<UpdateCategoryDto>(this as UpdateCategoryDto, _$identity);

  /// Serializes this UpdateCategoryDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is UpdateCategoryDto&&(identical(other.name, name) || other.name == name)&&(identical(other.color, color) || other.color == color)&&(identical(other.description, description) || other.description == description));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,name,color,description);

@override
String toString() {
  return 'UpdateCategoryDto(name: $name, color: $color, description: $description)';
}


}

/// @nodoc
abstract mixin class $UpdateCategoryDtoCopyWith<$Res>  {
  factory $UpdateCategoryDtoCopyWith(UpdateCategoryDto value, $Res Function(UpdateCategoryDto) _then) = _$UpdateCategoryDtoCopyWithImpl;
@useResult
$Res call({
 String? name, String? color, String? description
});




}
/// @nodoc
class _$UpdateCategoryDtoCopyWithImpl<$Res>
    implements $UpdateCategoryDtoCopyWith<$Res> {
  _$UpdateCategoryDtoCopyWithImpl(this._self, this._then);

  final UpdateCategoryDto _self;
  final $Res Function(UpdateCategoryDto) _then;

/// Create a copy of UpdateCategoryDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? name = freezed,Object? color = freezed,Object? description = freezed,}) {
  return _then(_self.copyWith(
name: freezed == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String?,color: freezed == color ? _self.color : color // ignore: cast_nullable_to_non_nullable
as String?,description: freezed == description ? _self.description : description // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}

}


/// Adds pattern-matching-related methods to [UpdateCategoryDto].
extension UpdateCategoryDtoPatterns on UpdateCategoryDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _UpdateCategoryDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _UpdateCategoryDto() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _UpdateCategoryDto value)  $default,){
final _that = this;
switch (_that) {
case _UpdateCategoryDto():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _UpdateCategoryDto value)?  $default,){
final _that = this;
switch (_that) {
case _UpdateCategoryDto() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( String? name,  String? color,  String? description)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _UpdateCategoryDto() when $default != null:
return $default(_that.name,_that.color,_that.description);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( String? name,  String? color,  String? description)  $default,) {final _that = this;
switch (_that) {
case _UpdateCategoryDto():
return $default(_that.name,_that.color,_that.description);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( String? name,  String? color,  String? description)?  $default,) {final _that = this;
switch (_that) {
case _UpdateCategoryDto() when $default != null:
return $default(_that.name,_that.color,_that.description);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _UpdateCategoryDto implements UpdateCategoryDto {
  const _UpdateCategoryDto({this.name, this.color, this.description});
  factory _UpdateCategoryDto.fromJson(Map<String, dynamic> json) => _$UpdateCategoryDtoFromJson(json);

@override final  String? name;
@override final  String? color;
@override final  String? description;

/// Create a copy of UpdateCategoryDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$UpdateCategoryDtoCopyWith<_UpdateCategoryDto> get copyWith => __$UpdateCategoryDtoCopyWithImpl<_UpdateCategoryDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$UpdateCategoryDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _UpdateCategoryDto&&(identical(other.name, name) || other.name == name)&&(identical(other.color, color) || other.color == color)&&(identical(other.description, description) || other.description == description));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,name,color,description);

@override
String toString() {
  return 'UpdateCategoryDto(name: $name, color: $color, description: $description)';
}


}

/// @nodoc
abstract mixin class _$UpdateCategoryDtoCopyWith<$Res> implements $UpdateCategoryDtoCopyWith<$Res> {
  factory _$UpdateCategoryDtoCopyWith(_UpdateCategoryDto value, $Res Function(_UpdateCategoryDto) _then) = __$UpdateCategoryDtoCopyWithImpl;
@override @useResult
$Res call({
 String? name, String? color, String? description
});




}
/// @nodoc
class __$UpdateCategoryDtoCopyWithImpl<$Res>
    implements _$UpdateCategoryDtoCopyWith<$Res> {
  __$UpdateCategoryDtoCopyWithImpl(this._self, this._then);

  final _UpdateCategoryDto _self;
  final $Res Function(_UpdateCategoryDto) _then;

/// Create a copy of UpdateCategoryDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? name = freezed,Object? color = freezed,Object? description = freezed,}) {
  return _then(_UpdateCategoryDto(
name: freezed == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String?,color: freezed == color ? _self.color : color // ignore: cast_nullable_to_non_nullable
as String?,description: freezed == description ? _self.description : description // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}


}


/// @nodoc
mixin _$TaskDto {

 String get id; String get name; String? get description; String? get categoryId; int? get scheduledDate; int? get completedAt;
/// Create a copy of TaskDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$TaskDtoCopyWith<TaskDto> get copyWith => _$TaskDtoCopyWithImpl<TaskDto>(this as TaskDto, _$identity);

  /// Serializes this TaskDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is TaskDto&&(identical(other.id, id) || other.id == id)&&(identical(other.name, name) || other.name == name)&&(identical(other.description, description) || other.description == description)&&(identical(other.categoryId, categoryId) || other.categoryId == categoryId)&&(identical(other.scheduledDate, scheduledDate) || other.scheduledDate == scheduledDate)&&(identical(other.completedAt, completedAt) || other.completedAt == completedAt));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,id,name,description,categoryId,scheduledDate,completedAt);

@override
String toString() {
  return 'TaskDto(id: $id, name: $name, description: $description, categoryId: $categoryId, scheduledDate: $scheduledDate, completedAt: $completedAt)';
}


}

/// @nodoc
abstract mixin class $TaskDtoCopyWith<$Res>  {
  factory $TaskDtoCopyWith(TaskDto value, $Res Function(TaskDto) _then) = _$TaskDtoCopyWithImpl;
@useResult
$Res call({
 String id, String name, String? description, String? categoryId, int? scheduledDate, int? completedAt
});




}
/// @nodoc
class _$TaskDtoCopyWithImpl<$Res>
    implements $TaskDtoCopyWith<$Res> {
  _$TaskDtoCopyWithImpl(this._self, this._then);

  final TaskDto _self;
  final $Res Function(TaskDto) _then;

/// Create a copy of TaskDto
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


/// Adds pattern-matching-related methods to [TaskDto].
extension TaskDtoPatterns on TaskDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _TaskDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _TaskDto() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _TaskDto value)  $default,){
final _that = this;
switch (_that) {
case _TaskDto():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _TaskDto value)?  $default,){
final _that = this;
switch (_that) {
case _TaskDto() when $default != null:
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
case _TaskDto() when $default != null:
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
case _TaskDto():
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
case _TaskDto() when $default != null:
return $default(_that.id,_that.name,_that.description,_that.categoryId,_that.scheduledDate,_that.completedAt);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _TaskDto implements TaskDto {
  const _TaskDto({required this.id, required this.name, this.description, this.categoryId, this.scheduledDate, this.completedAt});
  factory _TaskDto.fromJson(Map<String, dynamic> json) => _$TaskDtoFromJson(json);

@override final  String id;
@override final  String name;
@override final  String? description;
@override final  String? categoryId;
@override final  int? scheduledDate;
@override final  int? completedAt;

/// Create a copy of TaskDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$TaskDtoCopyWith<_TaskDto> get copyWith => __$TaskDtoCopyWithImpl<_TaskDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$TaskDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _TaskDto&&(identical(other.id, id) || other.id == id)&&(identical(other.name, name) || other.name == name)&&(identical(other.description, description) || other.description == description)&&(identical(other.categoryId, categoryId) || other.categoryId == categoryId)&&(identical(other.scheduledDate, scheduledDate) || other.scheduledDate == scheduledDate)&&(identical(other.completedAt, completedAt) || other.completedAt == completedAt));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,id,name,description,categoryId,scheduledDate,completedAt);

@override
String toString() {
  return 'TaskDto(id: $id, name: $name, description: $description, categoryId: $categoryId, scheduledDate: $scheduledDate, completedAt: $completedAt)';
}


}

/// @nodoc
abstract mixin class _$TaskDtoCopyWith<$Res> implements $TaskDtoCopyWith<$Res> {
  factory _$TaskDtoCopyWith(_TaskDto value, $Res Function(_TaskDto) _then) = __$TaskDtoCopyWithImpl;
@override @useResult
$Res call({
 String id, String name, String? description, String? categoryId, int? scheduledDate, int? completedAt
});




}
/// @nodoc
class __$TaskDtoCopyWithImpl<$Res>
    implements _$TaskDtoCopyWith<$Res> {
  __$TaskDtoCopyWithImpl(this._self, this._then);

  final _TaskDto _self;
  final $Res Function(_TaskDto) _then;

/// Create a copy of TaskDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? id = null,Object? name = null,Object? description = freezed,Object? categoryId = freezed,Object? scheduledDate = freezed,Object? completedAt = freezed,}) {
  return _then(_TaskDto(
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
mixin _$CategoryDto {

 String get id; String get name; String get color; String? get description; List<TaskDto> get tasks;
/// Create a copy of CategoryDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CategoryDtoCopyWith<CategoryDto> get copyWith => _$CategoryDtoCopyWithImpl<CategoryDto>(this as CategoryDto, _$identity);

  /// Serializes this CategoryDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CategoryDto&&(identical(other.id, id) || other.id == id)&&(identical(other.name, name) || other.name == name)&&(identical(other.color, color) || other.color == color)&&(identical(other.description, description) || other.description == description)&&const DeepCollectionEquality().equals(other.tasks, tasks));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,id,name,color,description,const DeepCollectionEquality().hash(tasks));

@override
String toString() {
  return 'CategoryDto(id: $id, name: $name, color: $color, description: $description, tasks: $tasks)';
}


}

/// @nodoc
abstract mixin class $CategoryDtoCopyWith<$Res>  {
  factory $CategoryDtoCopyWith(CategoryDto value, $Res Function(CategoryDto) _then) = _$CategoryDtoCopyWithImpl;
@useResult
$Res call({
 String id, String name, String color, String? description, List<TaskDto> tasks
});




}
/// @nodoc
class _$CategoryDtoCopyWithImpl<$Res>
    implements $CategoryDtoCopyWith<$Res> {
  _$CategoryDtoCopyWithImpl(this._self, this._then);

  final CategoryDto _self;
  final $Res Function(CategoryDto) _then;

/// Create a copy of CategoryDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? id = null,Object? name = null,Object? color = null,Object? description = freezed,Object? tasks = null,}) {
  return _then(_self.copyWith(
id: null == id ? _self.id : id // ignore: cast_nullable_to_non_nullable
as String,name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,color: null == color ? _self.color : color // ignore: cast_nullable_to_non_nullable
as String,description: freezed == description ? _self.description : description // ignore: cast_nullable_to_non_nullable
as String?,tasks: null == tasks ? _self.tasks : tasks // ignore: cast_nullable_to_non_nullable
as List<TaskDto>,
  ));
}

}


/// Adds pattern-matching-related methods to [CategoryDto].
extension CategoryDtoPatterns on CategoryDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _CategoryDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _CategoryDto() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _CategoryDto value)  $default,){
final _that = this;
switch (_that) {
case _CategoryDto():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _CategoryDto value)?  $default,){
final _that = this;
switch (_that) {
case _CategoryDto() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( String id,  String name,  String color,  String? description,  List<TaskDto> tasks)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _CategoryDto() when $default != null:
return $default(_that.id,_that.name,_that.color,_that.description,_that.tasks);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( String id,  String name,  String color,  String? description,  List<TaskDto> tasks)  $default,) {final _that = this;
switch (_that) {
case _CategoryDto():
return $default(_that.id,_that.name,_that.color,_that.description,_that.tasks);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( String id,  String name,  String color,  String? description,  List<TaskDto> tasks)?  $default,) {final _that = this;
switch (_that) {
case _CategoryDto() when $default != null:
return $default(_that.id,_that.name,_that.color,_that.description,_that.tasks);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _CategoryDto implements CategoryDto {
  const _CategoryDto({required this.id, required this.name, required this.color, this.description, required final  List<TaskDto> tasks}): _tasks = tasks;
  factory _CategoryDto.fromJson(Map<String, dynamic> json) => _$CategoryDtoFromJson(json);

@override final  String id;
@override final  String name;
@override final  String color;
@override final  String? description;
 final  List<TaskDto> _tasks;
@override List<TaskDto> get tasks {
  if (_tasks is EqualUnmodifiableListView) return _tasks;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_tasks);
}


/// Create a copy of CategoryDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$CategoryDtoCopyWith<_CategoryDto> get copyWith => __$CategoryDtoCopyWithImpl<_CategoryDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$CategoryDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _CategoryDto&&(identical(other.id, id) || other.id == id)&&(identical(other.name, name) || other.name == name)&&(identical(other.color, color) || other.color == color)&&(identical(other.description, description) || other.description == description)&&const DeepCollectionEquality().equals(other._tasks, _tasks));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,id,name,color,description,const DeepCollectionEquality().hash(_tasks));

@override
String toString() {
  return 'CategoryDto(id: $id, name: $name, color: $color, description: $description, tasks: $tasks)';
}


}

/// @nodoc
abstract mixin class _$CategoryDtoCopyWith<$Res> implements $CategoryDtoCopyWith<$Res> {
  factory _$CategoryDtoCopyWith(_CategoryDto value, $Res Function(_CategoryDto) _then) = __$CategoryDtoCopyWithImpl;
@override @useResult
$Res call({
 String id, String name, String color, String? description, List<TaskDto> tasks
});




}
/// @nodoc
class __$CategoryDtoCopyWithImpl<$Res>
    implements _$CategoryDtoCopyWith<$Res> {
  __$CategoryDtoCopyWithImpl(this._self, this._then);

  final _CategoryDto _self;
  final $Res Function(_CategoryDto) _then;

/// Create a copy of CategoryDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? id = null,Object? name = null,Object? color = null,Object? description = freezed,Object? tasks = null,}) {
  return _then(_CategoryDto(
id: null == id ? _self.id : id // ignore: cast_nullable_to_non_nullable
as String,name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,color: null == color ? _self.color : color // ignore: cast_nullable_to_non_nullable
as String,description: freezed == description ? _self.description : description // ignore: cast_nullable_to_non_nullable
as String?,tasks: null == tasks ? _self._tasks : tasks // ignore: cast_nullable_to_non_nullable
as List<TaskDto>,
  ));
}


}


/// @nodoc
mixin _$GetCategoriesResponseDto {

 List<CategoryDto> get categories;
/// Create a copy of GetCategoriesResponseDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GetCategoriesResponseDtoCopyWith<GetCategoriesResponseDto> get copyWith => _$GetCategoriesResponseDtoCopyWithImpl<GetCategoriesResponseDto>(this as GetCategoriesResponseDto, _$identity);

  /// Serializes this GetCategoriesResponseDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GetCategoriesResponseDto&&const DeepCollectionEquality().equals(other.categories, categories));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(categories));

@override
String toString() {
  return 'GetCategoriesResponseDto(categories: $categories)';
}


}

/// @nodoc
abstract mixin class $GetCategoriesResponseDtoCopyWith<$Res>  {
  factory $GetCategoriesResponseDtoCopyWith(GetCategoriesResponseDto value, $Res Function(GetCategoriesResponseDto) _then) = _$GetCategoriesResponseDtoCopyWithImpl;
@useResult
$Res call({
 List<CategoryDto> categories
});




}
/// @nodoc
class _$GetCategoriesResponseDtoCopyWithImpl<$Res>
    implements $GetCategoriesResponseDtoCopyWith<$Res> {
  _$GetCategoriesResponseDtoCopyWithImpl(this._self, this._then);

  final GetCategoriesResponseDto _self;
  final $Res Function(GetCategoriesResponseDto) _then;

/// Create a copy of GetCategoriesResponseDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? categories = null,}) {
  return _then(_self.copyWith(
categories: null == categories ? _self.categories : categories // ignore: cast_nullable_to_non_nullable
as List<CategoryDto>,
  ));
}

}


/// Adds pattern-matching-related methods to [GetCategoriesResponseDto].
extension GetCategoriesResponseDtoPatterns on GetCategoriesResponseDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _GetCategoriesResponseDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _GetCategoriesResponseDto() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _GetCategoriesResponseDto value)  $default,){
final _that = this;
switch (_that) {
case _GetCategoriesResponseDto():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _GetCategoriesResponseDto value)?  $default,){
final _that = this;
switch (_that) {
case _GetCategoriesResponseDto() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( List<CategoryDto> categories)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _GetCategoriesResponseDto() when $default != null:
return $default(_that.categories);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( List<CategoryDto> categories)  $default,) {final _that = this;
switch (_that) {
case _GetCategoriesResponseDto():
return $default(_that.categories);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( List<CategoryDto> categories)?  $default,) {final _that = this;
switch (_that) {
case _GetCategoriesResponseDto() when $default != null:
return $default(_that.categories);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _GetCategoriesResponseDto implements GetCategoriesResponseDto {
  const _GetCategoriesResponseDto({required final  List<CategoryDto> categories}): _categories = categories;
  factory _GetCategoriesResponseDto.fromJson(Map<String, dynamic> json) => _$GetCategoriesResponseDtoFromJson(json);

 final  List<CategoryDto> _categories;
@override List<CategoryDto> get categories {
  if (_categories is EqualUnmodifiableListView) return _categories;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_categories);
}


/// Create a copy of GetCategoriesResponseDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$GetCategoriesResponseDtoCopyWith<_GetCategoriesResponseDto> get copyWith => __$GetCategoriesResponseDtoCopyWithImpl<_GetCategoriesResponseDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$GetCategoriesResponseDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _GetCategoriesResponseDto&&const DeepCollectionEquality().equals(other._categories, _categories));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(_categories));

@override
String toString() {
  return 'GetCategoriesResponseDto(categories: $categories)';
}


}

/// @nodoc
abstract mixin class _$GetCategoriesResponseDtoCopyWith<$Res> implements $GetCategoriesResponseDtoCopyWith<$Res> {
  factory _$GetCategoriesResponseDtoCopyWith(_GetCategoriesResponseDto value, $Res Function(_GetCategoriesResponseDto) _then) = __$GetCategoriesResponseDtoCopyWithImpl;
@override @useResult
$Res call({
 List<CategoryDto> categories
});




}
/// @nodoc
class __$GetCategoriesResponseDtoCopyWithImpl<$Res>
    implements _$GetCategoriesResponseDtoCopyWith<$Res> {
  __$GetCategoriesResponseDtoCopyWithImpl(this._self, this._then);

  final _GetCategoriesResponseDto _self;
  final $Res Function(_GetCategoriesResponseDto) _then;

/// Create a copy of GetCategoriesResponseDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? categories = null,}) {
  return _then(_GetCategoriesResponseDto(
categories: null == categories ? _self._categories : categories // ignore: cast_nullable_to_non_nullable
as List<CategoryDto>,
  ));
}


}


/// @nodoc
mixin _$CreateCategoryResponseDto {

 bool get created;
/// Create a copy of CreateCategoryResponseDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CreateCategoryResponseDtoCopyWith<CreateCategoryResponseDto> get copyWith => _$CreateCategoryResponseDtoCopyWithImpl<CreateCategoryResponseDto>(this as CreateCategoryResponseDto, _$identity);

  /// Serializes this CreateCategoryResponseDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CreateCategoryResponseDto&&(identical(other.created, created) || other.created == created));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,created);

@override
String toString() {
  return 'CreateCategoryResponseDto(created: $created)';
}


}

/// @nodoc
abstract mixin class $CreateCategoryResponseDtoCopyWith<$Res>  {
  factory $CreateCategoryResponseDtoCopyWith(CreateCategoryResponseDto value, $Res Function(CreateCategoryResponseDto) _then) = _$CreateCategoryResponseDtoCopyWithImpl;
@useResult
$Res call({
 bool created
});




}
/// @nodoc
class _$CreateCategoryResponseDtoCopyWithImpl<$Res>
    implements $CreateCategoryResponseDtoCopyWith<$Res> {
  _$CreateCategoryResponseDtoCopyWithImpl(this._self, this._then);

  final CreateCategoryResponseDto _self;
  final $Res Function(CreateCategoryResponseDto) _then;

/// Create a copy of CreateCategoryResponseDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? created = null,}) {
  return _then(_self.copyWith(
created: null == created ? _self.created : created // ignore: cast_nullable_to_non_nullable
as bool,
  ));
}

}


/// Adds pattern-matching-related methods to [CreateCategoryResponseDto].
extension CreateCategoryResponseDtoPatterns on CreateCategoryResponseDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _CreateCategoryResponseDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _CreateCategoryResponseDto() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _CreateCategoryResponseDto value)  $default,){
final _that = this;
switch (_that) {
case _CreateCategoryResponseDto():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _CreateCategoryResponseDto value)?  $default,){
final _that = this;
switch (_that) {
case _CreateCategoryResponseDto() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( bool created)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _CreateCategoryResponseDto() when $default != null:
return $default(_that.created);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( bool created)  $default,) {final _that = this;
switch (_that) {
case _CreateCategoryResponseDto():
return $default(_that.created);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( bool created)?  $default,) {final _that = this;
switch (_that) {
case _CreateCategoryResponseDto() when $default != null:
return $default(_that.created);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _CreateCategoryResponseDto implements CreateCategoryResponseDto {
  const _CreateCategoryResponseDto({required this.created});
  factory _CreateCategoryResponseDto.fromJson(Map<String, dynamic> json) => _$CreateCategoryResponseDtoFromJson(json);

@override final  bool created;

/// Create a copy of CreateCategoryResponseDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$CreateCategoryResponseDtoCopyWith<_CreateCategoryResponseDto> get copyWith => __$CreateCategoryResponseDtoCopyWithImpl<_CreateCategoryResponseDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$CreateCategoryResponseDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _CreateCategoryResponseDto&&(identical(other.created, created) || other.created == created));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,created);

@override
String toString() {
  return 'CreateCategoryResponseDto(created: $created)';
}


}

/// @nodoc
abstract mixin class _$CreateCategoryResponseDtoCopyWith<$Res> implements $CreateCategoryResponseDtoCopyWith<$Res> {
  factory _$CreateCategoryResponseDtoCopyWith(_CreateCategoryResponseDto value, $Res Function(_CreateCategoryResponseDto) _then) = __$CreateCategoryResponseDtoCopyWithImpl;
@override @useResult
$Res call({
 bool created
});




}
/// @nodoc
class __$CreateCategoryResponseDtoCopyWithImpl<$Res>
    implements _$CreateCategoryResponseDtoCopyWith<$Res> {
  __$CreateCategoryResponseDtoCopyWithImpl(this._self, this._then);

  final _CreateCategoryResponseDto _self;
  final $Res Function(_CreateCategoryResponseDto) _then;

/// Create a copy of CreateCategoryResponseDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? created = null,}) {
  return _then(_CreateCategoryResponseDto(
created: null == created ? _self.created : created // ignore: cast_nullable_to_non_nullable
as bool,
  ));
}


}


/// @nodoc
mixin _$UpdateCategoryResponseDto {

 CategoryDto get updatedCategory;
/// Create a copy of UpdateCategoryResponseDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$UpdateCategoryResponseDtoCopyWith<UpdateCategoryResponseDto> get copyWith => _$UpdateCategoryResponseDtoCopyWithImpl<UpdateCategoryResponseDto>(this as UpdateCategoryResponseDto, _$identity);

  /// Serializes this UpdateCategoryResponseDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is UpdateCategoryResponseDto&&(identical(other.updatedCategory, updatedCategory) || other.updatedCategory == updatedCategory));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,updatedCategory);

@override
String toString() {
  return 'UpdateCategoryResponseDto(updatedCategory: $updatedCategory)';
}


}

/// @nodoc
abstract mixin class $UpdateCategoryResponseDtoCopyWith<$Res>  {
  factory $UpdateCategoryResponseDtoCopyWith(UpdateCategoryResponseDto value, $Res Function(UpdateCategoryResponseDto) _then) = _$UpdateCategoryResponseDtoCopyWithImpl;
@useResult
$Res call({
 CategoryDto updatedCategory
});


$CategoryDtoCopyWith<$Res> get updatedCategory;

}
/// @nodoc
class _$UpdateCategoryResponseDtoCopyWithImpl<$Res>
    implements $UpdateCategoryResponseDtoCopyWith<$Res> {
  _$UpdateCategoryResponseDtoCopyWithImpl(this._self, this._then);

  final UpdateCategoryResponseDto _self;
  final $Res Function(UpdateCategoryResponseDto) _then;

/// Create a copy of UpdateCategoryResponseDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? updatedCategory = null,}) {
  return _then(_self.copyWith(
updatedCategory: null == updatedCategory ? _self.updatedCategory : updatedCategory // ignore: cast_nullable_to_non_nullable
as CategoryDto,
  ));
}
/// Create a copy of UpdateCategoryResponseDto
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$CategoryDtoCopyWith<$Res> get updatedCategory {
  
  return $CategoryDtoCopyWith<$Res>(_self.updatedCategory, (value) {
    return _then(_self.copyWith(updatedCategory: value));
  });
}
}


/// Adds pattern-matching-related methods to [UpdateCategoryResponseDto].
extension UpdateCategoryResponseDtoPatterns on UpdateCategoryResponseDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _UpdateCategoryResponseDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _UpdateCategoryResponseDto() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _UpdateCategoryResponseDto value)  $default,){
final _that = this;
switch (_that) {
case _UpdateCategoryResponseDto():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _UpdateCategoryResponseDto value)?  $default,){
final _that = this;
switch (_that) {
case _UpdateCategoryResponseDto() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( CategoryDto updatedCategory)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _UpdateCategoryResponseDto() when $default != null:
return $default(_that.updatedCategory);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( CategoryDto updatedCategory)  $default,) {final _that = this;
switch (_that) {
case _UpdateCategoryResponseDto():
return $default(_that.updatedCategory);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( CategoryDto updatedCategory)?  $default,) {final _that = this;
switch (_that) {
case _UpdateCategoryResponseDto() when $default != null:
return $default(_that.updatedCategory);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _UpdateCategoryResponseDto implements UpdateCategoryResponseDto {
  const _UpdateCategoryResponseDto({required this.updatedCategory});
  factory _UpdateCategoryResponseDto.fromJson(Map<String, dynamic> json) => _$UpdateCategoryResponseDtoFromJson(json);

@override final  CategoryDto updatedCategory;

/// Create a copy of UpdateCategoryResponseDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$UpdateCategoryResponseDtoCopyWith<_UpdateCategoryResponseDto> get copyWith => __$UpdateCategoryResponseDtoCopyWithImpl<_UpdateCategoryResponseDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$UpdateCategoryResponseDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _UpdateCategoryResponseDto&&(identical(other.updatedCategory, updatedCategory) || other.updatedCategory == updatedCategory));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,updatedCategory);

@override
String toString() {
  return 'UpdateCategoryResponseDto(updatedCategory: $updatedCategory)';
}


}

/// @nodoc
abstract mixin class _$UpdateCategoryResponseDtoCopyWith<$Res> implements $UpdateCategoryResponseDtoCopyWith<$Res> {
  factory _$UpdateCategoryResponseDtoCopyWith(_UpdateCategoryResponseDto value, $Res Function(_UpdateCategoryResponseDto) _then) = __$UpdateCategoryResponseDtoCopyWithImpl;
@override @useResult
$Res call({
 CategoryDto updatedCategory
});


@override $CategoryDtoCopyWith<$Res> get updatedCategory;

}
/// @nodoc
class __$UpdateCategoryResponseDtoCopyWithImpl<$Res>
    implements _$UpdateCategoryResponseDtoCopyWith<$Res> {
  __$UpdateCategoryResponseDtoCopyWithImpl(this._self, this._then);

  final _UpdateCategoryResponseDto _self;
  final $Res Function(_UpdateCategoryResponseDto) _then;

/// Create a copy of UpdateCategoryResponseDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? updatedCategory = null,}) {
  return _then(_UpdateCategoryResponseDto(
updatedCategory: null == updatedCategory ? _self.updatedCategory : updatedCategory // ignore: cast_nullable_to_non_nullable
as CategoryDto,
  ));
}

/// Create a copy of UpdateCategoryResponseDto
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$CategoryDtoCopyWith<$Res> get updatedCategory {
  
  return $CategoryDtoCopyWith<$Res>(_self.updatedCategory, (value) {
    return _then(_self.copyWith(updatedCategory: value));
  });
}
}


/// @nodoc
mixin _$DeleteCategoriesResponseDto {

@JsonKey(name: 'deleted_ids') List<String> get deletedIds;
/// Create a copy of DeleteCategoriesResponseDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$DeleteCategoriesResponseDtoCopyWith<DeleteCategoriesResponseDto> get copyWith => _$DeleteCategoriesResponseDtoCopyWithImpl<DeleteCategoriesResponseDto>(this as DeleteCategoriesResponseDto, _$identity);

  /// Serializes this DeleteCategoriesResponseDto to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is DeleteCategoriesResponseDto&&const DeepCollectionEquality().equals(other.deletedIds, deletedIds));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(deletedIds));

@override
String toString() {
  return 'DeleteCategoriesResponseDto(deletedIds: $deletedIds)';
}


}

/// @nodoc
abstract mixin class $DeleteCategoriesResponseDtoCopyWith<$Res>  {
  factory $DeleteCategoriesResponseDtoCopyWith(DeleteCategoriesResponseDto value, $Res Function(DeleteCategoriesResponseDto) _then) = _$DeleteCategoriesResponseDtoCopyWithImpl;
@useResult
$Res call({
@JsonKey(name: 'deleted_ids') List<String> deletedIds
});




}
/// @nodoc
class _$DeleteCategoriesResponseDtoCopyWithImpl<$Res>
    implements $DeleteCategoriesResponseDtoCopyWith<$Res> {
  _$DeleteCategoriesResponseDtoCopyWithImpl(this._self, this._then);

  final DeleteCategoriesResponseDto _self;
  final $Res Function(DeleteCategoriesResponseDto) _then;

/// Create a copy of DeleteCategoriesResponseDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? deletedIds = null,}) {
  return _then(_self.copyWith(
deletedIds: null == deletedIds ? _self.deletedIds : deletedIds // ignore: cast_nullable_to_non_nullable
as List<String>,
  ));
}

}


/// Adds pattern-matching-related methods to [DeleteCategoriesResponseDto].
extension DeleteCategoriesResponseDtoPatterns on DeleteCategoriesResponseDto {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _DeleteCategoriesResponseDto value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _DeleteCategoriesResponseDto() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _DeleteCategoriesResponseDto value)  $default,){
final _that = this;
switch (_that) {
case _DeleteCategoriesResponseDto():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _DeleteCategoriesResponseDto value)?  $default,){
final _that = this;
switch (_that) {
case _DeleteCategoriesResponseDto() when $default != null:
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
case _DeleteCategoriesResponseDto() when $default != null:
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
case _DeleteCategoriesResponseDto():
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
case _DeleteCategoriesResponseDto() when $default != null:
return $default(_that.deletedIds);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _DeleteCategoriesResponseDto implements DeleteCategoriesResponseDto {
  const _DeleteCategoriesResponseDto({@JsonKey(name: 'deleted_ids') required final  List<String> deletedIds}): _deletedIds = deletedIds;
  factory _DeleteCategoriesResponseDto.fromJson(Map<String, dynamic> json) => _$DeleteCategoriesResponseDtoFromJson(json);

 final  List<String> _deletedIds;
@override@JsonKey(name: 'deleted_ids') List<String> get deletedIds {
  if (_deletedIds is EqualUnmodifiableListView) return _deletedIds;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_deletedIds);
}


/// Create a copy of DeleteCategoriesResponseDto
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$DeleteCategoriesResponseDtoCopyWith<_DeleteCategoriesResponseDto> get copyWith => __$DeleteCategoriesResponseDtoCopyWithImpl<_DeleteCategoriesResponseDto>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$DeleteCategoriesResponseDtoToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _DeleteCategoriesResponseDto&&const DeepCollectionEquality().equals(other._deletedIds, _deletedIds));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(_deletedIds));

@override
String toString() {
  return 'DeleteCategoriesResponseDto(deletedIds: $deletedIds)';
}


}

/// @nodoc
abstract mixin class _$DeleteCategoriesResponseDtoCopyWith<$Res> implements $DeleteCategoriesResponseDtoCopyWith<$Res> {
  factory _$DeleteCategoriesResponseDtoCopyWith(_DeleteCategoriesResponseDto value, $Res Function(_DeleteCategoriesResponseDto) _then) = __$DeleteCategoriesResponseDtoCopyWithImpl;
@override @useResult
$Res call({
@JsonKey(name: 'deleted_ids') List<String> deletedIds
});




}
/// @nodoc
class __$DeleteCategoriesResponseDtoCopyWithImpl<$Res>
    implements _$DeleteCategoriesResponseDtoCopyWith<$Res> {
  __$DeleteCategoriesResponseDtoCopyWithImpl(this._self, this._then);

  final _DeleteCategoriesResponseDto _self;
  final $Res Function(_DeleteCategoriesResponseDto) _then;

/// Create a copy of DeleteCategoriesResponseDto
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? deletedIds = null,}) {
  return _then(_DeleteCategoriesResponseDto(
deletedIds: null == deletedIds ? _self._deletedIds : deletedIds // ignore: cast_nullable_to_non_nullable
as List<String>,
  ));
}


}

// dart format on
