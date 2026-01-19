import 'package:easy_localization/easy_localization.dart';
import 'package:flutter/material.dart';
import 'package:flutter_markdown_plus/flutter_markdown_plus.dart';
import '../common/markdown_editor_input.dart';
import 'package:focus_flow_app/domain/entities/category.dart';
import 'package:focus_flow_app/domain/entities/category_with_tasks.dart';
import 'package:focus_flow_app/domain/entities/focus_session.dart';
import 'package:focus_flow_app/domain/entities/task.dart';
import 'package:focus_flow_app/domain/repositories/session_repository.dart';
import 'package:get_it/get_it.dart';
import 'package:focus_flow_app/presentation/widgets/focus/minimalist_timer_header.dart';

class SessionDetailsModal extends StatefulWidget {
  final FocusSession session;
  final Category? category;
  final Task? task;
  final List<CategoryWithTasks> categories;
  final List<Task> orphanTasks;
  final VoidCallback? onSessionUpdated;

  const SessionDetailsModal({
    super.key,
    required this.session,
    this.category,
    this.task,
    this.categories = const [],
    this.orphanTasks = const [],
    this.onSessionUpdated,
  });

  @override
  State<SessionDetailsModal> createState() => _SessionDetailsModalState();
}

class _SessionDetailsModalState extends State<SessionDetailsModal> {
  late bool _isEditing;
  late TextEditingController _notesController;
  late int _concentrationScore;
  late DateTime _startTime;
  late DateTime _endTime;
  String? _selectedCategoryId;
  String? _selectedTaskId;

  @override
  void initState() {
    super.initState();
    _isEditing = false;
    _notesController = TextEditingController(text: widget.session.notes);
    _concentrationScore = widget.session.concentrationScore ?? 0;
    // Ensure score is within 0-5 range
    if (_concentrationScore > 5) _concentrationScore = 5;
    if (_concentrationScore < 0) _concentrationScore = 0;

    _startTime = DateTime.fromMillisecondsSinceEpoch(
      widget.session.startedAt * 1000,
    );
    _endTime =
        widget.session.endedAt != null
            ? DateTime.fromMillisecondsSinceEpoch(
              widget.session.endedAt! * 1000,
            )
            : DateTime.now();
    _selectedCategoryId = widget.session.categoryId;
    _selectedTaskId = widget.session.taskId;
  }

  @override
  void dispose() {
    _notesController.dispose();
    super.dispose();
  }

