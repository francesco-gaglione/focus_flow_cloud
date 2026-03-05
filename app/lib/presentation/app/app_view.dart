import 'package:easy_localization/easy_localization.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import '../../core/di/service_locator.dart';
import '../../core/theme/app_theme.dart';
import 'locale_cubit.dart';
import 'theme_cubit.dart';
import 'app_router.dart';
import '../../presentation/focus/bloc/focus_bloc.dart';
import '../../presentation/focus/bloc/focus_event.dart';
import '../../presentation/version/cubit/version_cubit.dart';

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

          return MultiBlocProvider(
            providers: [
              BlocProvider(
                create: (_) => sl<VersionCubit>()..checkVersion(),
              ),
              BlocProvider(
                create:
                    (_) => FocusBloc(
                      getCategoriesAndTask: sl(),
                      fetchOrphanTasks: sl(),
                      websocketRepository: sl(),
                      getSessionsWithFilters: sl(),
                      getScheduledTasks: sl(),
                      sessionRepository: sl(),
                      userSettingsRepository: sl(),
                      notificationService: sl(),
                    )..add(InitState()),
              ),
            ],
            child: MaterialApp.router(
              theme: lightTheme,
              darkTheme: darkTheme,
              themeMode: state.isDarkMode ? ThemeMode.dark : ThemeMode.light,
              routerConfig: sl<AppRouter>().router,
              localizationsDelegates: context.localizationDelegates,
              supportedLocales: context.supportedLocales,
              locale: context.locale,
            ),
          );
        },
      ),
    );
  }
}
