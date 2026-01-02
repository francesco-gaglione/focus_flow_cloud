import 'dart:async';
import 'dart:convert';

import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:focus_flow_app/adapters/ws/ws_repository.dart';
import 'package:focus_flow_app/domain/entities/user_setting.dart';
import 'package:focus_flow_app/domain/entities/note_template.dart';
import 'package:focus_flow_app/domain/entities/category.dart';
import 'package:focus_flow_app/domain/entities/task.dart';
import 'package:focus_flow_app/domain/entities/focus_session.dart';
import 'package:focus_flow_app/domain/repositories/session_repository.dart';
import 'package:focus_flow_app/domain/repositories/user_settings_repository.dart';
import 'package:focus_flow_app/domain/usecases/categories_usecases/get_categories_and_tasks.dart';
import 'package:focus_flow_app/domain/usecases/sessions_usecases/get_sessions_with_filters.dart';
import 'package:focus_flow_app/domain/usecases/tasks_usecases/fetch_orphan_tasks.dart';
import 'package:focus_flow_app/presentation/focus/bloc/focus_event.dart';
import 'package:focus_flow_app/presentation/focus/bloc/focus_state.dart';
import 'package:logger/logger.dart';
import 'package:rxdart/rxdart.dart';

EventTransformer<E> debounce<E>(Duration duration) {
  return (events, mapper) {
    return events.debounceTime(duration).asyncExpand(mapper);
  };
}

class FocusBloc extends Bloc<FocusEvent, FocusState> {
  final Logger logger = Logger();
  final GetCategoriesAndTasks _getCategoriesAndTasks;
  final FetchOrphanTasks _fetchOrphanTasks;
  final WebsocketRepository _websocketRepository;
  final GetSessionsWithFilters _getSessionsWithFilters;
  final SessionRepository _sessionRepository;
  final UserSettingsRepository _userSettingsRepository;
  StreamSubscription? _serverResponsesSubscription;
  StreamSubscription? _broadcastEventsSubscription;
  StreamSubscription? _pomodoroStateUpdatesSubscription;
  StreamSubscription? _connectionStatusSubscription;

  FocusBloc({
    required GetCategoriesAndTasks getCategoriesAndTask,
    required FetchOrphanTasks fetchOrphanTasks,
    required WebsocketRepository websocketRepository,
    required GetSessionsWithFilters getSessionsWithFilters,
    required SessionRepository sessionRepository,
    required UserSettingsRepository userSettingsRepository,
  }) : _getCategoriesAndTasks = getCategoriesAndTask,
       _fetchOrphanTasks = fetchOrphanTasks,
       _websocketRepository = websocketRepository,
       _getSessionsWithFilters = getSessionsWithFilters,
       _sessionRepository = sessionRepository,
       _userSettingsRepository = userSettingsRepository,
       super(const FocusState()) {
    on<InitState>(_onInitState);
    on<CategorySelected>(_onCategorySelected);
    on<TaskSelected>(_onTaskSelected);
    on<PomodoroStateUpdated>(_onPomodoroStateUpdated);
    on<FocusLevelSelected>(_onFocusLevelSelected);
    on<UpdateNote>(
      _onUpdateNote,
      transformer: debounce(const Duration(milliseconds: 1000)),
    );
    on<StartFocus>(_onStartFocus);
    on<BreakFocus>(_onBreakFocus);
    on<TerminateFocus>(_onTerminateFocus);
    on<ReloadTodaySessions>(
      _onReloadTodaySessions,
      transformer: debounce(const Duration(milliseconds: 500)),
    );
    on<WebSocketConnectionUpdated>(_onWebSocketConnectionUpdated);
    on<CheckConnection>(_onCheckConnection);
    on<AddManualSession>(_onAddManualSession);
  }

  @override
  Future<void> close() {
    _serverResponsesSubscription?.cancel();
    _broadcastEventsSubscription?.cancel();
    _pomodoroStateUpdatesSubscription?.cancel();
    _connectionStatusSubscription?.cancel();
    return super.close();
  }

