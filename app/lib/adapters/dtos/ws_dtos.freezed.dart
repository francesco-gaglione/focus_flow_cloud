// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'ws_dtos.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$UpdateConcentrationScore {

 int get concentrationScore;
/// Create a copy of UpdateConcentrationScore
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$UpdateConcentrationScoreCopyWith<UpdateConcentrationScore> get copyWith => _$UpdateConcentrationScoreCopyWithImpl<UpdateConcentrationScore>(this as UpdateConcentrationScore, _$identity);

  /// Serializes this UpdateConcentrationScore to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is UpdateConcentrationScore&&(identical(other.concentrationScore, concentrationScore) || other.concentrationScore == concentrationScore));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,concentrationScore);

@override
String toString() {
  return 'UpdateConcentrationScore(concentrationScore: $concentrationScore)';
}


}

/// @nodoc
abstract mixin class $UpdateConcentrationScoreCopyWith<$Res>  {
  factory $UpdateConcentrationScoreCopyWith(UpdateConcentrationScore value, $Res Function(UpdateConcentrationScore) _then) = _$UpdateConcentrationScoreCopyWithImpl;
@useResult
$Res call({
 int concentrationScore
});




}
/// @nodoc
class _$UpdateConcentrationScoreCopyWithImpl<$Res>
    implements $UpdateConcentrationScoreCopyWith<$Res> {
  _$UpdateConcentrationScoreCopyWithImpl(this._self, this._then);

  final UpdateConcentrationScore _self;
  final $Res Function(UpdateConcentrationScore) _then;

/// Create a copy of UpdateConcentrationScore
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? concentrationScore = null,}) {
  return _then(_self.copyWith(
concentrationScore: null == concentrationScore ? _self.concentrationScore : concentrationScore // ignore: cast_nullable_to_non_nullable
as int,
  ));
}

}


/// Adds pattern-matching-related methods to [UpdateConcentrationScore].
extension UpdateConcentrationScorePatterns on UpdateConcentrationScore {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _UpdateConcentrationScore value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _UpdateConcentrationScore() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _UpdateConcentrationScore value)  $default,){
final _that = this;
switch (_that) {
case _UpdateConcentrationScore():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _UpdateConcentrationScore value)?  $default,){
final _that = this;
switch (_that) {
case _UpdateConcentrationScore() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( int concentrationScore)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _UpdateConcentrationScore() when $default != null:
return $default(_that.concentrationScore);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( int concentrationScore)  $default,) {final _that = this;
switch (_that) {
case _UpdateConcentrationScore():
return $default(_that.concentrationScore);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( int concentrationScore)?  $default,) {final _that = this;
switch (_that) {
case _UpdateConcentrationScore() when $default != null:
return $default(_that.concentrationScore);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _UpdateConcentrationScore implements UpdateConcentrationScore {
  const _UpdateConcentrationScore({required this.concentrationScore});
  factory _UpdateConcentrationScore.fromJson(Map<String, dynamic> json) => _$UpdateConcentrationScoreFromJson(json);

@override final  int concentrationScore;

/// Create a copy of UpdateConcentrationScore
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$UpdateConcentrationScoreCopyWith<_UpdateConcentrationScore> get copyWith => __$UpdateConcentrationScoreCopyWithImpl<_UpdateConcentrationScore>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$UpdateConcentrationScoreToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _UpdateConcentrationScore&&(identical(other.concentrationScore, concentrationScore) || other.concentrationScore == concentrationScore));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,concentrationScore);

@override
String toString() {
  return 'UpdateConcentrationScore(concentrationScore: $concentrationScore)';
}


}

/// @nodoc
abstract mixin class _$UpdateConcentrationScoreCopyWith<$Res> implements $UpdateConcentrationScoreCopyWith<$Res> {
  factory _$UpdateConcentrationScoreCopyWith(_UpdateConcentrationScore value, $Res Function(_UpdateConcentrationScore) _then) = __$UpdateConcentrationScoreCopyWithImpl;
@override @useResult
$Res call({
 int concentrationScore
});




}
/// @nodoc
class __$UpdateConcentrationScoreCopyWithImpl<$Res>
    implements _$UpdateConcentrationScoreCopyWith<$Res> {
  __$UpdateConcentrationScoreCopyWithImpl(this._self, this._then);

  final _UpdateConcentrationScore _self;
  final $Res Function(_UpdateConcentrationScore) _then;

/// Create a copy of UpdateConcentrationScore
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? concentrationScore = null,}) {
  return _then(_UpdateConcentrationScore(
concentrationScore: null == concentrationScore ? _self.concentrationScore : concentrationScore // ignore: cast_nullable_to_non_nullable
as int,
  ));
}


}


/// @nodoc
mixin _$NoteUpdate {

 String get newNote;
/// Create a copy of NoteUpdate
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$NoteUpdateCopyWith<NoteUpdate> get copyWith => _$NoteUpdateCopyWithImpl<NoteUpdate>(this as NoteUpdate, _$identity);

  /// Serializes this NoteUpdate to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is NoteUpdate&&(identical(other.newNote, newNote) || other.newNote == newNote));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,newNote);

@override
String toString() {
  return 'NoteUpdate(newNote: $newNote)';
}


}

/// @nodoc
abstract mixin class $NoteUpdateCopyWith<$Res>  {
  factory $NoteUpdateCopyWith(NoteUpdate value, $Res Function(NoteUpdate) _then) = _$NoteUpdateCopyWithImpl;
@useResult
$Res call({
 String newNote
});




}
/// @nodoc
class _$NoteUpdateCopyWithImpl<$Res>
    implements $NoteUpdateCopyWith<$Res> {
  _$NoteUpdateCopyWithImpl(this._self, this._then);

  final NoteUpdate _self;
  final $Res Function(NoteUpdate) _then;

/// Create a copy of NoteUpdate
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? newNote = null,}) {
  return _then(_self.copyWith(
newNote: null == newNote ? _self.newNote : newNote // ignore: cast_nullable_to_non_nullable
as String,
  ));
}

}


