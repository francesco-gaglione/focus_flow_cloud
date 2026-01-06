import '../entities/focus_session.dart';

abstract class SessionRepository {
  /// Get all sessions
  Future<List<FocusSession>> getAllSessions();

  /// Get a session by ID
  Future<FocusSession?> getSessionById(String id);

  /// Get sessions with filters
  Future<List<FocusSession>> getSessionsWithFilters({
    int? startDate,
    int? endDate,
    List<String>? categoryIds,
    String? taskId,
    SessionType? sessionType,
    int? minConcentrationScore,
    int? maxConcentrationScore,
    bool? hasNote,
  });

  /// Create a new manual session
  Future<FocusSession> createManualSession({
    required SessionType sessionType,
    required int startedAt,
    required int endedAt,
    String? taskId,
    String? categoryId,
    int? concentrationScore,
    String? notes,
  });

  /// Update an existing session
  Future<FocusSession> updateSession({
    required String id,
    SessionType? sessionType,
    int? startedAt,
    int? endedAt,
    int? actualDuration,
    String? taskId,
    String? categoryId,
    int? concentrationScore,
    String? notes,
  });

  /// Delete a session by ID
  Future<bool> deleteSession(String id);

  /// Check if a session exists by ID
  Future<bool> sessionExists(String id);

  /// Get sessions by category ID
  Future<List<FocusSession>> getSessionsByCategoryId(String categoryId);

  /// Get sessions by task ID
  Future<List<FocusSession>> getSessionsByTaskId(String taskId);

  /// Get sessions by date range
  Future<List<FocusSession>> getSessionsByDateRange(int startDate, int endDate);
}
