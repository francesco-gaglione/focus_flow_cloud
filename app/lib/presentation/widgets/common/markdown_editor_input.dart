import 'package:flutter/material.dart';
import 'package:flutter_markdown_plus/flutter_markdown_plus.dart';

class MarkdownEditorInput extends StatefulWidget {
  final TextEditingController controller;
  final String? label;
  final String? hint;
  final bool hideFullScreenButton;
  final Widget Function(BuildContext)? fullScreenOverlayBuilder;

  const MarkdownEditorInput({
    super.key,
    required this.controller,
    this.label,
    this.hint,
    this.hideFullScreenButton = false,
    this.fullScreenOverlayBuilder,
  });

  @override
  State<MarkdownEditorInput> createState() => _MarkdownEditorInputState();
}

class _MarkdownEditorInputState extends State<MarkdownEditorInput> {
  bool _isPreview = false;

  void _insertText(String text, {int selectionOffset = 0}) {
    final selection = widget.controller.selection;
    if (selection.start < 0) {
      // If no selection, append to end
      final newText = widget.controller.text + text;
      widget.controller.value = TextEditingValue(
        text: newText,
        selection: TextSelection.collapsed(
          offset: newText.length + selectionOffset,
        ),
      );
      return;
    }

    final newText = widget.controller.text.replaceRange(
      selection.start,
      selection.end,
      text,
    );
    final newSelectionIndex = selection.start + text.length + selectionOffset;

    widget.controller.value = TextEditingValue(
      text: newText,
      selection: TextSelection.collapsed(offset: newSelectionIndex),
    );
  }

  void _wrapSelection(String left, String right) {
    final selection = widget.controller.selection;
    if (selection.start < 0) {
      _insertText('$left$right', selectionOffset: -right.length);
      return;
    }

    final selectedText = widget.controller.text.substring(
      selection.start,
      selection.end,
    );
    final newText = '$left$selectedText$right';

    widget.controller.value = widget.controller.value.replaced(
      selection,
      newText,
    );
  }

  void _openFullScreen(BuildContext context) {
    Navigator.of(context).push(
      MaterialPageRoute(
        // fullscreenDialog: true,
        builder:
            (context) => Scaffold(
              appBar: AppBar(
                title: Text(widget.label ?? 'Editor'),
                actions: [
                  IconButton(
                    icon: const Icon(Icons.check),
                    onPressed: () => Navigator.of(context).pop(),
                  ),
                ],
              ),
              body: SafeArea(
                child: MarkdownEditorInput(
                  controller: widget.controller,
                  hint: widget.hint,
                  // Avoid recursive full screen button in the full screen mode
                  hideFullScreenButton: true,
                  fullScreenOverlayBuilder: widget.fullScreenOverlayBuilder,
                ),
              ),
              floatingActionButtonLocation:
                  FloatingActionButtonLocation.centerFloat,
              floatingActionButton:
                  widget.fullScreenOverlayBuilder != null
                      ? widget.fullScreenOverlayBuilder!(context)
                      : null,
            ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        if (widget.label != null) ...[
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Text(
                widget.label!,
                style: Theme.of(context).textTheme.titleSmall,
              ),
              Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  if (!widget.hideFullScreenButton)
                    IconButton(
                      icon: const Icon(Icons.open_in_full),
                      tooltip: 'Full Screen',
                      onPressed: () => _openFullScreen(context),
                      visualDensity: VisualDensity.compact,
                    ),
                  IconButton(
                    icon: Icon(_isPreview ? Icons.edit : Icons.visibility),
                    tooltip: _isPreview ? 'Edit' : 'Preview',
                    onPressed: () => setState(() => _isPreview = !_isPreview),
                    visualDensity: VisualDensity.compact,
                  ),
                ],
              ),
            ],
          ),
          const SizedBox(height: 4),
        ] else ...[
          Row(
            mainAxisAlignment: MainAxisAlignment.end,
            children: [
              if (!widget.hideFullScreenButton)
                IconButton(
                  icon: const Icon(Icons.open_in_full),
                  tooltip: 'Full Screen',
                  onPressed: () => _openFullScreen(context),
                  visualDensity: VisualDensity.compact,
                ),
              IconButton(
                icon: Icon(_isPreview ? Icons.edit : Icons.visibility),
                tooltip: _isPreview ? 'Edit' : 'Preview',
                onPressed: () => setState(() => _isPreview = !_isPreview),
                visualDensity: VisualDensity.compact,
              ),
            ],
          ),
        ],