/// Adds pattern-matching-related methods to [NoteUpdate].
extension NoteUpdatePatterns on NoteUpdate {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _NoteUpdate value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _NoteUpdate() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _NoteUpdate value)  $default,){
final _that = this;
switch (_that) {
case _NoteUpdate():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _NoteUpdate value)?  $default,){
final _that = this;
switch (_that) {
case _NoteUpdate() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( String newNote)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _NoteUpdate() when $default != null:
return $default(_that.newNote);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( String newNote)  $default,) {final _that = this;
switch (_that) {
case _NoteUpdate():
return $default(_that.newNote);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( String newNote)?  $default,) {final _that = this;
switch (_that) {
case _NoteUpdate() when $default != null:
return $default(_that.newNote);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _NoteUpdate implements NoteUpdate {
  const _NoteUpdate({required this.newNote});
  factory _NoteUpdate.fromJson(Map<String, dynamic> json) => _$NoteUpdateFromJson(json);

@override final  String newNote;

/// Create a copy of NoteUpdate
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$NoteUpdateCopyWith<_NoteUpdate> get copyWith => __$NoteUpdateCopyWithImpl<_NoteUpdate>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$NoteUpdateToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _NoteUpdate&&(identical(other.newNote, newNote) || other.newNote == newNote));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,newNote);

@override
String toString() {
  return 'NoteUpdate(newNote: $newNote)';
}


}

/// @nodoc
abstract mixin class _$NoteUpdateCopyWith<$Res> implements $NoteUpdateCopyWith<$Res> {
  factory _$NoteUpdateCopyWith(_NoteUpdate value, $Res Function(_NoteUpdate) _then) = __$NoteUpdateCopyWithImpl;
@override @useResult
$Res call({
 String newNote
});




}
/// @nodoc
class __$NoteUpdateCopyWithImpl<$Res>
    implements _$NoteUpdateCopyWith<$Res> {
  __$NoteUpdateCopyWithImpl(this._self, this._then);

  final _NoteUpdate _self;
  final $Res Function(_NoteUpdate) _then;

/// Create a copy of NoteUpdate
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? newNote = null,}) {
  return _then(_NoteUpdate(
newNote: null == newNote ? _self.newNote : newNote // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}


/// @nodoc
mixin _$UpdatePomodoroContext {

 String? get categoryId; String? get taskId;
/// Create a copy of UpdatePomodoroContext
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$UpdatePomodoroContextCopyWith<UpdatePomodoroContext> get copyWith => _$UpdatePomodoroContextCopyWithImpl<UpdatePomodoroContext>(this as UpdatePomodoroContext, _$identity);

  /// Serializes this UpdatePomodoroContext to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is UpdatePomodoroContext&&(identical(other.categoryId, categoryId) || other.categoryId == categoryId)&&(identical(other.taskId, taskId) || other.taskId == taskId));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,categoryId,taskId);

@override
String toString() {
  return 'UpdatePomodoroContext(categoryId: $categoryId, taskId: $taskId)';
}


}

/// @nodoc
abstract mixin class $UpdatePomodoroContextCopyWith<$Res>  {
  factory $UpdatePomodoroContextCopyWith(UpdatePomodoroContext value, $Res Function(UpdatePomodoroContext) _then) = _$UpdatePomodoroContextCopyWithImpl;
@useResult
$Res call({
 String? categoryId, String? taskId
});




}
/// @nodoc
class _$UpdatePomodoroContextCopyWithImpl<$Res>
    implements $UpdatePomodoroContextCopyWith<$Res> {
  _$UpdatePomodoroContextCopyWithImpl(this._self, this._then);

  final UpdatePomodoroContext _self;
  final $Res Function(UpdatePomodoroContext) _then;

/// Create a copy of UpdatePomodoroContext
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? categoryId = freezed,Object? taskId = freezed,}) {
  return _then(_self.copyWith(
categoryId: freezed == categoryId ? _self.categoryId : categoryId // ignore: cast_nullable_to_non_nullable
as String?,taskId: freezed == taskId ? _self.taskId : taskId // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}

}


/// Adds pattern-matching-related methods to [UpdatePomodoroContext].
extension UpdatePomodoroContextPatterns on UpdatePomodoroContext {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _UpdatePomodoroContext value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _UpdatePomodoroContext() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _UpdatePomodoroContext value)  $default,){
final _that = this;
switch (_that) {
case _UpdatePomodoroContext():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _UpdatePomodoroContext value)?  $default,){
final _that = this;
switch (_that) {
case _UpdatePomodoroContext() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( String? categoryId,  String? taskId)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _UpdatePomodoroContext() when $default != null:
return $default(_that.categoryId,_that.taskId);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( String? categoryId,  String? taskId)  $default,) {final _that = this;
switch (_that) {
case _UpdatePomodoroContext():
return $default(_that.categoryId,_that.taskId);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( String? categoryId,  String? taskId)?  $default,) {final _that = this;
switch (_that) {
case _UpdatePomodoroContext() when $default != null:
return $default(_that.categoryId,_that.taskId);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _UpdatePomodoroContext implements UpdatePomodoroContext {
  const _UpdatePomodoroContext({this.categoryId, this.taskId});
  factory _UpdatePomodoroContext.fromJson(Map<String, dynamic> json) => _$UpdatePomodoroContextFromJson(json);

@override final  String? categoryId;
@override final  String? taskId;

/// Create a copy of UpdatePomodoroContext
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$UpdatePomodoroContextCopyWith<_UpdatePomodoroContext> get copyWith => __$UpdatePomodoroContextCopyWithImpl<_UpdatePomodoroContext>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$UpdatePomodoroContextToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _UpdatePomodoroContext&&(identical(other.categoryId, categoryId) || other.categoryId == categoryId)&&(identical(other.taskId, taskId) || other.taskId == taskId));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,categoryId,taskId);

@override
String toString() {
  return 'UpdatePomodoroContext(categoryId: $categoryId, taskId: $taskId)';
}


}

/// @nodoc
abstract mixin class _$UpdatePomodoroContextCopyWith<$Res> implements $UpdatePomodoroContextCopyWith<$Res> {
  factory _$UpdatePomodoroContextCopyWith(_UpdatePomodoroContext value, $Res Function(_UpdatePomodoroContext) _then) = __$UpdatePomodoroContextCopyWithImpl;
@override @useResult
$Res call({
 String? categoryId, String? taskId
});




}
/// @nodoc
class __$UpdatePomodoroContextCopyWithImpl<$Res>
    implements _$UpdatePomodoroContextCopyWith<$Res> {
  __$UpdatePomodoroContextCopyWithImpl(this._self, this._then);

  final _UpdatePomodoroContext _self;
  final $Res Function(_UpdatePomodoroContext) _then;

/// Create a copy of UpdatePomodoroContext
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? categoryId = freezed,Object? taskId = freezed,}) {
  return _then(_UpdatePomodoroContext(
categoryId: freezed == categoryId ? _self.categoryId : categoryId // ignore: cast_nullable_to_non_nullable
as String?,taskId: freezed == taskId ? _self.taskId : taskId // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}


}


/// @nodoc
mixin _$UpdateWorkContext {

 String? get categoryId; String? get taskId;
/// Create a copy of UpdateWorkContext
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$UpdateWorkContextCopyWith<UpdateWorkContext> get copyWith => _$UpdateWorkContextCopyWithImpl<UpdateWorkContext>(this as UpdateWorkContext, _$identity);

  /// Serializes this UpdateWorkContext to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is UpdateWorkContext&&(identical(other.categoryId, categoryId) || other.categoryId == categoryId)&&(identical(other.taskId, taskId) || other.taskId == taskId));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,categoryId,taskId);

@override
String toString() {
  return 'UpdateWorkContext(categoryId: $categoryId, taskId: $taskId)';
}


}

/// @nodoc
abstract mixin class $UpdateWorkContextCopyWith<$Res>  {
  factory $UpdateWorkContextCopyWith(UpdateWorkContext value, $Res Function(UpdateWorkContext) _then) = _$UpdateWorkContextCopyWithImpl;
@useResult
$Res call({
 String? categoryId, String? taskId
});




}
/// @nodoc
class _$UpdateWorkContextCopyWithImpl<$Res>
    implements $UpdateWorkContextCopyWith<$Res> {
  _$UpdateWorkContextCopyWithImpl(this._self, this._then);

  final UpdateWorkContext _self;
  final $Res Function(UpdateWorkContext) _then;

/// Create a copy of UpdateWorkContext
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? categoryId = freezed,Object? taskId = freezed,}) {
  return _then(_self.copyWith(
categoryId: freezed == categoryId ? _self.categoryId : categoryId // ignore: cast_nullable_to_non_nullable
as String?,taskId: freezed == taskId ? _self.taskId : taskId // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}

}


/// Adds pattern-matching-related methods to [UpdateWorkContext].
extension UpdateWorkContextPatterns on UpdateWorkContext {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _UpdateWorkContext value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _UpdateWorkContext() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _UpdateWorkContext value)  $default,){
final _that = this;
switch (_that) {
case _UpdateWorkContext():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _UpdateWorkContext value)?  $default,){
final _that = this;
switch (_that) {
case _UpdateWorkContext() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( String? categoryId,  String? taskId)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _UpdateWorkContext() when $default != null:
return $default(_that.categoryId,_that.taskId);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( String? categoryId,  String? taskId)  $default,) {final _that = this;
switch (_that) {
case _UpdateWorkContext():
return $default(_that.categoryId,_that.taskId);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( String? categoryId,  String? taskId)?  $default,) {final _that = this;
switch (_that) {
case _UpdateWorkContext() when $default != null:
return $default(_that.categoryId,_that.taskId);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _UpdateWorkContext implements UpdateWorkContext {
  const _UpdateWorkContext({this.categoryId, this.taskId});
  factory _UpdateWorkContext.fromJson(Map<String, dynamic> json) => _$UpdateWorkContextFromJson(json);

@override final  String? categoryId;
@override final  String? taskId;

/// Create a copy of UpdateWorkContext
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$UpdateWorkContextCopyWith<_UpdateWorkContext> get copyWith => __$UpdateWorkContextCopyWithImpl<_UpdateWorkContext>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$UpdateWorkContextToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _UpdateWorkContext&&(identical(other.categoryId, categoryId) || other.categoryId == categoryId)&&(identical(other.taskId, taskId) || other.taskId == taskId));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,categoryId,taskId);

@override
String toString() {
  return 'UpdateWorkContext(categoryId: $categoryId, taskId: $taskId)';
}


}

/// @nodoc
abstract mixin class _$UpdateWorkContextCopyWith<$Res> implements $UpdateWorkContextCopyWith<$Res> {
  factory _$UpdateWorkContextCopyWith(_UpdateWorkContext value, $Res Function(_UpdateWorkContext) _then) = __$UpdateWorkContextCopyWithImpl;
@override @useResult
$Res call({
 String? categoryId, String? taskId
});




}
/// @nodoc
class __$UpdateWorkContextCopyWithImpl<$Res>
    implements _$UpdateWorkContextCopyWith<$Res> {
  __$UpdateWorkContextCopyWithImpl(this._self, this._then);

  final _UpdateWorkContext _self;
  final $Res Function(_UpdateWorkContext) _then;

/// Create a copy of UpdateWorkContext
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? categoryId = freezed,Object? taskId = freezed,}) {
  return _then(_UpdateWorkContext(
categoryId: freezed == categoryId ? _self.categoryId : categoryId // ignore: cast_nullable_to_non_nullable
as String?,taskId: freezed == taskId ? _self.taskId : taskId // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}


}


/// @nodoc
mixin _$UpdateCurrentSession {

 SessionTypeEnum get sessionType; int get sessionStartTime;// i64 maps to int in Dart
 String? get categoryId; String? get taskId; String? get note; int? get concentrationScore;
/// Create a copy of UpdateCurrentSession
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$UpdateCurrentSessionCopyWith<UpdateCurrentSession> get copyWith => _$UpdateCurrentSessionCopyWithImpl<UpdateCurrentSession>(this as UpdateCurrentSession, _$identity);

  /// Serializes this UpdateCurrentSession to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is UpdateCurrentSession&&(identical(other.sessionType, sessionType) || other.sessionType == sessionType)&&(identical(other.sessionStartTime, sessionStartTime) || other.sessionStartTime == sessionStartTime)&&(identical(other.categoryId, categoryId) || other.categoryId == categoryId)&&(identical(other.taskId, taskId) || other.taskId == taskId)&&(identical(other.note, note) || other.note == note)&&(identical(other.concentrationScore, concentrationScore) || other.concentrationScore == concentrationScore));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,sessionType,sessionStartTime,categoryId,taskId,note,concentrationScore);

@override
String toString() {
  return 'UpdateCurrentSession(sessionType: $sessionType, sessionStartTime: $sessionStartTime, categoryId: $categoryId, taskId: $taskId, note: $note, concentrationScore: $concentrationScore)';
}


}

/// @nodoc
abstract mixin class $UpdateCurrentSessionCopyWith<$Res>  {
  factory $UpdateCurrentSessionCopyWith(UpdateCurrentSession value, $Res Function(UpdateCurrentSession) _then) = _$UpdateCurrentSessionCopyWithImpl;
@useResult
$Res call({
 SessionTypeEnum sessionType, int sessionStartTime, String? categoryId, String? taskId, String? note, int? concentrationScore
});




}
/// @nodoc
class _$UpdateCurrentSessionCopyWithImpl<$Res>
    implements $UpdateCurrentSessionCopyWith<$Res> {
  _$UpdateCurrentSessionCopyWithImpl(this._self, this._then);

  final UpdateCurrentSession _self;
  final $Res Function(UpdateCurrentSession) _then;

/// Create a copy of UpdateCurrentSession
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? sessionType = null,Object? sessionStartTime = null,Object? categoryId = freezed,Object? taskId = freezed,Object? note = freezed,Object? concentrationScore = freezed,}) {
  return _then(_self.copyWith(
sessionType: null == sessionType ? _self.sessionType : sessionType // ignore: cast_nullable_to_non_nullable
as SessionTypeEnum,sessionStartTime: null == sessionStartTime ? _self.sessionStartTime : sessionStartTime // ignore: cast_nullable_to_non_nullable
as int,categoryId: freezed == categoryId ? _self.categoryId : categoryId // ignore: cast_nullable_to_non_nullable
as String?,taskId: freezed == taskId ? _self.taskId : taskId // ignore: cast_nullable_to_non_nullable
as String?,note: freezed == note ? _self.note : note // ignore: cast_nullable_to_non_nullable
as String?,concentrationScore: freezed == concentrationScore ? _self.concentrationScore : concentrationScore // ignore: cast_nullable_to_non_nullable
as int?,
  ));
}

}


/// Adds pattern-matching-related methods to [UpdateCurrentSession].
extension UpdateCurrentSessionPatterns on UpdateCurrentSession {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _UpdateCurrentSession value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _UpdateCurrentSession() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _UpdateCurrentSession value)  $default,){
final _that = this;
switch (_that) {
case _UpdateCurrentSession():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _UpdateCurrentSession value)?  $default,){
final _that = this;
switch (_that) {
case _UpdateCurrentSession() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( SessionTypeEnum sessionType,  int sessionStartTime,  String? categoryId,  String? taskId,  String? note,  int? concentrationScore)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _UpdateCurrentSession() when $default != null:
return $default(_that.sessionType,_that.sessionStartTime,_that.categoryId,_that.taskId,_that.note,_that.concentrationScore);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( SessionTypeEnum sessionType,  int sessionStartTime,  String? categoryId,  String? taskId,  String? note,  int? concentrationScore)  $default,) {final _that = this;
switch (_that) {
case _UpdateCurrentSession():
return $default(_that.sessionType,_that.sessionStartTime,_that.categoryId,_that.taskId,_that.note,_that.concentrationScore);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( SessionTypeEnum sessionType,  int sessionStartTime,  String? categoryId,  String? taskId,  String? note,  int? concentrationScore)?  $default,) {final _that = this;
switch (_that) {
case _UpdateCurrentSession() when $default != null:
return $default(_that.sessionType,_that.sessionStartTime,_that.categoryId,_that.taskId,_that.note,_that.concentrationScore);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _UpdateCurrentSession implements UpdateCurrentSession {
  const _UpdateCurrentSession({required this.sessionType, required this.sessionStartTime, this.categoryId, this.taskId, this.note, this.concentrationScore});
  factory _UpdateCurrentSession.fromJson(Map<String, dynamic> json) => _$UpdateCurrentSessionFromJson(json);

@override final  SessionTypeEnum sessionType;
@override final  int sessionStartTime;
// i64 maps to int in Dart
@override final  String? categoryId;
@override final  String? taskId;
@override final  String? note;
@override final  int? concentrationScore;

/// Create a copy of UpdateCurrentSession
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$UpdateCurrentSessionCopyWith<_UpdateCurrentSession> get copyWith => __$UpdateCurrentSessionCopyWithImpl<_UpdateCurrentSession>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$UpdateCurrentSessionToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _UpdateCurrentSession&&(identical(other.sessionType, sessionType) || other.sessionType == sessionType)&&(identical(other.sessionStartTime, sessionStartTime) || other.sessionStartTime == sessionStartTime)&&(identical(other.categoryId, categoryId) || other.categoryId == categoryId)&&(identical(other.taskId, taskId) || other.taskId == taskId)&&(identical(other.note, note) || other.note == note)&&(identical(other.concentrationScore, concentrationScore) || other.concentrationScore == concentrationScore));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,sessionType,sessionStartTime,categoryId,taskId,note,concentrationScore);

@override
String toString() {
  return 'UpdateCurrentSession(sessionType: $sessionType, sessionStartTime: $sessionStartTime, categoryId: $categoryId, taskId: $taskId, note: $note, concentrationScore: $concentrationScore)';
}


}

/// @nodoc
abstract mixin class _$UpdateCurrentSessionCopyWith<$Res> implements $UpdateCurrentSessionCopyWith<$Res> {
  factory _$UpdateCurrentSessionCopyWith(_UpdateCurrentSession value, $Res Function(_UpdateCurrentSession) _then) = __$UpdateCurrentSessionCopyWithImpl;
@override @useResult
$Res call({
 SessionTypeEnum sessionType, int sessionStartTime, String? categoryId, String? taskId, String? note, int? concentrationScore
});




}
/// @nodoc
class __$UpdateCurrentSessionCopyWithImpl<$Res>
    implements _$UpdateCurrentSessionCopyWith<$Res> {
  __$UpdateCurrentSessionCopyWithImpl(this._self, this._then);

  final _UpdateCurrentSession _self;
  final $Res Function(_UpdateCurrentSession) _then;

/// Create a copy of UpdateCurrentSession
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? sessionType = null,Object? sessionStartTime = null,Object? categoryId = freezed,Object? taskId = freezed,Object? note = freezed,Object? concentrationScore = freezed,}) {
  return _then(_UpdateCurrentSession(
sessionType: null == sessionType ? _self.sessionType : sessionType // ignore: cast_nullable_to_non_nullable
as SessionTypeEnum,sessionStartTime: null == sessionStartTime ? _self.sessionStartTime : sessionStartTime // ignore: cast_nullable_to_non_nullable
as int,categoryId: freezed == categoryId ? _self.categoryId : categoryId // ignore: cast_nullable_to_non_nullable
as String?,taskId: freezed == taskId ? _self.taskId : taskId // ignore: cast_nullable_to_non_nullable
as String?,note: freezed == note ? _self.note : note // ignore: cast_nullable_to_non_nullable
as String?,concentrationScore: freezed == concentrationScore ? _self.concentrationScore : concentrationScore // ignore: cast_nullable_to_non_nullable
as int?,
  ));
}


}


/// @nodoc
mixin _$UpdatePomodoroState {

 UpdateCurrentSession? get currentSession; UpdateWorkContext get workContext;
/// Create a copy of UpdatePomodoroState
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$UpdatePomodoroStateCopyWith<UpdatePomodoroState> get copyWith => _$UpdatePomodoroStateCopyWithImpl<UpdatePomodoroState>(this as UpdatePomodoroState, _$identity);

  /// Serializes this UpdatePomodoroState to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is UpdatePomodoroState&&(identical(other.currentSession, currentSession) || other.currentSession == currentSession)&&(identical(other.workContext, workContext) || other.workContext == workContext));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,currentSession,workContext);

@override
String toString() {
  return 'UpdatePomodoroState(currentSession: $currentSession, workContext: $workContext)';
}


}

/// @nodoc
abstract mixin class $UpdatePomodoroStateCopyWith<$Res>  {
  factory $UpdatePomodoroStateCopyWith(UpdatePomodoroState value, $Res Function(UpdatePomodoroState) _then) = _$UpdatePomodoroStateCopyWithImpl;
@useResult
$Res call({
 UpdateCurrentSession? currentSession, UpdateWorkContext workContext
});


$UpdateCurrentSessionCopyWith<$Res>? get currentSession;$UpdateWorkContextCopyWith<$Res> get workContext;

}
/// @nodoc
class _$UpdatePomodoroStateCopyWithImpl<$Res>
    implements $UpdatePomodoroStateCopyWith<$Res> {
  _$UpdatePomodoroStateCopyWithImpl(this._self, this._then);

  final UpdatePomodoroState _self;
  final $Res Function(UpdatePomodoroState) _then;

/// Create a copy of UpdatePomodoroState
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? currentSession = freezed,Object? workContext = null,}) {
  return _then(_self.copyWith(
currentSession: freezed == currentSession ? _self.currentSession : currentSession // ignore: cast_nullable_to_non_nullable
as UpdateCurrentSession?,workContext: null == workContext ? _self.workContext : workContext // ignore: cast_nullable_to_non_nullable
as UpdateWorkContext,
  ));
}
/// Create a copy of UpdatePomodoroState
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$UpdateCurrentSessionCopyWith<$Res>? get currentSession {
    if (_self.currentSession == null) {
    return null;
  }

  return $UpdateCurrentSessionCopyWith<$Res>(_self.currentSession!, (value) {
    return _then(_self.copyWith(currentSession: value));
  });
}/// Create a copy of UpdatePomodoroState
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$UpdateWorkContextCopyWith<$Res> get workContext {
  
  return $UpdateWorkContextCopyWith<$Res>(_self.workContext, (value) {
    return _then(_self.copyWith(workContext: value));
  });
}
}


/// Adds pattern-matching-related methods to [UpdatePomodoroState].
extension UpdatePomodoroStatePatterns on UpdatePomodoroState {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _UpdatePomodoroState value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _UpdatePomodoroState() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _UpdatePomodoroState value)  $default,){
final _that = this;
switch (_that) {
case _UpdatePomodoroState():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _UpdatePomodoroState value)?  $default,){
final _that = this;
switch (_that) {
case _UpdatePomodoroState() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( UpdateCurrentSession? currentSession,  UpdateWorkContext workContext)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _UpdatePomodoroState() when $default != null:
return $default(_that.currentSession,_that.workContext);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( UpdateCurrentSession? currentSession,  UpdateWorkContext workContext)  $default,) {final _that = this;
switch (_that) {
case _UpdatePomodoroState():
return $default(_that.currentSession,_that.workContext);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( UpdateCurrentSession? currentSession,  UpdateWorkContext workContext)?  $default,) {final _that = this;
switch (_that) {
case _UpdatePomodoroState() when $default != null:
return $default(_that.currentSession,_that.workContext);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _UpdatePomodoroState implements UpdatePomodoroState {
  const _UpdatePomodoroState({this.currentSession, required this.workContext});
  factory _UpdatePomodoroState.fromJson(Map<String, dynamic> json) => _$UpdatePomodoroStateFromJson(json);

@override final  UpdateCurrentSession? currentSession;
@override final  UpdateWorkContext workContext;

/// Create a copy of UpdatePomodoroState
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$UpdatePomodoroStateCopyWith<_UpdatePomodoroState> get copyWith => __$UpdatePomodoroStateCopyWithImpl<_UpdatePomodoroState>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$UpdatePomodoroStateToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _UpdatePomodoroState&&(identical(other.currentSession, currentSession) || other.currentSession == currentSession)&&(identical(other.workContext, workContext) || other.workContext == workContext));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,currentSession,workContext);

@override
String toString() {
  return 'UpdatePomodoroState(currentSession: $currentSession, workContext: $workContext)';
}


}

/// @nodoc
abstract mixin class _$UpdatePomodoroStateCopyWith<$Res> implements $UpdatePomodoroStateCopyWith<$Res> {
  factory _$UpdatePomodoroStateCopyWith(_UpdatePomodoroState value, $Res Function(_UpdatePomodoroState) _then) = __$UpdatePomodoroStateCopyWithImpl;
@override @useResult
$Res call({
 UpdateCurrentSession? currentSession, UpdateWorkContext workContext
});


@override $UpdateCurrentSessionCopyWith<$Res>? get currentSession;@override $UpdateWorkContextCopyWith<$Res> get workContext;

}
/// @nodoc
class __$UpdatePomodoroStateCopyWithImpl<$Res>
    implements _$UpdatePomodoroStateCopyWith<$Res> {
  __$UpdatePomodoroStateCopyWithImpl(this._self, this._then);

  final _UpdatePomodoroState _self;
  final $Res Function(_UpdatePomodoroState) _then;

/// Create a copy of UpdatePomodoroState
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? currentSession = freezed,Object? workContext = null,}) {
  return _then(_UpdatePomodoroState(
currentSession: freezed == currentSession ? _self.currentSession : currentSession // ignore: cast_nullable_to_non_nullable
as UpdateCurrentSession?,workContext: null == workContext ? _self.workContext : workContext // ignore: cast_nullable_to_non_nullable
as UpdateWorkContext,
  ));
}

/// Create a copy of UpdatePomodoroState
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$UpdateCurrentSessionCopyWith<$Res>? get currentSession {
    if (_self.currentSession == null) {
    return null;
  }

  return $UpdateCurrentSessionCopyWith<$Res>(_self.currentSession!, (value) {
    return _then(_self.copyWith(currentSession: value));
  });
}/// Create a copy of UpdatePomodoroState
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$UpdateWorkContextCopyWith<$Res> get workContext {
  
  return $UpdateWorkContextCopyWith<$Res>(_self.workContext, (value) {
    return _then(_self.copyWith(workContext: value));
  });
}
}


/// @nodoc
mixin _$WsClientRequest {

 String? get requestId; ClientMessage get message;
/// Create a copy of WsClientRequest
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$WsClientRequestCopyWith<WsClientRequest> get copyWith => _$WsClientRequestCopyWithImpl<WsClientRequest>(this as WsClientRequest, _$identity);

  /// Serializes this WsClientRequest to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is WsClientRequest&&(identical(other.requestId, requestId) || other.requestId == requestId)&&(identical(other.message, message) || other.message == message));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,requestId,message);

@override
String toString() {
  return 'WsClientRequest(requestId: $requestId, message: $message)';
}


}

/// @nodoc
abstract mixin class $WsClientRequestCopyWith<$Res>  {
  factory $WsClientRequestCopyWith(WsClientRequest value, $Res Function(WsClientRequest) _then) = _$WsClientRequestCopyWithImpl;
@useResult
$Res call({
 String? requestId, ClientMessage message
});


$ClientMessageCopyWith<$Res> get message;

}
/// @nodoc
class _$WsClientRequestCopyWithImpl<$Res>
    implements $WsClientRequestCopyWith<$Res> {
  _$WsClientRequestCopyWithImpl(this._self, this._then);

  final WsClientRequest _self;
  final $Res Function(WsClientRequest) _then;

/// Create a copy of WsClientRequest
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? requestId = freezed,Object? message = null,}) {
  return _then(_self.copyWith(
requestId: freezed == requestId ? _self.requestId : requestId // ignore: cast_nullable_to_non_nullable
as String?,message: null == message ? _self.message : message // ignore: cast_nullable_to_non_nullable
as ClientMessage,
  ));
}
/// Create a copy of WsClientRequest
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$ClientMessageCopyWith<$Res> get message {
  
  return $ClientMessageCopyWith<$Res>(_self.message, (value) {
    return _then(_self.copyWith(message: value));
  });
}
}


/// Adds pattern-matching-related methods to [WsClientRequest].
extension WsClientRequestPatterns on WsClientRequest {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _WsClientRequest value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _WsClientRequest() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _WsClientRequest value)  $default,){
final _that = this;
switch (_that) {
case _WsClientRequest():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _WsClientRequest value)?  $default,){
final _that = this;
switch (_that) {
case _WsClientRequest() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( String? requestId,  ClientMessage message)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _WsClientRequest() when $default != null:
return $default(_that.requestId,_that.message);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( String? requestId,  ClientMessage message)  $default,) {final _that = this;
switch (_that) {
case _WsClientRequest():
return $default(_that.requestId,_that.message);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( String? requestId,  ClientMessage message)?  $default,) {final _that = this;
switch (_that) {
case _WsClientRequest() when $default != null:
return $default(_that.requestId,_that.message);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _WsClientRequest implements WsClientRequest {
  const _WsClientRequest({this.requestId, required this.message});
  factory _WsClientRequest.fromJson(Map<String, dynamic> json) => _$WsClientRequestFromJson(json);

@override final  String? requestId;
@override final  ClientMessage message;

/// Create a copy of WsClientRequest
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$WsClientRequestCopyWith<_WsClientRequest> get copyWith => __$WsClientRequestCopyWithImpl<_WsClientRequest>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$WsClientRequestToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _WsClientRequest&&(identical(other.requestId, requestId) || other.requestId == requestId)&&(identical(other.message, message) || other.message == message));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,requestId,message);

@override
String toString() {
  return 'WsClientRequest(requestId: $requestId, message: $message)';
}


}

/// @nodoc
abstract mixin class _$WsClientRequestCopyWith<$Res> implements $WsClientRequestCopyWith<$Res> {
  factory _$WsClientRequestCopyWith(_WsClientRequest value, $Res Function(_WsClientRequest) _then) = __$WsClientRequestCopyWithImpl;
@override @useResult
$Res call({
 String? requestId, ClientMessage message
});


@override $ClientMessageCopyWith<$Res> get message;

}
/// @nodoc
class __$WsClientRequestCopyWithImpl<$Res>
    implements _$WsClientRequestCopyWith<$Res> {
  __$WsClientRequestCopyWithImpl(this._self, this._then);

  final _WsClientRequest _self;
  final $Res Function(_WsClientRequest) _then;

/// Create a copy of WsClientRequest
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? requestId = freezed,Object? message = null,}) {
  return _then(_WsClientRequest(
requestId: freezed == requestId ? _self.requestId : requestId // ignore: cast_nullable_to_non_nullable
as String?,message: null == message ? _self.message : message // ignore: cast_nullable_to_non_nullable
as ClientMessage,
  ));
}

/// Create a copy of WsClientRequest
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$ClientMessageCopyWith<$Res> get message {
  
  return $ClientMessageCopyWith<$Res>(_self.message, (value) {
    return _then(_self.copyWith(message: value));
  });
}
}

ClientMessage _$ClientMessageFromJson(
  Map<String, dynamic> json
) {
        switch (json['type']) {
                  case 'requestSync':
          return ClientMessageRequestSync.fromJson(
            json
          );
                case 'startEvent':
          return ClientMessageStartEvent.fromJson(
            json
          );
                case 'breakEvent':
          return ClientMessageBreakEvent.fromJson(
            json
          );
                case 'terminateEvent':
          return ClientMessageTerminateEvent.fromJson(
            json
          );
                case 'updatePomodoroContext':
          return ClientMessageUpdatePomodoroContext.fromJson(
            json
          );
                case 'updateNote':
          return ClientMessageUpdateNote.fromJson(
            json
          );
                case 'updateConcentrationScore':
          return ClientMessageUpdateConcentrationScore.fromJson(
            json
          );
        
          default:
            throw CheckedFromJsonException(
  json,
  'type',
  'ClientMessage',
  'Invalid union type "${json['type']}"!'
);
        }
      
}

/// @nodoc
mixin _$ClientMessage {



  /// Serializes this ClientMessage to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ClientMessage);
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'ClientMessage()';
}


}