  Future<void> _onInitState(InitState event, Emitter<FocusState> emit) async {
    emit(state.copyWith(isLoading: true, errorMessage: null));
    logger.d('Initializing FocusBloc');
    try {
      // Load categories and tasks
      final results = await Future.wait([
        _getCategoriesAndTasks.execute(),
        _fetchOrphanTasks.execute(),
        _userSettingsRepository.getUserSettings(),
      ]);

      final categoriesResult = results[0] as GetCategoriesAndTasksResult;
      final orphanTasksResult = results[1] as FetchOrphanTasksResult;
      final userSettings = results[2] as List<UserSetting>;

      if (categoriesResult.error != null || orphanTasksResult.error != null) {
        final errorMessage =
            '${categoriesResult.error ?? ''} ${orphanTasksResult.error ?? ''}'
                .trim();
        emit(state.copyWith(isLoading: false, errorMessage: errorMessage));
        return;
      }

      List<NoteTemplate> noteTemplates = [];
      try {
        final templatesJsonString =
            userSettings
                .firstWhere((s) => s.key == 'note_templates')
                .value;
        if (templatesJsonString.isNotEmpty) {
           final List<dynamic> jsonList = jsonDecode(templatesJsonString);
           noteTemplates = jsonList.map((json) => NoteTemplate.fromJson(json)).toList();
        }
      } catch (_) {
        // No templates found or invalid json
      }

      emit(
        state.copyWith(
          categories: categoriesResult.categoriesWithTasks,
          orphanTasks: orphanTasksResult.orphanTasks ?? [],
          noteTemplates: noteTemplates,
        ),
      );

      add(ReloadTodaySessions());

      // Setup WebSocket message handlers
      _handleWsMessage();

      // Initialize WebSocket AFTER loading data to ensure state is ready for syncData
      logger.d('Checking WebSocket connection...');
      if (!_websocketRepository.isConnected()) {
        logger.d('Connecting to WebSocket...');
        await _websocketRepository.connect();
        logger.d('WebSocket connected');
      }

      // Request initial sync
      _websocketRepository.requestSync();
      
      // Retry sync after a delay to ensure connection stability
      Future.delayed(const Duration(seconds: 1), () {
        if (!isClosed) {
          _websocketRepository.requestSync();
        }
      });
    } catch (e) {
      emit(state.copyWith(isLoading: false, errorMessage: e.toString()));
    } finally {
      emit(state.copyWith(isLoading: false));
    }
  }

  Future<void> _onReloadTodaySessions(
    ReloadTodaySessions event,
    Emitter<FocusState> emit,
  ) async {
    emit(state.copyWith(isLoading: false, errorMessage: null));
    try {
      final now = DateTime.now();
      final startOfDay = DateTime(now.year, now.month, now.day);
      final endOfDay = startOfDay.add(const Duration(days: 1));

      final todaySessionsResult = await _getSessionsWithFilters.execute(
        startDate: startOfDay.millisecondsSinceEpoch ~/ 1000,
        endDate: endOfDay.millisecondsSinceEpoch ~/ 1000,
      );

      logger.d('Today sessions: ${todaySessionsResult.sessions?.length}');

      emit(
        state.copyWith(
          todaySessions: todaySessionsResult.sessions ?? [],
          isLoading: false,
          errorMessage: null,
        ),
      );
    } catch (e) {
      emit(state.copyWith(isLoading: false, errorMessage: e.toString()));
    }
  }

  Future<void> _onCategorySelected(
    CategorySelected event,
    Emitter<FocusState> emit,
  ) async {
    logger.d('Category selected: ${event.category?.name}');
    emit(
      state.copyWith(
        selectedCategory: event.category,
        clearSelectedCategory: event.category == null,
        clearSelectedTask: true, // Also clear task when category changes
      ),
    );
    _websocketRepository.updatePomodoroContext(categoryId: event.category?.id);
  }

