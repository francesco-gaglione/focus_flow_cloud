import 'package:easy_localization/easy_localization.dart' hide TextDirection;
import 'package:flutter/material.dart';
import 'package:focus_flow_app/domain/entities/category.dart';
import 'package:focus_flow_app/domain/entities/category_with_tasks.dart';
import 'package:focus_flow_app/domain/entities/focus_session.dart';
import 'package:focus_flow_app/domain/entities/task.dart';
import 'package:focus_flow_app/presentation/widgets/focus/session_details_modal.dart';

class FocusTimelineWidget extends StatefulWidget {
  final List<FocusSession> sessions;
  final List<CategoryWithTasks> categories;
  final List<Task> orphanTasks;
  final VoidCallback? onSessionUpdated;

  const FocusTimelineWidget({
    super.key,
    this.sessions = const [],
    this.categories = const [],
    this.orphanTasks = const [],
    this.onSessionUpdated,
  });

  @override
  State<FocusTimelineWidget> createState() => _FocusTimelineWidgetState();
}

class _FocusTimelineWidgetState extends State<FocusTimelineWidget> {
  final ScrollController _scrollController = ScrollController();
  final double _hourHeight = 240.0;
  final int _startHour = 0;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      _scrollToCurrentTime();
    });
  }

  @override
  void dispose() {
    _scrollController.dispose();
    super.dispose();
  }

  @override
  void didUpdateWidget(FocusTimelineWidget oldWidget) {
    super.didUpdateWidget(oldWidget);
    // If we transition from empty to having sessions, try scrolling
    if (oldWidget.sessions.isEmpty && widget.sessions.isNotEmpty) {
      WidgetsBinding.instance.addPostFrameCallback((_) {
        _scrollToCurrentTime();
      });
    }
  }

  void _scrollToCurrentTime() {
    if (!_scrollController.hasClients) return;
    final now = DateTime.now();
    final currentMinute = now.hour * 60 + now.minute;
    // Calculate the top position of the current time line
    final topPosition = ((currentMinute / 60) - _startHour) * _hourHeight;
    // Center it in the 600px container (subtract half height = 300)
    final offset = topPosition - 300;
    
    _scrollController.animateTo(
      offset.clamp(0.0, _scrollController.position.maxScrollExtent),
      duration: const Duration(milliseconds: 500),
      curve: Curves.easeInOut,
    );
  }

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;
    const double timeLabelWidth = 60.0;
    const int endHour = 24;

    return Container(
      decoration: BoxDecoration(
        color: colorScheme.surfaceContainerHighest.withAlpha(
          (255 * 0.3).round(),
        ),
        borderRadius: BorderRadius.circular(24),
        border: Border.all(
          color: colorScheme.outlineVariant.withAlpha((255 * 0.2).round()),
        ),
      ),
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Container(
                  padding: const EdgeInsets.all(8),
                  decoration: BoxDecoration(
                    color: colorScheme.primary.withAlpha((255 * 0.1).round()),
                    shape: BoxShape.circle,
                  ),
                  child: Icon(
                    Icons.calendar_view_day,
                    color: colorScheme.primary,
                  ),
                ),
                const SizedBox(width: 12),
                Text(
                  context.tr('focus.timeline_title'),
                  style: Theme.of(context).textTheme.titleMedium?.copyWith(
                    fontWeight: FontWeight.bold,
                    letterSpacing: 0.5,
                  ),
                ),
                const Spacer(),
                IconButton(
                  onPressed: _scrollToCurrentTime,
                  icon: Icon(Icons.my_location, color: colorScheme.primary),
                  tooltip: 'Go to current time',
                ),
              ],
            ),
            const SizedBox(height: 24),

            if (widget.sessions.isEmpty)
              Center(
                child: Padding(
                  padding: const EdgeInsets.all(32),
                  child: Column(
                    children: [
                      Icon(
                        Icons.event_available_outlined,
                        size: 64,
                        color: colorScheme.outline.withAlpha(
                          (255 * 0.3).round(),
                        ),
                      ),
                      const SizedBox(height: 16),
                      Text(
                        context.tr('focus.timeline_empty'),
                        textAlign: TextAlign.center,
                        style: Theme.of(context).textTheme.bodyLarge?.copyWith(
                          color: colorScheme.onSurfaceVariant,
                        ),
                      ),
                    ],
                  ),
                ),
              )
            else
              Container(
                height: 600,
                decoration: BoxDecoration(
                  color: colorScheme.surface.withAlpha((255 * 0.5).round()),
                  borderRadius: BorderRadius.circular(16),
                  border: Border.all(
                    color: colorScheme.outlineVariant.withAlpha(
                      (255 * 0.2).round(),
                    ),
                  ),
                ),
                child: SingleChildScrollView(
                  controller: _scrollController,
                  padding: const EdgeInsets.symmetric(vertical: 16),
                  child: SizedBox(
                    height: (endHour - _startHour) * _hourHeight,
                    child: Stack(
                      children: [
                        // Grid Lines and Time Labels
                        ...List.generate(endHour - _startHour + 1, (index) {
                          final hour = _startHour + index;
                          final top = index * _hourHeight;
                          return Positioned(
                            top: top,
                            left: 0,
                            right: 0,
                            child: Row(
                              children: [
                                SizedBox(
                                  width: timeLabelWidth,
                                  child: Text(
                                    '${hour.toString().padLeft(2, '0')}:00',
                                    textAlign: TextAlign.end,
                                    style: Theme.of(
                                      context,
                                    ).textTheme.labelSmall?.copyWith(
                                      color: colorScheme.onSurfaceVariant,
                                    ),
                                  ),
                                ),
                                const SizedBox(width: 12),
                                Expanded(
                                  child: Container(
                                    height: 1,
                                    color: colorScheme.outlineVariant.withAlpha(
                                      (255 * 0.3).round(),
                                    ),
                                  ),
                                ),
                              ],
                            ),
                          );
                        }),

                        // Current Time Indicator
                        Builder(
                          builder: (context) {
                            final now = DateTime.now();
                            final currentMinute = now.hour * 60 + now.minute;
                            final top =
                                (currentMinute / 60) * _hourHeight -
                                (_startHour * _hourHeight);
                            return Positioned(
                              top: top,
                              left: 0,
                              right: 0,
                              child: Row(
                                children: [
                                  SizedBox(
                                    width: timeLabelWidth,
                                    child: Text(
                                      DateFormat('HH:mm').format(now),
                                      textAlign: TextAlign.end,
                                      style: Theme.of(
                                        context,
                                      ).textTheme.labelSmall?.copyWith(
                                        color: Colors.red,
                                        fontWeight: FontWeight.bold,
                                      ),
                                    ),
                                  ),
                                  const SizedBox(width: 12),
                                  Expanded(
                                    child: Container(
                                      height: 2,
                                      color: Colors.red,
                                    ),
                                  ),
                                ],
                              ),
                            );
                          },
                        ),

                        // Sessions
                        ...widget.sessions.map((session) {
                          final startTime = DateTime.fromMillisecondsSinceEpoch(
                            session.startedAt * 1000,
                          );
                          final endTime =
                              session.endedAt != null
                                  ? DateTime.fromMillisecondsSinceEpoch(
                                    session.endedAt! * 1000,
                                  )
                                  : DateTime.now();

                          final startMinuteOfDay =
                              startTime.hour * 60 + startTime.minute;
                          final endMinuteOfDay =
                              endTime.hour * 60 + endTime.minute;

                          final relativeStartMinute =
                              startMinuteOfDay - (_startHour * 60);
                          final durationMinutes =
                              endMinuteOfDay - startMinuteOfDay;

                          // Ensure visible minimum height (e.g., 5 mins = 15px at 3px/min)
                          final displayDuration =
                              durationMinutes < 5 ? 5 : durationMinutes;

                          final top =
                              (relativeStartMinute / 60) * _hourHeight;
                          final height =
                              (displayDuration / 60) * _hourHeight;

                          Category? category;
                          Task? task;
                          String title;
                          Color color;
                          IconData icon;

                          if (session.sessionType == SessionType.work) {
                            if (session.categoryId != null) {
                              try {
                                category =
                                    widget.categories
                                        .firstWhere(
                                          (CategoryWithTasks c) =>
                                              c.category.id ==
                                              session.categoryId,
                                        )
                                        .category;
                              } catch (e) {
                                category = null;
                              }
                            }

                            if (session.taskId != null) {
                              if (category != null) {
                                try {
                                  task = widget.categories
                                      .firstWhere(
                                        (CategoryWithTasks c) =>
                                            c.category.id == session.categoryId,
                                      )
                                      .tasks
                                      .firstWhere(
                                        (t) => t.id == session.taskId,
                                      );
                                } catch (e) {
                                  task = null;
                                }
                              } else {
                                try {
                                  task = widget.orphanTasks.firstWhere(
                                    (t) => t.id == session.taskId,
                                  );
                                } catch (e) {
                                  task = null;
                                }
                              }
                            }

                            title = category?.name ?? 'Uncategorized';
                            color =
                                category != null
                                    ? Color(
                                      int.parse(
                                        category.color.replaceFirst(
                                          '#',
                                          '0xFF',
                                        ),
                                      ),
                                    )
                                    : Colors.grey;
                            icon = Icons.work;
                          } else if (session.sessionType ==
                              SessionType.shortBreak) {
                            title = context.tr('focus.short_break_title');
                            color = Colors.green;
                            icon = Icons.coffee;
                          } else {
                            title = context.tr('focus.long_break_title');
                            color = Colors.blue;
                            icon = Icons.weekend;
                          }

                          final showTimeRange = height > 40;

                          return Positioned(
                            top: top,
                            left: timeLabelWidth + 12,
                            right: 16,
                            height: height,
                            child: InkWell(
                              onTap: () {
                                showModalBottomSheet(
                                  context: context,
                                  isScrollControlled: true,
                                  backgroundColor: Colors.transparent,
                                  builder:
                                      (context) => SessionDetailsModal(
                                        session: session,
                                        category: category,
                                        task: task,
                                        categories: widget.categories,
                                        orphanTasks: widget.orphanTasks,
                                        onSessionUpdated: widget.onSessionUpdated,
                                      ),
                                );
                              },
                              child: ClipRRect(
                                borderRadius: BorderRadius.circular(8),
                                child: Container(
                                  color: color.withValues(alpha: 0.2),
                                  child: Row(
                                    children: [
                                      // Left colored strip (simulates left border)
                                      Container(
                                        width: 4,
                                        color: color,
                                      ),
                                      Expanded(
                                        child: Padding(
                                          padding: EdgeInsets.symmetric(
                                            horizontal: 8,
                                            // Remove vertical padding for small items to ensure centering works
                                            vertical: 0,
                                          ),
                                          child: Row(
                                            children: [
                                              if (height >= 20) ...[
                                                Icon(
                                                  icon,
                                                  size: 14,
                                                  color: color,
                                                ),
                                                const SizedBox(width: 8),
                                              ],
                                              Expanded(
                                                child: Column(
                                                  crossAxisAlignment:
                                                      CrossAxisAlignment.start,
                                                  mainAxisAlignment:
                                                      MainAxisAlignment.center,
                                                  children: [
                                                    Text(
                                                      '$title • ${durationMinutes}m',
                                                      style: Theme.of(
                                                        context,
                                                      ).textTheme.labelSmall
                                                          ?.copyWith(
                                                            fontWeight:
                                                                FontWeight.bold,
                                                            color:
                                                                colorScheme
                                                                    .onSurface,
                                                            fontSize:
                                                                height < 20
                                                                    ? 10
                                                                    : null,
                                                          ),
                                                      maxLines: 1,
                                                      overflow:
                                                          TextOverflow.ellipsis,
                                                    ),
                                                    if (showTimeRange)
                                                      Text(
                                                        '${DateFormat('HH:mm').format(startTime)} - ${DateFormat('HH:mm').format(endTime)}',
                                                        style: Theme.of(
                                                          context,
                                                        ).textTheme.bodySmall
                                                            ?.copyWith(
                                                              fontSize: 10,
                                                              color:
                                                                  colorScheme
                                                                      .onSurfaceVariant,
                                                            ),
                                                        maxLines: 1,
                                                        overflow:
                                                            TextOverflow
                                                                .ellipsis,
                                                      ),
                                                  ],
                                                ),
                                              ),
                                            ],
                                          ),
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
        ),
      ),
    );
  }
}
