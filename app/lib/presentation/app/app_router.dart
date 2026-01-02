import 'dart:async';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import '../auth/cubit/auth_cubit.dart';
import '../auth/pages/login_page.dart';
import '../focus/focus_page.dart';
import '../category/category_page.dart';
import '../statistics/statistics_page.dart';
import '../settings/settings_page.dart';
import 'main_layout.dart';

class AppRouter {
  final AuthCubit authCubit;

  AppRouter(this.authCubit);

  late final GoRouter router = GoRouter(
    initialLocation: '/focus',
    refreshListenable: GoRouterRefreshStream(authCubit.stream),
    redirect: (context, state) {
      final isLoggedIn = authCubit.state is AuthAuthenticated;
      final isLoggingIn = state.uri.path == '/login';

      if (!isLoggedIn) return '/login';
      if (isLoggedIn && isLoggingIn) return '/focus';
      return null;
    },
    routes: [
      GoRoute(
        path: '/login',
        builder: (context, state) => const LoginPage(),
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
