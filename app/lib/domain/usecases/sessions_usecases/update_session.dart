import 'package:focus_flow_app/domain/entities/focus_session.dart';
import 'package:focus_flow_app/domain/repositories/session_repository.dart';

class UpdateSession {
  final SessionRepository sessionRepository;

  UpdateSession({required this.sessionRepository});

  Future<UpdateSessionResult> execute({
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
    try {
      final session = await sessionRepository.updateSession(
        id: id,
        sessionType: sessionType,
        startedAt: startedAt,
        endedAt: endedAt,
        actualDuration: actualDuration,
        taskId: taskId,
        categoryId: categoryId,
        concentrationScore: concentrationScore,
        notes: notes,
      );
      return UpdateSessionResult(success: true, session: session);
    } catch (e) {
      return UpdateSessionResult(success: false, error: e.toString());
    }
  }
}

class UpdateSessionResult {
  final bool success;
  final FocusSession? session;
  final String? error;

  UpdateSessionResult({
    required this.success,
    this.session,
    this.error,
  });
}