/// @nodoc
class $ClientMessageCopyWith<$Res>  {
$ClientMessageCopyWith(ClientMessage _, $Res Function(ClientMessage) __);
}


/// Adds pattern-matching-related methods to [ClientMessage].
extension ClientMessagePatterns on ClientMessage {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( ClientMessageRequestSync value)?  requestSync,TResult Function( ClientMessageStartEvent value)?  startEvent,TResult Function( ClientMessageBreakEvent value)?  breakEvent,TResult Function( ClientMessageTerminateEvent value)?  terminateEvent,TResult Function( ClientMessageUpdatePomodoroContext value)?  updatePomodoroContext,TResult Function( ClientMessageUpdateNote value)?  updateNote,TResult Function( ClientMessageUpdateConcentrationScore value)?  updateConcentrationScore,required TResult orElse(),}){
final _that = this;
switch (_that) {
case ClientMessageRequestSync() when requestSync != null:
return requestSync(_that);case ClientMessageStartEvent() when startEvent != null:
return startEvent(_that);case ClientMessageBreakEvent() when breakEvent != null:
return breakEvent(_that);case ClientMessageTerminateEvent() when terminateEvent != null:
return terminateEvent(_that);case ClientMessageUpdatePomodoroContext() when updatePomodoroContext != null:
return updatePomodoroContext(_that);case ClientMessageUpdateNote() when updateNote != null:
return updateNote(_that);case ClientMessageUpdateConcentrationScore() when updateConcentrationScore != null:
return updateConcentrationScore(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( ClientMessageRequestSync value)  requestSync,required TResult Function( ClientMessageStartEvent value)  startEvent,required TResult Function( ClientMessageBreakEvent value)  breakEvent,required TResult Function( ClientMessageTerminateEvent value)  terminateEvent,required TResult Function( ClientMessageUpdatePomodoroContext value)  updatePomodoroContext,required TResult Function( ClientMessageUpdateNote value)  updateNote,required TResult Function( ClientMessageUpdateConcentrationScore value)  updateConcentrationScore,}){
final _that = this;
switch (_that) {
case ClientMessageRequestSync():
return requestSync(_that);case ClientMessageStartEvent():
return startEvent(_that);case ClientMessageBreakEvent():
return breakEvent(_that);case ClientMessageTerminateEvent():
return terminateEvent(_that);case ClientMessageUpdatePomodoroContext():
return updatePomodoroContext(_that);case ClientMessageUpdateNote():
return updateNote(_that);case ClientMessageUpdateConcentrationScore():
return updateConcentrationScore(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( ClientMessageRequestSync value)?  requestSync,TResult? Function( ClientMessageStartEvent value)?  startEvent,TResult? Function( ClientMessageBreakEvent value)?  breakEvent,TResult? Function( ClientMessageTerminateEvent value)?  terminateEvent,TResult? Function( ClientMessageUpdatePomodoroContext value)?  updatePomodoroContext,TResult? Function( ClientMessageUpdateNote value)?  updateNote,TResult? Function( ClientMessageUpdateConcentrationScore value)?  updateConcentrationScore,}){
final _that = this;
switch (_that) {
case ClientMessageRequestSync() when requestSync != null:
return requestSync(_that);case ClientMessageStartEvent() when startEvent != null:
return startEvent(_that);case ClientMessageBreakEvent() when breakEvent != null:
return breakEvent(_that);case ClientMessageTerminateEvent() when terminateEvent != null:
return terminateEvent(_that);case ClientMessageUpdatePomodoroContext() when updatePomodoroContext != null:
return updatePomodoroContext(_that);case ClientMessageUpdateNote() when updateNote != null:
return updateNote(_that);case ClientMessageUpdateConcentrationScore() when updateConcentrationScore != null:
return updateConcentrationScore(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function()?  requestSync,TResult Function()?  startEvent,TResult Function()?  breakEvent,TResult Function()?  terminateEvent,TResult Function( UpdatePomodoroContext payload)?  updatePomodoroContext,TResult Function( NoteUpdate payload)?  updateNote,TResult Function( UpdateConcentrationScore payload)?  updateConcentrationScore,required TResult orElse(),}) {final _that = this;
switch (_that) {
case ClientMessageRequestSync() when requestSync != null:
return requestSync();case ClientMessageStartEvent() when startEvent != null:
return startEvent();case ClientMessageBreakEvent() when breakEvent != null:
return breakEvent();case ClientMessageTerminateEvent() when terminateEvent != null:
return terminateEvent();case ClientMessageUpdatePomodoroContext() when updatePomodoroContext != null:
return updatePomodoroContext(_that.payload);case ClientMessageUpdateNote() when updateNote != null:
return updateNote(_that.payload);case ClientMessageUpdateConcentrationScore() when updateConcentrationScore != null:
return updateConcentrationScore(_that.payload);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function()  requestSync,required TResult Function()  startEvent,required TResult Function()  breakEvent,required TResult Function()  terminateEvent,required TResult Function( UpdatePomodoroContext payload)  updatePomodoroContext,required TResult Function( NoteUpdate payload)  updateNote,required TResult Function( UpdateConcentrationScore payload)  updateConcentrationScore,}) {final _that = this;
switch (_that) {
case ClientMessageRequestSync():
return requestSync();case ClientMessageStartEvent():
return startEvent();case ClientMessageBreakEvent():
return breakEvent();case ClientMessageTerminateEvent():
return terminateEvent();case ClientMessageUpdatePomodoroContext():
return updatePomodoroContext(_that.payload);case ClientMessageUpdateNote():
return updateNote(_that.payload);case ClientMessageUpdateConcentrationScore():
return updateConcentrationScore(_that.payload);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function()?  requestSync,TResult? Function()?  startEvent,TResult? Function()?  breakEvent,TResult? Function()?  terminateEvent,TResult? Function( UpdatePomodoroContext payload)?  updatePomodoroContext,TResult? Function( NoteUpdate payload)?  updateNote,TResult? Function( UpdateConcentrationScore payload)?  updateConcentrationScore,}) {final _that = this;
switch (_that) {
case ClientMessageRequestSync() when requestSync != null:
return requestSync();case ClientMessageStartEvent() when startEvent != null:
return startEvent();case ClientMessageBreakEvent() when breakEvent != null:
return breakEvent();case ClientMessageTerminateEvent() when terminateEvent != null:
return terminateEvent();case ClientMessageUpdatePomodoroContext() when updatePomodoroContext != null:
return updatePomodoroContext(_that.payload);case ClientMessageUpdateNote() when updateNote != null:
return updateNote(_that.payload);case ClientMessageUpdateConcentrationScore() when updateConcentrationScore != null:
return updateConcentrationScore(_that.payload);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class ClientMessageRequestSync implements ClientMessage {
  const ClientMessageRequestSync({final  String? $type}): $type = $type ?? 'requestSync';
  factory ClientMessageRequestSync.fromJson(Map<String, dynamic> json) => _$ClientMessageRequestSyncFromJson(json);



@JsonKey(name: 'type')
final String $type;



@override
Map<String, dynamic> toJson() {
  return _$ClientMessageRequestSyncToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ClientMessageRequestSync);
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'ClientMessage.requestSync()';
}


}




/// @nodoc
@JsonSerializable()

class ClientMessageStartEvent implements ClientMessage {
  const ClientMessageStartEvent({final  String? $type}): $type = $type ?? 'startEvent';
  factory ClientMessageStartEvent.fromJson(Map<String, dynamic> json) => _$ClientMessageStartEventFromJson(json);



@JsonKey(name: 'type')
final String $type;



@override
Map<String, dynamic> toJson() {
  return _$ClientMessageStartEventToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ClientMessageStartEvent);
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'ClientMessage.startEvent()';
}


}




/// @nodoc
@JsonSerializable()

class ClientMessageBreakEvent implements ClientMessage {
  const ClientMessageBreakEvent({final  String? $type}): $type = $type ?? 'breakEvent';
  factory ClientMessageBreakEvent.fromJson(Map<String, dynamic> json) => _$ClientMessageBreakEventFromJson(json);



@JsonKey(name: 'type')
final String $type;



@override
Map<String, dynamic> toJson() {
  return _$ClientMessageBreakEventToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ClientMessageBreakEvent);
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'ClientMessage.breakEvent()';
}


}




/// @nodoc
@JsonSerializable()

class ClientMessageTerminateEvent implements ClientMessage {
  const ClientMessageTerminateEvent({final  String? $type}): $type = $type ?? 'terminateEvent';
  factory ClientMessageTerminateEvent.fromJson(Map<String, dynamic> json) => _$ClientMessageTerminateEventFromJson(json);



@JsonKey(name: 'type')
final String $type;



@override
Map<String, dynamic> toJson() {
  return _$ClientMessageTerminateEventToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ClientMessageTerminateEvent);
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'ClientMessage.terminateEvent()';
}


}




/// @nodoc
@JsonSerializable()

class ClientMessageUpdatePomodoroContext implements ClientMessage {
  const ClientMessageUpdatePomodoroContext(this.payload, {final  String? $type}): $type = $type ?? 'updatePomodoroContext';
  factory ClientMessageUpdatePomodoroContext.fromJson(Map<String, dynamic> json) => _$ClientMessageUpdatePomodoroContextFromJson(json);

