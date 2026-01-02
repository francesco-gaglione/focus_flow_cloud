import 'package:focus_flow_app/domain/entities/task.dart';
import 'package:focus_flow_app/domain/repositories/category_repository.dart';
import 'package:focus_flow_app/domain/repositories/task_repository.dart';

class CreateTask {
  final TaskRepository taskRepository;
  final CategoryRepository categoryRepository;

  CreateTask({required this.taskRepository, required this.categoryRepository});

  Future<CreateTaskResult> execute({
    required String name,
    String? description,
    String? categoryId,
    int? scheduledDate,
  }) async {
    try {
      // Validate inputs
      if (name.trim().isEmpty) {
        return CreateTaskResult(
          success: false,
          error: 'Task name cannot be empty',
          errorType: CreateTaskErrorType.validation,
        );
      }

      // Validate category exists if provided
      if (categoryId != null) {
        final categoryExists = await categoryRepository.categoryExists(
          categoryId,
        );
        if (!categoryExists) {
          return CreateTaskResult(
            success: false,
            error: 'Category not found',
            errorType: CreateTaskErrorType.validation,
          );
        }
      }

      // Create task
      final task = await taskRepository.createTask(
        name: name,
        description: description,
        categoryId: categoryId,
        scheduledDate: scheduledDate,
      );

      return CreateTaskResult(success: true, task: task);
    } catch (e) {
      return CreateTaskResult(
        success: false,
        error: e.toString(),
        errorType: CreateTaskErrorType.internal,
      );
    }
  }
}

enum CreateTaskErrorType { validation, conflict, internal }

class CreateTaskResult {
  final bool success;
  final Task? task;
  final String? error;
  final CreateTaskErrorType? errorType;

  CreateTaskResult({
    required this.success,
    this.task,
    this.error,
    this.errorType,
  });
}
