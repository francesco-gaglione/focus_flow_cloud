import 'package:freezed_annotation/freezed_annotation.dart';
import '../../domain/entities/focus_session.dart';

part 'session_dtos.freezed.dart';
part 'session_dtos.g.dart';

// Request DTOs

@freezed
class CreateManualSessionDto with _$CreateManualSessionDto {
  const CreateManualSessionDto._();

  const factory CreateManualSessionDto({
    required String sessionType,
    required int startedAt,
    required int endedAt,
    String? taskId,
    String? categoryId,
    int? concentrationScore,
    String? notes,
  }) = _CreateManualSessionDto;

  factory CreateManualSessionDto.fromJson(Map<String, dynamic> json) =>
      _$CreateManualSessionDtoFromJson(json);

  SessionType getSessionType() => SessionType.fromString(sessionType);
}

@freezed
class GetSessionFiltersDto with _$GetSessionFiltersDto {
  const GetSessionFiltersDto._();

  const factory GetSessionFiltersDto({
    int? startDate,
    int? endDate,
    List<String>? categoryIds,
    String? sessionType,
    int? minConcentrationScore,
    int? maxConcentrationScore,
  }) = _GetSessionFiltersDto;

  factory GetSessionFiltersDto.fromJson(Map<String, dynamic> json) =>
      _$GetSessionFiltersDtoFromJson(json);

  SessionType? getSessionType() =>
      sessionType != null ? SessionType.fromString(sessionType!) : null;
}

@freezed
class UpdateFocusSessionDto with _$UpdateFocusSessionDto {
  const factory UpdateFocusSessionDto({
    String? categoryId,
    String? taskId,
    String? notes,
    int? concentrationScore,
    int? startedAt,
    int? endedAt,
    int? actualDuration,
    String? sessionType,
  }) = _UpdateFocusSessionDto;

  factory UpdateFocusSessionDto.fromJson(Map<String, dynamic> json) =>
      _$UpdateFocusSessionDtoFromJson(json);
}

// Response DTOs

@freezed
class FocusSessionDto with _$FocusSessionDto {
  const factory FocusSessionDto({
    required String id,
    required String sessionType,
    required int startedAt,
    int? endedAt,
    int? actualDuration,
    String? taskId,
    String? categoryId,
    int? concentrationScore,
    String? notes,
    required int createdAt,
  }) = _FocusSessionDto;

  factory FocusSessionDto.fromJson(Map<String, dynamic> json) =>
      _$FocusSessionDtoFromJson(json);

  factory FocusSessionDto.fromEntity(FocusSession session) {
    return FocusSessionDto(
      id: session.id,
      sessionType: session.sessionType.value,
      startedAt: session.startedAt,
      endedAt: session.endedAt,
      actualDuration: session.actualDuration,
      taskId: session.taskId,
      categoryId: session.categoryId,
      concentrationScore: session.concentrationScore,
      notes: session.notes,
      createdAt: session.createdAt.millisecondsSinceEpoch,
    );
  }
}

@freezed
class CreateManualSessionResponseDto with _$CreateManualSessionResponseDto {
  const factory CreateManualSessionResponseDto({required String id}) =
      _CreateManualSessionResponseDto;

  factory CreateManualSessionResponseDto.fromJson(Map<String, dynamic> json) =>
      _$CreateManualSessionResponseDtoFromJson(json);
}

@freezed
class GetSessionFiltersResponseDto with _$GetSessionFiltersResponseDto {
  const factory GetSessionFiltersResponseDto({
    required List<FocusSessionDto> focusSessions,
  }) = _GetSessionFiltersResponseDto;

  factory GetSessionFiltersResponseDto.fromJson(Map<String, dynamic> json) =>
      _$GetSessionFiltersResponseDtoFromJson(json);
}