 final  UpdatePomodoroContext payload;

@JsonKey(name: 'type')
final String $type;


/// Create a copy of ClientMessage
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ClientMessageUpdatePomodoroContextCopyWith<ClientMessageUpdatePomodoroContext> get copyWith => _$ClientMessageUpdatePomodoroContextCopyWithImpl<ClientMessageUpdatePomodoroContext>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$ClientMessageUpdatePomodoroContextToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ClientMessageUpdatePomodoroContext&&(identical(other.payload, payload) || other.payload == payload));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,payload);

@override
String toString() {
  return 'ClientMessage.updatePomodoroContext(payload: $payload)';
}


}

/// @nodoc
abstract mixin class $ClientMessageUpdatePomodoroContextCopyWith<$Res> implements $ClientMessageCopyWith<$Res> {
  factory $ClientMessageUpdatePomodoroContextCopyWith(ClientMessageUpdatePomodoroContext value, $Res Function(ClientMessageUpdatePomodoroContext) _then) = _$ClientMessageUpdatePomodoroContextCopyWithImpl;
@useResult
$Res call({
 UpdatePomodoroContext payload
});


$UpdatePomodoroContextCopyWith<$Res> get payload;

}
/// @nodoc
class _$ClientMessageUpdatePomodoroContextCopyWithImpl<$Res>
    implements $ClientMessageUpdatePomodoroContextCopyWith<$Res> {
  _$ClientMessageUpdatePomodoroContextCopyWithImpl(this._self, this._then);

  final ClientMessageUpdatePomodoroContext _self;
  final $Res Function(ClientMessageUpdatePomodoroContext) _then;

/// Create a copy of ClientMessage
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? payload = null,}) {
  return _then(ClientMessageUpdatePomodoroContext(
null == payload ? _self.payload : payload // ignore: cast_nullable_to_non_nullable
as UpdatePomodoroContext,
  ));
}

