import 'package:focus_flow_app/domain/entities/task.dart';
import 'package:focus_flow_app/domain/repositories/task_repository.dart';
import 'package:logger/logger.dart';

class CompleteTask {
  final Logger logger = Logger();
  final TaskRepository taskRepository;

  CompleteTask({required this.taskRepository});

  Future<CompleteTaskResult> execute({required String taskId}) async {
    try {
      logger.i('Completing task with ID: $taskId');
      final updatedTask = await taskRepository.updateTask(
        id: taskId,
        completedAt: DateTime.now().millisecondsSinceEpoch,
      );
      return CompleteTaskResult(success: true, task: updatedTask);
    } catch (e) {
      logger.e('Failed to complete task: $e');
      return CompleteTaskResult(success: false, error: e.toString());
    }
  }
}

class CompleteTaskResult {
  final bool success;
  final Task? task;
  final String? error;

  CompleteTaskResult({required this.success, this.task, this.error});
}
