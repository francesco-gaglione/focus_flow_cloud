import 'package:focus_flow_app/domain/entities/user_setting.dart';

abstract class UserSettingsRepository {
  Future<List<UserSetting>> getUserSettings();
  Future<bool> updateUserSetting(UserSetting setting);
}
