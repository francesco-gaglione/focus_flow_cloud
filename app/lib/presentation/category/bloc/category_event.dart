import 'package:focus_flow_app/presentation/category/bloc/category_state.dart';

abstract class CategoryEvent {}

class InitState extends CategoryEvent {}

class LoadCategories extends CategoryEvent {}

class LoadOrphanTasks extends CategoryEvent {}

class CreateCategoryEvent extends CategoryEvent {
  final String name;
  final String? color;
  final String? description;

  CreateCategoryEvent({required this.name, this.color, this.description});
}

class CreateOrphanTaskEvent extends CategoryEvent {
  final String title;
  final String? description;

  CreateOrphanTaskEvent({required this.title, this.description});
}

class CreateTaskEvent extends CategoryEvent {
  final String categoryId;
  final String title;
  final String? description;

  CreateTaskEvent({
    required this.categoryId,
    required this.title,
    this.description,
  });
}

class UpdateCategoryEvent extends CategoryEvent {
  final String id;
  final String? name;
  final String? color;
  final String? description;

  UpdateCategoryEvent({
    required this.id,
    this.name,
    this.color,
    this.description,
  });
}

class UpdateTaskEvent extends CategoryEvent {
  final String id;
  final String? name;
  final String? description;

  UpdateTaskEvent({required this.id, this.name, this.description});
}

class DeleteCategoryEvent extends CategoryEvent {
  final String id;

  DeleteCategoryEvent({required this.id});
}

class DeleteTaskEvent extends CategoryEvent {
  final String id;

  DeleteTaskEvent({required this.id});
}

class SetTaskFilter extends CategoryEvent {
  final TaskFilter filter;

  SetTaskFilter(this.filter);
}

class ToggleTaskCompletion extends CategoryEvent {
  final String taskId;
  final bool isCompleted;

  ToggleTaskCompletion({required this.taskId, required this.isCompleted});
}
