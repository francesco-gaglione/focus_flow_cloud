import 'package:focus_flow_app/domain/repositories/task_repository.dart';

class DeleteTasks {
  final TaskRepository taskRepository;

  DeleteTasks({required this.taskRepository});

  Future<DeleteTasksResult> execute({required List<String> taskIds}) async {
    try {
      if (taskIds.isEmpty) {
        return DeleteTasksResult(
          success: false,
          error: 'Task IDs cannot be empty',
          errorType: DeleteTasksErrorType.validation,
        );
      }

      final deletedIds = await taskRepository.deleteTasks(taskIds);

      return DeleteTasksResult(success: true, deletedIds: deletedIds);
    } catch (e) {
      return DeleteTasksResult(
        success: false,
        error: e.toString(),
        errorType: DeleteTasksErrorType.internal,
      );
    }
  }
}

enum DeleteTasksErrorType { validation, notFound, internal }

class DeleteTasksResult {
  final bool success;
  final List<String>? deletedIds;
  final String? error;
  final DeleteTasksErrorType? errorType;

  DeleteTasksResult({
    required this.success,
    this.deletedIds,
    this.error,
    this.errorType,
  });
}