  Future<void> _onTaskSelected(
    TaskSelected event,
    Emitter<FocusState> emit,
  ) async {
    logger.d('Task selected: ${event.task?.name}');
    emit(
      state.copyWith(
        selectedTask: event.task,
        clearSelectedTask: event.task == null,
      ),
    );

    _websocketRepository.updatePomodoroContext(
      categoryId: state.selectedCategory?.id,
      taskId: event.task?.id,
    );
  }

  Future<void> _onFocusLevelSelected(
    FocusLevelSelected event,
    Emitter<FocusState> emit,
  ) async {
    logger.d('Focus level selected: ${event.focusLevel}');
    if (state.sessionState != null) {
      SessionState sessionState = state.sessionState!.copyWith(
        selectedFocusLevel: event.focusLevel,
      );
      emit(state.copyWith(sessionState: sessionState));
      _websocketRepository.updateConcentrationScore(event.focusLevel);
    } else {
      logger.e('Session state is null');
    }
  }

  Future<void> _onUpdateNote(UpdateNote event, Emitter<FocusState> emit) async {
    logger.d('Note updated: ${event.note}');
    if (state.sessionState != null) {
      SessionState sessionState = state.sessionState!.copyWith(
        note: event.note,
      );
      emit(state.copyWith(sessionState: sessionState));
      _websocketRepository.updateNote(event.note);
    } else {
      logger.e('Session state is null');
    }
  }

  Future<void> _onStartFocus(StartFocus event, Emitter<FocusState> emit) async {
    logger.d('Focus started');
    _websocketRepository.sendStartEvent();
    add(ReloadTodaySessions());
  }

  Future<void> _onBreakFocus(BreakFocus event, Emitter<FocusState> emit) async {
    logger.d('Focus broken');
    _websocketRepository.sendBreakEvent();
    add(ReloadTodaySessions());
  }

  Future<void> _onTerminateFocus(
    TerminateFocus event,
    Emitter<FocusState> emit,
  ) async {
    logger.d('Focus terminated');
    _websocketRepository.sendTerminateEvent();
    add(ReloadTodaySessions());
  }

  /// Handle WebSocket messages from the repository
  void _handleWsMessage() {
    // Listen to server responses (success, error, syncData)
    _serverResponsesSubscription = _websocketRepository.serverResponses.listen((
      response,
    ) {
      response.when(
        success: (message, requestId) {
          logger.i('Server success: $message (requestId: $requestId)');
        },
        error: (code, message, requestId) {
          logger.e('Server error: [$code] $message (requestId: $requestId)');
        },
        syncData: (pomodoroState) {
          logger.d('Received sync data: $pomodoroState');
          // Handle sync data - update state if needed
          _handlePomodoroStateUpdate(pomodoroState);
        },
      );
    });

    // Listen to broadcast events
    _broadcastEventsSubscription = _websocketRepository.broadcastEvents.listen((
      event,
    ) {
      event.when(
        pomodoroSessionUpdate: (pomodoroState) {
          logger.d('Received pomodoro session update: $pomodoroState');
          _handlePomodoroStateUpdate(pomodoroState);
        },
      );
    });

    // Listen to pomodoro state updates
    _pomodoroStateUpdatesSubscription = _websocketRepository
        .pomodoroStateUpdates
        .listen((pomodoroState) {
          logger.d('Received pomodoro state update: $pomodoroState');
          _handlePomodoroStateUpdate(pomodoroState);
        });

    // Listen to connection status
    _connectionStatusSubscription = _websocketRepository.connectionStatus
        .listen((isConnected) {
      add(WebSocketConnectionUpdated(isConnected));
    });

    // Check initial status
    add(WebSocketConnectionUpdated(_websocketRepository.isConnected()));

    add(ReloadTodaySessions());
  }

  /// Handle pomodoro state updates
  void _handlePomodoroStateUpdate(dynamic pomodoroState) {
    if (isClosed) return;
    add(PomodoroStateUpdated(pomodoroState));
  }

