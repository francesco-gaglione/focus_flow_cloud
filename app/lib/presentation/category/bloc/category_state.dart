import 'package:focus_flow_app/domain/entities/category_with_tasks.dart';
import 'package:focus_flow_app/domain/entities/task.dart';

enum TaskFilter { active, completed, all }

class CategoryState {
  final List<CategoryWithTasks> categories;
  final List<Task> orphanTasks;
  final bool isLoading;
  final String? errorMessage;
  final TaskFilter filter;

  const CategoryState({
    this.categories = const [],
    this.orphanTasks = const [],
    this.isLoading = false,
    this.errorMessage,
    this.filter = TaskFilter.active,
  });

  CategoryState copyWith({
    List<CategoryWithTasks>? categories,
    List<Task>? orphanTasks,
    bool? isLoading,
    String? errorMessage,
    TaskFilter? filter,
  }) {
    return CategoryState(
      categories: categories ?? this.categories,
      orphanTasks: orphanTasks ?? this.orphanTasks,
      isLoading: isLoading ?? this.isLoading,
      errorMessage: errorMessage,
      filter: filter ?? this.filter,
    );
  }
}
