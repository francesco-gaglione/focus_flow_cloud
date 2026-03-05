import 'package:easy_localization/easy_localization.dart' hide TextDirection;
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:focus_flow_app/domain/entities/task.dart';
import 'package:focus_flow_app/presentation/calendar/bloc/calendar_bloc.dart';
import 'package:focus_flow_app/presentation/calendar/bloc/calendar_event.dart';
import 'package:focus_flow_app/presentation/calendar/bloc/calendar_state.dart';
import 'package:focus_flow_app/presentation/calendar/widgets/scheduled_task_form_dialog.dart';

class WeeklyCalendarView extends StatelessWidget {
  const WeeklyCalendarView({super.key});

  List<DateTime> _weekDays(DateTime focused) {
    final monday = focused.subtract(Duration(days: focused.weekday - 1));
    return List.generate(7, (i) {
      final d = monday.add(Duration(days: i));
      return DateTime(d.year, d.month, d.day);
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

  void _showTaskForm(BuildContext context, CalendarState state, {Task? task, DateTime? day}) {
    showDialog(
      context: context,
      builder: (_) => ScheduledTaskFormDialog(
        task: task,
        initialDate: day ?? state.focusedDate,
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

  void _showTaskOptions(BuildContext context, CalendarState state, Task task) {
    showModalBottomSheet(
      context: context,
      builder: (ctx) => SafeArea(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            if (!task.isCompleted)
              ListTile(
                leading: const Icon(Icons.check_circle_outline, color: Colors.green),
                title: Text(context.tr('calendar.mark_complete')),
                onTap: () {
                  Navigator.pop(ctx);
                  context.read<CalendarBloc>().add(CompleteScheduledTask(task.id));
                },
              )
            else
              ListTile(
                leading: const Icon(Icons.radio_button_unchecked),
                title: Text(context.tr('calendar.mark_incomplete')),
                onTap: () {
                  Navigator.pop(ctx);
                  context.read<CalendarBloc>().add(UncompleteScheduledTask(task.id));
                },
              ),
            ListTile(
              leading: const Icon(Icons.edit_outlined),
              title: Text(context.tr('common.update')),
              onTap: () {
                Navigator.pop(ctx);
                _showTaskForm(context, state, task: task);
              },
            ),
            ListTile(
              leading: const Icon(Icons.delete_outline, color: Colors.red),
              title: Text(
                context.tr('common.delete'),
                style: const TextStyle(color: Colors.red),
              ),
              onTap: () {
                Navigator.pop(ctx);
                context.read<CalendarBloc>().add(DeleteScheduledTask(task.id));
              },
            ),
          ],
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<CalendarBloc, CalendarState>(
      builder: (context, state) {
        final colorScheme = Theme.of(context).colorScheme;
        final days = _weekDays(state.focusedDate);
        final today = DateTime.now();

        return Column(
          children: [
            // Day headers
            Row(
              children: days.map((day) {
                final isToday = DateUtils.isSameDay(day, today);
                return Expanded(
                  child: Container(
                    padding: const EdgeInsets.symmetric(vertical: 8),
                    decoration: BoxDecoration(
                      color: isToday
                          ? colorScheme.primaryContainer.withAlpha(120)
                          : null,
                      borderRadius: BorderRadius.circular(8),
                    ),
                    child: Column(
                      children: [
                        Text(
                          DateFormat('EEE').format(day),
                          style: Theme.of(context).textTheme.labelSmall?.copyWith(
                            color: isToday
                                ? colorScheme.primary
                                : colorScheme.onSurfaceVariant,
                          ),
                        ),
                        const SizedBox(height: 2),
                        Text(
                          day.day.toString(),
                          style: Theme.of(context).textTheme.titleSmall?.copyWith(
                            fontWeight: isToday ? FontWeight.bold : FontWeight.normal,
                            color: isToday ? colorScheme.primary : colorScheme.onSurface,
                          ),
                        ),
                      ],
                    ),
                  ),
                );
              }).toList(),
            ),
            const Divider(height: 1),

            // Week body
            Expanded(
              child: SingleChildScrollView(
                child: Row(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: days.map((day) {
                    final dayTasks = state.tasksForDay(day);
                    final isToday = DateUtils.isSameDay(day, today);

                    return Expanded(
                      child: Container(
                        constraints: const BoxConstraints(minHeight: 200),
                        decoration: BoxDecoration(
                          border: Border(
                            left: BorderSide(
                              color: colorScheme.outlineVariant.withAlpha(60),
                            ),
                          ),
                          color: isToday
                              ? colorScheme.primaryContainer.withAlpha(20)
                              : null,
                        ),
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.stretch,
                          children: [
                            // Task chips for this day
                            ...dayTasks.map((task) {
                              final color = _taskColor(task, state);
                              final isCompleted = task.isCompleted;
                              return Opacity(
                                opacity: isCompleted ? 0.5 : 1.0,
                                child: GestureDetector(
                                  onTap: () => _showTaskOptions(context, state, task),
                                  child: Container(
                                    margin: const EdgeInsets.symmetric(
                                      horizontal: 3,
                                      vertical: 2,
                                    ),
                                    padding: const EdgeInsets.symmetric(
                                      horizontal: 6,
                                      vertical: 3,
                                    ),
                                    decoration: BoxDecoration(
                                      color: color.withValues(alpha: 0.2),
                                      borderRadius: BorderRadius.circular(6),
                                      border: Border(
                                        left: BorderSide(color: color, width: 3),
                                      ),
                                    ),
                                    child: Column(
                                      crossAxisAlignment: CrossAxisAlignment.start,
                                      children: [
                                        Row(
                                          children: [
                                            if (isCompleted) ...[
                                              Icon(Icons.check_circle, size: 10, color: color),
                                              const SizedBox(width: 3),
                                            ],
                                            Expanded(
                                              child: Text(
                                                task.name,
                                                style: Theme.of(context)
                                                    .textTheme
                                                    .labelSmall
                                                    ?.copyWith(
                                                      fontWeight: FontWeight.w600,
                                                      fontSize: 10,
                                                      decoration: isCompleted ? TextDecoration.lineThrough : null,
                                                    ),
                                                maxLines: 2,
                                                overflow: TextOverflow.ellipsis,
                                              ),
                                            ),
                                          ],
                                        ),
                                        if (task.scheduledDate != null)
                                          Text(
                                            DateFormat('HH:mm').format(
                                              DateTime.fromMillisecondsSinceEpoch(
                                                task.scheduledDate! * 1000,
                                              ),
                                            ),
                                            style: Theme.of(context)
                                                .textTheme
                                                .labelSmall
                                                ?.copyWith(
                                                  fontSize: 9,
                                                  color: colorScheme.onSurfaceVariant,
                                                ),
                                          ),
                                      ],
                                    ),
                                  ),
                                ),
                              );
                            }),

                            // Add task button
                            InkWell(
                              onTap: () => _showTaskForm(context, state, day: day),
                              child: Container(
                                margin: const EdgeInsets.all(3),
                                padding: const EdgeInsets.symmetric(vertical: 4),
                                decoration: BoxDecoration(
                                  border: Border.all(
                                    color: colorScheme.outlineVariant.withAlpha(80),
                                    style: BorderStyle.solid,
                                  ),
                                  borderRadius: BorderRadius.circular(6),
                                ),
                                child: Icon(
                                  Icons.add,
                                  size: 14,
                                  color: colorScheme.onSurfaceVariant,
                                ),
                              ),
                            ),
                          ],
                        ),
                      ),
                    );
                  }).toList(),
                ),
              ),
            ),
          ],
        );
      },
    );
  }
}