        Expanded(
          flex: widget.hideFullScreenButton ? 1 : 0,
          child: Container(
            decoration: BoxDecoration(
              color: colorScheme.surfaceContainerHighest.withAlpha(50),
              borderRadius: BorderRadius.circular(12),
            ),
            clipBehavior: Clip.antiAlias,
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                // Toolbar (only visible in Edit mode)
                if (!_isPreview)
                  Container(
                    decoration: BoxDecoration(
                      color: colorScheme.surfaceContainerHighest.withAlpha(100),
                      border: Border(
                        bottom: BorderSide(color: colorScheme.outlineVariant),
                      ),
                    ),
                    height: 48,
                    child: ListView(
                      scrollDirection: Axis.horizontal,
                      padding: const EdgeInsets.symmetric(horizontal: 8),
                      children: [
                        _ToolbarButton(
                          icon: Icons.format_bold,
                          tooltip: 'Bold',
                          onPressed: () => _wrapSelection('**', '**'),
                        ),
                        _ToolbarButton(
                          icon: Icons.format_italic,
                          tooltip: 'Italic',
                          onPressed: () => _wrapSelection('*', '*'),
                        ),
                        _ToolbarButton(
                          icon: Icons.strikethrough_s,
                          tooltip: 'Strikethrough',
                          onPressed: () => _wrapSelection('~~', '~~'),
                        ),
                        const VerticalDivider(
                          indent: 8,
                          endIndent: 8,
                          width: 16,
                        ),
                        _ToolbarButton(
                          icon: Icons.list,
                          tooltip: 'Bullet List',
                          onPressed: () => _insertText('\n- '),
                        ),
                        _ToolbarButton(
                          icon: Icons.check_box_outlined,
                          tooltip: 'Task List',
                          onPressed: () => _insertText('\n- [ ] '),
                        ),
                        const VerticalDivider(
                          indent: 8,
                          endIndent: 8,
                          width: 16,
                        ),
                        _ToolbarButton(
                          icon: Icons.title,
                          tooltip: 'Heading 1',
                          onPressed: () => _insertText('\n# '),
                        ),
                        _ToolbarButton(
                          icon: Icons.code,
                          tooltip: 'Code',
                          onPressed: () => _wrapSelection('`', '`'),
                        ),
                        _ToolbarButton(
                          icon: Icons.data_object,
                          tooltip: 'Code Block',
                          onPressed: () => _wrapSelection('```\n', '\n```'),
                        ),
                      ],
                    ),
                  ),

                // Editor / Preview Area
                Expanded(
                  flex: widget.hideFullScreenButton ? 1 : 0,
                  child: AnimatedSwitcher(
                    duration: const Duration(milliseconds: 200),
                    child:
                        _isPreview
                            ? Container(
                              width: double.infinity,
                              constraints: const BoxConstraints(minHeight: 150),
                              padding: const EdgeInsets.all(16),
                              color: colorScheme.surface,
                              alignment: Alignment.topLeft,
                              child: MarkdownBody(
                                data:
                                    widget.controller.text.isEmpty
                                        ? '*${widget.hint ?? "No content"}*'
                                        : widget.controller.text,
                                styleSheet: MarkdownStyleSheet.fromTheme(
                                  Theme.of(context),
                                ).copyWith(
                                  blockquote: Theme.of(
                                    context,
                                  ).textTheme.bodyMedium?.copyWith(
                                    color: colorScheme.onSurfaceVariant,
                                    fontStyle: FontStyle.italic,
                                  ),
                                ),
                              ),
                            )
                            : TextField(
                              controller: widget.controller,
                              maxLines: null,
                              expands:
                                  widget
                                      .hideFullScreenButton, // Expand in full screen mode
                              minLines: widget.hideFullScreenButton ? null : 5,
                              keyboardType: TextInputType.multiline,
                              decoration: InputDecoration(
                                hintText: widget.hint,
                                border: InputBorder.none,
                                contentPadding: const EdgeInsets.all(16),
                                filled: true,
                                fillColor: colorScheme.surface,
                              ),
                            ),
                  ),
                ),
              ],
            ),
          ),
        ),
      ],
    );
  }
}

class _ToolbarButton extends StatelessWidget {
  final IconData icon;
  final String tooltip;
  final VoidCallback onPressed;

  const _ToolbarButton({
    required this.icon,
    required this.tooltip,
    required this.onPressed,
  });

  @override
  Widget build(BuildContext context) {
    return IconButton(
      icon: Icon(icon, size: 20),
      tooltip: tooltip,
      onPressed: onPressed,
      style: IconButton.styleFrom(
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
        padding: EdgeInsets.zero,
        tapTargetSize: MaterialTapTargetSize.shrinkWrap,
      ),
    );
  }
}