/// Create a copy of ClientMessage
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$UpdatePomodoroContextCopyWith<$Res> get payload {
  
  return $UpdatePomodoroContextCopyWith<$Res>(_self.payload, (value) {
    return _then(_self.copyWith(payload: value));
  });
}
}

/// @nodoc
@JsonSerializable()

class ClientMessageUpdateNote implements ClientMessage {
  const ClientMessageUpdateNote(this.payload, {final  String? $type}): $type = $type ?? 'updateNote';
  factory ClientMessageUpdateNote.fromJson(Map<String, dynamic> json) => _$ClientMessageUpdateNoteFromJson(json);

 final  NoteUpdate payload;

@JsonKey(name: 'type')
final String $type;


/// Create a copy of ClientMessage
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ClientMessageUpdateNoteCopyWith<ClientMessageUpdateNote> get copyWith => _$ClientMessageUpdateNoteCopyWithImpl<ClientMessageUpdateNote>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$ClientMessageUpdateNoteToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ClientMessageUpdateNote&&(identical(other.payload, payload) || other.payload == payload));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,payload);

@override
String toString() {
  return 'ClientMessage.updateNote(payload: $payload)';
}


}

/// @nodoc
abstract mixin class $ClientMessageUpdateNoteCopyWith<$Res> implements $ClientMessageCopyWith<$Res> {
  factory $ClientMessageUpdateNoteCopyWith(ClientMessageUpdateNote value, $Res Function(ClientMessageUpdateNote) _then) = _$ClientMessageUpdateNoteCopyWithImpl;
@useResult
$Res call({
 NoteUpdate payload
});


$NoteUpdateCopyWith<$Res> get payload;

}
/// @nodoc
class _$ClientMessageUpdateNoteCopyWithImpl<$Res>
    implements $ClientMessageUpdateNoteCopyWith<$Res> {
  _$ClientMessageUpdateNoteCopyWithImpl(this._self, this._then);

  final ClientMessageUpdateNote _self;
  final $Res Function(ClientMessageUpdateNote) _then;

/// Create a copy of ClientMessage
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? payload = null,}) {
  return _then(ClientMessageUpdateNote(
null == payload ? _self.payload : payload // ignore: cast_nullable_to_non_nullable
as NoteUpdate,
  ));
}

