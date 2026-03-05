import 'package:easy_localization/easy_localization.dart' hide TextDirection;
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:focus_flow_app/domain/entities/task.dart';
import 'package:focus_flow_app/presentation/calendar/bloc/calendar_bloc.dart';
import 'package:focus_flow_app/presentation/calendar/bloc/calendar_event.dart';
import 'package:focus_flow_app/presentation/calendar/bloc/calendar_state.dart';
import 'package:focus_flow_app/presentation/calendar/widgets/scheduled_task_form_dialog.dart';

class DailyCalendarView extends StatefulWidget {
  const DailyCalendarView({super.key});

  @override
  State<DailyCalendarView> createState() => _DailyCalendarViewState();
}

class _DailyCalendarViewState extends State<DailyCalendarView> {
  final ScrollController _scrollController = ScrollController();
  static const double _hourHeight = 80.0;
  static const double _timeLabelWidth = 56.0;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) => _scrollToCurrentTime());
  }

  @override
  void dispose() {
    _scrollController.dispose();
    super.dispose();
  }

  void _scrollToCurrentTime() {
    if (!_scrollController.hasClients) return;
    final now = TimeOfDay.now();
    final minuteOfDay = now.hour * 60 + now.minute;
    final offset = (minuteOfDay / 60) * _hourHeight - 200;
    _scrollController.animateTo(
      offset.clamp(0.0, _scrollController.position.maxScrollExtent),
      duration: const Duration(milliseconds: 400),
      curve: Curves.easeInOut,
    );
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

  void _showTaskForm(BuildContext context, CalendarState state, {Task? task, DateTime? tapTime}) {
    showDialog(
      context: context,
      builder: (_) => ScheduledTaskFormDialog(
        task: task,
        initialDate: tapTime ?? state.focusedDate,
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
                context
                    .read<CalendarBloc>()
                    .add(DeleteScheduledTask(task.id));
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
        final focusedDay = state.focusedDate;
        final dayStart = DateTime(focusedDay.year, focusedDay.month, focusedDay.day);
        final tasks = state.tasksForDay(focusedDay);
        final now = DateTime.now();
        final isToday = DateUtils.isSameDay(focusedDay, now);

        return Column(
          children: [
            // Day header
            Padding(
              padding: const EdgeInsets.symmetric(vertical: 8),
              child: Text(
                DateFormat('EEEE, d MMMM yyyy').format(focusedDay),
                style: Theme.of(context).textTheme.titleSmall?.copyWith(
                  color: colorScheme.onSurfaceVariant,
                ),
              ),
            ),
            Expanded(
              child: SingleChildScrollView(
                controller: _scrollController,
                child: SizedBox(
                  height: 24 * _hourHeight,
                  child: Stack(
                    children: [
                      // Hourly grid
                      ...List.generate(25, (i) {
                        return Positioned(
                          top: i * _hourHeight,
                          left: 0,
                          right: 0,
                          child: Row(
                            children: [
                              SizedBox(
                                width: _timeLabelWidth,
                                child: Text(
                                  '${i.toString().padLeft(2, '0')}:00',
                                  textAlign: TextAlign.end,
                                  style: Theme.of(context).textTheme.labelSmall?.copyWith(
                                    color: colorScheme.onSurfaceVariant,
                                  ),
                                ),
                              ),
                              const SizedBox(width: 8),
                              Expanded(
                                child: Container(
                                  height: 1,
                                  color: colorScheme.outlineVariant.withAlpha(80),
                                ),
                              ),
                            ],
                          ),
                        );
                      }),

                      // Tap to add task
                      Positioned.fill(
                        left: _timeLabelWidth + 8,
                        child: GestureDetector(
                          onTapUp: (details) {
                            final tapY = details.localPosition.dy;
                            final hour = (tapY / _hourHeight).floor().clamp(0, 23);
                            final minute = ((tapY % _hourHeight) / _hourHeight * 60).round();
                            final tapTime = dayStart.add(Duration(hours: hour, minutes: minute));
                            _showTaskForm(context, state, tapTime: tapTime);
                          },
                          child: Container(color: Colors.transparent),
                        ),
                      ),

                      // Current time indicator (only today)
                      if (isToday)
                        Builder(builder: (_) {
                          final minuteOfDay = now.hour * 60 + now.minute;
                          final top = (minuteOfDay / 60) * _hourHeight;
                          return Positioned(
                            top: top,
                            left: 0,
                            right: 0,
                            child: Row(
                              children: [
                                SizedBox(
                                  width: _timeLabelWidth,
                                  child: Text(
                                    DateFormat('HH:mm').format(now),
                                    textAlign: TextAlign.end,
                                    style: Theme.of(context).textTheme.labelSmall?.copyWith(
                                      color: Colors.red,
                                      fontWeight: FontWeight.bold,
                                    ),
                                  ),
                                ),
                                const SizedBox(width: 8),
                                Expanded(
                                  child: Container(height: 2, color: Colors.red),
                                ),
                              ],
                            ),
                          );
                        }),

                      // Task blocks
                      ...tasks.where((t) => t.scheduledDate != null).map((task) {
                        final startDt = DateTime.fromMillisecondsSinceEpoch(
                          task.scheduledDate! * 1000,
                        );
                        final endDt = task.scheduledEndDate != null
                            ? DateTime.fromMillisecondsSinceEpoch(
                                task.scheduledEndDate! * 1000,
                              )
                            : startDt.add(const Duration(minutes: 30));

                        final startMin = startDt.hour * 60 + startDt.minute;
                        final durationMin = endDt.difference(startDt).inMinutes;
                        final displayMin = durationMin < 15 ? 15 : durationMin;

                        final top = (startMin / 60) * _hourHeight;
                        final height = (displayMin / 60) * _hourHeight;
                        final color = _taskColor(task, state);

                        final isCompleted = task.isCompleted;
                        return Positioned(
                          top: top,
                          left: _timeLabelWidth + 12,
                          right: 8,
                          height: height,
                          child: Opacity(
                            opacity: isCompleted ? 0.5 : 1.0,
                            child: GestureDetector(
                              onTap: () => _showTaskOptions(context, state, task),
                              child: Container(
                                decoration: BoxDecoration(
                                  color: color.withValues(alpha: 0.2),
                                  borderRadius: BorderRadius.circular(8),
                                  border: Border(
                                    left: BorderSide(color: color, width: 4),
                                  ),
                                ),
                                padding: const EdgeInsets.symmetric(
                                  horizontal: 8,
                                  vertical: 4,
                                ),
                                child: Column(
                                  crossAxisAlignment: CrossAxisAlignment.start,
                                  mainAxisAlignment: MainAxisAlignment.center,
                                  children: [
                                    Row(
                                      children: [
                                        if (isCompleted) ...[
                                          Icon(Icons.check_circle, size: 12, color: color),
                                          const SizedBox(width: 4),
                                        ],
                                        Expanded(
                                          child: Text(
                                            task.name,
                                            style: Theme.of(context).textTheme.labelMedium?.copyWith(
                                              fontWeight: FontWeight.bold,
                                              color: colorScheme.onSurface,
                                              decoration: isCompleted ? TextDecoration.lineThrough : null,
                                            ),
                                            maxLines: 1,
                                            overflow: TextOverflow.ellipsis,
                                          ),
                                        ),
                                      ],
                                    ),
                                    if (height >= 32)
                                      Text(
                                        '${DateFormat('HH:mm').format(startDt)} – ${DateFormat('HH:mm').format(endDt)}',
                                        style: Theme.of(context).textTheme.labelSmall?.copyWith(
                                          color: colorScheme.onSurfaceVariant,
                                          fontSize: 10,
                                        ),
                                      ),
                                  ],
                                ),
                              ),
                            ),
                          ),
                        );
                      }),
                    ],
                  ),
                ),
              ),
            ),
          ],
        );
      },
    );
  }
}
