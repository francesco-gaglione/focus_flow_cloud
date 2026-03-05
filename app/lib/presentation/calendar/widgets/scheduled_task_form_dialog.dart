import 'package:easy_localization/easy_localization.dart';
import 'package:flutter/material.dart';
import 'package:focus_flow_app/domain/entities/category_with_tasks.dart';
import 'package:focus_flow_app/domain/entities/task.dart';
import 'package:focus_flow_app/presentation/widgets/common/custom_text_field.dart';

class ScheduledTaskFormDialog extends StatefulWidget {
  final Task? task; // null = create mode
  final DateTime? initialDate;
  final List<CategoryWithTasks> categories;
  final void Function({
    required String name,
    String? description,
    String? categoryId,
    required int scheduledDate,
    int? scheduledEndDate,
  }) onSubmit;

  const ScheduledTaskFormDialog({
    super.key,
    this.task,
    this.initialDate,
    required this.categories,
    required this.onSubmit,
  });

  @override
  State<ScheduledTaskFormDialog> createState() =>
      _ScheduledTaskFormDialogState();
}

class _ScheduledTaskFormDialogState extends State<ScheduledTaskFormDialog> {
  late TextEditingController _nameController;
  late TextEditingController _descController;
  String? _selectedCategoryId;
  DateTime? _selectedDate;
  TimeOfDay? _startTime;
  TimeOfDay? _endTime;

  bool get _isEditMode => widget.task != null;

  @override
  void initState() {
    super.initState();
    final task = widget.task;
    _nameController = TextEditingController(text: task?.name ?? '');
    _descController = TextEditingController(text: task?.description ?? '');
    _selectedCategoryId = task?.categoryId;

    if (task?.scheduledDate != null) {
      final dt = DateTime.fromMillisecondsSinceEpoch(task!.scheduledDate! * 1000);
      _selectedDate = DateTime(dt.year, dt.month, dt.day);
      _startTime = TimeOfDay(hour: dt.hour, minute: dt.minute);
    } else {
      _selectedDate = widget.initialDate != null
          ? DateTime(
              widget.initialDate!.year,
              widget.initialDate!.month,
              widget.initialDate!.day,
            )
          : DateTime.now();
      _startTime = TimeOfDay.now();
    }

    if (task?.scheduledEndDate != null) {
      final dt = DateTime.fromMillisecondsSinceEpoch(task!.scheduledEndDate! * 1000);
      _endTime = TimeOfDay(hour: dt.hour, minute: dt.minute);
    } else {
      // Default end time = start time + 1h
      if (_startTime != null) {
        final end = _startTime!.hour + 1;
        _endTime = TimeOfDay(hour: end % 24, minute: _startTime!.minute);
      }
    }
  }

  @override
  void dispose() {
    _nameController.dispose();
    _descController.dispose();
    super.dispose();
  }

  Future<void> _pickDate() async {
    final picked = await showDatePicker(
      context: context,
      initialDate: _selectedDate ?? DateTime.now(),
      firstDate: DateTime(2020),
      lastDate: DateTime(2030),
    );
    if (picked != null) setState(() => _selectedDate = picked);
  }

  Future<void> _pickStartTime() async {
    final picked = await showTimePicker(
      context: context,
      initialTime: _startTime ?? TimeOfDay.now(),
    );
    if (picked != null) setState(() => _startTime = picked);
  }

  Future<void> _pickEndTime() async {
    final picked = await showTimePicker(
      context: context,
      initialTime: _endTime ?? TimeOfDay.now(),
    );
    if (picked != null) setState(() => _endTime = picked);
  }

  int? _buildTimestamp(DateTime? date, TimeOfDay? time) {
    if (date == null) return null;
    final t = time ?? const TimeOfDay(hour: 0, minute: 0);
    return DateTime(date.year, date.month, date.day, t.hour, t.minute)
            .millisecondsSinceEpoch ~/
        1000;
  }

