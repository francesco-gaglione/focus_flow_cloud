import 'package:focus_flow_app/domain/entities/task.dart';
import 'package:focus_flow_app/domain/repositories/category_repository.dart';
import 'package:focus_flow_app/domain/repositories/task_repository.dart';

class UpdateTask {
  final TaskRepository taskRepository;
  final CategoryRepository categoryRepository;

  UpdateTask({required this.taskRepository, required this.categoryRepository});

  Future<UpdateTaskResult> execute({
    required String id,
    String? name,
    String? description,
    String? categoryId,
    int? scheduledDate,
    int? completedAt,
  }) async {
    try {
      // Validate name if provided
      if (name != null && name.trim().isEmpty) {
        return UpdateTaskResult(
          success: false,
          error: 'Task name cannot be empty',
          errorType: UpdateTaskErrorType.validation,
        );
      }

      // Validate category exists if provided
      if (categoryId != null) {
        final categoryExists = await categoryRepository.categoryExists(
          categoryId,
        );
        if (!categoryExists) {
          return UpdateTaskResult(
            success: false,
            error: 'Category not found',
            errorType: UpdateTaskErrorType.validation,
          );
        }
      }

      // Update task
      final updatedTask = await taskRepository.updateTask(
        id: id,
        name: name,
        description: description,
        categoryId: categoryId,
        scheduledDate: scheduledDate,
        completedAt: completedAt,
      );

      return UpdateTaskResult(success: true, task: updatedTask);
    } catch (e) {
      return UpdateTaskResult(
        success: false,
        error: e.toString(),
        errorType: UpdateTaskErrorType.internal,
      );
    }
  }
}

enum UpdateTaskErrorType { validation, notFound, conflict, internal }

class UpdateTaskResult {
  final bool success;
  final Task? task;
  final String? error;
  final UpdateTaskErrorType? errorType;

  UpdateTaskResult({
    required this.success,
    this.task,
    this.error,
    this.errorType,
  });
}
