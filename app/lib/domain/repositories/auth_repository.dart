import 'package:focus_flow_app/adapters/dtos/auth_dtos.dart';
import 'package:focus_flow_app/adapters/dtos/user_dtos.dart';

abstract class AuthRepository {
  Future<LoginResponseDto> login(String username, String password);
  Future<RefreshResponseDto> refreshToken(String refreshToken);
  Future<void> updatePassword(String oldPassword, String newPassword);
  Future<void> updateUsername(String newUsername);
  Future<void> createUser(String username, String password);
  Future<void> logout();
  Future<UserInfoResponseDto> getUserInfo();
}
