import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import 'package:flutter_markdown_plus/flutter_markdown_plus.dart';
import 'package:easy_localization/easy_localization.dart';
import 'package:focus_flow_app/domain/entities/focus_session.dart';
import 'package:focus_flow_app/presentation/notes/bloc/notes_bloc.dart';
import 'package:focus_flow_app/presentation/notes/bloc/notes_event.dart';
import 'package:focus_flow_app/presentation/notes/bloc/notes_state.dart';
import 'package:focus_flow_app/presentation/widgets/common/markdown_editor_input.dart';
import 'package:focus_flow_app/domain/entities/category.dart';
import 'package:focus_flow_app/domain/entities/category_with_tasks.dart';
import 'package:focus_flow_app/domain/entities/task.dart';
import 'package:focus_flow_app/presentation/widgets/focus/minimalist_timer_header.dart';

class NotesView extends StatelessWidget {
  const NotesView({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('notes.title'.tr())),
      body: BlocConsumer<NotesBloc, NotesState>(
        listener: (context, state) {
          if (state.errorMessage != null) {
            ScaffoldMessenger.of(
              context,
            ).showSnackBar(SnackBar(content: Text(state.errorMessage!)));
          }
        },
        builder: (context, state) {
          if (state.status == NotesStatus.loading) {
            return const Center(child: CircularProgressIndicator());
          }

          return CustomScrollView(
            slivers: [
              SliverToBoxAdapter(
                child: Padding(
                  padding: const EdgeInsets.symmetric(
                    horizontal: 16,
                    vertical: 8,
                  ),
                  child: SingleChildScrollView(
                    scrollDirection: Axis.horizontal,
                    child: Row(
                      children: [
                        _DateFilterChip(),
                        const SizedBox(width: 8),
                        _CategoryFilterChip(),
                        const SizedBox(width: 8),
                        _TaskFilterChip(),
                        const SizedBox(width: 8),
                        _ClearFiltersButton(),
                      ],
                    ),
                  ),
                ),
              ),
              if (state.sessions.isEmpty)
                SliverFillRemaining(
                  child: Center(
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Icon(
                          Icons.note_alt_outlined,
                          size: 64,
                          color: Theme.of(context).colorScheme.outline,
                        ),
                        const SizedBox(height: 16),
                        Text(
                          'notes.no_notes_found'.tr(),
                          style: Theme.of(
                            context,
                          ).textTheme.bodyLarge?.copyWith(
                            color:
                                Theme.of(context).colorScheme.onSurfaceVariant,
                          ),
                        ),
                      ],
                    ),
                  ),
                )
              else
                SliverPadding(
                  padding: const EdgeInsets.all(16),
                  sliver: SliverList(
                    delegate: SliverChildBuilderDelegate((context, index) {
                      final session = state.sessions[index];
                      return Padding(
                        padding: const EdgeInsets.only(bottom: 12),
                        child: _NoteCard(session: session),
                      );
                    }, childCount: state.sessions.length),
                  ),
                ),
            ],
          );
        },
      ),
    );
  }
}

class _DateFilterChip extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final state = context.watch<NotesBloc>().state;
    final isFiltered = state.startDate != null;
    final label =
        isFiltered
            ? '${DateFormat('MMM d', context.locale.toString()).format(state.startDate!)} - ${DateFormat('MMM d', context.locale.toString()).format(state.endDate!)}'
            : 'notes.date_range'.tr();

    return FilterChip(
      label: Text(label),
      selected: isFiltered,
      onSelected: (_) async {
        final result = await showDateRangePicker(
          context: context,
          firstDate: DateTime(2020),
          lastDate: DateTime.now(),
          initialDateRange:
              state.startDate != null && state.endDate != null
                  ? DateTimeRange(start: state.startDate!, end: state.endDate!)
                  : null,
        );

        if (context.mounted && result != null) {
          context.read<NotesBloc>().add(
            NotesDateRangeChanged(result.start, result.end),
          );
        }
      },
      avatar:
          isFiltered
              ? const Icon(Icons.close, size: 18)
              : const Icon(Icons.date_range, size: 18),
      showCheckmark: false,
    );
  }
}

