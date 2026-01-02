import 'package:focus_flow_app/domain/entities/task.dart';
import 'package:focus_flow_app/domain/repositories/task_repository.dart';

class FetchOrphanTasks {
  final TaskRepository taskRepository;

  FetchOrphanTasks({required this.taskRepository});

  Future<FetchOrphanTasksResult> execute() async {
    try {
      final orphanTasks = await taskRepository.getOrphanTasks();

      return FetchOrphanTasksResult(success: true, orphanTasks: orphanTasks);
    } catch (e) {
      return FetchOrphanTasksResult(success: false, error: e.toString());
    }
  }
}

class FetchOrphanTasksResult {
  final bool success;
  final List<Task>? orphanTasks;
  final String? error;

  FetchOrphanTasksResult({required this.success, this.orphanTasks, this.error});
}
