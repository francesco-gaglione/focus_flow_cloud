import 'package:focus_flow_app/domain/entities/user_setting.dart';
import 'package:focus_flow_app/domain/repositories/user_settings_repository.dart';

class UpdateUserSetting {
  final UserSettingsRepository userSettingsRepository;

  UpdateUserSetting({required this.userSettingsRepository});

  Future<UpdateUserSettingResult> execute(String key, String value) async {
    try {
      await userSettingsRepository.updateUserSetting(
        UserSetting(key: key, value: value),
      );

      return UpdateUserSettingResult(success: true);
    } catch (e) {
      return UpdateUserSettingResult(success: false, error: e.toString());
    }
  }
}

class UpdateUserSettingResult {
  final bool success;
  final String? error;

  UpdateUserSettingResult({required this.success, this.error});
}