  Future<void> _saveChanges() async {
    final repository = GetIt.I<SessionRepository>();
    try {
      await repository.updateSession(
        id: widget.session.id,
        notes: _notesController.text,
        concentrationScore: _concentrationScore,
        startedAt: _startTime.millisecondsSinceEpoch ~/ 1000,
        endedAt: _endTime.millisecondsSinceEpoch ~/ 1000,
        categoryId: _selectedCategoryId,
        taskId: _selectedTaskId,
      );
      if (mounted) {
        widget.onSessionUpdated?.call();
        Navigator.pop(context);
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(context.tr('focus.save_success'))),
        );
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('${context.tr('focus.save_error')}: $e')),
        );
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;
    final isWorkSession = widget.session.sessionType == SessionType.work;

    String title;
    Color color;
    IconData icon;

    // Determine display values based on current selection or initial state
    Category? currentCategory;
    if (_selectedCategoryId != null) {
      try {
        currentCategory =
            widget.categories
                .firstWhere((c) => c.category.id == _selectedCategoryId)
                .category;
      } catch (_) {}
    }

    switch (widget.session.sessionType) {
      case SessionType.work:
        title = currentCategory?.name ?? context.tr('focus.session_title');
        color =
            currentCategory != null
                ? Color(
                  int.parse(currentCategory.color.replaceFirst('#', '0xFF')),
                )
                : colorScheme.primary;
        icon = Icons.work;
        break;
      case SessionType.shortBreak:
        title = context.tr('focus.short_break_title');
        color = Colors.green;
        icon = Icons.coffee;
        break;
      case SessionType.longBreak:
        title = context.tr('focus.long_break_title');
        color = Colors.blue;
        icon = Icons.weekend;
        break;
    }

    return Container(
      padding: const EdgeInsets.all(24),
      decoration: BoxDecoration(
        color: colorScheme.surface,
        borderRadius: const BorderRadius.vertical(top: Radius.circular(24)),
      ),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Center(
            child: Container(
              width: 40,
              height: 4,
              decoration: BoxDecoration(
                color: colorScheme.outlineVariant,
                borderRadius: BorderRadius.circular(2),
              ),
            ),
          ),
          const SizedBox(height: 16),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              if (_isEditing)
                TextButton(
                  onPressed: () => setState(() => _isEditing = false),
                  child: Text(context.tr('common.cancel')),
                )
              else
                const SizedBox(width: 48), // Spacer to balance the row

              if (_isEditing)
                FilledButton.icon(
                  onPressed: _saveChanges,
                  icon: const Icon(Icons.save, size: 18),
                  label: Text(context.tr('common.save')),
                )
              else
                IconButton.filledTonal(
                  onPressed: () => setState(() => _isEditing = true),
                  icon: const Icon(Icons.edit),
                  tooltip: context.tr('focus.edit_session_title'),
                ),
            ],
          ),
          const SizedBox(height: 16),

          if (!_isEditing) ...[
            // VIEW MODE
            Row(
              children: [
                Container(
                  padding: const EdgeInsets.all(12),
                  decoration: BoxDecoration(
                    color: color.withAlpha((255 * 0.1).round()),
                    shape: BoxShape.circle,
                  ),
                  child: Icon(icon, color: color, size: 32),
                ),
                const SizedBox(width: 16),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        title,
                        style: Theme.of(
                          context,
                        ).textTheme.headlineSmall?.copyWith(
                          fontWeight: FontWeight.bold,
                          color: colorScheme.onSurface,
                        ),
                      ),
                      if (isWorkSession && widget.task != null)
                        Text(
                          widget.task!.name,
                          style: Theme.of(context).textTheme.bodyLarge
                              ?.copyWith(color: colorScheme.onSurfaceVariant),
                        ),
                    ],
                  ),
                ),
              ],
            ),
            const SizedBox(height: 32),
            _buildDetailRow(
              context,
              Icons.timer_outlined,
              context.tr('focus.duration_label'),
              _formatDuration(_endTime.difference(_startTime)),
            ),
            const SizedBox(height: 16),
            _buildDetailRow(
              context,
              Icons.access_time,
              context.tr('focus.time_range_label'),
              '${DateFormat('HH:mm').format(_startTime)} - ${DateFormat('HH:mm').format(_endTime)}',
            ),
            if (widget.session.concentrationScore != null) ...[
              const SizedBox(height: 16),
              _buildDetailRow(
                context,
                Icons.psychology,
                context.tr('focus.level_badge'),
                '${widget.session.concentrationScore}/5',
              ),
            ],
            if (widget.session.notes != null &&
                widget.session.notes!.isNotEmpty) ...[
              const SizedBox(height: 24),
              Text(
                context.tr('focus.notes_title'),
                style: Theme.of(context).textTheme.titleSmall?.copyWith(
                  fontWeight: FontWeight.bold,
                  color: colorScheme.onSurface,
                ),
              ),
              const SizedBox(height: 8),
              Container(
                width: double.infinity,
                padding: const EdgeInsets.all(16),
                decoration: BoxDecoration(
                  color: colorScheme.surfaceContainerHighest.withAlpha(100),
                  borderRadius: BorderRadius.circular(12),
                  border: Border.all(color: colorScheme.outlineVariant),
                ),
                child: MarkdownBody(
                  data: widget.session.notes!,
                  styleSheet: MarkdownStyleSheet.fromTheme(Theme.of(context)),
                ),
              ),
            ],
          ] else ...[
            // EDIT MODE
            Text(
              context.tr('focus.edit_session_title'),
              style: Theme.of(
                context,
              ).textTheme.headlineSmall?.copyWith(fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 24),

            if (isWorkSession) ...[
              DropdownButtonFormField<String>(
                initialValue: _selectedCategoryId,
                decoration: InputDecoration(
                  labelText: context.tr('common.category'),
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(12),
                  ),
                  prefixIcon: const Icon(Icons.folder_outlined),
                ),
                items:
                    widget.categories.map((c) {
                      final categoryColor = Color(
                        int.parse(c.category.color.replaceFirst('#', '0xFF')),
                      );
                      return DropdownMenuItem(
                        value: c.category.id,
                        child: Row(
                          children: [
                            Container(
                              width: 12,
                              height: 12,
                              decoration: BoxDecoration(
                                color: categoryColor,
                                shape: BoxShape.circle,
                              ),
                            ),
                            const SizedBox(width: 12),
                            Text(c.category.name),
                          ],
                        ),
                      );
                    }).toList(),
                onChanged: (value) {
                  setState(() {
                    _selectedCategoryId = value;
                    // Reset task if category changes
                    _selectedTaskId = null;
                  });
                },
              ),
              const SizedBox(height: 16),
              DropdownButtonFormField<String>(
                initialValue: _selectedTaskId,
                decoration: InputDecoration(
                  labelText: context.tr('common.task'),
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(12),
                  ),
                  prefixIcon: const Icon(Icons.check_circle_outline),
                ),
                items:
                    _getAvailableTasks().map((t) {
                      return DropdownMenuItem(value: t.id, child: Text(t.name));
                    }).toList(),
                onChanged: (value) => setState(() => _selectedTaskId = value),
              ),
              const SizedBox(height: 16),
            ],

            Row(
              children: [
                Expanded(
                  child: InkWell(
                    onTap: () => _pickTime(context, true),
                    child: InputDecorator(
                      decoration: InputDecoration(
                        labelText: context.tr('common.start_time'),
                        border: OutlineInputBorder(
                          borderRadius: BorderRadius.circular(12),
                        ),
                        prefixIcon: const Icon(Icons.access_time),
                      ),
                      child: Text(DateFormat('HH:mm').format(_startTime)),
                    ),
                  ),
                ),
                const SizedBox(width: 16),
                Expanded(
                  child: InkWell(
                    onTap: () => _pickTime(context, false),
                    child: InputDecorator(
                      decoration: InputDecoration(
                        labelText: context.tr('common.end_time'),
                        border: OutlineInputBorder(
                          borderRadius: BorderRadius.circular(12),
                        ),
                        prefixIcon: const Icon(Icons.access_time_filled),
                      ),
                      child: Text(DateFormat('HH:mm').format(_endTime)),
                    ),
                  ),
                ),
              ],
            ),
            const SizedBox(height: 24),

            Text(
              context.tr('focus.level_badge'),
              style: Theme.of(
                context,
              ).textTheme.titleSmall?.copyWith(fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 8),
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
              decoration: BoxDecoration(
                border: Border.all(color: colorScheme.outlineVariant),
                borderRadius: BorderRadius.circular(12),
              ),
              child: Column(
                children: [
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      Text('0', style: Theme.of(context).textTheme.labelSmall),
                      Text(
                        _concentrationScore.toString(),
                        style: Theme.of(context).textTheme.titleLarge?.copyWith(
                          color: colorScheme.primary,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      Text('5', style: Theme.of(context).textTheme.labelSmall),
                    ],
                  ),
                  Slider(
                    value: _concentrationScore.toDouble(),
                    min: 0,
                    max: 5,
                    divisions: 5,
                    label: _concentrationScore.toString(),
                    onChanged:
                        (value) =>
                            setState(() => _concentrationScore = value.round()),
                  ),
                ],
              ),
            ),
            const SizedBox(height: 24),

            MarkdownEditorInput(
              controller: _notesController,
              label: context.tr('focus.notes_title'),
              fullScreenOverlayBuilder:
                  (context) => const MinimalistTimerHeader(),
            ),
          ],

          const SizedBox(height: 32),
        ],
      ),
    );
  }

  List<Task> _getAvailableTasks() {
    if (_selectedCategoryId == null) return widget.orphanTasks;
    try {
      return widget.categories
          .firstWhere((c) => c.category.id == _selectedCategoryId)
          .tasks;
    } catch (_) {
      return [];
    }
  }

  Future<void> _pickTime(BuildContext context, bool isStart) async {
    final initialTime = TimeOfDay.fromDateTime(isStart ? _startTime : _endTime);
    final picked = await showTimePicker(
      context: context,
      initialTime: initialTime,
    );

    if (picked != null) {
      setState(() {
        final now = DateTime.now();
        final newDateTime = DateTime(
          now.year,
          now.month,
          now.day,
          picked.hour,
          picked.minute,
        );

        if (isStart) {
          _startTime = newDateTime;
        } else {
          _endTime = newDateTime;
        }
      });
    }
  }

  String _formatDuration(Duration duration) {
    return '${duration.inMinutes}m ${duration.inSeconds % 60}s';
  }

  Widget _buildDetailRow(
    BuildContext context,
    IconData icon,
    String label,
    String value,
  ) {
    final colorScheme = Theme.of(context).colorScheme;
    return Row(
      children: [
        Icon(icon, size: 20, color: colorScheme.onSurfaceVariant),
        const SizedBox(width: 12),
        Text(
          label,
          style: Theme.of(
            context,
          ).textTheme.bodyMedium?.copyWith(color: colorScheme.onSurfaceVariant),
        ),
        const Spacer(),
        Text(
          value,
          style: Theme.of(context).textTheme.bodyLarge?.copyWith(
            fontWeight: FontWeight.w600,
            color: colorScheme.onSurface,
          ),
        ),
      ],
    );
  }
}
