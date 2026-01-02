import 'package:easy_localization/easy_localization.dart';
import 'package:flutter/material.dart';
import 'package:focus_flow_app/presentation/widgets/common/custom_text_field.dart';

class TaskDialog extends StatefulWidget {
  final String? initialName;
  final String? initialDescription;
  final Function(String name, String? description) onSubmit;

  const TaskDialog({
    super.key,
    this.initialName,
    this.initialDescription,
    required this.onSubmit,
  });

  @override
  State<TaskDialog> createState() => _TaskDialogState();
}

class _TaskDialogState extends State<TaskDialog> {
  late TextEditingController nameController;
  late TextEditingController descController;

  static const Color _orphanTaskColor = Color(0xFFFFA726);

  @override
  void initState() {
    super.initState();
    nameController = TextEditingController(text: widget.initialName);
    descController = TextEditingController(text: widget.initialDescription);
  }

  @override
  void dispose() {
    nameController.dispose();
    descController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final bool isEditMode = widget.initialName != null;

    return AlertDialog(
      insetPadding: const EdgeInsets.symmetric(horizontal: 24),
      icon: Icon(
        isEditMode ? Icons.edit_note : Icons.add_task,
        color: _orphanTaskColor,
      ),
      title: Text(isEditMode ? context.tr('task.edit_orphan_title') : context.tr('task.create_orphan_title')),
      content: SizedBox(
        width: 400,
        child: SingleChildScrollView(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              CustomTextField(
                controller: nameController,
                label: context.tr('task.name_label'),
                icon: Icons.task_outlined,
                hint: context.tr('task.name_hint'),
                textCapitalization: TextCapitalization.sentences,
                autofocus: true,
              ),
              const SizedBox(height: 16),
              CustomTextField(
                controller: descController,
                label: context.tr('task.description_label'),
                icon: Icons.notes_outlined,
                hint: context.tr('task.description_hint'),
                maxLines: 4,
                minLines: 3,
                textCapitalization: TextCapitalization.sentences,
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
          style: FilledButton.styleFrom(backgroundColor: _orphanTaskColor),
          onPressed: () {
            if (nameController.text.trim().isEmpty) {
              ScaffoldMessenger.of(context).showSnackBar(
                SnackBar(
                  content: Text(context.tr('task.name_required')),
                  behavior: SnackBarBehavior.floating,
                ),
              );
              return;
            }
            widget.onSubmit(
              nameController.text.trim(),
              descController.text.trim().isEmpty
                  ? null
                  : descController.text.trim(),
            );
            Navigator.pop(context);
          },
          child: Text(isEditMode ? context.tr('common.update') : context.tr('common.create')),
        ),
      ],
    );
  }
}
