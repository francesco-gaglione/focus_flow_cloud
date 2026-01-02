import 'package:easy_localization/easy_localization.dart' hide TextDirection;
import 'package:flutter/material.dart';
import '../common/markdown_editor_input.dart';

import 'package:focus_flow_app/domain/entities/note_template.dart';

class FocusNotesWidget extends StatefulWidget {
  final String? initialNotes;
  final ValueChanged<String>? onNotesChanged;
  final List<NoteTemplate> templates;

  const FocusNotesWidget({
    super.key,
    this.initialNotes,
    this.onNotesChanged,
    this.templates = const [],
  });

  @override
  State<FocusNotesWidget> createState() => _FocusNotesWidgetState();
}

class _FocusNotesWidgetState extends State<FocusNotesWidget> {
  late TextEditingController _notesController;

  @override
  void initState() {
    super.initState();
    _notesController = TextEditingController(text: widget.initialNotes ?? '');
    _notesController.addListener(_onTextChanged);
  }

  @override
  void didUpdateWidget(covariant FocusNotesWidget oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (widget.initialNotes != oldWidget.initialNotes &&
        widget.initialNotes != _notesController.text) {
      _notesController.text = widget.initialNotes ?? '';
    }
  }

  void _onTextChanged() {
    if (widget.onNotesChanged != null) {
      widget.onNotesChanged!(_notesController.text);
    }
  }

  @override
  void dispose() {
    _notesController.removeListener(_onTextChanged);
    _notesController.dispose();
    super.dispose();
  }

  void _showTemplateSelector() {
    showModalBottomSheet(
      context: context,
      builder:
          (context) => Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Padding(
                padding: const EdgeInsets.all(16),
                child: Text(
                  context.tr('focus.select_template'),
                  style: Theme.of(context).textTheme.titleLarge,
                ),
              ),
              ...widget.templates.map(
                (t) => ListTile(
                  title: Text(t.name),
                  subtitle: Text(
                    t.content,
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                  ),
                  onTap: () {
                    if (_notesController.text.isNotEmpty) {
                       // Ask confirmation if overwriting? Or just append? 
                       // For simplicity let's replace but maybe we could confirm.
                       // Let's just replace for now as per "template" concept usually implies starting point.
                       // Or we could show a dialog: "Replace or Append?"
                       // Let's keep it simple: Replace.
                    }
                    _notesController.text = t.content;
                    Navigator.pop(context);
                  },
                ),
              ),
              const SizedBox(height: 16),
            ],
          ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;

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
                  child: Icon(Icons.edit_note, color: colorScheme.primary),
                ),
                const SizedBox(width: 12),
                Text(
                  context.tr('focus.notes_title'),
                  style: Theme.of(context).textTheme.titleMedium?.copyWith(
                    fontWeight: FontWeight.bold,
                    letterSpacing: 0.5,
                  ),
                ),
                const Spacer(),
                if (widget.templates.isNotEmpty)
                  IconButton(
                    icon: const Icon(Icons.playlist_add),
                    tooltip: context.tr('focus.load_template'),
                    onPressed: _showTemplateSelector,
                  ),
              ],
            ),
            const SizedBox(height: 16),
            MarkdownEditorInput(
              controller: _notesController,
              hint: context.tr('focus.notes_hint'),
            ),
          ],
        ),
      ),
    );
  }
}
