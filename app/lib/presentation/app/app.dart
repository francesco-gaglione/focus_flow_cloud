import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import '../../core/di/service_locator.dart';
import '../../domain/usecases/get_theme_settings.dart';
import '../../domain/usecases/toggle_theme.dart';
import '../../domain/usecases/update_accent_color.dart';
import 'app_view.dart';
import 'locale_cubit.dart';
import 'theme_cubit.dart';
import '../auth/cubit/auth_cubit.dart';

class App extends StatelessWidget {
  final GetThemeSettings getThemeSettings;
  final ToggleTheme toggleTheme;
  final UpdateAccentColor updateAccentColor;

  const App({
    super.key,
    required this.getThemeSettings,
    required this.toggleTheme,
    required this.updateAccentColor,
  });

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create:
          (_) => ThemeCubit(
            getThemeSettings: getThemeSettings,
            toggleTheme: toggleTheme,
            updateAccentColor: updateAccentColor,
          )..loadTheme(),
      child: BlocProvider(
        create: (_) => sl<LocaleCubit>()..loadLocale(),
        child: BlocProvider(
          create: (_) => sl<AuthCubit>()..checkAuthStatus(),
          child: const AppView(),
        ),
      ),
    );
  }
}
