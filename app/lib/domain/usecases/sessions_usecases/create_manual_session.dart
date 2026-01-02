import 'package:focus_flow_app/domain/entities/focus_session.dart';
import 'package:focus_flow_app/domain/repositories/category_repository.dart';
import 'package:focus_flow_app/domain/repositories/session_repository.dart';
import 'package:focus_flow_app/domain/repositories/task_repository.dart';

class CreateManualSession {
  final SessionRepository sessionRepository;
  final TaskRepository taskRepository;
  final CategoryRepository categoryRepository;

  CreateManualSession({
    required this.sessionRepository,
    required this.taskRepository,
    required this.categoryRepository,
  });

  Future<CreateManualSessionResult> execute({
    required SessionType sessionType,
    required int startedAt,
    required int endedAt,
    String? taskId,
    String? categoryId,
    int? concentrationScore,
    String? notes,
  }) async {
    try {
      // Validate date range
      if (startedAt >= endedAt) {
        return CreateManualSessionResult(
          success: false,
          error: 'Start time must be before end time',
          errorType: CreateManualSessionErrorType.validation,
        );
      }

      // Validate concentration score if provided
      if (concentrationScore != null &&
          (concentrationScore < 1 || concentrationScore > 5)) {
        return CreateManualSessionResult(
          success: false,
          error: 'Concentration score must be between 1 and 5',
          errorType: CreateManualSessionErrorType.validation,
        );
      }

      // Validate task exists if provided
      if (taskId != null) {
        final taskExists = await taskRepository.taskExists(taskId);
        if (!taskExists) {
          return CreateManualSessionResult(
            success: false,
            error: 'Task not found',
            errorType: CreateManualSessionErrorType.validation,
          );
        }
      }

      // Validate category exists if provided
      if (categoryId != null) {
        final categoryExists = await categoryRepository.categoryExists(
          categoryId,
        );
        if (!categoryExists) {
          return CreateManualSessionResult(
            success: false,
            error: 'Category not found',
            errorType: CreateManualSessionErrorType.validation,
          );
        }
      }

      // Create manual session
      final session = await sessionRepository.createManualSession(
        sessionType: sessionType,
        startedAt: startedAt,
        endedAt: endedAt,
        taskId: taskId,
        categoryId: categoryId,
        concentrationScore: concentrationScore,
        notes: notes,
      );

      return CreateManualSessionResult(success: true, session: session);
    } catch (e) {
      return CreateManualSessionResult(
        success: false,
        error: e.toString(),
        errorType: CreateManualSessionErrorType.internal,
      );
    }
  }
}

enum CreateManualSessionErrorType { validation, conflict, internal }

class CreateManualSessionResult {
  final bool success;
  final FocusSession? session;
  final String? error;
  final CreateManualSessionErrorType? errorType;

  CreateManualSessionResult({
    required this.success,
    this.session,
    this.error,
    this.errorType,
  });
}
