import 'package:flutter/material.dart';

class CategoryCard extends StatelessWidget {
  final String name;
  final String? description;
  final Color color;
  final int totalTasks;
  final int completedTasks;
  final VoidCallback onEdit;
  final VoidCallback onDelete;
  final VoidCallback? onAddTask;
  final List<Widget> taskWidgets;
  final bool showOptions;

  const CategoryCard({
    super.key,
    required this.name,
    this.description,
    required this.color,
    required this.totalTasks,
    required this.completedTasks,
    required this.onEdit,
    required this.onDelete,
    this.onAddTask,
    required this.taskWidgets,
    this.showOptions = true,
  });

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;

    return Card(
      margin: const EdgeInsets.only(bottom: 8),
      clipBehavior: Clip.antiAlias,
      child: ExpansionTile(
        shape: const Border(),
        collapsedShape: const Border(),
        tilePadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
        childrenPadding: const EdgeInsets.all(0),
        leading: CircleAvatar(
          backgroundColor: color,
          radius: 24,
          child: Text(
            name[0].toUpperCase(),
            style: Theme.of(context).textTheme.titleLarge?.copyWith(
              color: colorScheme.onPrimary,
              fontWeight: FontWeight.bold,
            ),
          ),
        ),
        title: Text(
          name,
          style: Theme.of(
            context,
          ).textTheme.titleMedium?.copyWith(fontWeight: FontWeight.w600),
        ),
        subtitle: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            if (description != null) ...[
              const SizedBox(height: 4),
              Text(
                description!,
                style: Theme.of(context).textTheme.bodySmall?.copyWith(
                  color: colorScheme.onSurfaceVariant,
                ),
                maxLines: 2,
                overflow: TextOverflow.ellipsis,
              ),
            ],
            const SizedBox(height: 8),
            Row(
              children: [
                Icon(
                  Icons.assignment_outlined,
                  size: 16,
                  color: colorScheme.onSurfaceVariant,
                ),
                const SizedBox(width: 4),
                Text(
                  '$totalTasks tasks',
                  style: Theme.of(context).textTheme.labelSmall?.copyWith(
                    color: colorScheme.onSurfaceVariant,
                  ),
                ),
                if (completedTasks > 0) ...[
                  const SizedBox(width: 12),
                  Icon(
                    Icons.check_circle_outline,
                    size: 16,
                    color: colorScheme.tertiary,
                  ),
                  const SizedBox(width: 4),
                  Text(
                    '$completedTasks completed',
                    style: Theme.of(context).textTheme.labelSmall?.copyWith(
                      color: colorScheme.tertiary,
                    ),
                  ),
                ],
              ],
            ),
          ],
        ),
        trailing:
            showOptions
                ? PopupMenuButton(
                  icon: const Icon(Icons.more_vert),
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(12),
                  ),
                  itemBuilder:
                      (context) => [
                        PopupMenuItem(
                          value: 'edit',
                          child: Row(
                            children: [
                              Icon(
                                Icons.edit_outlined,
                                size: 20,
                                color: colorScheme.primary,
                              ),
                              const SizedBox(width: 12),
                              const Text('Edit'),
                            ],
                          ),
                        ),
                        PopupMenuItem(
                          value: 'delete',
                          child: Row(
                            children: [
                              Icon(
                                Icons.delete_outline,
                                size: 20,
                                color: colorScheme.error,
                              ),
                              const SizedBox(width: 12),
                              Text(
                                'Delete',
                                style: TextStyle(color: colorScheme.error),
                              ),
                            ],
                          ),
                        ),
                      ],
                  onSelected: (value) {
                    if (value == 'edit') {
                      onEdit();
                    } else if (value == 'delete') {
                      onDelete();
                    }
                  },
                )
                : null,
        children:
            taskWidgets.isEmpty
                ? [
                  Padding(
                    padding: const EdgeInsets.all(24),
                    child: Column(
                      children: [
                        Icon(
                          Icons.task_outlined,
                          size: 48,
                          color: colorScheme.onSurfaceVariant.withAlpha(
                            (255 * 0.3).round(),
                          ),
                        ),
                        const SizedBox(height: 12),
                        Text(
                          'No tasks in this category',
                          style: Theme.of(context).textTheme.bodyMedium
                              ?.copyWith(color: colorScheme.onSurfaceVariant),
                        ),
                        if (onAddTask != null) ...[
                          const SizedBox(height: 16),
                          FilledButton.tonalIcon(
                            onPressed: onAddTask,
                            icon: const Icon(Icons.add),
                            label: const Text('Add Task'),
                          ),
                        ],
                      ],
                    ),
                  ),
                ]
                : [
                    ...taskWidgets,
                    if (onAddTask != null)
                      Padding(
                        padding: const EdgeInsets.fromLTRB(16, 8, 16, 16),
                        child: FilledButton.tonalIcon(
                          onPressed: onAddTask,
                          icon: const Icon(Icons.add),
                          label: const Text('Add Task'),
                          style: FilledButton.styleFrom(
                            minimumSize: const Size(double.infinity, 48),
                          ),
                        ),
                      ),
                  ],
      ),
    );
  }
}
