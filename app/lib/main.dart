import 'dart:convert';
import 'package:easy_localization/easy_localization.dart';
import 'package:easy_localization_loader/easy_localization_loader.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import 'core/di/service_locator.dart';
import 'domain/usecases/get_theme_settings.dart';
import 'domain/usecases/toggle_theme.dart';
import 'domain/usecases/update_accent_color.dart';
import 'presentation/app/app.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await EasyLocalization.ensureInitialized();

  // Load configuration from assets/config.json
  // This allows runtime configuration in Docker/K8s by modifying the file
  String baseUrl = 'http://localhost:8080';
  String wsUrl = 'ws://localhost:8080/ws/workspace/session';

  try {
    final configString = await rootBundle.loadString('assets/config.json');
    final config = json.decode(configString) as Map<String, dynamic>;

    // Prioritize config.json, fallback to dart-define (if empty in config), then default
    if (config['BASE_URL'] != null &&
        (config['BASE_URL'] as String).isNotEmpty) {
      baseUrl = config['BASE_URL'];
    } else if (const String.fromEnvironment('BASE_URL').isNotEmpty) {
      baseUrl = const String.fromEnvironment('BASE_URL');
    }

    if (config['WS_URL'] != null && (config['WS_URL'] as String).isNotEmpty) {
      wsUrl = config['WS_URL'];
    } else if (const String.fromEnvironment('WS_URL').isNotEmpty) {
      wsUrl = const String.fromEnvironment('WS_URL');
    }
  } catch (e) {
    debugPrint("Error loading config.json: $e");
    // Fallback to dart-define if config load fails
    if (const String.fromEnvironment('BASE_URL').isNotEmpty) {
      baseUrl = const String.fromEnvironment('BASE_URL');
    }
    if (const String.fromEnvironment('WS_URL').isNotEmpty) {
      wsUrl = const String.fromEnvironment('WS_URL');
    }
  }

  // Initialize the service locator with the loaded configuration
  await setupDependencies(baseUrl, wsUrl);

  runApp(
    EasyLocalization(
      supportedLocales: const [Locale('en'), Locale('it')],
      path: 'assets/translations',
      assetLoader: YamlAssetLoader(),
      fallbackLocale: const Locale('en'),
      child: App(
        getThemeSettings: sl<GetThemeSettings>(),
        toggleTheme: sl<ToggleTheme>(),
        updateAccentColor: sl<UpdateAccentColor>(),
      ),
    ),
  );
}
