import 'package:easy_localization/easy_localization.dart';
import 'package:easy_localization_loader/easy_localization_loader.dart';
import 'package:flutter/material.dart';
import 'package:logger/logger.dart';

import 'core/services/configuration_service.dart';
import 'core/di/service_locator.dart' as di;
import 'domain/usecases/get_theme_settings.dart';
import 'domain/usecases/toggle_theme.dart';
import 'domain/usecases/update_accent_color.dart';
import 'presentation/app/app.dart';
import 'presentation/pages/onboarding/backend_config_page.dart';

final logger = Logger();

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await EasyLocalization.ensureInitialized();

  // Initialize Configuration Service
  final configService = await ConfigurationService.init();

  runApp(
    EasyLocalization(
      supportedLocales: const [Locale('en'), Locale('it')],
      path: 'assets/translations',
      assetLoader: YamlAssetLoader(),
      fallbackLocale: const Locale('en'),
      child: FocusFlowApp(configService: configService),
    ),
  );
}

// Global function to restart app (rebuilds widget tree and re-runs init)
void restartApp(BuildContext context) {
  context.findAncestorStateOfType<_FocusFlowAppState>()?.restart();
}

class FocusFlowApp extends StatefulWidget {
  final ConfigurationService configService;
  
  const FocusFlowApp({super.key, required this.configService});

  @override
  State<FocusFlowApp> createState() => _FocusFlowAppState();
}

class _FocusFlowAppState extends State<FocusFlowApp> {
  Key _uniqueKey = UniqueKey();
  bool _isInitialized = false;

  void restart() {
    setState(() {
      _uniqueKey = UniqueKey();
      _isInitialized = false; 
    });
  }

  @override
  Widget build(BuildContext context) {
    return KeyedSubtree(
      key: _uniqueKey,
      child: _AppLoader(
        configService: widget.configService,
        isInitialized: _isInitialized,
        onInitialized: () => setState(() => _isInitialized = true),
      ),
    );
  }
}

class _AppLoader extends StatefulWidget {
  final ConfigurationService configService;
  final bool isInitialized;
  final VoidCallback onInitialized;

  const _AppLoader({
    required this.configService,
    required this.isInitialized,
    required this.onInitialized,
  });

  @override
  State<_AppLoader> createState() => _AppLoaderState();
}

class _AppLoaderState extends State<_AppLoader> {
  @override
  void initState() {
    super.initState();
    _init();
  }

  Future<void> _init() async {
    final baseUrl = widget.configService.apiBaseUrl;

    if (baseUrl != null && baseUrl.isNotEmpty) {
      // If we have a URL, setup dependencies
      await di.resetDependencies(); // Clear old instances if any
      
      final wsScheme = baseUrl.startsWith('https') ? 'wss' : 'ws';
      final cleanBaseUrl = baseUrl.replaceFirst(RegExp(r'^https?://'), '');
      final wsUrl = '$wsScheme://$cleanBaseUrl/ws/session';
      
      await di.setupDependencies(baseUrl, wsUrl);
      
      if (mounted) {
        widget.onInitialized();
      }
    } else {
      // No URL, skip setup, show config page
      // Dependencies are NOT set up yet
    }
  }

  @override
  Widget build(BuildContext context) {
    if (widget.configService.apiBaseUrl == null) {
      return MaterialApp(
        debugShowCheckedModeBanner: false,
        theme: ThemeData.light(), // Simple theme for config page
        localizationsDelegates: context.localizationDelegates,
        supportedLocales: context.supportedLocales,
        locale: context.locale,
        home: BackendConfigPage(
          configService: widget.configService,
          isFirstRun: true,
        ),
      );
    }

    if (!widget.isInitialized) {
      return const MaterialApp(
        home: Scaffold(
          body: Center(child: CircularProgressIndicator()),
        ),
      );
    }

    return App(
        getThemeSettings: di.sl<GetThemeSettings>(),
        toggleTheme: di.sl<ToggleTheme>(),
        updateAccentColor: di.sl<UpdateAccentColor>(),
    );
  }
}
