import 'package:easy_localization/easy_localization.dart' hide TextDirection;
import 'package:flutter/material.dart';
import 'package:focus_flow_app/domain/entities/category.dart';
import 'package:focus_flow_app/domain/entities/category_with_tasks.dart';
import 'package:focus_flow_app/domain/entities/task.dart';

class CategoryTaskSelector extends StatefulWidget {
  final List<CategoryWithTasks> categories;
  final List<Task> orphanTasks;
  final ValueChanged<Category?>? onCategoryChanged;
  final ValueChanged<Task?>? onTaskChanged;
  final String? initialCategoryId;
  final String? initialTaskId;
  final bool enabled;

  const CategoryTaskSelector({
    super.key,
    required this.categories,
    required this.orphanTasks,
    this.onCategoryChanged,
    this.onTaskChanged,
    this.initialCategoryId,
    this.initialTaskId,
    this.enabled = true,
  });

  @override
  State<CategoryTaskSelector> createState() => _CategoryTaskSelectorState();
}

class _CategoryTaskSelectorState extends State<CategoryTaskSelector> {
  String? selectedCategoryId;
  String? selectedTaskId;
  final Set<String> _expandedCategories = {};
  bool _isOrphanExpanded = false;

  @override
  void initState() {
    super.initState();
    selectedCategoryId = widget.initialCategoryId;
    selectedTaskId = widget.initialTaskId;
    _expandSelected();
  }

