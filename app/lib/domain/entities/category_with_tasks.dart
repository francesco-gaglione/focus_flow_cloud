import 'package:equatable/equatable.dart';
import 'package:focus_flow_app/domain/entities/category.dart';
import 'package:focus_flow_app/domain/entities/task.dart';

class CategoryWithTasks extends Equatable {
  final Category category;
  final List<Task> tasks;

  const CategoryWithTasks({required this.category, required this.tasks});

  @override
  List<Object?> get props => [category, tasks];
}
