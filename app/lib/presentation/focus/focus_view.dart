import 'package:easy_localization/easy_localization.dart' hide TextDirection;
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:focus_flow_app/domain/entities/theme_settings.dart';
import 'package:focus_flow_app/presentation/app/theme_cubit.dart';
import 'package:focus_flow_app/presentation/focus/bloc/focus_bloc.dart';
import 'package:focus_flow_app/presentation/focus/bloc/focus_event.dart';
import 'package:focus_flow_app/presentation/focus/bloc/focus_state.dart';
import 'package:focus_flow_app/presentation/widgets/focus/category_task_selector.dart';
import 'package:focus_flow_app/presentation/widgets/focus/focus_level_selector.dart';
import 'package:focus_flow_app/presentation/widgets/focus/focus_notes.dart';
import 'package:focus_flow_app/presentation/widgets/focus/focus_timeline.dart';
import 'package:focus_flow_app/presentation/widgets/focus/focus_timer.dart';
import 'package:focus_flow_app/presentation/widgets/focus/manual_session_bottom_sheet.dart';
import 'package:focus_flow_app/adapters/dtos/ws_dtos.dart';
import 'package:logger/logger.dart';

class FocusView extends StatefulWidget {
  const FocusView({super.key});

  @override
  FocusViewState createState() => FocusViewState();
}

class FocusViewState extends State<FocusView> with WidgetsBindingObserver {
  final Logger logger = Logger();

