import 'package:focus_flow_app/domain/entities/focus_session.dart';
import 'package:focus_flow_app/domain/repositories/session_repository.dart';

class GetSessionsWithFilters {
  final SessionRepository sessionRepository;

  GetSessionsWithFilters({required this.sessionRepository});

  Future<GetSessionsWithFiltersResult> execute({
    int? startDate,
    int? endDate,
    List<String>? categoryIds,
    SessionType? sessionType,
    int? minConcentrationScore,
    int? maxConcentrationScore,
    bool? hasNote,
  }) async {
    try {
      // Validate concentration scores if provided
      if (minConcentrationScore != null &&
          (minConcentrationScore < 1 || minConcentrationScore > 5)) {
        return GetSessionsWithFiltersResult(
          success: false,
          error: 'Minimum concentration score must be between 1 and 5',
          errorType: GetSessionsWithFiltersErrorType.validation,
        );
      }

      if (maxConcentrationScore != null &&
          (maxConcentrationScore < 1 || maxConcentrationScore > 5)) {
        return GetSessionsWithFiltersResult(
          success: false,
          error: 'Maximum concentration score must be between 1 and 5',
          errorType: GetSessionsWithFiltersErrorType.validation,
        );
      }

      if (minConcentrationScore != null &&
          maxConcentrationScore != null &&
          minConcentrationScore > maxConcentrationScore) {
        return GetSessionsWithFiltersResult(
          success: false,
          error:
              'Minimum concentration score cannot be greater than maximum concentration score',
          errorType: GetSessionsWithFiltersErrorType.validation,
        );
      }

      // Validate date range if provided
      if (startDate != null && endDate != null && startDate > endDate) {
        return GetSessionsWithFiltersResult(
          success: false,
          error: 'Start date cannot be after end date',
          errorType: GetSessionsWithFiltersErrorType.validation,
        );
      }

      // Get filtered sessions
      final sessions = await sessionRepository.getSessionsWithFilters(
        startDate: startDate,
        endDate: endDate,
        categoryIds: categoryIds,
        sessionType: sessionType,
        minConcentrationScore: minConcentrationScore,
        maxConcentrationScore: maxConcentrationScore,
        hasNote: hasNote,
      );

      return GetSessionsWithFiltersResult(success: true, sessions: sessions);
    } catch (e) {
      return GetSessionsWithFiltersResult(
        success: false,
        error: e.toString(),
        errorType: GetSessionsWithFiltersErrorType.internal,
      );
    }
  }
}

enum GetSessionsWithFiltersErrorType { validation, internal }

class GetSessionsWithFiltersResult {
  final bool success;
  final List<FocusSession>? sessions;
  final String? error;
  final GetSessionsWithFiltersErrorType? errorType;

  GetSessionsWithFiltersResult({
    required this.success,
    this.sessions,
    this.error,
    this.errorType,
  });
}
