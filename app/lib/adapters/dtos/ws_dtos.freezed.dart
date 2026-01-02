// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'ws_dtos.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
  'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models',
);

UpdateConcentrationScore _$UpdateConcentrationScoreFromJson(
  Map<String, dynamic> json,
) {
  return _UpdateConcentrationScore.fromJson(json);
}

/// @nodoc
mixin _$UpdateConcentrationScore {
  int get concentrationScore => throw _privateConstructorUsedError;

  /// Serializes this UpdateConcentrationScore to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of UpdateConcentrationScore
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $UpdateConcentrationScoreCopyWith<UpdateConcentrationScore> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $UpdateConcentrationScoreCopyWith<$Res> {
  factory $UpdateConcentrationScoreCopyWith(
    UpdateConcentrationScore value,
    $Res Function(UpdateConcentrationScore) then,
  ) = _$UpdateConcentrationScoreCopyWithImpl<$Res, UpdateConcentrationScore>;
  @useResult
  $Res call({int concentrationScore});
}

/// @nodoc
class _$UpdateConcentrationScoreCopyWithImpl<
  $Res,
  $Val extends UpdateConcentrationScore
>
    implements $UpdateConcentrationScoreCopyWith<$Res> {
  _$UpdateConcentrationScoreCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of UpdateConcentrationScore
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? concentrationScore = null}) {
    return _then(
      _value.copyWith(
            concentrationScore:
                null == concentrationScore
                    ? _value.concentrationScore
                    : concentrationScore // ignore: cast_nullable_to_non_nullable
                        as int,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$UpdateConcentrationScoreImplCopyWith<$Res>
    implements $UpdateConcentrationScoreCopyWith<$Res> {
  factory _$$UpdateConcentrationScoreImplCopyWith(
    _$UpdateConcentrationScoreImpl value,
    $Res Function(_$UpdateConcentrationScoreImpl) then,
  ) = __$$UpdateConcentrationScoreImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int concentrationScore});
}

/// @nodoc
class __$$UpdateConcentrationScoreImplCopyWithImpl<$Res>
    extends
        _$UpdateConcentrationScoreCopyWithImpl<
          $Res,
          _$UpdateConcentrationScoreImpl
        >
    implements _$$UpdateConcentrationScoreImplCopyWith<$Res> {
  __$$UpdateConcentrationScoreImplCopyWithImpl(
    _$UpdateConcentrationScoreImpl _value,
    $Res Function(_$UpdateConcentrationScoreImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of UpdateConcentrationScore
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? concentrationScore = null}) {
    return _then(
      _$UpdateConcentrationScoreImpl(
        concentrationScore:
            null == concentrationScore
                ? _value.concentrationScore
                : concentrationScore // ignore: cast_nullable_to_non_nullable
                    as int,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$UpdateConcentrationScoreImpl implements _UpdateConcentrationScore {
  const _$UpdateConcentrationScoreImpl({required this.concentrationScore});

  factory _$UpdateConcentrationScoreImpl.fromJson(Map<String, dynamic> json) =>
      _$$UpdateConcentrationScoreImplFromJson(json);

  @override
  final int concentrationScore;

  @override
  String toString() {
    return 'UpdateConcentrationScore(concentrationScore: $concentrationScore)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$UpdateConcentrationScoreImpl &&
            (identical(other.concentrationScore, concentrationScore) ||
                other.concentrationScore == concentrationScore));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, concentrationScore);

  /// Create a copy of UpdateConcentrationScore
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$UpdateConcentrationScoreImplCopyWith<_$UpdateConcentrationScoreImpl>
  get copyWith => __$$UpdateConcentrationScoreImplCopyWithImpl<
    _$UpdateConcentrationScoreImpl
  >(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$UpdateConcentrationScoreImplToJson(this);
  }
}

abstract class _UpdateConcentrationScore implements UpdateConcentrationScore {
  const factory _UpdateConcentrationScore({
    required final int concentrationScore,
  }) = _$UpdateConcentrationScoreImpl;

  factory _UpdateConcentrationScore.fromJson(Map<String, dynamic> json) =
      _$UpdateConcentrationScoreImpl.fromJson;

  @override
  int get concentrationScore;

  /// Create a copy of UpdateConcentrationScore
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$UpdateConcentrationScoreImplCopyWith<_$UpdateConcentrationScoreImpl>
  get copyWith => throw _privateConstructorUsedError;
}

NoteUpdate _$NoteUpdateFromJson(Map<String, dynamic> json) {
  return _NoteUpdate.fromJson(json);
}

/// @nodoc
mixin _$NoteUpdate {
  String get newNote => throw _privateConstructorUsedError;

  /// Serializes this NoteUpdate to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of NoteUpdate
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $NoteUpdateCopyWith<NoteUpdate> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $NoteUpdateCopyWith<$Res> {
  factory $NoteUpdateCopyWith(
    NoteUpdate value,
    $Res Function(NoteUpdate) then,
  ) = _$NoteUpdateCopyWithImpl<$Res, NoteUpdate>;
  @useResult
  $Res call({String newNote});
}

/// @nodoc
class _$NoteUpdateCopyWithImpl<$Res, $Val extends NoteUpdate>
    implements $NoteUpdateCopyWith<$Res> {
  _$NoteUpdateCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of NoteUpdate
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? newNote = null}) {
    return _then(
      _value.copyWith(
            newNote:
                null == newNote
                    ? _value.newNote
                    : newNote // ignore: cast_nullable_to_non_nullable
                        as String,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$NoteUpdateImplCopyWith<$Res>
    implements $NoteUpdateCopyWith<$Res> {
  factory _$$NoteUpdateImplCopyWith(
    _$NoteUpdateImpl value,
    $Res Function(_$NoteUpdateImpl) then,
  ) = __$$NoteUpdateImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String newNote});
}

/// @nodoc
class __$$NoteUpdateImplCopyWithImpl<$Res>
    extends _$NoteUpdateCopyWithImpl<$Res, _$NoteUpdateImpl>
    implements _$$NoteUpdateImplCopyWith<$Res> {
  __$$NoteUpdateImplCopyWithImpl(
    _$NoteUpdateImpl _value,
    $Res Function(_$NoteUpdateImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of NoteUpdate
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? newNote = null}) {
    return _then(
      _$NoteUpdateImpl(
        newNote:
            null == newNote
                ? _value.newNote
                : newNote // ignore: cast_nullable_to_non_nullable
                    as String,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$NoteUpdateImpl implements _NoteUpdate {
  const _$NoteUpdateImpl({required this.newNote});

  factory _$NoteUpdateImpl.fromJson(Map<String, dynamic> json) =>
      _$$NoteUpdateImplFromJson(json);

  @override
  final String newNote;

  @override
  String toString() {
    return 'NoteUpdate(newNote: $newNote)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$NoteUpdateImpl &&
            (identical(other.newNote, newNote) || other.newNote == newNote));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, newNote);

  /// Create a copy of NoteUpdate
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$NoteUpdateImplCopyWith<_$NoteUpdateImpl> get copyWith =>
      __$$NoteUpdateImplCopyWithImpl<_$NoteUpdateImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$NoteUpdateImplToJson(this);
  }
}

abstract class _NoteUpdate implements NoteUpdate {
  const factory _NoteUpdate({required final String newNote}) = _$NoteUpdateImpl;

  factory _NoteUpdate.fromJson(Map<String, dynamic> json) =
      _$NoteUpdateImpl.fromJson;

  @override
  String get newNote;

  /// Create a copy of NoteUpdate
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$NoteUpdateImplCopyWith<_$NoteUpdateImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

UpdatePomodoroContext _$UpdatePomodoroContextFromJson(
  Map<String, dynamic> json,
) {
  return _UpdatePomodoroContext.fromJson(json);
}

/// @nodoc
mixin _$UpdatePomodoroContext {
  String? get categoryId => throw _privateConstructorUsedError;
  String? get taskId => throw _privateConstructorUsedError;

  /// Serializes this UpdatePomodoroContext to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of UpdatePomodoroContext
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $UpdatePomodoroContextCopyWith<UpdatePomodoroContext> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $UpdatePomodoroContextCopyWith<$Res> {
  factory $UpdatePomodoroContextCopyWith(
    UpdatePomodoroContext value,
    $Res Function(UpdatePomodoroContext) then,
  ) = _$UpdatePomodoroContextCopyWithImpl<$Res, UpdatePomodoroContext>;
  @useResult
  $Res call({String? categoryId, String? taskId});
}

/// @nodoc
class _$UpdatePomodoroContextCopyWithImpl<
  $Res,
  $Val extends UpdatePomodoroContext
>
    implements $UpdatePomodoroContextCopyWith<$Res> {
  _$UpdatePomodoroContextCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of UpdatePomodoroContext
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? categoryId = freezed, Object? taskId = freezed}) {
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
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$UpdatePomodoroContextImplCopyWith<$Res>
    implements $UpdatePomodoroContextCopyWith<$Res> {
  factory _$$UpdatePomodoroContextImplCopyWith(
    _$UpdatePomodoroContextImpl value,
    $Res Function(_$UpdatePomodoroContextImpl) then,
  ) = __$$UpdatePomodoroContextImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String? categoryId, String? taskId});
}

/// @nodoc
class __$$UpdatePomodoroContextImplCopyWithImpl<$Res>
    extends
        _$UpdatePomodoroContextCopyWithImpl<$Res, _$UpdatePomodoroContextImpl>
    implements _$$UpdatePomodoroContextImplCopyWith<$Res> {
  __$$UpdatePomodoroContextImplCopyWithImpl(
    _$UpdatePomodoroContextImpl _value,
    $Res Function(_$UpdatePomodoroContextImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of UpdatePomodoroContext
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? categoryId = freezed, Object? taskId = freezed}) {
    return _then(
      _$UpdatePomodoroContextImpl(
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
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$UpdatePomodoroContextImpl implements _UpdatePomodoroContext {
  const _$UpdatePomodoroContextImpl({this.categoryId, this.taskId});

  factory _$UpdatePomodoroContextImpl.fromJson(Map<String, dynamic> json) =>
      _$$UpdatePomodoroContextImplFromJson(json);

  @override
  final String? categoryId;
  @override
  final String? taskId;

  @override
  String toString() {
    return 'UpdatePomodoroContext(categoryId: $categoryId, taskId: $taskId)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$UpdatePomodoroContextImpl &&
            (identical(other.categoryId, categoryId) ||
                other.categoryId == categoryId) &&
            (identical(other.taskId, taskId) || other.taskId == taskId));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, categoryId, taskId);

  /// Create a copy of UpdatePomodoroContext
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$UpdatePomodoroContextImplCopyWith<_$UpdatePomodoroContextImpl>
  get copyWith =>
      __$$UpdatePomodoroContextImplCopyWithImpl<_$UpdatePomodoroContextImpl>(
        this,
        _$identity,
      );

  @override
  Map<String, dynamic> toJson() {
    return _$$UpdatePomodoroContextImplToJson(this);
  }
}

abstract class _UpdatePomodoroContext implements UpdatePomodoroContext {
  const factory _UpdatePomodoroContext({
    final String? categoryId,
    final String? taskId,
  }) = _$UpdatePomodoroContextImpl;

  factory _UpdatePomodoroContext.fromJson(Map<String, dynamic> json) =
      _$UpdatePomodoroContextImpl.fromJson;

  @override
  String? get categoryId;
  @override
  String? get taskId;

  /// Create a copy of UpdatePomodoroContext
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$UpdatePomodoroContextImplCopyWith<_$UpdatePomodoroContextImpl>
  get copyWith => throw _privateConstructorUsedError;
}

UpdateWorkContext _$UpdateWorkContextFromJson(Map<String, dynamic> json) {
  return _UpdateWorkContext.fromJson(json);
}

/// @nodoc
mixin _$UpdateWorkContext {
  String? get categoryId => throw _privateConstructorUsedError;
  String? get taskId => throw _privateConstructorUsedError;

  /// Serializes this UpdateWorkContext to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of UpdateWorkContext
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $UpdateWorkContextCopyWith<UpdateWorkContext> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $UpdateWorkContextCopyWith<$Res> {
  factory $UpdateWorkContextCopyWith(
    UpdateWorkContext value,
    $Res Function(UpdateWorkContext) then,
  ) = _$UpdateWorkContextCopyWithImpl<$Res, UpdateWorkContext>;
  @useResult
  $Res call({String? categoryId, String? taskId});
}

/// @nodoc
class _$UpdateWorkContextCopyWithImpl<$Res, $Val extends UpdateWorkContext>
    implements $UpdateWorkContextCopyWith<$Res> {
  _$UpdateWorkContextCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of UpdateWorkContext
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? categoryId = freezed, Object? taskId = freezed}) {
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
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$UpdateWorkContextImplCopyWith<$Res>
    implements $UpdateWorkContextCopyWith<$Res> {
  factory _$$UpdateWorkContextImplCopyWith(
    _$UpdateWorkContextImpl value,
    $Res Function(_$UpdateWorkContextImpl) then,
  ) = __$$UpdateWorkContextImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String? categoryId, String? taskId});
}

/// @nodoc
class __$$UpdateWorkContextImplCopyWithImpl<$Res>
    extends _$UpdateWorkContextCopyWithImpl<$Res, _$UpdateWorkContextImpl>
    implements _$$UpdateWorkContextImplCopyWith<$Res> {
  __$$UpdateWorkContextImplCopyWithImpl(
    _$UpdateWorkContextImpl _value,
    $Res Function(_$UpdateWorkContextImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of UpdateWorkContext
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? categoryId = freezed, Object? taskId = freezed}) {
    return _then(
      _$UpdateWorkContextImpl(
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
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$UpdateWorkContextImpl implements _UpdateWorkContext {
  const _$UpdateWorkContextImpl({this.categoryId, this.taskId});

  factory _$UpdateWorkContextImpl.fromJson(Map<String, dynamic> json) =>
      _$$UpdateWorkContextImplFromJson(json);

  @override
  final String? categoryId;
  @override
  final String? taskId;

  @override
  String toString() {
    return 'UpdateWorkContext(categoryId: $categoryId, taskId: $taskId)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$UpdateWorkContextImpl &&
            (identical(other.categoryId, categoryId) ||
                other.categoryId == categoryId) &&
            (identical(other.taskId, taskId) || other.taskId == taskId));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, categoryId, taskId);

  /// Create a copy of UpdateWorkContext
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$UpdateWorkContextImplCopyWith<_$UpdateWorkContextImpl> get copyWith =>
      __$$UpdateWorkContextImplCopyWithImpl<_$UpdateWorkContextImpl>(
        this,
        _$identity,
      );

  @override
  Map<String, dynamic> toJson() {
    return _$$UpdateWorkContextImplToJson(this);
  }
}

abstract class _UpdateWorkContext implements UpdateWorkContext {
  const factory _UpdateWorkContext({
    final String? categoryId,
    final String? taskId,
  }) = _$UpdateWorkContextImpl;

  factory _UpdateWorkContext.fromJson(Map<String, dynamic> json) =
      _$UpdateWorkContextImpl.fromJson;

  @override
  String? get categoryId;
  @override
  String? get taskId;

  /// Create a copy of UpdateWorkContext
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$UpdateWorkContextImplCopyWith<_$UpdateWorkContextImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

UpdateCurrentSession _$UpdateCurrentSessionFromJson(Map<String, dynamic> json) {
  return _UpdateCurrentSession.fromJson(json);
}

/// @nodoc
mixin _$UpdateCurrentSession {
  SessionTypeEnum get sessionType => throw _privateConstructorUsedError;
  int get sessionStartTime =>
      throw _privateConstructorUsedError; // i64 maps to int in Dart
  String? get categoryId => throw _privateConstructorUsedError;
  String? get taskId => throw _privateConstructorUsedError;
  String? get note => throw _privateConstructorUsedError;
  int? get concentrationScore => throw _privateConstructorUsedError;

  /// Serializes this UpdateCurrentSession to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of UpdateCurrentSession
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $UpdateCurrentSessionCopyWith<UpdateCurrentSession> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $UpdateCurrentSessionCopyWith<$Res> {
  factory $UpdateCurrentSessionCopyWith(
    UpdateCurrentSession value,
    $Res Function(UpdateCurrentSession) then,
  ) = _$UpdateCurrentSessionCopyWithImpl<$Res, UpdateCurrentSession>;
  @useResult
  $Res call({
    SessionTypeEnum sessionType,
    int sessionStartTime,
    String? categoryId,
    String? taskId,
    String? note,
    int? concentrationScore,
  });
}

/// @nodoc
class _$UpdateCurrentSessionCopyWithImpl<
  $Res,
  $Val extends UpdateCurrentSession
>
    implements $UpdateCurrentSessionCopyWith<$Res> {
  _$UpdateCurrentSessionCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of UpdateCurrentSession
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? sessionType = null,
    Object? sessionStartTime = null,
    Object? categoryId = freezed,
    Object? taskId = freezed,
    Object? note = freezed,
    Object? concentrationScore = freezed,
  }) {
    return _then(
      _value.copyWith(
            sessionType:
                null == sessionType
                    ? _value.sessionType
                    : sessionType // ignore: cast_nullable_to_non_nullable
                        as SessionTypeEnum,
            sessionStartTime:
                null == sessionStartTime
                    ? _value.sessionStartTime
                    : sessionStartTime // ignore: cast_nullable_to_non_nullable
                        as int,
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
            note:
                freezed == note
                    ? _value.note
                    : note // ignore: cast_nullable_to_non_nullable
                        as String?,
            concentrationScore:
                freezed == concentrationScore
                    ? _value.concentrationScore
                    : concentrationScore // ignore: cast_nullable_to_non_nullable
                        as int?,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$UpdateCurrentSessionImplCopyWith<$Res>
    implements $UpdateCurrentSessionCopyWith<$Res> {
  factory _$$UpdateCurrentSessionImplCopyWith(
    _$UpdateCurrentSessionImpl value,
    $Res Function(_$UpdateCurrentSessionImpl) then,
  ) = __$$UpdateCurrentSessionImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({
    SessionTypeEnum sessionType,
    int sessionStartTime,
    String? categoryId,
    String? taskId,
    String? note,
    int? concentrationScore,
  });
}

/// @nodoc
class __$$UpdateCurrentSessionImplCopyWithImpl<$Res>
    extends _$UpdateCurrentSessionCopyWithImpl<$Res, _$UpdateCurrentSessionImpl>
    implements _$$UpdateCurrentSessionImplCopyWith<$Res> {
  __$$UpdateCurrentSessionImplCopyWithImpl(
    _$UpdateCurrentSessionImpl _value,
    $Res Function(_$UpdateCurrentSessionImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of UpdateCurrentSession
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? sessionType = null,
    Object? sessionStartTime = null,
    Object? categoryId = freezed,
    Object? taskId = freezed,
    Object? note = freezed,
    Object? concentrationScore = freezed,
  }) {
    return _then(
      _$UpdateCurrentSessionImpl(
        sessionType:
            null == sessionType
                ? _value.sessionType
                : sessionType // ignore: cast_nullable_to_non_nullable
                    as SessionTypeEnum,
        sessionStartTime:
            null == sessionStartTime
                ? _value.sessionStartTime
                : sessionStartTime // ignore: cast_nullable_to_non_nullable
                    as int,
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
        note:
            freezed == note
                ? _value.note
                : note // ignore: cast_nullable_to_non_nullable
                    as String?,
        concentrationScore:
            freezed == concentrationScore
                ? _value.concentrationScore
                : concentrationScore // ignore: cast_nullable_to_non_nullable
                    as int?,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$UpdateCurrentSessionImpl implements _UpdateCurrentSession {
  const _$UpdateCurrentSessionImpl({
    required this.sessionType,
    required this.sessionStartTime,
    this.categoryId,
    this.taskId,
    this.note,
    this.concentrationScore,
  });

  factory _$UpdateCurrentSessionImpl.fromJson(Map<String, dynamic> json) =>
      _$$UpdateCurrentSessionImplFromJson(json);

  @override
  final SessionTypeEnum sessionType;
  @override
  final int sessionStartTime;
  // i64 maps to int in Dart
  @override
  final String? categoryId;
  @override
  final String? taskId;
  @override
  final String? note;
  @override
  final int? concentrationScore;

  @override
  String toString() {
    return 'UpdateCurrentSession(sessionType: $sessionType, sessionStartTime: $sessionStartTime, categoryId: $categoryId, taskId: $taskId, note: $note, concentrationScore: $concentrationScore)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$UpdateCurrentSessionImpl &&
            (identical(other.sessionType, sessionType) ||
                other.sessionType == sessionType) &&
            (identical(other.sessionStartTime, sessionStartTime) ||
                other.sessionStartTime == sessionStartTime) &&
            (identical(other.categoryId, categoryId) ||
                other.categoryId == categoryId) &&
            (identical(other.taskId, taskId) || other.taskId == taskId) &&
            (identical(other.note, note) || other.note == note) &&
            (identical(other.concentrationScore, concentrationScore) ||
                other.concentrationScore == concentrationScore));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
    runtimeType,
    sessionType,
    sessionStartTime,
    categoryId,
    taskId,
    note,
    concentrationScore,
  );

  /// Create a copy of UpdateCurrentSession
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$UpdateCurrentSessionImplCopyWith<_$UpdateCurrentSessionImpl>
  get copyWith =>
      __$$UpdateCurrentSessionImplCopyWithImpl<_$UpdateCurrentSessionImpl>(
        this,
        _$identity,
      );

  @override
  Map<String, dynamic> toJson() {
    return _$$UpdateCurrentSessionImplToJson(this);
  }
}

abstract class _UpdateCurrentSession implements UpdateCurrentSession {
  const factory _UpdateCurrentSession({
    required final SessionTypeEnum sessionType,
    required final int sessionStartTime,
    final String? categoryId,
    final String? taskId,
    final String? note,
    final int? concentrationScore,
  }) = _$UpdateCurrentSessionImpl;

  factory _UpdateCurrentSession.fromJson(Map<String, dynamic> json) =
      _$UpdateCurrentSessionImpl.fromJson;

  @override
  SessionTypeEnum get sessionType;
  @override
  int get sessionStartTime; // i64 maps to int in Dart
  @override
  String? get categoryId;
  @override
  String? get taskId;
  @override
  String? get note;
  @override
  int? get concentrationScore;

  /// Create a copy of UpdateCurrentSession
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$UpdateCurrentSessionImplCopyWith<_$UpdateCurrentSessionImpl>
  get copyWith => throw _privateConstructorUsedError;
}

UpdatePomodoroState _$UpdatePomodoroStateFromJson(Map<String, dynamic> json) {
  return _UpdatePomodoroState.fromJson(json);
}

/// @nodoc
mixin _$UpdatePomodoroState {
  UpdateCurrentSession? get currentSession =>
      throw _privateConstructorUsedError;
  UpdateWorkContext get workContext => throw _privateConstructorUsedError;

  /// Serializes this UpdatePomodoroState to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of UpdatePomodoroState
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $UpdatePomodoroStateCopyWith<UpdatePomodoroState> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $UpdatePomodoroStateCopyWith<$Res> {
  factory $UpdatePomodoroStateCopyWith(
    UpdatePomodoroState value,
    $Res Function(UpdatePomodoroState) then,
  ) = _$UpdatePomodoroStateCopyWithImpl<$Res, UpdatePomodoroState>;
  @useResult
  $Res call({
    UpdateCurrentSession? currentSession,
    UpdateWorkContext workContext,
  });

  $UpdateCurrentSessionCopyWith<$Res>? get currentSession;
  $UpdateWorkContextCopyWith<$Res> get workContext;
}

/// @nodoc
class _$UpdatePomodoroStateCopyWithImpl<$Res, $Val extends UpdatePomodoroState>
    implements $UpdatePomodoroStateCopyWith<$Res> {
  _$UpdatePomodoroStateCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of UpdatePomodoroState
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? currentSession = freezed, Object? workContext = null}) {
    return _then(
      _value.copyWith(
            currentSession:
                freezed == currentSession
                    ? _value.currentSession
                    : currentSession // ignore: cast_nullable_to_non_nullable
                        as UpdateCurrentSession?,
            workContext:
                null == workContext
                    ? _value.workContext
                    : workContext // ignore: cast_nullable_to_non_nullable
                        as UpdateWorkContext,
          )
          as $Val,
    );
  }

  /// Create a copy of UpdatePomodoroState
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $UpdateCurrentSessionCopyWith<$Res>? get currentSession {
    if (_value.currentSession == null) {
      return null;
    }

    return $UpdateCurrentSessionCopyWith<$Res>(_value.currentSession!, (value) {
      return _then(_value.copyWith(currentSession: value) as $Val);
    });
  }

  /// Create a copy of UpdatePomodoroState
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $UpdateWorkContextCopyWith<$Res> get workContext {
    return $UpdateWorkContextCopyWith<$Res>(_value.workContext, (value) {
      return _then(_value.copyWith(workContext: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$UpdatePomodoroStateImplCopyWith<$Res>
    implements $UpdatePomodoroStateCopyWith<$Res> {
  factory _$$UpdatePomodoroStateImplCopyWith(
    _$UpdatePomodoroStateImpl value,
    $Res Function(_$UpdatePomodoroStateImpl) then,
  ) = __$$UpdatePomodoroStateImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({
    UpdateCurrentSession? currentSession,
    UpdateWorkContext workContext,
  });

  @override
  $UpdateCurrentSessionCopyWith<$Res>? get currentSession;
  @override
  $UpdateWorkContextCopyWith<$Res> get workContext;
}

/// @nodoc
class __$$UpdatePomodoroStateImplCopyWithImpl<$Res>
    extends _$UpdatePomodoroStateCopyWithImpl<$Res, _$UpdatePomodoroStateImpl>
    implements _$$UpdatePomodoroStateImplCopyWith<$Res> {
  __$$UpdatePomodoroStateImplCopyWithImpl(
    _$UpdatePomodoroStateImpl _value,
    $Res Function(_$UpdatePomodoroStateImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of UpdatePomodoroState
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? currentSession = freezed, Object? workContext = null}) {
    return _then(
      _$UpdatePomodoroStateImpl(
        currentSession:
            freezed == currentSession
                ? _value.currentSession
                : currentSession // ignore: cast_nullable_to_non_nullable
                    as UpdateCurrentSession?,
        workContext:
            null == workContext
                ? _value.workContext
                : workContext // ignore: cast_nullable_to_non_nullable
                    as UpdateWorkContext,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$UpdatePomodoroStateImpl implements _UpdatePomodoroState {
  const _$UpdatePomodoroStateImpl({
    this.currentSession,
    required this.workContext,
  });

  factory _$UpdatePomodoroStateImpl.fromJson(Map<String, dynamic> json) =>
      _$$UpdatePomodoroStateImplFromJson(json);

  @override
  final UpdateCurrentSession? currentSession;
  @override
  final UpdateWorkContext workContext;

  @override
  String toString() {
    return 'UpdatePomodoroState(currentSession: $currentSession, workContext: $workContext)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$UpdatePomodoroStateImpl &&
            (identical(other.currentSession, currentSession) ||
                other.currentSession == currentSession) &&
            (identical(other.workContext, workContext) ||
                other.workContext == workContext));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, currentSession, workContext);

  /// Create a copy of UpdatePomodoroState
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$UpdatePomodoroStateImplCopyWith<_$UpdatePomodoroStateImpl> get copyWith =>
      __$$UpdatePomodoroStateImplCopyWithImpl<_$UpdatePomodoroStateImpl>(
        this,
        _$identity,
      );

  @override
  Map<String, dynamic> toJson() {
    return _$$UpdatePomodoroStateImplToJson(this);
  }
}

abstract class _UpdatePomodoroState implements UpdatePomodoroState {
  const factory _UpdatePomodoroState({
    final UpdateCurrentSession? currentSession,
    required final UpdateWorkContext workContext,
  }) = _$UpdatePomodoroStateImpl;

  factory _UpdatePomodoroState.fromJson(Map<String, dynamic> json) =
      _$UpdatePomodoroStateImpl.fromJson;

  @override
  UpdateCurrentSession? get currentSession;
  @override
  UpdateWorkContext get workContext;

  /// Create a copy of UpdatePomodoroState
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$UpdatePomodoroStateImplCopyWith<_$UpdatePomodoroStateImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

WsClientRequest _$WsClientRequestFromJson(Map<String, dynamic> json) {
  return _WsClientRequest.fromJson(json);
}

/// @nodoc
mixin _$WsClientRequest {
  String? get requestId => throw _privateConstructorUsedError;
  ClientMessage get message => throw _privateConstructorUsedError;

  /// Serializes this WsClientRequest to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of WsClientRequest
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $WsClientRequestCopyWith<WsClientRequest> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $WsClientRequestCopyWith<$Res> {
  factory $WsClientRequestCopyWith(
    WsClientRequest value,
    $Res Function(WsClientRequest) then,
  ) = _$WsClientRequestCopyWithImpl<$Res, WsClientRequest>;
  @useResult
  $Res call({String? requestId, ClientMessage message});

  $ClientMessageCopyWith<$Res> get message;
}

/// @nodoc
class _$WsClientRequestCopyWithImpl<$Res, $Val extends WsClientRequest>
    implements $WsClientRequestCopyWith<$Res> {
  _$WsClientRequestCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of WsClientRequest
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? requestId = freezed, Object? message = null}) {
    return _then(
      _value.copyWith(
            requestId:
                freezed == requestId
                    ? _value.requestId
                    : requestId // ignore: cast_nullable_to_non_nullable
                        as String?,
            message:
                null == message
                    ? _value.message
                    : message // ignore: cast_nullable_to_non_nullable
                        as ClientMessage,
          )
          as $Val,
    );
  }

  /// Create a copy of WsClientRequest
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $ClientMessageCopyWith<$Res> get message {
    return $ClientMessageCopyWith<$Res>(_value.message, (value) {
      return _then(_value.copyWith(message: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$WsClientRequestImplCopyWith<$Res>
    implements $WsClientRequestCopyWith<$Res> {
  factory _$$WsClientRequestImplCopyWith(
    _$WsClientRequestImpl value,
    $Res Function(_$WsClientRequestImpl) then,
  ) = __$$WsClientRequestImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String? requestId, ClientMessage message});

  @override
  $ClientMessageCopyWith<$Res> get message;
}

/// @nodoc
class __$$WsClientRequestImplCopyWithImpl<$Res>
    extends _$WsClientRequestCopyWithImpl<$Res, _$WsClientRequestImpl>
    implements _$$WsClientRequestImplCopyWith<$Res> {
  __$$WsClientRequestImplCopyWithImpl(
    _$WsClientRequestImpl _value,
    $Res Function(_$WsClientRequestImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of WsClientRequest
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? requestId = freezed, Object? message = null}) {
    return _then(
      _$WsClientRequestImpl(
        requestId:
            freezed == requestId
                ? _value.requestId
                : requestId // ignore: cast_nullable_to_non_nullable
                    as String?,
        message:
            null == message
                ? _value.message
                : message // ignore: cast_nullable_to_non_nullable
                    as ClientMessage,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$WsClientRequestImpl implements _WsClientRequest {
  const _$WsClientRequestImpl({this.requestId, required this.message});

  factory _$WsClientRequestImpl.fromJson(Map<String, dynamic> json) =>
      _$$WsClientRequestImplFromJson(json);

  @override
  final String? requestId;
  @override
  final ClientMessage message;

  @override
  String toString() {
    return 'WsClientRequest(requestId: $requestId, message: $message)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$WsClientRequestImpl &&
            (identical(other.requestId, requestId) ||
                other.requestId == requestId) &&
            (identical(other.message, message) || other.message == message));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, requestId, message);

  /// Create a copy of WsClientRequest
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$WsClientRequestImplCopyWith<_$WsClientRequestImpl> get copyWith =>
      __$$WsClientRequestImplCopyWithImpl<_$WsClientRequestImpl>(
        this,
        _$identity,
      );

  @override
  Map<String, dynamic> toJson() {
    return _$$WsClientRequestImplToJson(this);
  }
}

abstract class _WsClientRequest implements WsClientRequest {
  const factory _WsClientRequest({
    final String? requestId,
    required final ClientMessage message,
  }) = _$WsClientRequestImpl;

  factory _WsClientRequest.fromJson(Map<String, dynamic> json) =
      _$WsClientRequestImpl.fromJson;

  @override
  String? get requestId;
  @override
  ClientMessage get message;

  /// Create a copy of WsClientRequest
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$WsClientRequestImplCopyWith<_$WsClientRequestImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

ClientMessage _$ClientMessageFromJson(Map<String, dynamic> json) {
  switch (json['type']) {
    case 'requestSync':
      return ClientMessageRequestSync.fromJson(json);
    case 'startEvent':
      return ClientMessageStartEvent.fromJson(json);
    case 'breakEvent':
      return ClientMessageBreakEvent.fromJson(json);
    case 'terminateEvent':
      return ClientMessageTerminateEvent.fromJson(json);
    case 'updatePomodoroContext':
      return ClientMessageUpdatePomodoroContext.fromJson(json);
    case 'updateNote':
      return ClientMessageUpdateNote.fromJson(json);
    case 'updateConcentrationScore':
      return ClientMessageUpdateConcentrationScore.fromJson(json);

    default:
      throw CheckedFromJsonException(
        json,
        'type',
        'ClientMessage',
        'Invalid union type "${json['type']}"!',
      );
  }
}

/// @nodoc
mixin _$ClientMessage {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() requestSync,
    required TResult Function() startEvent,
    required TResult Function() breakEvent,
    required TResult Function() terminateEvent,
    required TResult Function(UpdatePomodoroContext payload)
    updatePomodoroContext,
    required TResult Function(NoteUpdate payload) updateNote,
    required TResult Function(UpdateConcentrationScore payload)
    updateConcentrationScore,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? requestSync,
    TResult? Function()? startEvent,
    TResult? Function()? breakEvent,
    TResult? Function()? terminateEvent,
    TResult? Function(UpdatePomodoroContext payload)? updatePomodoroContext,
    TResult? Function(NoteUpdate payload)? updateNote,
    TResult? Function(UpdateConcentrationScore payload)?
    updateConcentrationScore,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? requestSync,
    TResult Function()? startEvent,
    TResult Function()? breakEvent,
    TResult Function()? terminateEvent,
    TResult Function(UpdatePomodoroContext payload)? updatePomodoroContext,
    TResult Function(NoteUpdate payload)? updateNote,
    TResult Function(UpdateConcentrationScore payload)?
    updateConcentrationScore,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ClientMessageRequestSync value) requestSync,
    required TResult Function(ClientMessageStartEvent value) startEvent,
    required TResult Function(ClientMessageBreakEvent value) breakEvent,
    required TResult Function(ClientMessageTerminateEvent value) terminateEvent,
    required TResult Function(ClientMessageUpdatePomodoroContext value)
    updatePomodoroContext,
    required TResult Function(ClientMessageUpdateNote value) updateNote,
    required TResult Function(ClientMessageUpdateConcentrationScore value)
    updateConcentrationScore,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ClientMessageRequestSync value)? requestSync,
    TResult? Function(ClientMessageStartEvent value)? startEvent,
    TResult? Function(ClientMessageBreakEvent value)? breakEvent,
    TResult? Function(ClientMessageTerminateEvent value)? terminateEvent,
    TResult? Function(ClientMessageUpdatePomodoroContext value)?
    updatePomodoroContext,
    TResult? Function(ClientMessageUpdateNote value)? updateNote,
    TResult? Function(ClientMessageUpdateConcentrationScore value)?
    updateConcentrationScore,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ClientMessageRequestSync value)? requestSync,
    TResult Function(ClientMessageStartEvent value)? startEvent,
    TResult Function(ClientMessageBreakEvent value)? breakEvent,
    TResult Function(ClientMessageTerminateEvent value)? terminateEvent,
    TResult Function(ClientMessageUpdatePomodoroContext value)?
    updatePomodoroContext,
    TResult Function(ClientMessageUpdateNote value)? updateNote,
    TResult Function(ClientMessageUpdateConcentrationScore value)?
    updateConcentrationScore,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;

  /// Serializes this ClientMessage to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ClientMessageCopyWith<$Res> {
  factory $ClientMessageCopyWith(
    ClientMessage value,
    $Res Function(ClientMessage) then,
  ) = _$ClientMessageCopyWithImpl<$Res, ClientMessage>;
}

/// @nodoc
class _$ClientMessageCopyWithImpl<$Res, $Val extends ClientMessage>
    implements $ClientMessageCopyWith<$Res> {
  _$ClientMessageCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of ClientMessage
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$ClientMessageRequestSyncImplCopyWith<$Res> {
  factory _$$ClientMessageRequestSyncImplCopyWith(
    _$ClientMessageRequestSyncImpl value,
    $Res Function(_$ClientMessageRequestSyncImpl) then,
  ) = __$$ClientMessageRequestSyncImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$ClientMessageRequestSyncImplCopyWithImpl<$Res>
    extends _$ClientMessageCopyWithImpl<$Res, _$ClientMessageRequestSyncImpl>
    implements _$$ClientMessageRequestSyncImplCopyWith<$Res> {
  __$$ClientMessageRequestSyncImplCopyWithImpl(
    _$ClientMessageRequestSyncImpl _value,
    $Res Function(_$ClientMessageRequestSyncImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ClientMessage
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
@JsonSerializable()
class _$ClientMessageRequestSyncImpl implements ClientMessageRequestSync {
  const _$ClientMessageRequestSyncImpl({final String? $type})
    : $type = $type ?? 'requestSync';

  factory _$ClientMessageRequestSyncImpl.fromJson(Map<String, dynamic> json) =>
      _$$ClientMessageRequestSyncImplFromJson(json);

  @JsonKey(name: 'type')
  final String $type;

  @override
  String toString() {
    return 'ClientMessage.requestSync()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ClientMessageRequestSyncImpl);
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() requestSync,
    required TResult Function() startEvent,
    required TResult Function() breakEvent,
    required TResult Function() terminateEvent,
    required TResult Function(UpdatePomodoroContext payload)
    updatePomodoroContext,
    required TResult Function(NoteUpdate payload) updateNote,
    required TResult Function(UpdateConcentrationScore payload)
    updateConcentrationScore,
  }) {
    return requestSync();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? requestSync,
    TResult? Function()? startEvent,
    TResult? Function()? breakEvent,
    TResult? Function()? terminateEvent,
    TResult? Function(UpdatePomodoroContext payload)? updatePomodoroContext,
    TResult? Function(NoteUpdate payload)? updateNote,
    TResult? Function(UpdateConcentrationScore payload)?
    updateConcentrationScore,
  }) {
    return requestSync?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? requestSync,
    TResult Function()? startEvent,
    TResult Function()? breakEvent,
    TResult Function()? terminateEvent,
    TResult Function(UpdatePomodoroContext payload)? updatePomodoroContext,
    TResult Function(NoteUpdate payload)? updateNote,
    TResult Function(UpdateConcentrationScore payload)?
    updateConcentrationScore,
    required TResult orElse(),
  }) {
    if (requestSync != null) {
      return requestSync();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ClientMessageRequestSync value) requestSync,
    required TResult Function(ClientMessageStartEvent value) startEvent,
    required TResult Function(ClientMessageBreakEvent value) breakEvent,
    required TResult Function(ClientMessageTerminateEvent value) terminateEvent,
    required TResult Function(ClientMessageUpdatePomodoroContext value)
    updatePomodoroContext,
    required TResult Function(ClientMessageUpdateNote value) updateNote,
    required TResult Function(ClientMessageUpdateConcentrationScore value)
    updateConcentrationScore,
  }) {
    return requestSync(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ClientMessageRequestSync value)? requestSync,
    TResult? Function(ClientMessageStartEvent value)? startEvent,
    TResult? Function(ClientMessageBreakEvent value)? breakEvent,
    TResult? Function(ClientMessageTerminateEvent value)? terminateEvent,
    TResult? Function(ClientMessageUpdatePomodoroContext value)?
    updatePomodoroContext,
    TResult? Function(ClientMessageUpdateNote value)? updateNote,
    TResult? Function(ClientMessageUpdateConcentrationScore value)?
    updateConcentrationScore,
  }) {
    return requestSync?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ClientMessageRequestSync value)? requestSync,
    TResult Function(ClientMessageStartEvent value)? startEvent,
    TResult Function(ClientMessageBreakEvent value)? breakEvent,
    TResult Function(ClientMessageTerminateEvent value)? terminateEvent,
    TResult Function(ClientMessageUpdatePomodoroContext value)?
    updatePomodoroContext,
    TResult Function(ClientMessageUpdateNote value)? updateNote,
    TResult Function(ClientMessageUpdateConcentrationScore value)?
    updateConcentrationScore,
    required TResult orElse(),
  }) {
    if (requestSync != null) {
      return requestSync(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$ClientMessageRequestSyncImplToJson(this);
  }
}

abstract class ClientMessageRequestSync implements ClientMessage {
  const factory ClientMessageRequestSync() = _$ClientMessageRequestSyncImpl;

  factory ClientMessageRequestSync.fromJson(Map<String, dynamic> json) =
      _$ClientMessageRequestSyncImpl.fromJson;
}

/// @nodoc
abstract class _$$ClientMessageStartEventImplCopyWith<$Res> {
  factory _$$ClientMessageStartEventImplCopyWith(
    _$ClientMessageStartEventImpl value,
    $Res Function(_$ClientMessageStartEventImpl) then,
  ) = __$$ClientMessageStartEventImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$ClientMessageStartEventImplCopyWithImpl<$Res>
    extends _$ClientMessageCopyWithImpl<$Res, _$ClientMessageStartEventImpl>
    implements _$$ClientMessageStartEventImplCopyWith<$Res> {
  __$$ClientMessageStartEventImplCopyWithImpl(
    _$ClientMessageStartEventImpl _value,
    $Res Function(_$ClientMessageStartEventImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ClientMessage
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
@JsonSerializable()
class _$ClientMessageStartEventImpl implements ClientMessageStartEvent {
  const _$ClientMessageStartEventImpl({final String? $type})
    : $type = $type ?? 'startEvent';

  factory _$ClientMessageStartEventImpl.fromJson(Map<String, dynamic> json) =>
      _$$ClientMessageStartEventImplFromJson(json);

  @JsonKey(name: 'type')
  final String $type;

  @override
  String toString() {
    return 'ClientMessage.startEvent()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ClientMessageStartEventImpl);
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() requestSync,
    required TResult Function() startEvent,
    required TResult Function() breakEvent,
    required TResult Function() terminateEvent,
    required TResult Function(UpdatePomodoroContext payload)
    updatePomodoroContext,
    required TResult Function(NoteUpdate payload) updateNote,
    required TResult Function(UpdateConcentrationScore payload)
    updateConcentrationScore,
  }) {
    return startEvent();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? requestSync,
    TResult? Function()? startEvent,
    TResult? Function()? breakEvent,
    TResult? Function()? terminateEvent,
    TResult? Function(UpdatePomodoroContext payload)? updatePomodoroContext,
    TResult? Function(NoteUpdate payload)? updateNote,
    TResult? Function(UpdateConcentrationScore payload)?
    updateConcentrationScore,
  }) {
    return startEvent?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? requestSync,
    TResult Function()? startEvent,
    TResult Function()? breakEvent,
    TResult Function()? terminateEvent,
    TResult Function(UpdatePomodoroContext payload)? updatePomodoroContext,
    TResult Function(NoteUpdate payload)? updateNote,
    TResult Function(UpdateConcentrationScore payload)?
    updateConcentrationScore,
    required TResult orElse(),
  }) {
    if (startEvent != null) {
      return startEvent();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ClientMessageRequestSync value) requestSync,
    required TResult Function(ClientMessageStartEvent value) startEvent,
    required TResult Function(ClientMessageBreakEvent value) breakEvent,
    required TResult Function(ClientMessageTerminateEvent value) terminateEvent,
    required TResult Function(ClientMessageUpdatePomodoroContext value)
    updatePomodoroContext,
    required TResult Function(ClientMessageUpdateNote value) updateNote,
    required TResult Function(ClientMessageUpdateConcentrationScore value)
    updateConcentrationScore,
  }) {
    return startEvent(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ClientMessageRequestSync value)? requestSync,
    TResult? Function(ClientMessageStartEvent value)? startEvent,
    TResult? Function(ClientMessageBreakEvent value)? breakEvent,
    TResult? Function(ClientMessageTerminateEvent value)? terminateEvent,
    TResult? Function(ClientMessageUpdatePomodoroContext value)?
    updatePomodoroContext,
    TResult? Function(ClientMessageUpdateNote value)? updateNote,
    TResult? Function(ClientMessageUpdateConcentrationScore value)?
    updateConcentrationScore,
  }) {
    return startEvent?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ClientMessageRequestSync value)? requestSync,
    TResult Function(ClientMessageStartEvent value)? startEvent,
    TResult Function(ClientMessageBreakEvent value)? breakEvent,
    TResult Function(ClientMessageTerminateEvent value)? terminateEvent,
    TResult Function(ClientMessageUpdatePomodoroContext value)?
    updatePomodoroContext,
    TResult Function(ClientMessageUpdateNote value)? updateNote,
    TResult Function(ClientMessageUpdateConcentrationScore value)?
    updateConcentrationScore,
    required TResult orElse(),
  }) {
    if (startEvent != null) {
      return startEvent(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$ClientMessageStartEventImplToJson(this);
  }
}

abstract class ClientMessageStartEvent implements ClientMessage {
  const factory ClientMessageStartEvent() = _$ClientMessageStartEventImpl;

  factory ClientMessageStartEvent.fromJson(Map<String, dynamic> json) =
      _$ClientMessageStartEventImpl.fromJson;
}

/// @nodoc
abstract class _$$ClientMessageBreakEventImplCopyWith<$Res> {
  factory _$$ClientMessageBreakEventImplCopyWith(
    _$ClientMessageBreakEventImpl value,
    $Res Function(_$ClientMessageBreakEventImpl) then,
  ) = __$$ClientMessageBreakEventImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$ClientMessageBreakEventImplCopyWithImpl<$Res>
    extends _$ClientMessageCopyWithImpl<$Res, _$ClientMessageBreakEventImpl>
    implements _$$ClientMessageBreakEventImplCopyWith<$Res> {
  __$$ClientMessageBreakEventImplCopyWithImpl(
    _$ClientMessageBreakEventImpl _value,
    $Res Function(_$ClientMessageBreakEventImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ClientMessage
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
@JsonSerializable()
class _$ClientMessageBreakEventImpl implements ClientMessageBreakEvent {
  const _$ClientMessageBreakEventImpl({final String? $type})
    : $type = $type ?? 'breakEvent';

  factory _$ClientMessageBreakEventImpl.fromJson(Map<String, dynamic> json) =>
      _$$ClientMessageBreakEventImplFromJson(json);

  @JsonKey(name: 'type')
  final String $type;

  @override
  String toString() {
    return 'ClientMessage.breakEvent()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ClientMessageBreakEventImpl);
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() requestSync,
    required TResult Function() startEvent,
    required TResult Function() breakEvent,
    required TResult Function() terminateEvent,
    required TResult Function(UpdatePomodoroContext payload)
    updatePomodoroContext,
    required TResult Function(NoteUpdate payload) updateNote,
    required TResult Function(UpdateConcentrationScore payload)
    updateConcentrationScore,
  }) {
    return breakEvent();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? requestSync,
    TResult? Function()? startEvent,
    TResult? Function()? breakEvent,
    TResult? Function()? terminateEvent,
    TResult? Function(UpdatePomodoroContext payload)? updatePomodoroContext,
    TResult? Function(NoteUpdate payload)? updateNote,
    TResult? Function(UpdateConcentrationScore payload)?
    updateConcentrationScore,
  }) {
    return breakEvent?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? requestSync,
    TResult Function()? startEvent,
    TResult Function()? breakEvent,
    TResult Function()? terminateEvent,
    TResult Function(UpdatePomodoroContext payload)? updatePomodoroContext,
    TResult Function(NoteUpdate payload)? updateNote,
    TResult Function(UpdateConcentrationScore payload)?
    updateConcentrationScore,
    required TResult orElse(),
  }) {
    if (breakEvent != null) {
      return breakEvent();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ClientMessageRequestSync value) requestSync,
    required TResult Function(ClientMessageStartEvent value) startEvent,
    required TResult Function(ClientMessageBreakEvent value) breakEvent,
    required TResult Function(ClientMessageTerminateEvent value) terminateEvent,
    required TResult Function(ClientMessageUpdatePomodoroContext value)
    updatePomodoroContext,
    required TResult Function(ClientMessageUpdateNote value) updateNote,
    required TResult Function(ClientMessageUpdateConcentrationScore value)
    updateConcentrationScore,
  }) {
    return breakEvent(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ClientMessageRequestSync value)? requestSync,
    TResult? Function(ClientMessageStartEvent value)? startEvent,
    TResult? Function(ClientMessageBreakEvent value)? breakEvent,
    TResult? Function(ClientMessageTerminateEvent value)? terminateEvent,
    TResult? Function(ClientMessageUpdatePomodoroContext value)?
    updatePomodoroContext,
    TResult? Function(ClientMessageUpdateNote value)? updateNote,
    TResult? Function(ClientMessageUpdateConcentrationScore value)?
    updateConcentrationScore,
  }) {
    return breakEvent?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ClientMessageRequestSync value)? requestSync,
    TResult Function(ClientMessageStartEvent value)? startEvent,
    TResult Function(ClientMessageBreakEvent value)? breakEvent,
    TResult Function(ClientMessageTerminateEvent value)? terminateEvent,
    TResult Function(ClientMessageUpdatePomodoroContext value)?
    updatePomodoroContext,
    TResult Function(ClientMessageUpdateNote value)? updateNote,
    TResult Function(ClientMessageUpdateConcentrationScore value)?
    updateConcentrationScore,
    required TResult orElse(),
  }) {
    if (breakEvent != null) {
      return breakEvent(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$ClientMessageBreakEventImplToJson(this);
  }
}

abstract class ClientMessageBreakEvent implements ClientMessage {
  const factory ClientMessageBreakEvent() = _$ClientMessageBreakEventImpl;

  factory ClientMessageBreakEvent.fromJson(Map<String, dynamic> json) =
      _$ClientMessageBreakEventImpl.fromJson;
}

/// @nodoc
abstract class _$$ClientMessageTerminateEventImplCopyWith<$Res> {
  factory _$$ClientMessageTerminateEventImplCopyWith(
    _$ClientMessageTerminateEventImpl value,
    $Res Function(_$ClientMessageTerminateEventImpl) then,
  ) = __$$ClientMessageTerminateEventImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$ClientMessageTerminateEventImplCopyWithImpl<$Res>
    extends _$ClientMessageCopyWithImpl<$Res, _$ClientMessageTerminateEventImpl>
    implements _$$ClientMessageTerminateEventImplCopyWith<$Res> {
  __$$ClientMessageTerminateEventImplCopyWithImpl(
    _$ClientMessageTerminateEventImpl _value,
    $Res Function(_$ClientMessageTerminateEventImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ClientMessage
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
@JsonSerializable()
class _$ClientMessageTerminateEventImpl implements ClientMessageTerminateEvent {
  const _$ClientMessageTerminateEventImpl({final String? $type})
    : $type = $type ?? 'terminateEvent';

  factory _$ClientMessageTerminateEventImpl.fromJson(
    Map<String, dynamic> json,
  ) => _$$ClientMessageTerminateEventImplFromJson(json);

  @JsonKey(name: 'type')
  final String $type;

  @override
  String toString() {
    return 'ClientMessage.terminateEvent()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ClientMessageTerminateEventImpl);
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() requestSync,
    required TResult Function() startEvent,
    required TResult Function() breakEvent,
    required TResult Function() terminateEvent,
    required TResult Function(UpdatePomodoroContext payload)
    updatePomodoroContext,
    required TResult Function(NoteUpdate payload) updateNote,
    required TResult Function(UpdateConcentrationScore payload)
    updateConcentrationScore,
  }) {
    return terminateEvent();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? requestSync,
    TResult? Function()? startEvent,
    TResult? Function()? breakEvent,
    TResult? Function()? terminateEvent,
    TResult? Function(UpdatePomodoroContext payload)? updatePomodoroContext,
    TResult? Function(NoteUpdate payload)? updateNote,
    TResult? Function(UpdateConcentrationScore payload)?
    updateConcentrationScore,
  }) {
    return terminateEvent?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? requestSync,
    TResult Function()? startEvent,
    TResult Function()? breakEvent,
    TResult Function()? terminateEvent,
    TResult Function(UpdatePomodoroContext payload)? updatePomodoroContext,
    TResult Function(NoteUpdate payload)? updateNote,
    TResult Function(UpdateConcentrationScore payload)?
    updateConcentrationScore,
    required TResult orElse(),
  }) {
    if (terminateEvent != null) {
      return terminateEvent();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ClientMessageRequestSync value) requestSync,
    required TResult Function(ClientMessageStartEvent value) startEvent,
    required TResult Function(ClientMessageBreakEvent value) breakEvent,
    required TResult Function(ClientMessageTerminateEvent value) terminateEvent,
    required TResult Function(ClientMessageUpdatePomodoroContext value)
    updatePomodoroContext,
    required TResult Function(ClientMessageUpdateNote value) updateNote,
    required TResult Function(ClientMessageUpdateConcentrationScore value)
    updateConcentrationScore,
  }) {
    return terminateEvent(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ClientMessageRequestSync value)? requestSync,
    TResult? Function(ClientMessageStartEvent value)? startEvent,
    TResult? Function(ClientMessageBreakEvent value)? breakEvent,
    TResult? Function(ClientMessageTerminateEvent value)? terminateEvent,
    TResult? Function(ClientMessageUpdatePomodoroContext value)?
    updatePomodoroContext,
    TResult? Function(ClientMessageUpdateNote value)? updateNote,
    TResult? Function(ClientMessageUpdateConcentrationScore value)?
    updateConcentrationScore,
  }) {
    return terminateEvent?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ClientMessageRequestSync value)? requestSync,
    TResult Function(ClientMessageStartEvent value)? startEvent,
    TResult Function(ClientMessageBreakEvent value)? breakEvent,
    TResult Function(ClientMessageTerminateEvent value)? terminateEvent,
    TResult Function(ClientMessageUpdatePomodoroContext value)?
    updatePomodoroContext,
    TResult Function(ClientMessageUpdateNote value)? updateNote,
    TResult Function(ClientMessageUpdateConcentrationScore value)?
    updateConcentrationScore,
    required TResult orElse(),
  }) {
    if (terminateEvent != null) {
      return terminateEvent(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$ClientMessageTerminateEventImplToJson(this);
  }
}

abstract class ClientMessageTerminateEvent implements ClientMessage {
  const factory ClientMessageTerminateEvent() =
      _$ClientMessageTerminateEventImpl;

  factory ClientMessageTerminateEvent.fromJson(Map<String, dynamic> json) =
      _$ClientMessageTerminateEventImpl.fromJson;
}

/// @nodoc
abstract class _$$ClientMessageUpdatePomodoroContextImplCopyWith<$Res> {
  factory _$$ClientMessageUpdatePomodoroContextImplCopyWith(
    _$ClientMessageUpdatePomodoroContextImpl value,
    $Res Function(_$ClientMessageUpdatePomodoroContextImpl) then,
  ) = __$$ClientMessageUpdatePomodoroContextImplCopyWithImpl<$Res>;
  @useResult
  $Res call({UpdatePomodoroContext payload});

  $UpdatePomodoroContextCopyWith<$Res> get payload;
}

/// @nodoc
class __$$ClientMessageUpdatePomodoroContextImplCopyWithImpl<$Res>
    extends
        _$ClientMessageCopyWithImpl<
          $Res,
          _$ClientMessageUpdatePomodoroContextImpl
        >
    implements _$$ClientMessageUpdatePomodoroContextImplCopyWith<$Res> {
  __$$ClientMessageUpdatePomodoroContextImplCopyWithImpl(
    _$ClientMessageUpdatePomodoroContextImpl _value,
    $Res Function(_$ClientMessageUpdatePomodoroContextImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ClientMessage
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? payload = null}) {
    return _then(
      _$ClientMessageUpdatePomodoroContextImpl(
        null == payload
            ? _value.payload
            : payload // ignore: cast_nullable_to_non_nullable
                as UpdatePomodoroContext,
      ),
    );
  }

  /// Create a copy of ClientMessage
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $UpdatePomodoroContextCopyWith<$Res> get payload {
    return $UpdatePomodoroContextCopyWith<$Res>(_value.payload, (value) {
      return _then(_value.copyWith(payload: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _$ClientMessageUpdatePomodoroContextImpl
    implements ClientMessageUpdatePomodoroContext {
  const _$ClientMessageUpdatePomodoroContextImpl(
    this.payload, {
    final String? $type,
  }) : $type = $type ?? 'updatePomodoroContext';

  factory _$ClientMessageUpdatePomodoroContextImpl.fromJson(
    Map<String, dynamic> json,
  ) => _$$ClientMessageUpdatePomodoroContextImplFromJson(json);

  @override
  final UpdatePomodoroContext payload;

  @JsonKey(name: 'type')
  final String $type;

  @override
  String toString() {
    return 'ClientMessage.updatePomodoroContext(payload: $payload)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ClientMessageUpdatePomodoroContextImpl &&
            (identical(other.payload, payload) || other.payload == payload));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, payload);

  /// Create a copy of ClientMessage
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ClientMessageUpdatePomodoroContextImplCopyWith<
    _$ClientMessageUpdatePomodoroContextImpl
  >
  get copyWith => __$$ClientMessageUpdatePomodoroContextImplCopyWithImpl<
    _$ClientMessageUpdatePomodoroContextImpl
  >(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() requestSync,
    required TResult Function() startEvent,
    required TResult Function() breakEvent,
    required TResult Function() terminateEvent,
    required TResult Function(UpdatePomodoroContext payload)
    updatePomodoroContext,
    required TResult Function(NoteUpdate payload) updateNote,
    required TResult Function(UpdateConcentrationScore payload)
    updateConcentrationScore,
  }) {
    return updatePomodoroContext(payload);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? requestSync,
    TResult? Function()? startEvent,
    TResult? Function()? breakEvent,
    TResult? Function()? terminateEvent,
    TResult? Function(UpdatePomodoroContext payload)? updatePomodoroContext,
    TResult? Function(NoteUpdate payload)? updateNote,
    TResult? Function(UpdateConcentrationScore payload)?
    updateConcentrationScore,
  }) {
    return updatePomodoroContext?.call(payload);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? requestSync,
    TResult Function()? startEvent,
    TResult Function()? breakEvent,
    TResult Function()? terminateEvent,
    TResult Function(UpdatePomodoroContext payload)? updatePomodoroContext,
    TResult Function(NoteUpdate payload)? updateNote,
    TResult Function(UpdateConcentrationScore payload)?
    updateConcentrationScore,
    required TResult orElse(),
  }) {
    if (updatePomodoroContext != null) {
      return updatePomodoroContext(payload);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ClientMessageRequestSync value) requestSync,
    required TResult Function(ClientMessageStartEvent value) startEvent,
    required TResult Function(ClientMessageBreakEvent value) breakEvent,
    required TResult Function(ClientMessageTerminateEvent value) terminateEvent,
    required TResult Function(ClientMessageUpdatePomodoroContext value)
    updatePomodoroContext,
    required TResult Function(ClientMessageUpdateNote value) updateNote,
    required TResult Function(ClientMessageUpdateConcentrationScore value)
    updateConcentrationScore,
  }) {
    return updatePomodoroContext(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ClientMessageRequestSync value)? requestSync,
    TResult? Function(ClientMessageStartEvent value)? startEvent,
    TResult? Function(ClientMessageBreakEvent value)? breakEvent,
    TResult? Function(ClientMessageTerminateEvent value)? terminateEvent,
    TResult? Function(ClientMessageUpdatePomodoroContext value)?
    updatePomodoroContext,
    TResult? Function(ClientMessageUpdateNote value)? updateNote,
    TResult? Function(ClientMessageUpdateConcentrationScore value)?
    updateConcentrationScore,
  }) {
    return updatePomodoroContext?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ClientMessageRequestSync value)? requestSync,
    TResult Function(ClientMessageStartEvent value)? startEvent,
    TResult Function(ClientMessageBreakEvent value)? breakEvent,
    TResult Function(ClientMessageTerminateEvent value)? terminateEvent,
    TResult Function(ClientMessageUpdatePomodoroContext value)?
    updatePomodoroContext,
    TResult Function(ClientMessageUpdateNote value)? updateNote,
    TResult Function(ClientMessageUpdateConcentrationScore value)?
    updateConcentrationScore,
    required TResult orElse(),
  }) {
    if (updatePomodoroContext != null) {
      return updatePomodoroContext(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$ClientMessageUpdatePomodoroContextImplToJson(this);
  }
}

abstract class ClientMessageUpdatePomodoroContext implements ClientMessage {
  const factory ClientMessageUpdatePomodoroContext(
    final UpdatePomodoroContext payload,
  ) = _$ClientMessageUpdatePomodoroContextImpl;

  factory ClientMessageUpdatePomodoroContext.fromJson(
    Map<String, dynamic> json,
  ) = _$ClientMessageUpdatePomodoroContextImpl.fromJson;

  UpdatePomodoroContext get payload;

  /// Create a copy of ClientMessage
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ClientMessageUpdatePomodoroContextImplCopyWith<
    _$ClientMessageUpdatePomodoroContextImpl
  >
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ClientMessageUpdateNoteImplCopyWith<$Res> {
  factory _$$ClientMessageUpdateNoteImplCopyWith(
    _$ClientMessageUpdateNoteImpl value,
    $Res Function(_$ClientMessageUpdateNoteImpl) then,
  ) = __$$ClientMessageUpdateNoteImplCopyWithImpl<$Res>;
  @useResult
  $Res call({NoteUpdate payload});

  $NoteUpdateCopyWith<$Res> get payload;
}

/// @nodoc
class __$$ClientMessageUpdateNoteImplCopyWithImpl<$Res>
    extends _$ClientMessageCopyWithImpl<$Res, _$ClientMessageUpdateNoteImpl>
    implements _$$ClientMessageUpdateNoteImplCopyWith<$Res> {
  __$$ClientMessageUpdateNoteImplCopyWithImpl(
    _$ClientMessageUpdateNoteImpl _value,
    $Res Function(_$ClientMessageUpdateNoteImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ClientMessage
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? payload = null}) {
    return _then(
      _$ClientMessageUpdateNoteImpl(
        null == payload
            ? _value.payload
            : payload // ignore: cast_nullable_to_non_nullable
                as NoteUpdate,
      ),
    );
  }

  /// Create a copy of ClientMessage
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $NoteUpdateCopyWith<$Res> get payload {
    return $NoteUpdateCopyWith<$Res>(_value.payload, (value) {
      return _then(_value.copyWith(payload: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _$ClientMessageUpdateNoteImpl implements ClientMessageUpdateNote {
  const _$ClientMessageUpdateNoteImpl(this.payload, {final String? $type})
    : $type = $type ?? 'updateNote';

  factory _$ClientMessageUpdateNoteImpl.fromJson(Map<String, dynamic> json) =>
      _$$ClientMessageUpdateNoteImplFromJson(json);

  @override
  final NoteUpdate payload;

  @JsonKey(name: 'type')
  final String $type;

  @override
  String toString() {
    return 'ClientMessage.updateNote(payload: $payload)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ClientMessageUpdateNoteImpl &&
            (identical(other.payload, payload) || other.payload == payload));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, payload);

  /// Create a copy of ClientMessage
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ClientMessageUpdateNoteImplCopyWith<_$ClientMessageUpdateNoteImpl>
  get copyWith => __$$ClientMessageUpdateNoteImplCopyWithImpl<
    _$ClientMessageUpdateNoteImpl
  >(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() requestSync,
    required TResult Function() startEvent,
    required TResult Function() breakEvent,
    required TResult Function() terminateEvent,
    required TResult Function(UpdatePomodoroContext payload)
    updatePomodoroContext,
    required TResult Function(NoteUpdate payload) updateNote,
    required TResult Function(UpdateConcentrationScore payload)
    updateConcentrationScore,
  }) {
    return updateNote(payload);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? requestSync,
    TResult? Function()? startEvent,
    TResult? Function()? breakEvent,
    TResult? Function()? terminateEvent,
    TResult? Function(UpdatePomodoroContext payload)? updatePomodoroContext,
    TResult? Function(NoteUpdate payload)? updateNote,
    TResult? Function(UpdateConcentrationScore payload)?
    updateConcentrationScore,
  }) {
    return updateNote?.call(payload);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? requestSync,
    TResult Function()? startEvent,
    TResult Function()? breakEvent,
    TResult Function()? terminateEvent,
    TResult Function(UpdatePomodoroContext payload)? updatePomodoroContext,
    TResult Function(NoteUpdate payload)? updateNote,
    TResult Function(UpdateConcentrationScore payload)?
    updateConcentrationScore,
    required TResult orElse(),
  }) {
    if (updateNote != null) {
      return updateNote(payload);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ClientMessageRequestSync value) requestSync,
    required TResult Function(ClientMessageStartEvent value) startEvent,
    required TResult Function(ClientMessageBreakEvent value) breakEvent,
    required TResult Function(ClientMessageTerminateEvent value) terminateEvent,
    required TResult Function(ClientMessageUpdatePomodoroContext value)
    updatePomodoroContext,
    required TResult Function(ClientMessageUpdateNote value) updateNote,
    required TResult Function(ClientMessageUpdateConcentrationScore value)
    updateConcentrationScore,
  }) {
    return updateNote(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ClientMessageRequestSync value)? requestSync,
    TResult? Function(ClientMessageStartEvent value)? startEvent,
    TResult? Function(ClientMessageBreakEvent value)? breakEvent,
    TResult? Function(ClientMessageTerminateEvent value)? terminateEvent,
    TResult? Function(ClientMessageUpdatePomodoroContext value)?
    updatePomodoroContext,
    TResult? Function(ClientMessageUpdateNote value)? updateNote,
    TResult? Function(ClientMessageUpdateConcentrationScore value)?
    updateConcentrationScore,
  }) {
    return updateNote?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ClientMessageRequestSync value)? requestSync,
    TResult Function(ClientMessageStartEvent value)? startEvent,
    TResult Function(ClientMessageBreakEvent value)? breakEvent,
    TResult Function(ClientMessageTerminateEvent value)? terminateEvent,
    TResult Function(ClientMessageUpdatePomodoroContext value)?
    updatePomodoroContext,
    TResult Function(ClientMessageUpdateNote value)? updateNote,
    TResult Function(ClientMessageUpdateConcentrationScore value)?
    updateConcentrationScore,
    required TResult orElse(),
  }) {
    if (updateNote != null) {
      return updateNote(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$ClientMessageUpdateNoteImplToJson(this);
  }
}

abstract class ClientMessageUpdateNote implements ClientMessage {
  const factory ClientMessageUpdateNote(final NoteUpdate payload) =
      _$ClientMessageUpdateNoteImpl;

  factory ClientMessageUpdateNote.fromJson(Map<String, dynamic> json) =
      _$ClientMessageUpdateNoteImpl.fromJson;

  NoteUpdate get payload;

  /// Create a copy of ClientMessage
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ClientMessageUpdateNoteImplCopyWith<_$ClientMessageUpdateNoteImpl>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ClientMessageUpdateConcentrationScoreImplCopyWith<$Res> {
  factory _$$ClientMessageUpdateConcentrationScoreImplCopyWith(
    _$ClientMessageUpdateConcentrationScoreImpl value,
    $Res Function(_$ClientMessageUpdateConcentrationScoreImpl) then,
  ) = __$$ClientMessageUpdateConcentrationScoreImplCopyWithImpl<$Res>;
  @useResult
  $Res call({UpdateConcentrationScore payload});

  $UpdateConcentrationScoreCopyWith<$Res> get payload;
}

/// @nodoc
class __$$ClientMessageUpdateConcentrationScoreImplCopyWithImpl<$Res>
    extends
        _$ClientMessageCopyWithImpl<
          $Res,
          _$ClientMessageUpdateConcentrationScoreImpl
        >
    implements _$$ClientMessageUpdateConcentrationScoreImplCopyWith<$Res> {
  __$$ClientMessageUpdateConcentrationScoreImplCopyWithImpl(
    _$ClientMessageUpdateConcentrationScoreImpl _value,
    $Res Function(_$ClientMessageUpdateConcentrationScoreImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ClientMessage
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? payload = null}) {
    return _then(
      _$ClientMessageUpdateConcentrationScoreImpl(
        null == payload
            ? _value.payload
            : payload // ignore: cast_nullable_to_non_nullable
                as UpdateConcentrationScore,
      ),
    );
  }

  /// Create a copy of ClientMessage
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $UpdateConcentrationScoreCopyWith<$Res> get payload {
    return $UpdateConcentrationScoreCopyWith<$Res>(_value.payload, (value) {
      return _then(_value.copyWith(payload: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _$ClientMessageUpdateConcentrationScoreImpl
    implements ClientMessageUpdateConcentrationScore {
  const _$ClientMessageUpdateConcentrationScoreImpl(
    this.payload, {
    final String? $type,
  }) : $type = $type ?? 'updateConcentrationScore';

  factory _$ClientMessageUpdateConcentrationScoreImpl.fromJson(
    Map<String, dynamic> json,
  ) => _$$ClientMessageUpdateConcentrationScoreImplFromJson(json);

  @override
  final UpdateConcentrationScore payload;

  @JsonKey(name: 'type')
  final String $type;

  @override
  String toString() {
    return 'ClientMessage.updateConcentrationScore(payload: $payload)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ClientMessageUpdateConcentrationScoreImpl &&
            (identical(other.payload, payload) || other.payload == payload));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, payload);

  /// Create a copy of ClientMessage
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ClientMessageUpdateConcentrationScoreImplCopyWith<
    _$ClientMessageUpdateConcentrationScoreImpl
  >
  get copyWith => __$$ClientMessageUpdateConcentrationScoreImplCopyWithImpl<
    _$ClientMessageUpdateConcentrationScoreImpl
  >(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() requestSync,
    required TResult Function() startEvent,
    required TResult Function() breakEvent,
    required TResult Function() terminateEvent,
    required TResult Function(UpdatePomodoroContext payload)
    updatePomodoroContext,
    required TResult Function(NoteUpdate payload) updateNote,
    required TResult Function(UpdateConcentrationScore payload)
    updateConcentrationScore,
  }) {
    return updateConcentrationScore(payload);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? requestSync,
    TResult? Function()? startEvent,
    TResult? Function()? breakEvent,
    TResult? Function()? terminateEvent,
    TResult? Function(UpdatePomodoroContext payload)? updatePomodoroContext,
    TResult? Function(NoteUpdate payload)? updateNote,
    TResult? Function(UpdateConcentrationScore payload)?
    updateConcentrationScore,
  }) {
    return updateConcentrationScore?.call(payload);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? requestSync,
    TResult Function()? startEvent,
    TResult Function()? breakEvent,
    TResult Function()? terminateEvent,
    TResult Function(UpdatePomodoroContext payload)? updatePomodoroContext,
    TResult Function(NoteUpdate payload)? updateNote,
    TResult Function(UpdateConcentrationScore payload)?
    updateConcentrationScore,
    required TResult orElse(),
  }) {
    if (updateConcentrationScore != null) {
      return updateConcentrationScore(payload);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ClientMessageRequestSync value) requestSync,
    required TResult Function(ClientMessageStartEvent value) startEvent,
    required TResult Function(ClientMessageBreakEvent value) breakEvent,
    required TResult Function(ClientMessageTerminateEvent value) terminateEvent,
    required TResult Function(ClientMessageUpdatePomodoroContext value)
    updatePomodoroContext,
    required TResult Function(ClientMessageUpdateNote value) updateNote,
    required TResult Function(ClientMessageUpdateConcentrationScore value)
    updateConcentrationScore,
  }) {
    return updateConcentrationScore(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ClientMessageRequestSync value)? requestSync,
    TResult? Function(ClientMessageStartEvent value)? startEvent,
    TResult? Function(ClientMessageBreakEvent value)? breakEvent,
    TResult? Function(ClientMessageTerminateEvent value)? terminateEvent,
    TResult? Function(ClientMessageUpdatePomodoroContext value)?
    updatePomodoroContext,
    TResult? Function(ClientMessageUpdateNote value)? updateNote,
    TResult? Function(ClientMessageUpdateConcentrationScore value)?
    updateConcentrationScore,
  }) {
    return updateConcentrationScore?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ClientMessageRequestSync value)? requestSync,
    TResult Function(ClientMessageStartEvent value)? startEvent,
    TResult Function(ClientMessageBreakEvent value)? breakEvent,
    TResult Function(ClientMessageTerminateEvent value)? terminateEvent,
    TResult Function(ClientMessageUpdatePomodoroContext value)?
    updatePomodoroContext,
    TResult Function(ClientMessageUpdateNote value)? updateNote,
    TResult Function(ClientMessageUpdateConcentrationScore value)?
    updateConcentrationScore,
    required TResult orElse(),
  }) {
    if (updateConcentrationScore != null) {
      return updateConcentrationScore(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$ClientMessageUpdateConcentrationScoreImplToJson(this);
  }
}

abstract class ClientMessageUpdateConcentrationScore implements ClientMessage {
  const factory ClientMessageUpdateConcentrationScore(
    final UpdateConcentrationScore payload,
  ) = _$ClientMessageUpdateConcentrationScoreImpl;

  factory ClientMessageUpdateConcentrationScore.fromJson(
    Map<String, dynamic> json,
  ) = _$ClientMessageUpdateConcentrationScoreImpl.fromJson;

  UpdateConcentrationScore get payload;

  /// Create a copy of ClientMessage
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ClientMessageUpdateConcentrationScoreImplCopyWith<
    _$ClientMessageUpdateConcentrationScoreImpl
  >
  get copyWith => throw _privateConstructorUsedError;
}

ServerResponse _$ServerResponseFromJson(Map<String, dynamic> json) {
  switch (json['type']) {
    case 'success':
      return ServerResponseSuccess.fromJson(json);
    case 'error':
      return ServerResponseError.fromJson(json);
    case 'syncData':
      return ServerResponseSyncData.fromJson(json);

    default:
      throw CheckedFromJsonException(
        json,
        'type',
        'ServerResponse',
        'Invalid union type "${json['type']}"!',
      );
  }
}

/// @nodoc
mixin _$ServerResponse {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message, String? requestId) success,
    required TResult Function(String code, String message, String? requestId)
    error,
    required TResult Function(UpdatePomodoroState payload) syncData,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message, String? requestId)? success,
    TResult? Function(String code, String message, String? requestId)? error,
    TResult? Function(UpdatePomodoroState payload)? syncData,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message, String? requestId)? success,
    TResult Function(String code, String message, String? requestId)? error,
    TResult Function(UpdatePomodoroState payload)? syncData,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ServerResponseSuccess value) success,
    required TResult Function(ServerResponseError value) error,
    required TResult Function(ServerResponseSyncData value) syncData,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ServerResponseSuccess value)? success,
    TResult? Function(ServerResponseError value)? error,
    TResult? Function(ServerResponseSyncData value)? syncData,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ServerResponseSuccess value)? success,
    TResult Function(ServerResponseError value)? error,
    TResult Function(ServerResponseSyncData value)? syncData,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;

  /// Serializes this ServerResponse to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ServerResponseCopyWith<$Res> {
  factory $ServerResponseCopyWith(
    ServerResponse value,
    $Res Function(ServerResponse) then,
  ) = _$ServerResponseCopyWithImpl<$Res, ServerResponse>;
}

/// @nodoc
class _$ServerResponseCopyWithImpl<$Res, $Val extends ServerResponse>
    implements $ServerResponseCopyWith<$Res> {
  _$ServerResponseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of ServerResponse
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$ServerResponseSuccessImplCopyWith<$Res> {
  factory _$$ServerResponseSuccessImplCopyWith(
    _$ServerResponseSuccessImpl value,
    $Res Function(_$ServerResponseSuccessImpl) then,
  ) = __$$ServerResponseSuccessImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String message, String? requestId});
}

/// @nodoc
class __$$ServerResponseSuccessImplCopyWithImpl<$Res>
    extends _$ServerResponseCopyWithImpl<$Res, _$ServerResponseSuccessImpl>
    implements _$$ServerResponseSuccessImplCopyWith<$Res> {
  __$$ServerResponseSuccessImplCopyWithImpl(
    _$ServerResponseSuccessImpl _value,
    $Res Function(_$ServerResponseSuccessImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ServerResponse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? message = null, Object? requestId = freezed}) {
    return _then(
      _$ServerResponseSuccessImpl(
        message:
            null == message
                ? _value.message
                : message // ignore: cast_nullable_to_non_nullable
                    as String,
        requestId:
            freezed == requestId
                ? _value.requestId
                : requestId // ignore: cast_nullable_to_non_nullable
                    as String?,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$ServerResponseSuccessImpl implements ServerResponseSuccess {
  const _$ServerResponseSuccessImpl({
    required this.message,
    this.requestId,
    final String? $type,
  }) : $type = $type ?? 'success';

  factory _$ServerResponseSuccessImpl.fromJson(Map<String, dynamic> json) =>
      _$$ServerResponseSuccessImplFromJson(json);

  @override
  final String message;
  @override
  final String? requestId;

  @JsonKey(name: 'type')
  final String $type;

  @override
  String toString() {
    return 'ServerResponse.success(message: $message, requestId: $requestId)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ServerResponseSuccessImpl &&
            (identical(other.message, message) || other.message == message) &&
            (identical(other.requestId, requestId) ||
                other.requestId == requestId));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, message, requestId);

  /// Create a copy of ServerResponse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ServerResponseSuccessImplCopyWith<_$ServerResponseSuccessImpl>
  get copyWith =>
      __$$ServerResponseSuccessImplCopyWithImpl<_$ServerResponseSuccessImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message, String? requestId) success,
    required TResult Function(String code, String message, String? requestId)
    error,
    required TResult Function(UpdatePomodoroState payload) syncData,
  }) {
    return success(message, requestId);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message, String? requestId)? success,
    TResult? Function(String code, String message, String? requestId)? error,
    TResult? Function(UpdatePomodoroState payload)? syncData,
  }) {
    return success?.call(message, requestId);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message, String? requestId)? success,
    TResult Function(String code, String message, String? requestId)? error,
    TResult Function(UpdatePomodoroState payload)? syncData,
    required TResult orElse(),
  }) {
    if (success != null) {
      return success(message, requestId);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ServerResponseSuccess value) success,
    required TResult Function(ServerResponseError value) error,
    required TResult Function(ServerResponseSyncData value) syncData,
  }) {
    return success(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ServerResponseSuccess value)? success,
    TResult? Function(ServerResponseError value)? error,
    TResult? Function(ServerResponseSyncData value)? syncData,
  }) {
    return success?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ServerResponseSuccess value)? success,
    TResult Function(ServerResponseError value)? error,
    TResult Function(ServerResponseSyncData value)? syncData,
    required TResult orElse(),
  }) {
    if (success != null) {
      return success(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$ServerResponseSuccessImplToJson(this);
  }
}

abstract class ServerResponseSuccess implements ServerResponse {
  const factory ServerResponseSuccess({
    required final String message,
    final String? requestId,
  }) = _$ServerResponseSuccessImpl;

  factory ServerResponseSuccess.fromJson(Map<String, dynamic> json) =
      _$ServerResponseSuccessImpl.fromJson;

  String get message;
  String? get requestId;

  /// Create a copy of ServerResponse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ServerResponseSuccessImplCopyWith<_$ServerResponseSuccessImpl>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ServerResponseErrorImplCopyWith<$Res> {
  factory _$$ServerResponseErrorImplCopyWith(
    _$ServerResponseErrorImpl value,
    $Res Function(_$ServerResponseErrorImpl) then,
  ) = __$$ServerResponseErrorImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String code, String message, String? requestId});
}

/// @nodoc
class __$$ServerResponseErrorImplCopyWithImpl<$Res>
    extends _$ServerResponseCopyWithImpl<$Res, _$ServerResponseErrorImpl>
    implements _$$ServerResponseErrorImplCopyWith<$Res> {
  __$$ServerResponseErrorImplCopyWithImpl(
    _$ServerResponseErrorImpl _value,
    $Res Function(_$ServerResponseErrorImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ServerResponse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? code = null,
    Object? message = null,
    Object? requestId = freezed,
  }) {
    return _then(
      _$ServerResponseErrorImpl(
        code:
            null == code
                ? _value.code
                : code // ignore: cast_nullable_to_non_nullable
                    as String,
        message:
            null == message
                ? _value.message
                : message // ignore: cast_nullable_to_non_nullable
                    as String,
        requestId:
            freezed == requestId
                ? _value.requestId
                : requestId // ignore: cast_nullable_to_non_nullable
                    as String?,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$ServerResponseErrorImpl implements ServerResponseError {
  const _$ServerResponseErrorImpl({
    required this.code,
    required this.message,
    this.requestId,
    final String? $type,
  }) : $type = $type ?? 'error';

  factory _$ServerResponseErrorImpl.fromJson(Map<String, dynamic> json) =>
      _$$ServerResponseErrorImplFromJson(json);

  @override
  final String code;
  @override
  final String message;
  @override
  final String? requestId;

  @JsonKey(name: 'type')
  final String $type;

  @override
  String toString() {
    return 'ServerResponse.error(code: $code, message: $message, requestId: $requestId)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ServerResponseErrorImpl &&
            (identical(other.code, code) || other.code == code) &&
            (identical(other.message, message) || other.message == message) &&
            (identical(other.requestId, requestId) ||
                other.requestId == requestId));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, code, message, requestId);

  /// Create a copy of ServerResponse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ServerResponseErrorImplCopyWith<_$ServerResponseErrorImpl> get copyWith =>
      __$$ServerResponseErrorImplCopyWithImpl<_$ServerResponseErrorImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message, String? requestId) success,
    required TResult Function(String code, String message, String? requestId)
    error,
    required TResult Function(UpdatePomodoroState payload) syncData,
  }) {
    return error(code, message, requestId);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message, String? requestId)? success,
    TResult? Function(String code, String message, String? requestId)? error,
    TResult? Function(UpdatePomodoroState payload)? syncData,
  }) {
    return error?.call(code, message, requestId);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message, String? requestId)? success,
    TResult Function(String code, String message, String? requestId)? error,
    TResult Function(UpdatePomodoroState payload)? syncData,
    required TResult orElse(),
  }) {
    if (error != null) {
      return error(code, message, requestId);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ServerResponseSuccess value) success,
    required TResult Function(ServerResponseError value) error,
    required TResult Function(ServerResponseSyncData value) syncData,
  }) {
    return error(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ServerResponseSuccess value)? success,
    TResult? Function(ServerResponseError value)? error,
    TResult? Function(ServerResponseSyncData value)? syncData,
  }) {
    return error?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ServerResponseSuccess value)? success,
    TResult Function(ServerResponseError value)? error,
    TResult Function(ServerResponseSyncData value)? syncData,
    required TResult orElse(),
  }) {
    if (error != null) {
      return error(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$ServerResponseErrorImplToJson(this);
  }
}

abstract class ServerResponseError implements ServerResponse {
  const factory ServerResponseError({
    required final String code,
    required final String message,
    final String? requestId,
  }) = _$ServerResponseErrorImpl;

  factory ServerResponseError.fromJson(Map<String, dynamic> json) =
      _$ServerResponseErrorImpl.fromJson;

  String get code;
  String get message;
  String? get requestId;

  /// Create a copy of ServerResponse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ServerResponseErrorImplCopyWith<_$ServerResponseErrorImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ServerResponseSyncDataImplCopyWith<$Res> {
  factory _$$ServerResponseSyncDataImplCopyWith(
    _$ServerResponseSyncDataImpl value,
    $Res Function(_$ServerResponseSyncDataImpl) then,
  ) = __$$ServerResponseSyncDataImplCopyWithImpl<$Res>;
  @useResult
  $Res call({UpdatePomodoroState payload});

  $UpdatePomodoroStateCopyWith<$Res> get payload;
}

/// @nodoc
class __$$ServerResponseSyncDataImplCopyWithImpl<$Res>
    extends _$ServerResponseCopyWithImpl<$Res, _$ServerResponseSyncDataImpl>
    implements _$$ServerResponseSyncDataImplCopyWith<$Res> {
  __$$ServerResponseSyncDataImplCopyWithImpl(
    _$ServerResponseSyncDataImpl _value,
    $Res Function(_$ServerResponseSyncDataImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ServerResponse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? payload = null}) {
    return _then(
      _$ServerResponseSyncDataImpl(
        null == payload
            ? _value.payload
            : payload // ignore: cast_nullable_to_non_nullable
                as UpdatePomodoroState,
      ),
    );
  }

  /// Create a copy of ServerResponse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $UpdatePomodoroStateCopyWith<$Res> get payload {
    return $UpdatePomodoroStateCopyWith<$Res>(_value.payload, (value) {
      return _then(_value.copyWith(payload: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _$ServerResponseSyncDataImpl implements ServerResponseSyncData {
  const _$ServerResponseSyncDataImpl(this.payload, {final String? $type})
    : $type = $type ?? 'syncData';

  factory _$ServerResponseSyncDataImpl.fromJson(Map<String, dynamic> json) =>
      _$$ServerResponseSyncDataImplFromJson(json);

  @override
  final UpdatePomodoroState payload;

  @JsonKey(name: 'type')
  final String $type;

  @override
  String toString() {
    return 'ServerResponse.syncData(payload: $payload)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ServerResponseSyncDataImpl &&
            (identical(other.payload, payload) || other.payload == payload));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, payload);

  /// Create a copy of ServerResponse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ServerResponseSyncDataImplCopyWith<_$ServerResponseSyncDataImpl>
  get copyWith =>
      __$$ServerResponseSyncDataImplCopyWithImpl<_$ServerResponseSyncDataImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message, String? requestId) success,
    required TResult Function(String code, String message, String? requestId)
    error,
    required TResult Function(UpdatePomodoroState payload) syncData,
  }) {
    return syncData(payload);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message, String? requestId)? success,
    TResult? Function(String code, String message, String? requestId)? error,
    TResult? Function(UpdatePomodoroState payload)? syncData,
  }) {
    return syncData?.call(payload);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message, String? requestId)? success,
    TResult Function(String code, String message, String? requestId)? error,
    TResult Function(UpdatePomodoroState payload)? syncData,
    required TResult orElse(),
  }) {
    if (syncData != null) {
      return syncData(payload);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ServerResponseSuccess value) success,
    required TResult Function(ServerResponseError value) error,
    required TResult Function(ServerResponseSyncData value) syncData,
  }) {
    return syncData(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ServerResponseSuccess value)? success,
    TResult? Function(ServerResponseError value)? error,
    TResult? Function(ServerResponseSyncData value)? syncData,
  }) {
    return syncData?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ServerResponseSuccess value)? success,
    TResult Function(ServerResponseError value)? error,
    TResult Function(ServerResponseSyncData value)? syncData,
    required TResult orElse(),
  }) {
    if (syncData != null) {
      return syncData(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$ServerResponseSyncDataImplToJson(this);
  }
}

abstract class ServerResponseSyncData implements ServerResponse {
  const factory ServerResponseSyncData(final UpdatePomodoroState payload) =
      _$ServerResponseSyncDataImpl;

  factory ServerResponseSyncData.fromJson(Map<String, dynamic> json) =
      _$ServerResponseSyncDataImpl.fromJson;

  UpdatePomodoroState get payload;

  /// Create a copy of ServerResponse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ServerResponseSyncDataImplCopyWith<_$ServerResponseSyncDataImpl>
  get copyWith => throw _privateConstructorUsedError;
}

BroadcastEvent _$BroadcastEventFromJson(Map<String, dynamic> json) {
  return BroadcastEventPomodoroSessionUpdate.fromJson(json);
}

/// @nodoc
mixin _$BroadcastEvent {
  UpdatePomodoroState get payload => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(UpdatePomodoroState payload)
    pomodoroSessionUpdate,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(UpdatePomodoroState payload)? pomodoroSessionUpdate,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(UpdatePomodoroState payload)? pomodoroSessionUpdate,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(BroadcastEventPomodoroSessionUpdate value)
    pomodoroSessionUpdate,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(BroadcastEventPomodoroSessionUpdate value)?
    pomodoroSessionUpdate,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(BroadcastEventPomodoroSessionUpdate value)?
    pomodoroSessionUpdate,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;

  /// Serializes this BroadcastEvent to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of BroadcastEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $BroadcastEventCopyWith<BroadcastEvent> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $BroadcastEventCopyWith<$Res> {
  factory $BroadcastEventCopyWith(
    BroadcastEvent value,
    $Res Function(BroadcastEvent) then,
  ) = _$BroadcastEventCopyWithImpl<$Res, BroadcastEvent>;
  @useResult
  $Res call({UpdatePomodoroState payload});

  $UpdatePomodoroStateCopyWith<$Res> get payload;
}

/// @nodoc
class _$BroadcastEventCopyWithImpl<$Res, $Val extends BroadcastEvent>
    implements $BroadcastEventCopyWith<$Res> {
  _$BroadcastEventCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of BroadcastEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? payload = null}) {
    return _then(
      _value.copyWith(
            payload:
                null == payload
                    ? _value.payload
                    : payload // ignore: cast_nullable_to_non_nullable
                        as UpdatePomodoroState,
          )
          as $Val,
    );
  }

  /// Create a copy of BroadcastEvent
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $UpdatePomodoroStateCopyWith<$Res> get payload {
    return $UpdatePomodoroStateCopyWith<$Res>(_value.payload, (value) {
      return _then(_value.copyWith(payload: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$BroadcastEventPomodoroSessionUpdateImplCopyWith<$Res>
    implements $BroadcastEventCopyWith<$Res> {
  factory _$$BroadcastEventPomodoroSessionUpdateImplCopyWith(
    _$BroadcastEventPomodoroSessionUpdateImpl value,
    $Res Function(_$BroadcastEventPomodoroSessionUpdateImpl) then,
  ) = __$$BroadcastEventPomodoroSessionUpdateImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({UpdatePomodoroState payload});

  @override
  $UpdatePomodoroStateCopyWith<$Res> get payload;
}

/// @nodoc
class __$$BroadcastEventPomodoroSessionUpdateImplCopyWithImpl<$Res>
    extends
        _$BroadcastEventCopyWithImpl<
          $Res,
          _$BroadcastEventPomodoroSessionUpdateImpl
        >
    implements _$$BroadcastEventPomodoroSessionUpdateImplCopyWith<$Res> {
  __$$BroadcastEventPomodoroSessionUpdateImplCopyWithImpl(
    _$BroadcastEventPomodoroSessionUpdateImpl _value,
    $Res Function(_$BroadcastEventPomodoroSessionUpdateImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of BroadcastEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? payload = null}) {
    return _then(
      _$BroadcastEventPomodoroSessionUpdateImpl(
        null == payload
            ? _value.payload
            : payload // ignore: cast_nullable_to_non_nullable
                as UpdatePomodoroState,
      ),
    );
  }
}

/// @nodoc
@JsonSerializable()
class _$BroadcastEventPomodoroSessionUpdateImpl
    implements BroadcastEventPomodoroSessionUpdate {
  const _$BroadcastEventPomodoroSessionUpdateImpl(this.payload);

  factory _$BroadcastEventPomodoroSessionUpdateImpl.fromJson(
    Map<String, dynamic> json,
  ) => _$$BroadcastEventPomodoroSessionUpdateImplFromJson(json);

  @override
  final UpdatePomodoroState payload;

  @override
  String toString() {
    return 'BroadcastEvent.pomodoroSessionUpdate(payload: $payload)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$BroadcastEventPomodoroSessionUpdateImpl &&
            (identical(other.payload, payload) || other.payload == payload));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, payload);

  /// Create a copy of BroadcastEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$BroadcastEventPomodoroSessionUpdateImplCopyWith<
    _$BroadcastEventPomodoroSessionUpdateImpl
  >
  get copyWith => __$$BroadcastEventPomodoroSessionUpdateImplCopyWithImpl<
    _$BroadcastEventPomodoroSessionUpdateImpl
  >(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(UpdatePomodoroState payload)
    pomodoroSessionUpdate,
  }) {
    return pomodoroSessionUpdate(payload);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(UpdatePomodoroState payload)? pomodoroSessionUpdate,
  }) {
    return pomodoroSessionUpdate?.call(payload);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(UpdatePomodoroState payload)? pomodoroSessionUpdate,
    required TResult orElse(),
  }) {
    if (pomodoroSessionUpdate != null) {
      return pomodoroSessionUpdate(payload);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(BroadcastEventPomodoroSessionUpdate value)
    pomodoroSessionUpdate,
  }) {
    return pomodoroSessionUpdate(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(BroadcastEventPomodoroSessionUpdate value)?
    pomodoroSessionUpdate,
  }) {
    return pomodoroSessionUpdate?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(BroadcastEventPomodoroSessionUpdate value)?
    pomodoroSessionUpdate,
    required TResult orElse(),
  }) {
    if (pomodoroSessionUpdate != null) {
      return pomodoroSessionUpdate(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$BroadcastEventPomodoroSessionUpdateImplToJson(this);
  }
}

abstract class BroadcastEventPomodoroSessionUpdate implements BroadcastEvent {
  const factory BroadcastEventPomodoroSessionUpdate(
    final UpdatePomodoroState payload,
  ) = _$BroadcastEventPomodoroSessionUpdateImpl;

  factory BroadcastEventPomodoroSessionUpdate.fromJson(
    Map<String, dynamic> json,
  ) = _$BroadcastEventPomodoroSessionUpdateImpl.fromJson;

  @override
  UpdatePomodoroState get payload;

  /// Create a copy of BroadcastEvent
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$BroadcastEventPomodoroSessionUpdateImplCopyWith<
    _$BroadcastEventPomodoroSessionUpdateImpl
  >
  get copyWith => throw _privateConstructorUsedError;
}