/// Create a copy of ClientMessage
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$NoteUpdateCopyWith<$Res> get payload {
  
  return $NoteUpdateCopyWith<$Res>(_self.payload, (value) {
    return _then(_self.copyWith(payload: value));
  });
}
}

/// @nodoc
@JsonSerializable()

class ClientMessageUpdateConcentrationScore implements ClientMessage {
  const ClientMessageUpdateConcentrationScore(this.payload, {final  String? $type}): $type = $type ?? 'updateConcentrationScore';
  factory ClientMessageUpdateConcentrationScore.fromJson(Map<String, dynamic> json) => _$ClientMessageUpdateConcentrationScoreFromJson(json);

 final  UpdateConcentrationScore payload;

@JsonKey(name: 'type')
final String $type;


/// Create a copy of ClientMessage
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ClientMessageUpdateConcentrationScoreCopyWith<ClientMessageUpdateConcentrationScore> get copyWith => _$ClientMessageUpdateConcentrationScoreCopyWithImpl<ClientMessageUpdateConcentrationScore>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$ClientMessageUpdateConcentrationScoreToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ClientMessageUpdateConcentrationScore&&(identical(other.payload, payload) || other.payload == payload));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,payload);

@override
String toString() {
  return 'ClientMessage.updateConcentrationScore(payload: $payload)';
}


}

