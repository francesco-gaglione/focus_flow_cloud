import 'package:easy_localization/easy_localization.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import '../../core/di/service_locator.dart';
import '../../core/theme/app_theme.dart';
import 'locale_cubit.dart';
import 'theme_cubit.dart';
import 'app_router.dart';

class AppView extends StatelessWidget {
  const AppView({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocListener<LocaleCubit, LocaleState>(
      listener: (context, state) {
        if (state.locale != null && state.locale != context.locale) {
          context.setLocale(state.locale!);
        }
      },
      child: BlocBuilder<ThemeCubit, ThemeState>(
        builder: (_, state) {
          final lightTheme = AppTheme.light(state.accentColor);
          final darkTheme = AppTheme.dark(state.accentColor);

          return MaterialApp.router(
            theme: lightTheme,
            darkTheme: darkTheme,
            themeMode: state.isDarkMode ? ThemeMode.dark : ThemeMode.light,
            routerConfig: sl<AppRouter>().router,
            localizationsDelegates: context.localizationDelegates,
            supportedLocales: context.supportedLocales,
            locale: context.locale,
          );
        },
      ),
    );
  }
}
