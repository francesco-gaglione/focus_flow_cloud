import 'package:dio/dio.dart';
import 'package:logger/logger.dart';
import '../../domain/entities/focus_session.dart';
import '../../domain/repositories/session_repository.dart';
import '../dtos/session_dtos.dart';

class HttpSessionRepository implements SessionRepository {
  final logger = Logger();
  final Dio _dio;
  final String baseUrl;

  HttpSessionRepository({
    required Dio dio,
    this.baseUrl = 'http://localhost:3000',
  }) : _dio = dio;

  @override
  Future<List<FocusSession>> getAllSessions() async {
    final response = await _dio.get('$baseUrl/api/focus-session');
    final dto = GetSessionFiltersResponseDto.fromJson(response.data);
    return dto.focusSessions
        .map(
          (session) => FocusSession(
            id: session.id,
            sessionType: SessionType.fromString(session.sessionType),
            startedAt: session.startedAt,
            endedAt: session.endedAt,
            actualDuration: session.actualDuration,
            taskId: session.taskId,
            categoryId: session.categoryId,
            concentrationScore: session.concentrationScore,
            notes: session.notes,
            createdAt: DateTime.fromMillisecondsSinceEpoch(session.createdAt * 1000, isUtc: true),
          ),
        )
        .toList();
  }

  @override
  Future<FocusSession?> getSessionById(String id) async {
    try {
      final sessions = await getAllSessions();
      return sessions.cast<FocusSession?>().firstWhere(
        (s) => s?.id == id,
        orElse: () => null,
      );
    } catch (e) {
      return null;
    }
  }

  @override
  Future<List<FocusSession>> getSessionsWithFilters({
    int? startDate,
    int? endDate,
    List<String>? categoryIds,
    SessionType? sessionType,
    int? minConcentrationScore,
    int? maxConcentrationScore,
    bool? hasNote,
  }) async {
    final queryParams = <String, dynamic>{};
    if (startDate != null) queryParams['startDate'] = startDate;
    if (endDate != null) queryParams['endDate'] = endDate;
    if (categoryIds != null && categoryIds.isNotEmpty) {
      queryParams['categoryIds'] = categoryIds.join(',');
    }
    if (sessionType != null) queryParams['sessionType'] = sessionType.value;
    if (minConcentrationScore != null) {
      queryParams['minConcentrationScore'] = minConcentrationScore;
    }
    if (maxConcentrationScore != null) {
      queryParams['maxConcentrationScore'] = maxConcentrationScore;
    }
    if (hasNote != null) queryParams['hasNotes'] = hasNote;

    logger.i('Fetching sessions with params: $queryParams');

    try {
      final response = await _dio.get(
        '$baseUrl/api/focus-session',
        queryParameters: queryParams,
        options: Options(listFormat: ListFormat.multi),
      );
      final dto = GetSessionFiltersResponseDto.fromJson(response.data);
      return dto.focusSessions
          .map(
            (session) => FocusSession(
              id: session.id,
              sessionType: SessionType.fromString(session.sessionType),
              startedAt: session.startedAt,
              endedAt: session.endedAt,
              actualDuration: session.actualDuration,
              taskId: session.taskId,
              categoryId: session.categoryId,
              concentrationScore: session.concentrationScore,
              notes: session.notes,
              createdAt: DateTime.fromMillisecondsSinceEpoch(session.createdAt * 1000, isUtc: true),
            ),
          )
          .toList();
    } on DioException catch (e) {
      if (e.response != null) {
        logger.e(
          'Error fetching sessions: ${e.response?.statusCode}',
          error: e.response?.data,
        );
      } else {
        logger.e('Error fetching sessions: ${e.message}');
      }
      rethrow;
    }
  }

  @override
  Future<FocusSession> createManualSession({
    required SessionType sessionType,
    required int startedAt,
    required int endedAt,
    String? taskId,
    String? categoryId,
    int? concentrationScore,
    String? notes,
  }) async {
    final dto = CreateManualSessionDto(
      sessionType: sessionType.value,
      startedAt: startedAt,
      endedAt: endedAt,
      taskId: taskId,
      categoryId: categoryId,
      concentrationScore: concentrationScore,
      notes: notes,
    );
    final response = await _dio.post(
      '$baseUrl/api/focus-session/manual',
      data: dto.toJson(),
    );
    final responseDto = CreateManualSessionResponseDto.fromJson(response.data);
    return FocusSession(
      id: responseDto.id,
      sessionType: sessionType,
      startedAt: startedAt,
      endedAt: endedAt,
      actualDuration: endedAt - startedAt,
      taskId: taskId,
      categoryId: categoryId,
      concentrationScore: concentrationScore,
      notes: notes,
      createdAt: DateTime.now(),
    );
  }

  @override
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
  }) async {
    final dto = UpdateFocusSessionDto(
      sessionType: sessionType?.value,
      startedAt: startedAt,
      endedAt: endedAt,
      actualDuration: actualDuration,
      taskId: taskId,
      categoryId: categoryId,
      concentrationScore: concentrationScore,
      notes: notes,
    );

    final response = await _dio.put(
      '$baseUrl/api/focus-session/$id',
      data: dto.toJson(),
    );

    if (response.statusCode == 204 ||
        (response.data is Map && (response.data as Map).isEmpty)) {
      // Return a dummy session since we will refresh the list anyway
      return FocusSession(
        id: id,
        sessionType: sessionType ?? SessionType.work,
        startedAt: startedAt ?? 0,
        endedAt: endedAt,
        actualDuration: actualDuration,
        taskId: taskId,
        categoryId: categoryId,
        concentrationScore: concentrationScore,
        notes: notes,
        createdAt: DateTime.now(),
      );
    }

    final json = response.data;
    return FocusSession(
      id: json['id'],
      sessionType: SessionType.fromString(json['sessionType']),
      startedAt: json['startedAt'],
      endedAt: json['endedAt'],
      actualDuration: json['actualDuration'],
      taskId: json['taskId'],
      categoryId: json['categoryId'],
      concentrationScore: json['concentrationScore'],
      notes: json['notes'],
      createdAt: DateTime.fromMillisecondsSinceEpoch(json['createdAt'] * 1000, isUtc: true),
    );
  }

  @override
  Future<bool> deleteSession(String id) async {
    try {
      await _dio.delete('$baseUrl/api/focus-session/$id');
      return true;
    } catch (e) {
      return false;
    }
  }

  @override
  Future<bool> sessionExists(String id) async {
    final session = await getSessionById(id);
    return session != null;
  }

  @override
  Future<List<FocusSession>> getSessionsByCategoryId(String categoryId) async {
    return getSessionsWithFilters(categoryIds: [categoryId]);
  }

  @override
  Future<List<FocusSession>> getSessionsByTaskId(String taskId) async {
    final allSessions = await getAllSessions();
    return allSessions.where((s) => s.taskId == taskId).toList();
  }

  @override
  Future<List<FocusSession>> getSessionsByDateRange(
    int startDate,
    int endDate,
  ) async {
    return getSessionsWithFilters(startDate: startDate, endDate: endDate);
  }
}
