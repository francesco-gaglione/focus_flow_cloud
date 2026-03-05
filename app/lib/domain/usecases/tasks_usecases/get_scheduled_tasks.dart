import 'package:focus_flow_app/domain/entities/task.dart';
import 'package:focus_flow_app/domain/repositories/task_repository.dart';

class GetScheduledTasks {
  final TaskRepository taskRepository;

  GetScheduledTasks({required this.taskRepository});

  Future<GetScheduledTasksResult> execute({
    bool? completed,
    int? from,
    int? to,
  }) async {
    try {
      final tasks = await taskRepository.getScheduledTasks(
        completed: completed,
        from: from,
        to: to,
      );
      return GetScheduledTasksResult(success: true, tasks: tasks);
    } catch (e) {
      return GetScheduledTasksResult(
        success: false,
        error: e.toString(),
        tasks: [],
      );
    }
  }
}

class GetScheduledTasksResult {
  final bool success;
  final List<Task> tasks;
  final String? error;

  GetScheduledTasksResult({
    required this.success,
    required this.tasks,
    this.error,
  });
}
