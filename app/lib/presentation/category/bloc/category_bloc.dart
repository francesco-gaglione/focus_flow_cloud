import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:focus_flow_app/domain/usecases/categories_usecases/create_category.dart';
import 'package:focus_flow_app/domain/usecases/categories_usecases/delete_category.dart';
import 'package:focus_flow_app/domain/usecases/categories_usecases/get_categories_and_tasks.dart';
import 'package:focus_flow_app/domain/usecases/categories_usecases/update_category.dart';
import 'package:focus_flow_app/domain/usecases/tasks_usecases/create_task.dart';
import 'package:focus_flow_app/domain/usecases/tasks_usecases/delete_tasks.dart';
import 'package:focus_flow_app/domain/usecases/tasks_usecases/fetch_orphan_tasks.dart';
import 'package:focus_flow_app/domain/usecases/tasks_usecases/update_task.dart';
import 'package:focus_flow_app/presentation/category/bloc/category_event.dart';
import 'package:focus_flow_app/presentation/category/bloc/category_state.dart';
import 'package:logger/logger.dart';

// Bloc
class CategoryBloc extends Bloc<CategoryEvent, CategoryState> {
  final Logger logger = Logger();
  final GetCategoriesAndTasks _getCategoriesAndTasks;
  final FetchOrphanTasks _fetchOrphanTasks;
  final CreateCategory _createCategory;
  final CreateTask _createTask;
  final UpdateCategory _updateCategory;
  final UpdateTask _updateTask;
  final DeleteCategory _deleteCategory;
  final DeleteTasks _deleteTasks;

  CategoryBloc({
    required GetCategoriesAndTasks getCategoriesAndTasks,
    required FetchOrphanTasks fetchOrphanTasks,
    required CreateCategory createCategory,
    required CreateTask createTask,
    required UpdateCategory updateCategory,
    required UpdateTask updateTask,
    required DeleteCategory deleteCategory,
    required DeleteTasks deleteTasks,
  }) : _getCategoriesAndTasks = getCategoriesAndTasks,
       _fetchOrphanTasks = fetchOrphanTasks,
       _createCategory = createCategory,
       _createTask = createTask,
       _updateCategory = updateCategory,
       _updateTask = updateTask,
       _deleteCategory = deleteCategory,
       _deleteTasks = deleteTasks,
       super(const CategoryState(isLoading: true)) {
    on<InitState>(_onInitState);
    on<LoadCategories>(_onLoadCategories);
    on<LoadOrphanTasks>(_onLoadOrphanTasks);
    on<CreateCategoryEvent>(_onCreateCategory);
    on<CreateTaskEvent>(_onCreateTask);
    on<CreateOrphanTaskEvent>(_onCreateOrphanTask);
    on<UpdateCategoryEvent>(_onUpdateCategory);
    on<UpdateTaskEvent>(_onUpdateTask);
    on<DeleteCategoryEvent>(_onDeleteCategory);
    on<DeleteTaskEvent>(_onDeleteTask);
  }

  Future<void> _onInitState(
    InitState event,
    Emitter<CategoryState> emit,
  ) async {
    logger.d('Initializing category state...');
    emit(state.copyWith(isLoading: true, errorMessage: null));
    try {
      final results = await Future.wait([
        _getCategoriesAndTasks.execute(),
        _fetchOrphanTasks.execute(),
      ]);

      final categoriesResult = results[0] as GetCategoriesAndTasksResult;
      final orphanTasksResult = results[1] as FetchOrphanTasksResult;

      if (categoriesResult.success &&
          orphanTasksResult.success &&
          categoriesResult.categoriesWithTasks != null) {
        emit(
          CategoryState(
            categories: categoriesResult.categoriesWithTasks!,
            orphanTasks: orphanTasksResult.orphanTasks ?? [],
            isLoading: false,
            errorMessage: null,
          ),
        );
      } else {
        emit(
          state.copyWith(
            isLoading: false,
            errorMessage:
                categoriesResult.error ??
                orphanTasksResult.error ??
                'Unknown error',
          ),
        );
      }
    } catch (e) {
      emit(state.copyWith(isLoading: false, errorMessage: e.toString()));
    }
  }

  Future<void> _onLoadCategories(
    LoadCategories event,
    Emitter<CategoryState> emit,
  ) async {
    logger.d('Loading categories...');
    emit(state.copyWith(isLoading: true, errorMessage: null));
    try {
      final result = await _getCategoriesAndTasks.execute();
      if (result.success && result.categoriesWithTasks != null) {
        emit(
          state.copyWith(
            categories: result.categoriesWithTasks,
            isLoading: false,
            errorMessage: null,
          ),
        );
      } else {
        emit(
          state.copyWith(
            isLoading: false,
            errorMessage: result.error ?? 'Unknown error',
          ),
        );
      }
    } catch (e) {
      emit(state.copyWith(isLoading: false, errorMessage: e.toString()));
    }
  }

