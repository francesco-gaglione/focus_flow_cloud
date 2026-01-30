import 'dart:async';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import '../auth/cubit/auth_cubit.dart';
import '../auth/pages/login_page.dart';
import '../focus/focus_page.dart';
import '../category/category_page.dart';
import '../statistics/statistics_page.dart';
import '../settings/settings_page.dart';
import '../notes/notes_page.dart';
import '../notes/notes_page.dart';
import 'main_layout.dart';
import '../../core/services/configuration_service.dart';
import '../../core/di/service_locator.dart';
import '../pages/onboarding/backend_config_page.dart';

class AppRouter {
  final AuthCubit authCubit;

  AppRouter(this.authCubit);

  late final GoRouter router = GoRouter(
    initialLocation: '/focus',
    refreshListenable: GoRouterRefreshStream(authCubit.stream),
    redirect: (context, state) {
      final isLoggedIn = authCubit.state is AuthAuthenticated;
      final isLoggingIn = state.uri.path == '/login';
      final isConfiguring = state.uri.path == '/config';

      if (isConfiguring) return null;
      if (!isLoggedIn) return '/login';
      if (isLoggedIn && isLoggingIn) return '/focus';
      return null;
    },
    routes: [
      GoRoute(
        path: '/login',
        builder: (context, state) => const LoginPage(),
      ),
      GoRoute(
        path: '/config',
        builder: (context, state) {
           return BackendConfigPage(
              configService: sl<ConfigurationService>(),
              isFirstRun: false, 
           );
        },
      ),
      ShellRoute(
        builder: (context, state, child) {
          return MainLayout(currentPath: state.uri.path, child: child);
        },
        routes: [
          GoRoute(path: '/focus', builder: (context, state) => const FocusPage()),
          GoRoute(
            path: '/categories',
            builder: (context, state) => const CategoryPage(),
          ),
          GoRoute(
            path: '/stats',
            builder: (context, state) => const StatisticsPage(),
          ),
          GoRoute(
            path: '/settings',
            builder: (context, state) => const SettingsPage(),
          ),
          GoRoute(
            path: '/notes',
            builder: (context, state) => const NotesPage(),
          ),
        ],
      ),
    ],
  );
}

class GoRouterRefreshStream extends ChangeNotifier {
  GoRouterRefreshStream(Stream<dynamic> stream) {
    notifyListeners();
    _subscription = stream.asBroadcastStream().listen(
      (dynamic _) => notifyListeners(),
    );
  }

  late final StreamSubscription<dynamic> _subscription;

  @override
  void dispose() {
    _subscription.cancel();
    super.dispose();
  }
}