/// @nodoc
abstract mixin class $ClientMessageUpdateConcentrationScoreCopyWith<$Res> implements $ClientMessageCopyWith<$Res> {
  factory $ClientMessageUpdateConcentrationScoreCopyWith(ClientMessageUpdateConcentrationScore value, $Res Function(ClientMessageUpdateConcentrationScore) _then) = _$ClientMessageUpdateConcentrationScoreCopyWithImpl;
@useResult
$Res call({
 UpdateConcentrationScore payload
});


$UpdateConcentrationScoreCopyWith<$Res> get payload;

}
/// @nodoc
class _$ClientMessageUpdateConcentrationScoreCopyWithImpl<$Res>
    implements $ClientMessageUpdateConcentrationScoreCopyWith<$Res> {
  _$ClientMessageUpdateConcentrationScoreCopyWithImpl(this._self, this._then);

  final ClientMessageUpdateConcentrationScore _self;
  final $Res Function(ClientMessageUpdateConcentrationScore) _then;

/// Create a copy of ClientMessage
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? payload = null,}) {
  return _then(ClientMessageUpdateConcentrationScore(
null == payload ? _self.payload : payload // ignore: cast_nullable_to_non_nullable
as UpdateConcentrationScore,
  ));
}

/// Create a copy of ClientMessage
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$UpdateConcentrationScoreCopyWith<$Res> get payload {
  
  return $UpdateConcentrationScoreCopyWith<$Res>(_self.payload, (value) {
    return _then(_self.copyWith(payload: value));
  });
}
}

ServerResponse _$ServerResponseFromJson(
  Map<String, dynamic> json
) {
        switch (json['type']) {
                  case 'success':
          return ServerResponseSuccess.fromJson(
            json
          );
                case 'error':
          return ServerResponseError.fromJson(
            json
          );
                case 'syncData':
          return ServerResponseSyncData.fromJson(
            json
          );
        
          default:
            throw CheckedFromJsonException(
  json,
  'type',
  'ServerResponse',
  'Invalid union type "${json['type']}"!'
);
        }
      
}

/// @nodoc
mixin _$ServerResponse {



  /// Serializes this ServerResponse to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ServerResponse);
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'ServerResponse()';
}


}

/// @nodoc
class $ServerResponseCopyWith<$Res>  {
$ServerResponseCopyWith(ServerResponse _, $Res Function(ServerResponse) __);
}


/// Adds pattern-matching-related methods to [ServerResponse].
extension ServerResponsePatterns on ServerResponse {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( ServerResponseSuccess value)?  success,TResult Function( ServerResponseError value)?  error,TResult Function( ServerResponseSyncData value)?  syncData,required TResult orElse(),}){
final _that = this;
switch (_that) {
case ServerResponseSuccess() when success != null:
return success(_that);case ServerResponseError() when error != null:
return error(_that);case ServerResponseSyncData() when syncData != null:
return syncData(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( ServerResponseSuccess value)  success,required TResult Function( ServerResponseError value)  error,required TResult Function( ServerResponseSyncData value)  syncData,}){
final _that = this;
switch (_that) {
case ServerResponseSuccess():
return success(_that);case ServerResponseError():
return error(_that);case ServerResponseSyncData():
return syncData(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( ServerResponseSuccess value)?  success,TResult? Function( ServerResponseError value)?  error,TResult? Function( ServerResponseSyncData value)?  syncData,}){
final _that = this;
switch (_that) {
case ServerResponseSuccess() when success != null:
return success(_that);case ServerResponseError() when error != null:
return error(_that);case ServerResponseSyncData() when syncData != null:
return syncData(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( String message,  String? requestId)?  success,TResult Function( String code,  String message,  String? requestId)?  error,TResult Function( UpdatePomodoroState payload)?  syncData,required TResult orElse(),}) {final _that = this;
switch (_that) {
case ServerResponseSuccess() when success != null:
return success(_that.message,_that.requestId);case ServerResponseError() when error != null:
return error(_that.code,_that.message,_that.requestId);case ServerResponseSyncData() when syncData != null:
return syncData(_that.payload);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( String message,  String? requestId)  success,required TResult Function( String code,  String message,  String? requestId)  error,required TResult Function( UpdatePomodoroState payload)  syncData,}) {final _that = this;
switch (_that) {
case ServerResponseSuccess():
return success(_that.message,_that.requestId);case ServerResponseError():
return error(_that.code,_that.message,_that.requestId);case ServerResponseSyncData():
return syncData(_that.payload);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( String message,  String? requestId)?  success,TResult? Function( String code,  String message,  String? requestId)?  error,TResult? Function( UpdatePomodoroState payload)?  syncData,}) {final _that = this;
switch (_that) {
case ServerResponseSuccess() when success != null:
return success(_that.message,_that.requestId);case ServerResponseError() when error != null:
return error(_that.code,_that.message,_that.requestId);case ServerResponseSyncData() when syncData != null:
return syncData(_that.payload);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class ServerResponseSuccess implements ServerResponse {
  const ServerResponseSuccess({required this.message, this.requestId, final  String? $type}): $type = $type ?? 'success';
  factory ServerResponseSuccess.fromJson(Map<String, dynamic> json) => _$ServerResponseSuccessFromJson(json);

 final  String message;
 final  String? requestId;

@JsonKey(name: 'type')
final String $type;


/// Create a copy of ServerResponse
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ServerResponseSuccessCopyWith<ServerResponseSuccess> get copyWith => _$ServerResponseSuccessCopyWithImpl<ServerResponseSuccess>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$ServerResponseSuccessToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ServerResponseSuccess&&(identical(other.message, message) || other.message == message)&&(identical(other.requestId, requestId) || other.requestId == requestId));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,message,requestId);

@override
String toString() {
  return 'ServerResponse.success(message: $message, requestId: $requestId)';
}


}

/// @nodoc
abstract mixin class $ServerResponseSuccessCopyWith<$Res> implements $ServerResponseCopyWith<$Res> {
  factory $ServerResponseSuccessCopyWith(ServerResponseSuccess value, $Res Function(ServerResponseSuccess) _then) = _$ServerResponseSuccessCopyWithImpl;
@useResult
$Res call({
 String message, String? requestId
});




}
/// @nodoc
class _$ServerResponseSuccessCopyWithImpl<$Res>
    implements $ServerResponseSuccessCopyWith<$Res> {
  _$ServerResponseSuccessCopyWithImpl(this._self, this._then);

  final ServerResponseSuccess _self;
  final $Res Function(ServerResponseSuccess) _then;

/// Create a copy of ServerResponse
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? message = null,Object? requestId = freezed,}) {
  return _then(ServerResponseSuccess(
message: null == message ? _self.message : message // ignore: cast_nullable_to_non_nullable
as String,requestId: freezed == requestId ? _self.requestId : requestId // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}


}

/// @nodoc
@JsonSerializable()

class ServerResponseError implements ServerResponse {
  const ServerResponseError({required this.code, required this.message, this.requestId, final  String? $type}): $type = $type ?? 'error';
  factory ServerResponseError.fromJson(Map<String, dynamic> json) => _$ServerResponseErrorFromJson(json);

 final  String code;
 final  String message;
 final  String? requestId;

@JsonKey(name: 'type')
final String $type;


/// Create a copy of ServerResponse
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ServerResponseErrorCopyWith<ServerResponseError> get copyWith => _$ServerResponseErrorCopyWithImpl<ServerResponseError>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$ServerResponseErrorToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ServerResponseError&&(identical(other.code, code) || other.code == code)&&(identical(other.message, message) || other.message == message)&&(identical(other.requestId, requestId) || other.requestId == requestId));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,code,message,requestId);

@override
String toString() {
  return 'ServerResponse.error(code: $code, message: $message, requestId: $requestId)';
}


}

/// @nodoc
abstract mixin class $ServerResponseErrorCopyWith<$Res> implements $ServerResponseCopyWith<$Res> {
  factory $ServerResponseErrorCopyWith(ServerResponseError value, $Res Function(ServerResponseError) _then) = _$ServerResponseErrorCopyWithImpl;
@useResult
$Res call({
 String code, String message, String? requestId
});




}
/// @nodoc
class _$ServerResponseErrorCopyWithImpl<$Res>
    implements $ServerResponseErrorCopyWith<$Res> {
  _$ServerResponseErrorCopyWithImpl(this._self, this._then);

  final ServerResponseError _self;
  final $Res Function(ServerResponseError) _then;

/// Create a copy of ServerResponse
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? code = null,Object? message = null,Object? requestId = freezed,}) {
  return _then(ServerResponseError(
code: null == code ? _self.code : code // ignore: cast_nullable_to_non_nullable
as String,message: null == message ? _self.message : message // ignore: cast_nullable_to_non_nullable
as String,requestId: freezed == requestId ? _self.requestId : requestId // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}


}

/// @nodoc
@JsonSerializable()

class ServerResponseSyncData implements ServerResponse {
  const ServerResponseSyncData(this.payload, {final  String? $type}): $type = $type ?? 'syncData';
  factory ServerResponseSyncData.fromJson(Map<String, dynamic> json) => _$ServerResponseSyncDataFromJson(json);

 final  UpdatePomodoroState payload;

@JsonKey(name: 'type')
final String $type;


/// Create a copy of ServerResponse
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ServerResponseSyncDataCopyWith<ServerResponseSyncData> get copyWith => _$ServerResponseSyncDataCopyWithImpl<ServerResponseSyncData>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$ServerResponseSyncDataToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ServerResponseSyncData&&(identical(other.payload, payload) || other.payload == payload));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,payload);

@override
String toString() {
  return 'ServerResponse.syncData(payload: $payload)';
}


}

/// @nodoc
abstract mixin class $ServerResponseSyncDataCopyWith<$Res> implements $ServerResponseCopyWith<$Res> {
  factory $ServerResponseSyncDataCopyWith(ServerResponseSyncData value, $Res Function(ServerResponseSyncData) _then) = _$ServerResponseSyncDataCopyWithImpl;
@useResult
$Res call({
 UpdatePomodoroState payload
});


$UpdatePomodoroStateCopyWith<$Res> get payload;

}
/// @nodoc
class _$ServerResponseSyncDataCopyWithImpl<$Res>
    implements $ServerResponseSyncDataCopyWith<$Res> {
  _$ServerResponseSyncDataCopyWithImpl(this._self, this._then);

  final ServerResponseSyncData _self;
  final $Res Function(ServerResponseSyncData) _then;

/// Create a copy of ServerResponse
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? payload = null,}) {
  return _then(ServerResponseSyncData(
null == payload ? _self.payload : payload // ignore: cast_nullable_to_non_nullable
as UpdatePomodoroState,
  ));
}