  Future<void> _onLoadOrphanTasks(
    LoadOrphanTasks event,
    Emitter<CategoryState> emit,
  ) async {
    emit(state.copyWith(isLoading: true, errorMessage: null));
    try {
      final result = await _fetchOrphanTasks.execute();
      if (result.success && result.orphanTasks != null) {
        emit(
          state.copyWith(
            orphanTasks: result.orphanTasks,
            isLoading: false,
            errorMessage: null,
          ),
        );
      } else {
        emit(
          state.copyWith(
            isLoading: false,
            errorMessage: result.error ?? 'Unknown error',
          ),
        );
      }
    } catch (e) {
      emit(state.copyWith(isLoading: false, errorMessage: e.toString()));
    }
  }

  Future<void> _onCreateCategory(
    CreateCategoryEvent event,
    Emitter<CategoryState> emit,
  ) async {
    try {
      final result = await _createCategory.execute(
        name: event.name,
        color: event.color,
        description: event.description,
      );
      if (result.success) {
        add(InitState());
      } else {
        emit(
          state.copyWith(
            errorMessage: result.error ?? 'Failed to create category',
          ),
        );
      }
    } catch (e) {
      emit(state.copyWith(errorMessage: e.toString()));
    }
  }

  Future<void> _onCreateTask(
    CreateTaskEvent event,
    Emitter<CategoryState> emit,
  ) async {
    try {
      final result = await _createTask.execute(
        name: event.title,
        description: event.description,
        categoryId: event.categoryId,
      );
      if (result.success) {
        add(InitState());
      } else {
        emit(
          state.copyWith(
            errorMessage: result.error ?? 'Failed to create category',
          ),
        );
      }
    } catch (e) {
      emit(state.copyWith(errorMessage: e.toString()));
    }
  }

  Future<void> _onCreateOrphanTask(
    CreateOrphanTaskEvent event,
    Emitter<CategoryState> emit,
  ) async {
    try {
      final result = await _createTask.execute(
        name: event.title,
        description: event.description,
      );
      if (result.success) {
        add(InitState());
      } else {
        emit(
          state.copyWith(
            errorMessage: result.error ?? 'Failed to create orphan task',
          ),
        );
      }
    } catch (e) {
      emit(state.copyWith(errorMessage: e.toString()));
    }
  }

  Future<void> _onUpdateCategory(
    UpdateCategoryEvent event,
    Emitter<CategoryState> emit,
  ) async {
    try {
      final result = await _updateCategory.execute(
        id: event.id,
        name: event.name,
        color: event.color,
        description: event.description,
      );
      if (result.success) {
        add(InitState());
      } else {
        emit(
          state.copyWith(
            errorMessage: result.error ?? 'Failed to update category',
          ),
        );
      }
    } catch (e) {
      emit(state.copyWith(errorMessage: e.toString()));
    }
  }

  Future<void> _onUpdateTask(
    UpdateTaskEvent event,
    Emitter<CategoryState> emit,
  ) async {
    try {
      final result = await _updateTask.execute(
        id: event.id,
        name: event.name,
        description: event.description,
      );
      if (result.success) {
        add(InitState());
      } else {
        emit(
          state.copyWith(errorMessage: result.error ?? 'Failed to update task'),
        );
      }
    } catch (e) {
      emit(state.copyWith(errorMessage: e.toString()));
    }
  }

  Future<void> _onDeleteCategory(
    DeleteCategoryEvent event,
    Emitter<CategoryState> emit,
  ) async {
    try {
      logger.d('Deleting category with id ${event.id}');
      final result = await _deleteCategory.execute(id: event.id);
      if (result.success) {
        add(InitState());
      } else {
        logger.e(
          'Failed to delete category with id ${event.id}: ${result.error}',
        );
        emit(
          state.copyWith(
            errorMessage: result.error ?? 'Failed to delete category',
          ),
        );
      }
    } catch (e) {
      emit(state.copyWith(errorMessage: e.toString()));
    }
  }

  Future<void> _onDeleteTask(
    DeleteTaskEvent event,
    Emitter<CategoryState> emit,
  ) async {
    try {
      logger.d('Deleting task with id ${event.id}');
      final result = await _deleteTasks.execute(taskIds: [event.id]);
      if (result.success) {
        add(InitState());
      } else {
        logger.e('Failed to delete task with id ${event.id}: ${result.error}');
        emit(
          state.copyWith(errorMessage: result.error ?? 'Failed to delete task'),
        );
      }
    } catch (e) {
      emit(state.copyWith(errorMessage: e.toString()));
    }
  }
}