class _CategoryFilterChip extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final state = context.watch<NotesBloc>().state;
    final selectedId = state.selectedCategoryId;
    final selectedCategoryWithTasks = state.categories
        .cast<CategoryWithTasks?>()
        .firstWhere((c) => c?.category.id == selectedId, orElse: () => null);

    final isFiltered = selectedId != null;
    final label =
        isFiltered
            ? selectedCategoryWithTasks?.category.name ?? 'Unknown'
            : 'common.category'.tr();

    return PopupMenuButton<String?>(
      tooltip: 'notes.filter_by_category'.tr(),
      initialValue: selectedId,
      itemBuilder: (context) {
        return state.categories
            .map(
              (c) => PopupMenuItem(
                value: c.category.id,
                child: Text(c.category.name),
              ),
            )
            .toList();
      },
      onSelected: (value) {
        context.read<NotesBloc>().add(NotesCategoryChanged(value));
      },
      child: FilterChip(
        label: Text(label),
        selected: isFiltered,
        onSelected: null, // Handled by PopupMenuButton
        avatar:
            isFiltered
                ? const Icon(Icons.close, size: 18)
                : const Icon(Icons.category_outlined, size: 18),
        showCheckmark: false,
      ),
    );
  }
}

class _TaskFilterChip extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final state = context.watch<NotesBloc>().state;
    final selectedTaskId = state.selectedTaskId;
    final selectedCategoryId = state.selectedCategoryId;

    // Get available tasks based on selected category
    final List<Task> availableTasks;
    if (selectedCategoryId != null) {
      final categoryWithTasks = state.categories.firstWhere(
        (c) => c.category.id == selectedCategoryId,
        orElse:
            () => CategoryWithTasks(
              category: Category(
                id: '',
                name: '',
                color: '',
                createdAt: DateTime.fromMillisecondsSinceEpoch(0),
                updatedAt: DateTime.fromMillisecondsSinceEpoch(0),
              ), // Should ideally handle gracefully
              tasks: [],
            ),
      );
      availableTasks = categoryWithTasks.tasks;
    } else {
      availableTasks = state.categories.expand((c) => c.tasks).toList();
    }

    final selectedTask = availableTasks.cast<Task?>().firstWhere(
      (t) => t?.id == selectedTaskId,
      orElse: () => null,
    );

    final isFiltered = selectedTaskId != null;
    final label =
        isFiltered ? selectedTask?.name ?? 'Unknown' : 'common.task'.tr();

    // Disable chip if no tasks available
    if (availableTasks.isEmpty) return const SizedBox.shrink();

    return PopupMenuButton<String?>(
      tooltip: 'notes.filter_by_task'.tr(),
      initialValue: selectedTaskId,
      itemBuilder: (context) {
        return availableTasks
            .map(
              (task) => PopupMenuItem(value: task.id, child: Text(task.name)),
            )
            .toList();
      },
      onSelected: (value) {
        context.read<NotesBloc>().add(NotesTaskChanged(value));
      },
      child: FilterChip(
        label: Text(label),
        selected: isFiltered,
        onSelected: null, // Handled by PopupMenuButton
        avatar:
            isFiltered
                ? const Icon(Icons.close, size: 18)
                : const Icon(Icons.check_circle_outline, size: 18),
        showCheckmark: false,
      ),
    );
  }
}

class _NoteCard extends StatelessWidget {
  final FocusSession session;

  const _NoteCard({required this.session});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final categoryName = _getCategoryName(context, session.categoryId);