/// Create a copy of ServerResponse
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$UpdatePomodoroStateCopyWith<$Res> get payload {
  
  return $UpdatePomodoroStateCopyWith<$Res>(_self.payload, (value) {
    return _then(_self.copyWith(payload: value));
  });
}
}

BroadcastEvent _$BroadcastEventFromJson(
  Map<String, dynamic> json
) {
    return BroadcastEventPomodoroSessionUpdate.fromJson(
      json
    );
}

/// @nodoc
mixin _$BroadcastEvent {

 UpdatePomodoroState get payload;
/// Create a copy of BroadcastEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$BroadcastEventCopyWith<BroadcastEvent> get copyWith => _$BroadcastEventCopyWithImpl<BroadcastEvent>(this as BroadcastEvent, _$identity);

  /// Serializes this BroadcastEvent to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is BroadcastEvent&&(identical(other.payload, payload) || other.payload == payload));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,payload);

@override
String toString() {
  return 'BroadcastEvent(payload: $payload)';
}


}

/// @nodoc
abstract mixin class $BroadcastEventCopyWith<$Res>  {
  factory $BroadcastEventCopyWith(BroadcastEvent value, $Res Function(BroadcastEvent) _then) = _$BroadcastEventCopyWithImpl;
@useResult
$Res call({
 UpdatePomodoroState payload
});


$UpdatePomodoroStateCopyWith<$Res> get payload;

}
/// @nodoc
class _$BroadcastEventCopyWithImpl<$Res>
    implements $BroadcastEventCopyWith<$Res> {
  _$BroadcastEventCopyWithImpl(this._self, this._then);

  final BroadcastEvent _self;
  final $Res Function(BroadcastEvent) _then;

/// Create a copy of BroadcastEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? payload = null,}) {
  return _then(_self.copyWith(
payload: null == payload ? _self.payload : payload // ignore: cast_nullable_to_non_nullable
as UpdatePomodoroState,
  ));
}
/// Create a copy of BroadcastEvent
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$UpdatePomodoroStateCopyWith<$Res> get payload {
  
  return $UpdatePomodoroStateCopyWith<$Res>(_self.payload, (value) {
    return _then(_self.copyWith(payload: value));
  });
}
}


/// Adds pattern-matching-related methods to [BroadcastEvent].
extension BroadcastEventPatterns on BroadcastEvent {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( BroadcastEventPomodoroSessionUpdate value)?  pomodoroSessionUpdate,required TResult orElse(),}){
final _that = this;
switch (_that) {
case BroadcastEventPomodoroSessionUpdate() when pomodoroSessionUpdate != null:
return pomodoroSessionUpdate(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( BroadcastEventPomodoroSessionUpdate value)  pomodoroSessionUpdate,}){
final _that = this;
switch (_that) {
case BroadcastEventPomodoroSessionUpdate():
return pomodoroSessionUpdate(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( BroadcastEventPomodoroSessionUpdate value)?  pomodoroSessionUpdate,}){
final _that = this;
switch (_that) {
case BroadcastEventPomodoroSessionUpdate() when pomodoroSessionUpdate != null:
return pomodoroSessionUpdate(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( UpdatePomodoroState payload)?  pomodoroSessionUpdate,required TResult orElse(),}) {final _that = this;
switch (_that) {
case BroadcastEventPomodoroSessionUpdate() when pomodoroSessionUpdate != null:
return pomodoroSessionUpdate(_that.payload);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( UpdatePomodoroState payload)  pomodoroSessionUpdate,}) {final _that = this;
switch (_that) {
case BroadcastEventPomodoroSessionUpdate():
return pomodoroSessionUpdate(_that.payload);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( UpdatePomodoroState payload)?  pomodoroSessionUpdate,}) {final _that = this;
switch (_that) {
case BroadcastEventPomodoroSessionUpdate() when pomodoroSessionUpdate != null:
return pomodoroSessionUpdate(_that.payload);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class BroadcastEventPomodoroSessionUpdate implements BroadcastEvent {
  const BroadcastEventPomodoroSessionUpdate(this.payload);
  factory BroadcastEventPomodoroSessionUpdate.fromJson(Map<String, dynamic> json) => _$BroadcastEventPomodoroSessionUpdateFromJson(json);

@override final  UpdatePomodoroState payload;

/// Create a copy of BroadcastEvent
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$BroadcastEventPomodoroSessionUpdateCopyWith<BroadcastEventPomodoroSessionUpdate> get copyWith => _$BroadcastEventPomodoroSessionUpdateCopyWithImpl<BroadcastEventPomodoroSessionUpdate>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$BroadcastEventPomodoroSessionUpdateToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is BroadcastEventPomodoroSessionUpdate&&(identical(other.payload, payload) || other.payload == payload));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,payload);

@override
String toString() {
  return 'BroadcastEvent.pomodoroSessionUpdate(payload: $payload)';
}


}

/// @nodoc
abstract mixin class $BroadcastEventPomodoroSessionUpdateCopyWith<$Res> implements $BroadcastEventCopyWith<$Res> {
  factory $BroadcastEventPomodoroSessionUpdateCopyWith(BroadcastEventPomodoroSessionUpdate value, $Res Function(BroadcastEventPomodoroSessionUpdate) _then) = _$BroadcastEventPomodoroSessionUpdateCopyWithImpl;
@override @useResult
$Res call({
 UpdatePomodoroState payload
});


@override $UpdatePomodoroStateCopyWith<$Res> get payload;

}
/// @nodoc
class _$BroadcastEventPomodoroSessionUpdateCopyWithImpl<$Res>
    implements $BroadcastEventPomodoroSessionUpdateCopyWith<$Res> {
  _$BroadcastEventPomodoroSessionUpdateCopyWithImpl(this._self, this._then);

  final BroadcastEventPomodoroSessionUpdate _self;
  final $Res Function(BroadcastEventPomodoroSessionUpdate) _then;

/// Create a copy of BroadcastEvent
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? payload = null,}) {
  return _then(BroadcastEventPomodoroSessionUpdate(
null == payload ? _self.payload : payload // ignore: cast_nullable_to_non_nullable
as UpdatePomodoroState,
  ));
}

/// Create a copy of BroadcastEvent
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$UpdatePomodoroStateCopyWith<$Res> get payload {
  
  return $UpdatePomodoroStateCopyWith<$Res>(_self.payload, (value) {
    return _then(_self.copyWith(payload: value));
  });
}
}

// dart format on