  void _submit() {
    final name = _nameController.text.trim();
    if (name.isEmpty) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(context.tr('task.name_required')),
          behavior: SnackBarBehavior.floating,
        ),
      );
      return;
    }
    if (_selectedDate == null) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(context.tr('calendar.date_required')),
          behavior: SnackBarBehavior.floating,
        ),
      );
      return;
    }

    final scheduledDate = _buildTimestamp(_selectedDate, _startTime)!;
    final scheduledEndDate = _buildTimestamp(_selectedDate, _endTime);

    if (scheduledEndDate != null && scheduledEndDate <= scheduledDate) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(context.tr('calendar.end_before_start')),
          behavior: SnackBarBehavior.floating,
        ),
      );
      return;
    }

    widget.onSubmit(
      name: name,
      description: _descController.text.trim().isEmpty
          ? null
          : _descController.text.trim(),
      categoryId: _selectedCategoryId,
      scheduledDate: scheduledDate,
      scheduledEndDate: scheduledEndDate,
    );
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;
    final dateStr = _selectedDate != null
        ? DateFormat('EEE, d MMM yyyy').format(_selectedDate!)
        : context.tr('calendar.pick_date');
    final startStr = _startTime != null ? _startTime!.format(context) : '--:--';
    final endStr = _endTime != null ? _endTime!.format(context) : '--:--';

    return AlertDialog(
      insetPadding: const EdgeInsets.symmetric(horizontal: 24),
      icon: Icon(
        _isEditMode ? Icons.edit_calendar : Icons.calendar_month,
        color: colorScheme.primary,
      ),
      title: Text(
        _isEditMode
            ? context.tr('calendar.edit_task')
            : context.tr('calendar.create_task'),
      ),
      content: SizedBox(
        width: 420,
        child: SingleChildScrollView(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              CustomTextField(
                controller: _nameController,
                label: context.tr('task.name_label'),
                icon: Icons.task_outlined,
                hint: context.tr('task.name_hint'),
                textCapitalization: TextCapitalization.sentences,
                autofocus: true,
              ),
              const SizedBox(height: 12),
              CustomTextField(
                controller: _descController,
                label: context.tr('task.description_label'),
                icon: Icons.notes_outlined,
                hint: context.tr('task.description_hint'),
                maxLines: 3,
                minLines: 2,
                textCapitalization: TextCapitalization.sentences,
              ),
              const SizedBox(height: 16),

              // Category selector
              Text(
                context.tr('focus.category_label'),
                style: Theme.of(context).textTheme.labelMedium?.copyWith(
                  color: colorScheme.onSurfaceVariant,
                ),
              ),
              const SizedBox(height: 6),
              DropdownButtonFormField<String?>(
                initialValue: _selectedCategoryId,
                decoration: InputDecoration(
                  contentPadding: const EdgeInsets.symmetric(
                    horizontal: 12,
                    vertical: 8,
                  ),
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(12),
                  ),
                  prefixIcon: const Icon(Icons.category_outlined),
                ),
                hint: Text(context.tr('calendar.no_category')),
                items: [
                  DropdownMenuItem<String?>(
                    value: null,
                    child: Text(context.tr('calendar.no_category')),
                  ),
                  ...widget.categories.map(
                    (cat) => DropdownMenuItem<String?>(
                      value: cat.category.id,
                      child: Row(
                        children: [
                          Container(
                            width: 12,
                            height: 12,
                            decoration: BoxDecoration(
                              color: Color(
                                int.parse(
                                  cat.category.color.replaceFirst('#', '0xFF'),
                                ),
                              ),
                              shape: BoxShape.circle,
                            ),
                          ),
                          const SizedBox(width: 8),
                          Text(cat.category.name),
                        ],
                      ),
                    ),
                  ),
                ],
                onChanged: (val) => setState(() => _selectedCategoryId = val),
              ),
              const SizedBox(height: 16),

              // Date picker
              Text(
                context.tr('calendar.date_label'),
                style: Theme.of(context).textTheme.labelMedium?.copyWith(
                  color: colorScheme.onSurfaceVariant,
                ),
              ),
              const SizedBox(height: 6),
              OutlinedButton.icon(
                icon: const Icon(Icons.calendar_today, size: 18),
                label: Text(dateStr),
                onPressed: _pickDate,
                style: OutlinedButton.styleFrom(
                  minimumSize: const Size.fromHeight(44),
                ),
              ),
              const SizedBox(height: 16),

              // Time range
              Text(
                context.tr('focus.time_range_label'),
                style: Theme.of(context).textTheme.labelMedium?.copyWith(
                  color: colorScheme.onSurfaceVariant,
                ),
              ),
              const SizedBox(height: 6),
              Row(
                children: [
                  Expanded(
                    child: OutlinedButton.icon(
                      icon: const Icon(Icons.schedule, size: 16),
                      label: Text(startStr),
                      onPressed: _pickStartTime,
                    ),
                  ),
                  const Padding(
                    padding: EdgeInsets.symmetric(horizontal: 8),
                    child: Text('→'),
                  ),
                  Expanded(
                    child: OutlinedButton.icon(
                      icon: const Icon(Icons.schedule, size: 16),
                      label: Text(endStr),
                      onPressed: _pickEndTime,
                    ),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.pop(context),
          child: Text(context.tr('common.cancel')),
        ),
        FilledButton(
          onPressed: _submit,
          child: Text(
            _isEditMode
                ? context.tr('common.update')
                : context.tr('common.create'),
          ),
        ),
      ],
    );
  }
}
