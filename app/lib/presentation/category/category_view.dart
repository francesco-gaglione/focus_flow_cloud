import 'package:easy_localization/easy_localization.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:focus_flow_app/domain/entities/task.dart';
import 'package:focus_flow_app/presentation/category/bloc/category_bloc.dart';
import 'package:focus_flow_app/presentation/category/bloc/category_event.dart';
import 'package:focus_flow_app/presentation/category/bloc/category_state.dart';
import 'package:focus_flow_app/presentation/widgets/category/category_card.dart';
import 'package:focus_flow_app/presentation/widgets/category/dialogs/task_form_dialog.dart';
import 'package:focus_flow_app/presentation/widgets/category/task_list_item.dart';
import 'package:focus_flow_app/presentation/widgets/category/dialogs/category_form_dialog.dart';
import 'package:focus_flow_app/presentation/widgets/common/confirmation_dialog.dart';
import 'package:focus_flow_app/presentation/widgets/common/empty_state.dart';
import 'package:focus_flow_app/presentation/widgets/common/error_state.dart';

class CategoryView extends StatelessWidget {
  const CategoryView({super.key});

  static const Color _orphanTaskColor = Color(0xFFFFA726);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(context.tr('category.title')),
        centerTitle: false,
      ),
      body: BlocBuilder<CategoryBloc, CategoryState>(
        builder: (context, state) {
          if (state.isLoading) {
            return const Center(child: CircularProgressIndicator());
          }

          if (state.errorMessage != null) {
            return ErrorState(
              message: state.errorMessage!,
              onRetry: () => context.read<CategoryBloc>().add(LoadCategories()),
            );
          }

          final categories = state.categories;
          final orphanTasks = state.orphanTasks;

          if (categories.isEmpty && orphanTasks.isEmpty) {
            return EmptyState(
              icon: Icons.category_outlined,
              title: context.tr('category.empty_state_title'),
              message: context.tr('category.empty_state_message'),
            );
          }

          final itemCount =
              (orphanTasks.isNotEmpty ? 1 : 0) + categories.length;

          return ListView.builder(
            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
            itemCount: itemCount,
            itemBuilder: (context, index) {
              if (orphanTasks.isNotEmpty && index == 0) {
                return _buildOrphanTasksSection(context, orphanTasks);
              }

              final categoryIndex = orphanTasks.isNotEmpty ? index - 1 : index;
              final categoryWithTasks = categories[categoryIndex];
              final category = categoryWithTasks.category;
              final tasks = categoryWithTasks.tasks;

              return CategoryCard(
                name: category.name,
                description: category.description,
                color: _parseColor(category.color),
                totalTasks: tasks.length,
                completedTasks:
                    tasks.where((t) => t.completedAt != null).length,
                onEdit: () => _showEditCategoryDialog(context, category),
                onDelete: () => _showDeleteCategoryDialog(context, category.id),
                onAddTask: () => _showCreateTaskDialog(context, category.id),
                taskWidgets: _buildTaskList(context, tasks),
              );
            },
          );
        },
      ),
      floatingActionButton: _buildFABStack(context),
    );
  }

  Widget _buildOrphanTasksSection(
    BuildContext context,
    List<Task> orphanTasks,
  ) {
    return CategoryCard(
      name: context.tr('category.unassigned_tasks'),
      description: context.tr('category.unassigned_task_description'),
      color: _orphanTaskColor,
      totalTasks: orphanTasks.length,
      completedTasks: orphanTasks.where((t) => t.completedAt != null).length,
      onEdit:
          () => debugPrint("TODO: edit on orphan tasks card is not allowed"),
      onDelete:
          () => debugPrint("TODO: delete on orphan tasks card is not allowed"),
      taskWidgets: _buildTaskList(context, orphanTasks),
      showOptions: false,
    );
  }

  Widget _buildFABStack(BuildContext context) {
    return Stack(
      children: [
        Positioned(
          bottom: 80,
          right: 0,
          child: FloatingActionButton(
            heroTag: 'orphanTask',
            onPressed: () => _showCreateOrphanTaskDialog(context),
            child: const Icon(Icons.add_task),
          ),
        ),
        Positioned(
          bottom: 0,
          right: 0,
          child: FloatingActionButton.extended(
            heroTag: 'category',
            onPressed: () => _showCreateCategoryDialog(context),
            icon: const Icon(Icons.add),
            label: Text(context.tr('category.add_category_button')),
          ),
        ),
      ],
    );
  }

  List<Widget> _buildTaskList(BuildContext context, List<dynamic> tasks) {
    if (tasks.isEmpty) return [];

    return [
      ListView.separated(
        shrinkWrap: true,
        physics: const NeverScrollableScrollPhysics(),
        itemCount: tasks.length,
        separatorBuilder:
            (context, index) => const Divider(
              height: 1,
              indent: 10,
              thickness: 0.5,
              endIndent: 10,
            ),
        itemBuilder: (context, index) {
          final task = tasks[index];
          return TaskListItem(
            name: task.name,
            description: task.description,
            isCompleted: task.completedAt != null,
            onEdit: () {
              _showEditOrphanTaskDialog(context, task);
            },
            onDelete: () {
              _showDeleteTaskDialog(context, task.id);
            },
          );
        },
      ),
    ];
  }

  void _showCreateCategoryDialog(BuildContext context) {
    showDialog(
      context: context,
      builder:
          (dialogContext) => CategoryFormDialog(
            title: context.tr('category.create_category_dialog_title'),
            icon: Icons.create_new_folder_outlined,
            onSubmit: (name, description, color) {
              context.read<CategoryBloc>().add(
                CreateCategoryEvent(
                  name: name,
                  color: _colorToHex(color),
                  description: description,
                ),
              );
            },
          ),
    );
  }

  void _showEditCategoryDialog(BuildContext context, dynamic category) {
    showDialog(
      context: context,
      builder:
          (dialogContext) => CategoryFormDialog(
            title: context.tr('category.edit_category_dialog_title'),
            icon: Icons.edit_outlined,
            initialName: category.name,
            initialDescription: category.description,
            initialColor: _parseColor(category.color),
            onSubmit: (name, description, color) {
              context.read<CategoryBloc>().add(
                UpdateCategoryEvent(
                  id: category.id,
                  name: name,
                  color: _colorToHex(color),
                  description: description,
                ),
              );
            },
          ),
    );
  }

  void _showEditOrphanTaskDialog(BuildContext context, dynamic category) {
    showDialog(
      context: context,
      builder:
          (dialogContext) => TaskDialog(
            initialName: category.name,
            initialDescription: category.description,
            onSubmit: (name, description) {
              context.read<CategoryBloc>().add(
                UpdateTaskEvent(
                  id: category.id,
                  name: name,
                  description: description,
                ),
              );
            },
          ),
    );
  }

  void _showDeleteCategoryDialog(BuildContext context, String categoryId) {
    showDialog(
      context: context,
      builder:
          (dialogContext) => ConfirmationDialog(
            title: context.tr('category.delete_category_dialog_title'),
            message: context.tr('category.delete_category_dialog_message'),
            confirmText: context.tr('category.delete_button'),
            onConfirm: () {
              context.read<CategoryBloc>().add(
                DeleteCategoryEvent(id: categoryId),
              );
            },
          ),
    );
  }

  void _showDeleteTaskDialog(BuildContext context, String taskId) {
    showDialog(
      context: context,
      builder:
          (dialogContext) => ConfirmationDialog(
            title: context.tr('category.delete_task_dialog_title'),
            message: context.tr('category.delete_task_dialog_message'),
            confirmText: context.tr('category.delete_button'),
            onConfirm: () {
              context.read<CategoryBloc>().add(DeleteTaskEvent(id: taskId));
            },
          ),
    );
  }

  void _showCreateOrphanTaskDialog(BuildContext context) {
    showDialog(
      context: context,
      builder:
          (dialogContext) => TaskDialog(
            onSubmit: (name, description) {
              context.read<CategoryBloc>().add(
                CreateOrphanTaskEvent(description: description, title: name),
              );
            },
          ),
    );
  }

  void _showCreateTaskDialog(BuildContext context, String categoryId) {
    showDialog(
      context: context,
      builder:
          (dialogContext) => TaskDialog(
            onSubmit: (name, description) {
              context.read<CategoryBloc>().add(
                CreateTaskEvent(
                  categoryId: categoryId,
                  description: description,
                  title: name,
                ),
              );
            },
          ),
    );
  }

  Color _parseColor(String colorString) {
    try {
      return Color(int.parse(colorString.replaceFirst('#', '0xFF')));
    } catch (e) {
      return Colors.blue;
    }
  }

  String _colorToHex(Color color) {
    return '#${color.toARGB32().toRadixString(16).padLeft(8, '0').substring(2).toUpperCase()}';
  }
}