  static const double desktopBreakpoint = 900;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addObserver(this);
    // Check initial state for active session color
    WidgetsBinding.instance.addPostFrameCallback((_) {
      final state = context.read<FocusBloc>().state;
      context.read<FocusBloc>().add(ReloadCategoriesAndTasks());
      if (state.selectedCategory != null) {
        try {
          int colorInt = int.parse(
            state.selectedCategory!.color.replaceFirst('#', '0xFF'),
          );
          context.read<ThemeCubit>().updateAccentColor(colorInt);
        } catch (e) {
          logger.e('Error parsing initial category color', error: e);
        }
      }
    });
  }

  @override
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);
    super.dispose();
  }

  @override
  void didChangeAppLifecycleState(AppLifecycleState state) {
    if (state == AppLifecycleState.resumed) {
      logger.d('App resumed, checking WebSocket connection');
      context.read<FocusBloc>().add(CheckConnection());
      context.read<FocusBloc>().add(ReloadCategoriesAndTasks());
    }
  }

  void _onFocusLevelChanged(BuildContext context, int level) {
    logger.d('Focus level changed to $level');
    context.read<FocusBloc>().add(FocusLevelSelected(focusLevel: level));
  }

  void _onCategoryChanged(BuildContext context, category) {
    logger.d('Category changed to $category');
    context.read<FocusBloc>().add(CategorySelected(category: category));
  }

  void _onTaskChanged(BuildContext context, task) {
    context.read<FocusBloc>().add(TaskSelected(task: task));
  }

  void _onNoteChanged(BuildContext context, String note) {
    context.read<FocusBloc>().add(UpdateNote(note: note));
  }

  void _onStart() {
    logger.d('Starting timer');
    context.read<FocusBloc>().add(StartFocus());
  }

  void _onBreak() {
    context.read<FocusBloc>().add(BreakFocus());
  }

  void _onTerminate() {
    context.read<FocusBloc>().add(TerminateFocus());
  }

  void _refreshSessions() {
    logger.d('Refreshing sessions');
    context.read<FocusBloc>().add(ReloadTodaySessions());
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(context.tr('focus.title')),
        centerTitle: false,
        actions: [
          BlocBuilder<FocusBloc, FocusState>(
            buildWhen: (previous, current) =>
                previous.isWebSocketConnected != current.isWebSocketConnected,
            builder: (context, state) {
              return Padding(
                padding: const EdgeInsets.only(right: 16.0),
                child: Tooltip(
                  message:
                      state.isWebSocketConnected
                          ? 'WebSocket Connected'
                          : 'WebSocket Disconnected',
                  child: Icon(
                    state.isWebSocketConnected ? Icons.wifi : Icons.wifi_off,
                    color:
                        state.isWebSocketConnected ? Colors.green : Colors.red,
                    size: 20,
                  ),
                ),
              );
            },
          ),
        ],
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          final focusBloc = context.read<FocusBloc>();
          showModalBottomSheet(
            context: context,
            isScrollControlled: true,
            builder:
                (context) => BlocProvider.value(
                  value: focusBloc,
                  child: const ManualSessionBottomSheet(),
                ),
          );
        },
        child: const Icon(Icons.add),
      ),
      body: Container(
        decoration: BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [
              Theme.of(context).colorScheme.surface,
              Theme.of(context).colorScheme.surfaceContainerLowest,
              Theme.of(context).colorScheme.surfaceContainerLow,
            ],
            stops: const [0.0, 0.5, 1.0],
          ),
        ),
        child: BlocConsumer<FocusBloc, FocusState>(
          listener: (context, state) {
            if (state.selectedCategory != null) {
              try {
                int colorInt = int.parse(
                  state.selectedCategory!.color.replaceFirst('#', '0xFF'),
                );
                context.read<ThemeCubit>().updateAccentColor(colorInt);
              } catch (e) {
                // Handle error
              }
            } else {
              context.read<ThemeCubit>().updateAccentColor(
                ThemeSettings.defaultAccentColor,
              );
            }
          },
          builder: (context, state) {
            logger.d(
              'Building FocusView with sessionState: ${state.sessionState}',
            );
            if (state.isLoading) {
              return const Center(child: CircularProgressIndicator());
            }

            // Enforce color consistency
            if (state.selectedCategory != null) {
              try {
                final targetColor = int.parse(
                  state.selectedCategory!.color.replaceFirst('#', '0xFF'),
                );
                final currentColor = Theme.of(context).colorScheme.primary.toARGB32();
                
                if (currentColor != targetColor) {
                  WidgetsBinding.instance.addPostFrameCallback((_) {
                    if (context.mounted) {
                      context.read<ThemeCubit>().updateAccentColor(targetColor);
                    }
                  });
                }
              } catch (e) {
                // Ignore parsing errors
              }
            }

            final isDesktop =
                MediaQuery.of(context).size.width >= desktopBreakpoint;

            final isWorkSession =
                state.sessionState?.sessionType == SessionTypeEnum.focus ||
                    state.sessionState?.sessionType == SessionTypeEnum.work;

            final focusLevelSelector =
                state.sessionState != null && isWorkSession
                    ? FocusLevelSelector(
                      initialLevel: state.sessionState?.selectedFocusLevel,
                      onFocusLevelChanged:
                          (level) => _onFocusLevelChanged(context, level),
                    )
                    : SizedBox(height: 0);

            final notesWidget =
                state.sessionState != null && isWorkSession
                    ? FocusNotesWidget(
                      initialNotes: state.sessionState?.note,
                      onNotesChanged: (notes) => _onNoteChanged(context, notes),
                      templates: state.noteTemplates,
                    )
                    : SizedBox(height: 0);

            final isSelectionEnabled =
                state.sessionState == null ||
                (state.sessionState!.sessionType != SessionTypeEnum.focus &&
                    state.sessionState!.sessionType != SessionTypeEnum.work);

            final categoryTaskSelector = CategoryTaskSelector(
              categories: state.categories,
              orphanTasks: state.orphanTasks,
              initialCategoryId: state.selectedCategory?.id,
              initialTaskId: state.selectedTask?.id,
              onCategoryChanged: (cat) => _onCategoryChanged(context, cat),
              onTaskChanged: (task) => _onTaskChanged(context, task),
              enabled: isSelectionEnabled,
            );

            final timelineWidget = FocusTimelineWidget(
              sessions: state.todaySessions,
              categories: state.categories,
              orphanTasks: state.orphanTasks,
              onSessionUpdated: _refreshSessions,
            );

            final focusTimerWidget = FocusTimerWidget(
              startDate:
                  state.sessionState != null
                      ? DateTime.fromMillisecondsSinceEpoch(
                        state.sessionState!.startDate * 1000,
                      )
                      : null,
              sessionType: state.sessionState?.sessionType,
              onStart: _onStart,
              onBreak: _onBreak,
              onTerminate: _onTerminate,
            );

            if (isDesktop) {
              return Row(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Expanded(
                    flex: 4,
                    child: SingleChildScrollView(
                      padding: const EdgeInsets.all(32),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.stretch,
                        children: [
                          focusTimerWidget,
                          const SizedBox(height: 32),
                          focusLevelSelector,
                          const SizedBox(height: 32),
                          notesWidget,
                        ],
                      ),
                    ),
                  ),
                  Expanded(
                    flex: 5,
                    child: SingleChildScrollView(
                      padding: const EdgeInsets.all(32),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.stretch,
                        children: [
                          categoryTaskSelector,
                          const SizedBox(height: 32),
                          timelineWidget,
                        ],
                      ),
                    ),
                  ),
                ],
              );
            } else {
              return SingleChildScrollView(
                padding: const EdgeInsets.all(20),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    focusTimerWidget,
                    const SizedBox(height: 24),
                    categoryTaskSelector,
                    const SizedBox(height: 24),
                    focusLevelSelector,
                    const SizedBox(height: 24),
                    notesWidget,
                    const SizedBox(height: 24),
                    timelineWidget,
                    const SizedBox(height: 80), // Bottom padding for FAB or navigation
                  ],
                ),
              );
            }
          },
        ),
      ),
    );
  }
}
