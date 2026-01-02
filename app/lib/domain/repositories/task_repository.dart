import '../entities/task.dart';

abstract class TaskRepository {
  /// Get all tasks
  Future<List<Task>> getAllTasks();

  /// Get a task by ID
  Future<Task?> getTaskById(String id);

  /// Get orphan tasks (tasks without a category)
  Future<List<Task>> getOrphanTasks();

  /// Create a new task
  Future<Task> createTask({
    required String name,
    String? description,
    String? categoryId,
    int? scheduledDate,
  });

  /// Update an existing task
  Future<Task> updateTask({
    required String id,
    String? name,
    String? description,
    String? categoryId,
    int? scheduledDate,
    int? completedAt,
  });

  /// Delete tasks by IDs
  Future<List<String>> deleteTasks(List<String> taskIds);

  /// Check if a task exists by name
  Future<bool> taskExistsByName(String name);

  /// Check if a task exists by ID
  Future<bool> taskExists(String id);

  /// Get tasks by IDs
  Future<List<Task>> getTasksByIds(List<String> taskIds);
}
