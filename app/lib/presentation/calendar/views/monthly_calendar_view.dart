import 'package:easy_localization/easy_localization.dart' hide TextDirection;
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:focus_flow_app/domain/entities/task.dart';
import 'package:focus_flow_app/presentation/calendar/bloc/calendar_bloc.dart';
import 'package:focus_flow_app/presentation/calendar/bloc/calendar_event.dart';
import 'package:focus_flow_app/presentation/calendar/bloc/calendar_state.dart';
import 'package:focus_flow_app/presentation/calendar/widgets/scheduled_task_form_dialog.dart';

class MonthlyCalendarView extends StatelessWidget {
  const MonthlyCalendarView({super.key});

  /// Returns all days to display in the grid (includes trailing/leading days).
  List<DateTime?> _buildCalendarGrid(DateTime focusedMonth) {
    final firstDay = DateTime(focusedMonth.year, focusedMonth.month, 1);
    final lastDay = DateTime(focusedMonth.year, focusedMonth.month + 1, 0);
    // Fill leading nulls for Mon=1, ..., Sun=7
    final leadingBlanks = firstDay.weekday - 1;
    final totalCells = leadingBlanks + lastDay.day;
    final rows = (totalCells / 7).ceil();
    final cells = rows * 7;
    return List.generate(cells, (i) {
      final dayIndex = i - leadingBlanks + 1;
      if (dayIndex < 1 || dayIndex > lastDay.day) return null;
      return DateTime(focusedMonth.year, focusedMonth.month, dayIndex);
    });
  }

  Color _taskColor(Task task, CalendarState state) {
    if (task.categoryId != null) {
      try {
        final cat = state.categories
            .firstWhere((c) => c.category.id == task.categoryId)
            .category;
        return Color(int.parse(cat.color.replaceFirst('#', '0xFF')));
      } catch (_) {}
    }
    return const Color(0xFFFFA726);
  }

