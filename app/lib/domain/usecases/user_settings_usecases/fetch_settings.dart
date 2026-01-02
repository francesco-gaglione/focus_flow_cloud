import 'package:focus_flow_app/domain/entities/user_setting.dart';
import 'package:focus_flow_app/domain/repositories/user_settings_repository.dart';

class FetchUserSettings {
  final UserSettingsRepository userSettingsRepository;

  FetchUserSettings({required this.userSettingsRepository});

  Future<FetchUserSettingsResult> execute() async {
    try {
      final userSettings = await userSettingsRepository.getUserSettings();

      return FetchUserSettingsResult(success: true, userSettings: userSettings);
    } catch (e) {
      return FetchUserSettingsResult(success: false, error: e.toString());
    }
  }
}

class FetchUserSettingsResult {
  final bool success;
  final List<UserSetting>? userSettings;
  final String? error;

  FetchUserSettingsResult({
    required this.success,
    this.userSettings,
    this.error,
  });
}
