import 'package:dio/dio.dart';
import 'package:focus_flow_app/adapters/repositories/http_category_repository.dart';
import 'package:focus_flow_app/adapters/repositories/http_session_repository.dart';
import 'package:focus_flow_app/adapters/repositories/http_statistics_repository.dart';
import 'package:focus_flow_app/adapters/repositories/http_task_repository.dart';
import 'package:focus_flow_app/adapters/repositories/http_user_settings_repository.dart';
import 'package:focus_flow_app/adapters/ws/ws_repository.dart';
import 'package:focus_flow_app/domain/repositories/category_repository.dart';
import 'package:focus_flow_app/domain/repositories/session_repository.dart';
import 'package:focus_flow_app/domain/repositories/statistics_repository.dart';
import 'package:focus_flow_app/domain/repositories/task_repository.dart';
import 'package:focus_flow_app/domain/repositories/user_settings_repository.dart';
import 'package:focus_flow_app/domain/usecases/calculate_stats_by_period.dart';
import 'package:focus_flow_app/domain/usecases/categories_usecases/create_category.dart';
import 'package:focus_flow_app/domain/usecases/categories_usecases/delete_category.dart';
import 'package:focus_flow_app/domain/usecases/categories_usecases/get_categories_and_tasks.dart';
import 'package:focus_flow_app/domain/usecases/categories_usecases/update_category.dart';
import 'package:focus_flow_app/domain/usecases/sessions_usecases/create_manual_session.dart';
import 'package:focus_flow_app/domain/usecases/sessions_usecases/get_sessions_with_filters.dart';
import 'package:focus_flow_app/domain/usecases/tasks_usecases/create_task.dart';
import 'package:focus_flow_app/domain/usecases/tasks_usecases/delete_tasks.dart';
import 'package:focus_flow_app/domain/usecases/tasks_usecases/fetch_orphan_tasks.dart';
import 'package:focus_flow_app/domain/usecases/tasks_usecases/update_task.dart';
import 'package:get_it/get_it.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../../adapters/theme/user_settings_theme_repository.dart';
import '../../domain/repositories/theme_repository.dart';
import '../../domain/usecases/get_app_version.dart';
import '../../domain/usecases/get_saved_locale.dart';
import '../../domain/usecases/get_theme_settings.dart';
import '../../domain/usecases/save_locale.dart';
import '../../domain/usecases/toggle_theme.dart';
import '../../domain/usecases/update_accent_color.dart';
import '../../presentation/app/locale_cubit.dart';
import '../../core/services/token_service.dart';
import '../../adapters/repositories/http_auth_repository.dart';
import '../../domain/repositories/auth_repository.dart';
import '../../domain/usecases/login_user.dart';
import '../../domain/usecases/logout_user.dart';
import '../../domain/usecases/user_usecases/get_user_info.dart';
import '../../domain/usecases/user_usecases/update_password.dart';
import '../../domain/usecases/user_usecases/update_username.dart';
import '../../domain/usecases/user_usecases/create_user.dart';
import '../../presentation/auth/cubit/auth_cubit.dart';
import '../../presentation/settings/cubit/account_cubit.dart';
import '../../presentation/app/app_router.dart';
import '../../presentation/settings/bloc/settings_bloc.dart';

final sl = GetIt.instance;

