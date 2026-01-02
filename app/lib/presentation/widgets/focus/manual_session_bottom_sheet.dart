import 'package:easy_localization/easy_localization.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:focus_flow_app/domain/entities/category.dart';
import 'package:focus_flow_app/domain/entities/task.dart';
import 'package:focus_flow_app/presentation/focus/bloc/focus_bloc.dart';
import 'package:focus_flow_app/presentation/focus/bloc/focus_event.dart';
import 'package:focus_flow_app/presentation/focus/bloc/focus_state.dart';

class ManualSessionBottomSheet extends StatefulWidget {
  const ManualSessionBottomSheet({super.key});

  @override
  State<ManualSessionBottomSheet> createState() =>
      _ManualSessionBottomSheetState();
}

class _ManualSessionBottomSheetState extends State<ManualSessionBottomSheet> {
  Category? _selectedCategory;
  Task? _selectedTask;
  DateTime _startDate = DateTime.now().subtract(const Duration(minutes: 25));
  DateTime _endDate = DateTime.now();
  int _focusLevel = 3;
  final TextEditingController _notesController = TextEditingController();

  @override
  void dispose() {
    _notesController.dispose();
    super.dispose();
  }

  Future<void> _pickDateTime(bool isStart) async {
    final initialDate = isStart ? _startDate : _endDate;
    final pickedDate = await showDatePicker(
      context: context,
      initialDate: initialDate,
      firstDate: DateTime(2000),
      lastDate: DateTime.now(),
    );

    if (pickedDate != null && mounted) {
      final pickedTime = await showTimePicker(
        context: context,
        initialTime: TimeOfDay.fromDateTime(initialDate),
      );

      if (pickedTime != null) {
        final newDateTime = DateTime(
          pickedDate.year,
          pickedDate.month,
          pickedDate.day,
          pickedTime.hour,
          pickedTime.minute,
        );

        setState(() {
          if (isStart) {
            _startDate = newDateTime;
            // Ensure end date is after start date
            if (_endDate.isBefore(_startDate)) {
              _endDate = _startDate.add(const Duration(minutes: 25));
            }
          } else {
            _endDate = newDateTime;
          }
        });
      }
    }
  }

  void _onSave() {
    if (_endDate.isBefore(_startDate)) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('manual_session.error_end_time'.tr())),
      );
      return;
    }

    if (_selectedCategory == null) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('manual_session.error_category'.tr())),
      );
      return;
    }

    context.read<FocusBloc>().add(
      AddManualSession(
        category: _selectedCategory,
        task: _selectedTask,
        startTime: _startDate,
        endTime: _endDate,
        focusLevel: _focusLevel,
        note: _notesController.text,
      ),
    );
    Navigator.of(context).pop();
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<FocusBloc, FocusState>(
      builder: (context, state) {
        final categories = state.categories;

        return Padding(
          padding: EdgeInsets.only(
            bottom: MediaQuery.of(context).viewInsets.bottom,
            left: 16,
            right: 16,
            top: 16,
          ),
          child: SingleChildScrollView(
            child: Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                Text(
                  'manual_session.title'.tr(),
                  style: Theme.of(context).textTheme.headlineSmall,
                ),
                const SizedBox(height: 16),

                // Category Selector
                DropdownButtonFormField<Category>(
                  key: ValueKey(_selectedCategory),
                  initialValue: _selectedCategory,
                  decoration: InputDecoration(
                    labelText: 'manual_session.category'.tr(),
                  ),
                  items:
                      categories.map((catWithTask) {
                        return DropdownMenuItem(
                          value: catWithTask.category,
                          child: Text(catWithTask.category.name),
                        );
                      }).toList(),
                  onChanged: (value) {
                    setState(() {
                      _selectedCategory = value;
                      _selectedTask = null; // Reset task
                    });
                  },
                ),
                const SizedBox(height: 16),

                // Task Selector (Optional)
                if (_selectedCategory != null)
                  DropdownButtonFormField<Task>(
                    key: ValueKey('task_${_selectedCategory?.id}_$_selectedTask'),
                    initialValue: _selectedTask,
                    decoration: InputDecoration(
                      labelText: 'manual_session.task'.tr(),
                    ),
                    items:
                        categories
                            .firstWhere(
                              (c) => c.category.id == _selectedCategory!.id,
                            )
                            .tasks
                            .map((task) {
                              return DropdownMenuItem(
                                value: task,
                                child: Text(task.name),
                              );
                            })
                            .toList(),
                    onChanged: (value) {
                      setState(() {
                        _selectedTask = value;
                      });
                    },
                  ),
                if (_selectedCategory != null) const SizedBox(height: 16),

                // Date Time Pickers
                Row(
                  children: [
                    Expanded(
                      child: InkWell(
                        onTap: () => _pickDateTime(true),
                        child: InputDecorator(
                          decoration: InputDecoration(
                            labelText: 'manual_session.start_time'.tr(),
                          ),
                          child: Text(
                            DateFormat('MM/dd HH:mm').format(_startDate),
                          ),
                        ),
                      ),
                    ),
                    const SizedBox(width: 16),
                    Expanded(
                      child: InkWell(
                        onTap: () => _pickDateTime(false),
                        child: InputDecorator(
                          decoration: InputDecoration(
                            labelText: 'manual_session.end_time'.tr(),
                          ),
                          child: Text(
                            DateFormat('MM/dd HH:mm').format(_endDate),
                          ),
                        ),
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 16),

                // Focus Level
                Text('manual_session.focus_level'.tr()),
                Slider(
                  value: _focusLevel.toDouble(),
                  min: 1,
                  max: 5,
                  divisions: 4,
                  label: _focusLevel.toString(),
                  onChanged: (value) {
                    setState(() {
                      _focusLevel = value.toInt();
                    });
                  },
                ),
                const SizedBox(height: 16),

                // Notes
                TextField(
                  controller: _notesController,
                  decoration: InputDecoration(
                    labelText: 'manual_session.notes'.tr(),
                    alignLabelWithHint: true,
                  ),
                  maxLines: 3,
                ),
                const SizedBox(height: 24),

                // Actions
                Row(
                  mainAxisAlignment: MainAxisAlignment.end,
                  children: [
                    TextButton(
                      onPressed: () => Navigator.of(context).pop(),
                      child: Text('common.cancel'.tr()),
                    ),
                    const SizedBox(width: 16),
                    FilledButton(
                      onPressed: _onSave,
                      child: Text('common.save'.tr()),
                    ),
                  ],
                ),
                const SizedBox(height: 24),
              ],
            ),
          ),
        );
      },
    );
  }
}