    return Card(
      elevation: 2,
      shadowColor: Colors.black12,
      surfaceTintColor: theme.colorScheme.surface,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(16),
        side: BorderSide(color: theme.colorScheme.outlineVariant.withAlpha(80)),
      ),
      child: InkWell(
        borderRadius: BorderRadius.circular(16),
        onTap: () => _openDetails(context, session),
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Expanded(
                    child: Wrap(
                      spacing: 8,
                      runSpacing: 8,
                      children: [
                        Container(
                          padding: const EdgeInsets.symmetric(
                            horizontal: 8,
                            vertical: 4,
                          ),
                          decoration: BoxDecoration(
                            color: theme.colorScheme.primaryContainer.withAlpha(
                              100,
                            ),
                            borderRadius: BorderRadius.circular(8),
                          ),
                          child: Row(
                            mainAxisSize: MainAxisSize.min,
                            children: [
                              Icon(
                                _getSessionTypeIcon(session.sessionType),
                                size: 14,
                                color: theme.colorScheme.onPrimaryContainer,
                              ),
                              const SizedBox(width: 4),
                              Text(
                                session.sessionType.value,
                                style: theme.textTheme.labelSmall?.copyWith(
                                  color: theme.colorScheme.onPrimaryContainer,
                                  fontWeight: FontWeight.bold,
                                ),
                              ),
                            ],
                          ),
                        ),
                        if (categoryName != null)
                          Container(
                            padding: const EdgeInsets.symmetric(
                              horizontal: 8,
                              vertical: 4,
                            ),
                            decoration: BoxDecoration(
                              color: theme.colorScheme.tertiaryContainer
                                  .withAlpha(100),
                              borderRadius: BorderRadius.circular(8),
                            ),
                            child: Text(
                              categoryName,
                              style: theme.textTheme.labelSmall?.copyWith(
                                color: theme.colorScheme.onTertiaryContainer,
                              ),
                            ),
                          ),
                        if (session.taskId != null)
                          Builder(
                            builder: (context) {
                              final taskName = _getTaskName(
                                context,
                                session.taskId,
                              );
                              if (taskName == null) {
                                return const SizedBox.shrink();
                              }

                              return Container(
                                padding: const EdgeInsets.symmetric(
                                  horizontal: 8,
                                  vertical: 4,
                                ),
                                decoration: BoxDecoration(
                                  color: theme.colorScheme.secondaryContainer
                                      .withAlpha(100),
                                  borderRadius: BorderRadius.circular(8),
                                ),
                                child: Row(
                                  mainAxisSize: MainAxisSize.min,
                                  children: [
                                    Icon(
                                      Icons.check_circle_outline,
                                      size: 14,
                                      color:
                                          theme
                                              .colorScheme
                                              .onSecondaryContainer,
                                    ),
                                    const SizedBox(width: 4),
                                    Text(
                                      taskName,
                                      style: theme.textTheme.labelSmall
                                          ?.copyWith(
                                            color:
                                                theme
                                                    .colorScheme
                                                    .onSecondaryContainer,
                                          ),
                                    ),
                                  ],
                                ),
                              );
                            },
                          ),
                      ],
                    ),
                  ),
                  const SizedBox(width: 8),
                  Text(
                    DateFormat(
                      'MMM d, y • HH:mm',
                      context.locale.toString(),
                    ).format(session.createdAt),
                    style: theme.textTheme.labelSmall?.copyWith(
                      color: theme.colorScheme.outline,
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 12),
              Text(
                session.notes ?? '',
                maxLines: 3,
                overflow: TextOverflow.ellipsis,
                style: theme.textTheme.bodyMedium?.copyWith(
                  height: 1.5,
                  color: theme.colorScheme.onSurfaceVariant,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  String? _getCategoryName(BuildContext context, String? categoryId) {
    if (categoryId == null) return null;
    final state = context.read<NotesBloc>().state;
    try {
      return state.categories
          .firstWhere((c) => c.category.id == categoryId)
          .category
          .name;
    } catch (_) {
      return null;
    }
  }

  String? _getTaskName(BuildContext context, String? taskId) {
    if (taskId == null) return null;
    final state = context.read<NotesBloc>().state;
    try {
      return state.categories
          .expand((c) => c.tasks)
          .firstWhere((t) => t.id == taskId)
          .name;
    } catch (_) {
      return null;
    }
  }

  IconData _getSessionTypeIcon(SessionType type) {
    switch (type) {
      case SessionType.work:
        return Icons.work_outline;
      case SessionType.shortBreak:
        return Icons.coffee;
      case SessionType.longBreak:
        return Icons.weekend;
    }
  }

  void _openDetails(BuildContext context, FocusSession session) {
    showDialog(
      context: context,
      builder:
          (_) => _NoteDetailsDialog(
            session: session,
            bloc: context.read<NotesBloc>(),
          ),
    );
  }
}

class _NoteDetailsDialog extends StatefulWidget {
  final FocusSession session;
  final NotesBloc bloc;

  const _NoteDetailsDialog({required this.session, required this.bloc});

  @override
  State<_NoteDetailsDialog> createState() => _NoteDetailsDialogState();
}

class _NoteDetailsDialogState extends State<_NoteDetailsDialog> {
  late TextEditingController _controller;
  bool _isEditing = false;

  @override
  void initState() {
    super.initState();
    _controller = TextEditingController(text: widget.session.notes);
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    String? categoryName;
    try {
      final state = context.read<NotesBloc>().state;
      categoryName =
          state.categories
              .firstWhere((c) => c.category.id == widget.session.categoryId)
              .category
              .name;
    } catch (_) {}

    return Dialog(
      insetPadding: const EdgeInsets.all(16),
      backgroundColor: theme.colorScheme.surface,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(24)),
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 600, maxHeight: 800),
        child: ClipRRect(
          borderRadius: BorderRadius.circular(24),
          child: Scaffold(
            backgroundColor: theme.colorScheme.surface,
            floatingActionButton: FloatingActionButton.extended(
              onPressed: () {
                if (_isEditing) {
                  widget.bloc.add(
                    NotesNoteSaved(widget.session.id, _controller.text),
                  );
                  Navigator.of(context).pop();
                } else {
                  setState(() => _isEditing = true);
                }
              },
              icon: Icon(_isEditing ? Icons.save : Icons.edit),
              label: Text(
                _isEditing ? 'notes.save_changes'.tr() : 'notes.edit_note'.tr(),
              ),
            ),
            body: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                Container(
                  padding: const EdgeInsets.fromLTRB(16, 16, 16, 16),
                  color: theme.colorScheme.surfaceContainerHighest.withAlpha(
                    50,
                  ),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          IconButton(
                            icon: const Icon(Icons.close),
                            onPressed: () => Navigator.of(context).pop(),
                            style: IconButton.styleFrom(
                              backgroundColor: theme.colorScheme.surface,
                            ),
                          ),
                          if (_isEditing)
                            Container(
                              padding: const EdgeInsets.symmetric(
                                horizontal: 12,
                                vertical: 6,
                              ),
                              decoration: BoxDecoration(
                                color: theme.colorScheme.primaryContainer,
                                borderRadius: BorderRadius.circular(20),
                              ),
                              child: Text(
                                'notes.editing'.tr(),
                                style: theme.textTheme.labelMedium?.copyWith(
                                  color: theme.colorScheme.onPrimaryContainer,
                                  fontWeight: FontWeight.bold,
                                ),
                              ),
                            ),
                        ],
                      ),
                      const SizedBox(height: 16),
                      if (!_isEditing) ...[
                        Text(
                          DateFormat.yMMMMEEEEd(
                            context.locale.toString(),
                          ).format(widget.session.createdAt),
                          style: theme.textTheme.headlineSmall?.copyWith(
                            fontWeight: FontWeight.bold,
                            color: theme.colorScheme.onSurface,
                          ),
                        ),
                        const SizedBox(height: 4),
                        Text(
                          '${DateFormat.Hm(context.locale.toString()).format(widget.session.createdAt)} • ${widget.session.sessionType.value}',
                          style: theme.textTheme.bodyMedium?.copyWith(
                            color: theme.colorScheme.onSurfaceVariant,
                          ),
                        ),
                        if (categoryName != null) ...[
                          const SizedBox(height: 12),
                          Container(
                            padding: const EdgeInsets.symmetric(
                              horizontal: 10,
                              vertical: 4,
                            ),
                            decoration: BoxDecoration(
                              color: theme.colorScheme.tertiaryContainer,
                              borderRadius: BorderRadius.circular(8),
                            ),
                            child: Text(
                              categoryName,
                              style: theme.textTheme.labelSmall?.copyWith(
                                color: theme.colorScheme.onTertiaryContainer,
                                fontWeight: FontWeight.bold,
                              ),
                            ),
                          ),
                        ],
                      ],
                    ],
                  ),
                ),
                Expanded(
                  child:
                      _isEditing
                          ? Padding(
                            padding: const EdgeInsets.all(16.0),
                            child: MarkdownEditorInput(
                              controller: _controller,
                              hint: 'notes.write_hint'.tr(),
                              fullScreenOverlayBuilder:
                                  (context) => const MinimalistTimerHeader(),
                            ),
                          )
                          : SingleChildScrollView(
                            padding: const EdgeInsets.all(24),
                            child: MarkdownBody(
                              data:
                                  _controller.text.isEmpty
                                      ? 'notes.no_content'.tr()
                                      : _controller.text,
                              styleSheet: MarkdownStyleSheet.fromTheme(
                                theme,
                              ).copyWith(
                                p: theme.textTheme.bodyLarge?.copyWith(
                                  height: 1.6,
                                ),
                                h1: theme.textTheme.headlineMedium,
                                h2: theme.textTheme.headlineSmall,
                                listBullet: theme.textTheme.bodyLarge,
                              ),
                            ),
                          ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

class _ClearFiltersButton extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    // Only rebuild when filters changed
    final hasFilters = context.select((NotesBloc bloc) {
      final state = bloc.state;
      return state.startDate != null ||
          state.endDate != null ||
          state.selectedCategoryId != null ||
          state.selectedTaskId != null;
    });

    if (!hasFilters) return const SizedBox.shrink();

    return IconButton(
      onPressed: () {
        context.read<NotesBloc>().add(NotesFilterCleared());
      },
      icon: const Icon(Icons.refresh),
      tooltip: 'notes.clear_filters'.tr(),
      style: IconButton.styleFrom(
        backgroundColor: Theme.of(context).colorScheme.surfaceContainerHighest,
        foregroundColor: Theme.of(context).colorScheme.onSurfaceVariant,
      ),
    );
  }
}