Future<void> setupDependencies(String baseUrl, String wsUrl) async {
  // External
  final sharedPreferences = await SharedPreferences.getInstance();
  sl.registerSingleton<SharedPreferences>(sharedPreferences);

  // Core Services
  sl.registerLazySingleton<TokenService>(() => TokenService(sl()));

  // Dio
  sl.registerLazySingleton<Dio>(() {
    final dio = Dio(
      BaseOptions(
        baseUrl: baseUrl,
        connectTimeout: const Duration(seconds: 30),
        receiveTimeout: const Duration(seconds: 15),
      ),
    );
    dio.interceptors.add(
      InterceptorsWrapper(
        onRequest: (options, handler) {
          final token = sl<TokenService>().getToken();
          if (token != null) {
            options.headers['Authorization'] = 'Bearer $token';
          }
          handler.next(options);
        },
        onError: (error, handler) async {
          if (error.type == DioExceptionType.cancel) {
            handler.next(error);
            return;
          }

          if (error.response?.statusCode == 401) {
            final path = error.requestOptions.path;
            // Avoid loops: don't refresh if we failed on login or refresh endpoints
            if (path.contains('/api/auth/login') ||
                path.contains('/api/auth/refresh')) {
              handler.next(error);
              return;
            }

            final refreshToken = sl<TokenService>().getRefreshToken();
            if (refreshToken != null) {
              try {
                // Attempt refresh
                final response = await sl<AuthRepository>().refreshToken(
                  refreshToken,
                );

                // Save new tokens
                await sl<TokenService>().saveToken(response.token);
                await sl<TokenService>().saveRefreshToken(
                  response.refreshToken,
                );

                // Retry the original request
                final options = error.requestOptions;
                options.headers['Authorization'] = 'Bearer ${response.token}';

                final cloneReq = await sl<Dio>().fetch(options);
                handler.resolve(cloneReq);
                return;
              } catch (e) {
                // Refresh failed, clear session
                await sl<TokenService>().clearToken();
              }
            } else {
              await sl<TokenService>().clearToken();
            }
          }
          handler.next(error);
        },
      ),
    );
    return dio;
  });

  // Repositories - User Settings
  sl.registerLazySingleton<UserSettingsRepository>(
    () => HttpUserSettingsRepository(dio: sl(), baseUrl: baseUrl),
  );

  // Repositories - Theme
  sl.registerLazySingleton<ThemeRepository>(
    () => UserSettingsThemeRepository(sl()),
  );

  // Repositories - HTTP
  sl.registerLazySingleton<CategoryRepository>(
    () => HttpCategoryRepository(dio: sl(), baseUrl: baseUrl),
  );

  sl.registerLazySingleton<TaskRepository>(
    () => HttpTaskRepository(dio: sl(), baseUrl: baseUrl),
  );

  sl.registerLazySingleton<SessionRepository>(
    () => HttpSessionRepository(dio: sl(), baseUrl: baseUrl),
  );

  sl.registerLazySingleton<StatisticsRepository>(
    () => HttpStatisticsRepository(dio: sl(), baseUrl: baseUrl),
  );

  sl.registerLazySingleton<AuthRepository>(
    () => HttpAuthRepository(dio: sl(), baseUrl: baseUrl),
  );

  // Repositories - WebSocket
  sl.registerLazySingleton<WebsocketRepository>(
    //TODO read ws url from config
    () => WebsocketRepository(wsUrl, sl()),
  );

  // Use Cases - Theme
  sl.registerLazySingleton<GetThemeSettings>(
    () => GetThemeSettings(sl<ThemeRepository>()),
  );

  sl.registerLazySingleton<ToggleTheme>(
    () => ToggleTheme(sl<ThemeRepository>()),
  );

  sl.registerLazySingleton<UpdateAccentColor>(
    () => UpdateAccentColor(sl<ThemeRepository>()),
  );

  // Use Cases - Locale
  sl.registerLazySingleton<GetSavedLocale>(
    () => GetSavedLocale(sl<UserSettingsRepository>()),
  );

  sl.registerLazySingleton<SaveLocale>(
    () => SaveLocale(sl<UserSettingsRepository>()),
  );

  sl.registerLazySingleton<GetAppVersion>(() => GetAppVersion());

  // Use Cases - Auth & User
  sl.registerLazySingleton<LoginUser>(() => LoginUser(sl()));

  sl.registerLazySingleton<LogoutUser>(() => LogoutUser(sl()));

  sl.registerLazySingleton<UpdatePassword>(() => UpdatePassword(sl()));

  sl.registerLazySingleton<UpdateUsername>(() => UpdateUsername(sl()));

  sl.registerLazySingleton<GetUserInfo>(() => GetUserInfo(sl()));

  sl.registerLazySingleton<CreateUser>(() => CreateUser(sl()));

  // Cubits
  sl.registerFactory(() => LocaleCubit(getSavedLocale: sl(), saveLocale: sl()));

  sl.registerFactory(
    () => AccountCubit(
      updatePassword: sl(),
      updateUsername: sl(),
      getUserInfo: sl(),
      createUserUseCase: sl(),
    ),
  );

  // AuthCubit needs to be a singleton because it's used by the Router which is a singleton.
  // OR factory if we re-create router? No, router is long lived.
  sl.registerLazySingleton(
    () => AuthCubit(loginUser: sl(), logoutUser: sl(), tokenService: sl()),
  );

  // Router
  sl.registerLazySingleton(() => AppRouter(sl()));

  // Use Cases - Category
  sl.registerLazySingleton<GetCategoriesAndTasks>(
    () => GetCategoriesAndTasks(categoryRepository: sl()),
  );

  sl.registerLazySingleton<CreateCategory>(
    () => CreateCategory(categoryRepository: sl()),
  );

  sl.registerLazySingleton<UpdateCategory>(
    () => UpdateCategory(categoryRepository: sl()),
  );

  sl.registerLazySingleton<DeleteCategory>(
    () => DeleteCategory(categoryRepository: sl()),
  );

  // Use Cases - Task
  sl.registerLazySingleton<CreateTask>(
    () => CreateTask(taskRepository: sl(), categoryRepository: sl()),
  );

  sl.registerLazySingleton<UpdateTask>(
    () => UpdateTask(taskRepository: sl(), categoryRepository: sl()),
  );

  sl.registerLazySingleton<DeleteTasks>(
    () => DeleteTasks(taskRepository: sl()),
  );

  sl.registerLazySingleton<FetchOrphanTasks>(
    () => FetchOrphanTasks(taskRepository: sl()),
  );

  // Use Cases - Session
  sl.registerLazySingleton<GetSessionsWithFilters>(
    () => GetSessionsWithFilters(sessionRepository: sl()),
  );

  sl.registerLazySingleton<CreateManualSession>(
    () => CreateManualSession(
      sessionRepository: sl(),
      taskRepository: sl(),
      categoryRepository: sl(),
    ),
  );

  // Use Cases - Statistics
  sl.registerLazySingleton<CalculateStatsByPeriod>(
    () => CalculateStatsByPeriod(statisticsRepository: sl()),
  );
  // Cubits - Settings
  sl.registerFactory(() => SettingsBloc(userSettingsRepository: sl()));
}