  void _showTaskForm(BuildContext context, CalendarState state, {DateTime? day}) {
    showDialog(
      context: context,
      builder: (_) => ScheduledTaskFormDialog(
        initialDate: day ?? state.focusedDate,
        categories: state.categories,
        onSubmit: ({
          required String name,
          String? description,
          String? categoryId,
          required int scheduledDate,
          int? scheduledEndDate,
        }) {
          context.read<CalendarBloc>().add(
            CreateScheduledTask(
              name: name,
              description: description,
              categoryId: categoryId,
              scheduledDate: scheduledDate,
              scheduledEndDate: scheduledEndDate,
            ),
          );
        },
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<CalendarBloc, CalendarState>(
      builder: (context, state) {
        final colorScheme = Theme.of(context).colorScheme;
        final today = DateTime.now();
        final cells = _buildCalendarGrid(state.focusedDate);

        return Column(
          children: [
            // Month header
            Padding(
              padding: const EdgeInsets.symmetric(vertical: 8),
              child: Text(
                DateFormat('MMMM yyyy').format(state.focusedDate),
                style: Theme.of(context).textTheme.titleSmall?.copyWith(
                  color: colorScheme.onSurfaceVariant,
                ),
              ),
            ),

            // Weekday headers
            Row(
              children: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun']
                  .map(
                    (d) => Expanded(
                      child: Text(
                        d,
                        textAlign: TextAlign.center,
                        style: Theme.of(context).textTheme.labelSmall?.copyWith(
                          color: colorScheme.onSurfaceVariant,
                          fontWeight: FontWeight.w600,
                        ),
                      ),
                    ),
                  )
                  .toList(),
            ),
            const SizedBox(height: 4),

            // Calendar grid
            Expanded(
              child: GridView.builder(
                padding: EdgeInsets.zero,
                physics: const NeverScrollableScrollPhysics(),
                gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
                  crossAxisCount: 7,
                  childAspectRatio: 0.85,
                ),
                itemCount: cells.length,
                itemBuilder: (context, index) {
                  final day = cells[index];
                  if (day == null) {
                    return const SizedBox.shrink();
                  }

                  final isToday = DateUtils.isSameDay(day, today);
                  final isSelected = state.selectedDay != null &&
                      DateUtils.isSameDay(day, state.selectedDay!);
                  final dayTasks = state.tasksForDay(day);

                  return GestureDetector(
                    onTap: () {
                      context.read<CalendarBloc>().add(SelectDay(day));
                    },
                    onLongPress: () => _showTaskForm(context, state, day: day),
                    child: AnimatedContainer(
                      duration: const Duration(milliseconds: 150),
                      margin: const EdgeInsets.all(2),
                      decoration: BoxDecoration(
                        color: isSelected
                            ? colorScheme.primaryContainer
                            : isToday
                                ? colorScheme.primaryContainer.withAlpha(80)
                                : null,
                        borderRadius: BorderRadius.circular(8),
                        border: isToday
                            ? Border.all(
                                color: colorScheme.primary,
                                width: 1.5,
                              )
                            : null,
                      ),
                      child: Column(
                        mainAxisAlignment: MainAxisAlignment.start,
                        children: [
                          // Day number
                          Padding(
                            padding: const EdgeInsets.only(top: 4),
                            child: Text(
                              day.day.toString(),
                              style: Theme.of(context).textTheme.labelMedium?.copyWith(
                                fontWeight: isToday || isSelected
                                    ? FontWeight.bold
                                    : FontWeight.normal,
                                color: isSelected
                                    ? colorScheme.onPrimaryContainer
                                    : isToday
                                        ? colorScheme.primary
                                        : colorScheme.onSurface,
                              ),
                            ),
                          ),
                          const SizedBox(height: 2),

                          // Task dots / chips
                          if (dayTasks.isNotEmpty)
                            Wrap(
                              alignment: WrapAlignment.center,
                              spacing: 2,
                              runSpacing: 2,
                              children: dayTasks.take(3).map((task) {
                                final color = _taskColor(task, state);
                                return Container(
                                  width: 6,
                                  height: 6,
                                  decoration: BoxDecoration(
                                    color: color,
                                    shape: BoxShape.circle,
                                  ),
                                );
                              }).toList(),
                            ),

                          if (dayTasks.length > 3)
                            Text(
                              '+${dayTasks.length - 3}',
                              style: Theme.of(context).textTheme.labelSmall?.copyWith(
                                fontSize: 8,
                                color: colorScheme.onSurfaceVariant,
                              ),
                            ),
                        ],
                      ),
                    ),
                  );
                },
              ),
            ),

            // Selected day tasks list
            if (state.selectedDay != null)
              _SelectedDayTaskList(
                day: state.selectedDay!,
                tasks: state.tasksForDay(state.selectedDay!),
                state: state,
              ),
          ],
        );
      },
    );
  }
}

class _SelectedDayTaskList extends StatelessWidget {
  final DateTime day;
  final List<Task> tasks;
  final CalendarState state;

  const _SelectedDayTaskList({
    required this.day,
    required this.tasks,
    required this.state,
  });

  void _showTaskForm(BuildContext context, {Task? task}) {
    showDialog(
      context: context,
      builder: (_) => ScheduledTaskFormDialog(
        task: task,
        initialDate: day,
        categories: state.categories,
        onSubmit: ({
          required String name,
          String? description,
          String? categoryId,
          required int scheduledDate,
          int? scheduledEndDate,
        }) {
          if (task == null) {
            context.read<CalendarBloc>().add(
              CreateScheduledTask(
                name: name,
                description: description,
                categoryId: categoryId,
                scheduledDate: scheduledDate,
                scheduledEndDate: scheduledEndDate,
              ),
            );
          } else {
            context.read<CalendarBloc>().add(
              UpdateScheduledTask(
                id: task.id,
                name: name,
                description: description,
                categoryId: categoryId,
                scheduledDate: scheduledDate,
                scheduledEndDate: scheduledEndDate,
              ),
            );
          }
        },
      ),
    );
  }