  Future<void> _onPomodoroStateUpdated(
    PomodoroStateUpdated event,
    Emitter<FocusState> emit,
  ) async {
    final pomodoroState = event.pomodoroState;
    logger.d('Pomodoro state - Work context: ${pomodoroState.workContext}');
    Category? selectedCategory;
    Task? selectedTask;
    final categoryId = pomodoroState.workContext.categoryId;
    final taskId = pomodoroState.workContext.taskId;

    if (categoryId != null) {
      try {
        final categoryWithTasks = state.categories.firstWhere(
          (category) => category.category.id == categoryId,
        );
        selectedCategory = categoryWithTasks.category;

        if (taskId != null) {
          try {
            selectedTask = categoryWithTasks.tasks.firstWhere(
              (task) => task.id == taskId,
            );
          } catch (e) {
            logger.w('Task with id $taskId not found in category $categoryId.');
            selectedTask = null; // Explicitly set to null
          }
        }
      } catch (e) {
        logger.w('Category with id $categoryId not found.');
        selectedCategory = null; // Explicitly set to null
      }
    }

    // Handle orphan tasks if no category is selected
    if (selectedCategory == null && taskId != null) {
      try {
        selectedTask = state.orphanTasks.firstWhere(
          (task) => task.id == taskId,
        );
      } catch (e) {
        logger.w('Orphan task with id $taskId not found.');
        selectedTask = null; // Explicitly set to null
      }
    }

    SessionState? sessionState;
    if (pomodoroState.currentSession != null) {
      sessionState = SessionState(
        sessionType: pomodoroState.currentSession!.sessionType,
        startDate: pomodoroState.currentSession!.sessionStartTime,
        note: pomodoroState.currentSession!.note,
        selectedFocusLevel: pomodoroState.currentSession!.concentrationScore,
      );


    }
    
    // Original ReloadTodaySessions call
    add(ReloadTodaySessions());

    logger.d(
      'Updating state - Category: ${selectedCategory?.name}, Task: ${selectedTask?.name}',
    );
    emit(
      state.copyWith(
        selectedCategory: selectedCategory,
        selectedTask: selectedTask,
        clearSelectedCategory: selectedCategory == null,
        clearSelectedTask: selectedTask == null,
        sessionState: sessionState,
        clearSessionState: sessionState == null,
      ),
    );
  }
  Future<void> _onWebSocketConnectionUpdated(
    WebSocketConnectionUpdated event,
    Emitter<FocusState> emit,
  ) async {
    logger.d('WebSocket connection updated: ${event.isConnected}');
    emit(state.copyWith(isWebSocketConnected: event.isConnected));
    if (event.isConnected) {
      _websocketRepository.requestSync();
    }
  }

  Future<void> _onAddManualSession(
    AddManualSession event,
    Emitter<FocusState> emit,
  ) async {
    try {
      emit(state.copyWith(isLoading: true));
      await _sessionRepository.createManualSession(
        sessionType: SessionType.work,
        startedAt: event.startTime.millisecondsSinceEpoch ~/ 1000,
        endedAt: event.endTime.millisecondsSinceEpoch ~/ 1000,
        taskId: event.task?.id,
        categoryId: event.category?.id,
        concentrationScore: event.focusLevel,
        notes: event.note,
      );
      add(ReloadTodaySessions());
      emit(state.copyWith(isLoading: false));
    } catch (e) {
      emit(state.copyWith(isLoading: false, errorMessage: e.toString()));
    }
  }

  Future<void> _onCheckConnection(
    CheckConnection event,
    Emitter<FocusState> emit,
  ) async {
    logger.d('Checking WebSocket connection...');
    if (!_websocketRepository.isConnected()) {
      logger.d('WebSocket not connected, attempting to connect...');
      await _websocketRepository.connect();
    } else {
      logger.d('WebSocket already connected');
      // Ensure state reflects reality
      if (!state.isWebSocketConnected) {
        emit(state.copyWith(isWebSocketConnected: true));
      }
    }
  }
}