  @override
  void didUpdateWidget(CategoryTaskSelector oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.initialCategoryId != widget.initialCategoryId ||
        oldWidget.initialTaskId != widget.initialTaskId) {
      setState(() {
        selectedCategoryId = widget.initialCategoryId;
        selectedTaskId = widget.initialTaskId;
        _expandSelected();
      });
    }
  }

  void _expandSelected() {
    if (selectedCategoryId != null) {
      _expandedCategories.add(selectedCategoryId!);
    }
    if (selectedTaskId != null && selectedCategoryId == null) {
      _isOrphanExpanded = true;
    }
  }

  Color _parseColor(String colorString) {
    try {
      return Color(int.parse(colorString.replaceFirst('#', '0xFF')));
    } catch (e) {
      return Colors.blue;
    }
  }

  void _handleCategorySelect(Category category) {
    setState(() {
      if (selectedCategoryId == category.id && selectedTaskId == null) {
        // Already selected, maybe deselect? Or do nothing.
        // Let's keep it selected.
      } else {
        selectedCategoryId = category.id;
        selectedTaskId = null;
      }
    });
    widget.onCategoryChanged?.call(category);
    widget.onTaskChanged?.call(null);
  }

  void _handleTaskSelect(Task task, Category? category) {
    setState(() {
      selectedTaskId = task.id;
      if (category != null) {
        selectedCategoryId = category.id;
      } else {
        selectedCategoryId = null;
      }
    });

    if (category != null) {
      widget.onCategoryChanged?.call(category);
    } else {
      widget.onCategoryChanged?.call(null);
    }
    widget.onTaskChanged?.call(task);
  }

  void _toggleCategoryExpansion(String categoryId) {
    setState(() {
      if (_expandedCategories.contains(categoryId)) {
        _expandedCategories.remove(categoryId);
      } else {
        _expandedCategories.add(categoryId);
      }
    });
  }

  void _toggleOrphanExpansion() {
    setState(() {
      _isOrphanExpanded = !_isOrphanExpanded;
    });
  }

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;

    return Card(
      elevation: 0,
      color: colorScheme.surfaceContainerHighest,
      clipBehavior: Clip.antiAlias,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Padding(
            padding: const EdgeInsets.all(16.0),
            child: Row(
              children: [
                Icon(Icons.category_outlined, color: colorScheme.primary),
                const SizedBox(width: 8),
                Expanded(
                  child: Text(
                    context.tr('focus.select_category_task'),
                    style: Theme.of(context).textTheme.titleMedium?.copyWith(
                          fontWeight: FontWeight.w600,
                        ),
                  ),
                ),
              ],
            ),
          ),
          const Divider(height: 1),
          ListView.builder(
            shrinkWrap: true,
            physics: const NeverScrollableScrollPhysics(),
            itemCount:
                widget.categories.length +
                (widget.orphanTasks.isNotEmpty ? 1 : 0),
            itemBuilder: (context, index) {
              if (index < widget.categories.length) {
                final catWithTasks = widget.categories[index];
                final category = catWithTasks.category;
                final tasks = catWithTasks.tasks;
                final isCategorySelected = selectedCategoryId == category.id;
                final isExpanded = _expandedCategories.contains(category.id);

                return Column(
                  children: [
                    _buildCategoryItem(
                      context,
                      category,
                      isCategorySelected,
                      isExpanded,
                    ),
                    if (isExpanded)
                      ...tasks.map(
                        (task) => _buildTaskItem(
                          context,
                          task,
                          category,
                          selectedTaskId == task.id,
                        ),
                      ),
                  ],
                );
              } else {
                // Orphan tasks section
                return Column(
                  children: [
                    _buildOrphanHeader(context),
                    if (_isOrphanExpanded)
                      ...widget.orphanTasks.map(
                        (task) => _buildTaskItem(
                          context,
                          task,
                          null,
                          selectedTaskId == task.id,
                        ),
                      ),
                  ],
                );
              }
            },
          ),
        ],
      ),
    );
  }

  Widget _buildCategoryItem(
    BuildContext context,
    Category category,
    bool isSelected,
    bool isExpanded,
  ) {
    final colorScheme = Theme.of(context).colorScheme;
    final isEnabled = widget.enabled;
    
    return Material(
      color:
          isSelected
              ? colorScheme.primaryContainer.withAlpha((255 * 0.4).round())
              : Colors.transparent,
      child: InkWell(
        onTap: isEnabled ? () => _handleCategorySelect(category) : null,
        child: Padding(
          padding: const EdgeInsets.symmetric(vertical: 8.0, horizontal: 4.0),
          child: Opacity(
            opacity: isEnabled ? 1.0 : 0.5,
            child: Row(
              children: [
                IconButton(
                  icon: Icon(
                    isExpanded
                        ? Icons.keyboard_arrow_down
                        : Icons.keyboard_arrow_right,
                    color: colorScheme.onSurfaceVariant,
                  ),
                  onPressed: () => _toggleCategoryExpansion(category.id),
                  visualDensity: VisualDensity.compact,
                ),
                Container(
                  width: 12,
                  height: 12,
                  decoration: BoxDecoration(
                    color: _parseColor(category.color),
                    shape: BoxShape.circle,
                  ),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: Text(
                    category.name,
                    style: TextStyle(
                      fontWeight:
                          isSelected ? FontWeight.bold : FontWeight.normal,
                      color:
                          isSelected
                              ? colorScheme.primary
                              : colorScheme.onSurface,
                    ),
                  ),
                ),
                if (isSelected && selectedTaskId == null)
                  Padding(
                    padding: const EdgeInsets.only(right: 16.0),
                    child: Icon(
                      Icons.check,
                      size: 16,
                      color: colorScheme.primary,
                    ),
                  ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildOrphanHeader(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;

    // Orphan header is not really selectable as a category, but maybe we want to show it?
    // Actually, "Orphan Tasks" is just a container. You can't "select" the container itself
    // in the same way you select a category, unless "No Category" is a valid selection?
    // The requirement is to select category OR task.
    // If I select a task in orphan list, category is null.
    // If I want to select "No Category" explicitly? Usually that means clearing selection?
    // Let's assume the header is just for expansion for now, unless user wants to filter by "No Category".
    // But let's keep it simple: just expansion.

    return Material(
      color: Colors.transparent,
      child: InkWell(
        onTap: _toggleOrphanExpansion, // Tapping header toggles expansion
        child: Padding(
          padding: const EdgeInsets.symmetric(vertical: 8.0, horizontal: 4.0),
          child: Row(
            children: [
              IconButton(
                icon: Icon(
                  _isOrphanExpanded
                      ? Icons.keyboard_arrow_down
                      : Icons.keyboard_arrow_right,
                  color: colorScheme.onSurfaceVariant,
                ),
                onPressed: _toggleOrphanExpansion,
                visualDensity: VisualDensity.compact,
              ),
              const SizedBox(width: 12), // Align with text
              Icon(
                Icons.folder_off_outlined,
                size: 20,
                color: colorScheme.onSurfaceVariant,
              ),
              const SizedBox(width: 12),
              Expanded(
                child: Text(
                  context.tr('focus.orphan_tasks_label'),
                  style: TextStyle(color: colorScheme.onSurface),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildTaskItem(
    BuildContext context,
    Task task,
    Category? category,
    bool isSelected,
  ) {
    final colorScheme = Theme.of(context).colorScheme;
    final isEnabled = widget.enabled;

    return Material(
      color:
          isSelected
              ? colorScheme.primaryContainer.withAlpha((255 * 0.4).round())
              : Colors.transparent,
      child: InkWell(
        onTap: isEnabled ? () => _handleTaskSelect(task, category) : null,
        child: Padding(
          padding: const EdgeInsets.fromLTRB(56, 12, 16, 12),
          child: Opacity(
            opacity: isEnabled ? 1.0 : 0.5,
            child: Row(
              children: [
                Expanded(
                  child: Text(
                    task.name,
                    style: TextStyle(
                      color:
                          isSelected
                              ? colorScheme.primary
                              : colorScheme.onSurfaceVariant,
                      fontWeight:
                          isSelected ? FontWeight.w500 : FontWeight.normal,
                    ),
                  ),
                ),
                if (isSelected)
                  Icon(Icons.check, size: 16, color: colorScheme.primary),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