  Color _taskColor(Task task) {
    if (task.categoryId != null) {
      try {
        final cat = state.categories
            .firstWhere((c) => c.category.id == task.categoryId)
            .category;
        return Color(int.parse(cat.color.replaceFirst('#', '0xFF')));
      } catch (_) {}
    }
    return const Color(0xFFFFA726);
  }

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;

    return Container(
      constraints: const BoxConstraints(maxHeight: 200),
      decoration: BoxDecoration(
        color: colorScheme.surfaceContainerHighest.withAlpha(80),
        borderRadius: const BorderRadius.vertical(top: Radius.circular(16)),
        border: Border(
          top: BorderSide(
            color: colorScheme.outlineVariant.withAlpha(80),
          ),
        ),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisSize: MainAxisSize.min,
        children: [
          Padding(
            padding: const EdgeInsets.fromLTRB(16, 12, 8, 4),
            child: Row(
              children: [
                Text(
                  DateFormat('d MMMM').format(day),
                  style: Theme.of(context).textTheme.titleSmall,
                ),
                const Spacer(),
                IconButton(
                  icon: const Icon(Icons.add, size: 20),
                  onPressed: () => _showTaskForm(context),
                  tooltip: context.tr('calendar.create_task'),
                ),
              ],
            ),
          ),
          if (tasks.isEmpty)
            Padding(
              padding: const EdgeInsets.fromLTRB(16, 4, 16, 12),
              child: Text(
                context.tr('calendar.no_tasks_day'),
                style: Theme.of(context).textTheme.bodySmall?.copyWith(
                  color: colorScheme.onSurfaceVariant,
                ),
              ),
            )
          else
            Flexible(
              child: ListView.builder(
                shrinkWrap: true,
                itemCount: tasks.length,
                itemBuilder: (_, i) {
                  final task = tasks[i];
                  final color = _taskColor(task);
                  final isCompleted = task.isCompleted;
                  return Opacity(
                    opacity: isCompleted ? 0.6 : 1.0,
                    child: ListTile(
                      dense: true,
                      leading: isCompleted
                          ? Icon(Icons.check_circle, size: 14, color: color)
                          : Container(
                              width: 12,
                              height: 12,
                              decoration: BoxDecoration(
                                color: color,
                                shape: BoxShape.circle,
                              ),
                            ),
                      title: Text(
                        task.name,
                        style: TextStyle(
                          decoration: isCompleted ? TextDecoration.lineThrough : null,
                        ),
                      ),
                      subtitle: task.scheduledDate != null
                          ? Text(
                              DateFormat('HH:mm').format(
                                DateTime.fromMillisecondsSinceEpoch(
                                  task.scheduledDate! * 1000,
                                ),
                              ),
                            )
                          : null,
                      trailing: Row(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          if (!isCompleted)
                            IconButton(
                              icon: const Icon(Icons.check_circle_outline, size: 18, color: Colors.green),
                              tooltip: context.tr('calendar.mark_complete'),
                              onPressed: () {
                                context.read<CalendarBloc>().add(CompleteScheduledTask(task.id));
                              },
                            )
                          else
                            IconButton(
                              icon: const Icon(Icons.radio_button_unchecked, size: 18),
                              tooltip: context.tr('calendar.mark_incomplete'),
                              onPressed: () {
                                context.read<CalendarBloc>().add(UncompleteScheduledTask(task.id));
                              },
                            ),
                          IconButton(
                            icon: const Icon(Icons.edit_outlined, size: 18),
                            onPressed: () => _showTaskForm(context, task: task),
                          ),
                          IconButton(
                            icon: const Icon(
                              Icons.delete_outline,
                              size: 18,
                              color: Colors.red,
                            ),
                            onPressed: () {
                              context
                                  .read<CalendarBloc>()
                                  .add(DeleteScheduledTask(task.id));
                            },
                          ),
                        ],
                      ),
                    ),
                  );
                },
              ),
            ),
        ],
      ),
    );
  }
}
