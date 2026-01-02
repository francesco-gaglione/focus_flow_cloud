import 'package:dio/dio.dart';
import 'package:focus_flow_app/adapters/dtos/user_dtos.dart';
import '../../domain/repositories/auth_repository.dart';
import '../dtos/auth_dtos.dart';

class HttpAuthRepository implements AuthRepository {
  final Dio _dio;
  final String baseUrl;

  HttpAuthRepository({required Dio dio, required this.baseUrl}) : _dio = dio;

  @override
  Future<LoginResponseDto> login(String username, String password) async {
    final dto = LoginDto(username: username, password: password);
    final response = await _dio.post(
      '$baseUrl/api/auth/login',
      data: dto.toJson(),
    );
    return LoginResponseDto.fromJson(response.data);
  }

  @override
  Future<RefreshResponseDto> refreshToken(String refreshToken) async {
    final dto = RefreshDto(refreshToken: refreshToken);
    final response = await _dio.post(
      '$baseUrl/api/auth/refresh',
      data: dto.toJson(),
    );
    return RefreshResponseDto.fromJson(response.data);
  }

  @override
  Future<void> updatePassword(String oldPassword, String newPassword) async {
    final dto = UpdatePasswordDto(
      oldPassword: oldPassword,
      newPassword: newPassword,
    );
    await _dio.put('$baseUrl/api/users/password', data: dto.toJson());
  }

  @override
  Future<void> updateUsername(String newUsername) async {
    final dto = UpdateUsernameDto(newUsername: newUsername);
    await _dio.put('$baseUrl/api/users/username', data: dto.toJson());
  }

  @override
  Future<void> createUser(String username, String password) async {
    final dto = CreateUserDto(username: username, password: password);
    await _dio.post('$baseUrl/api/users', data: dto.toJson());
  }

  @override
  Future<void> logout() async {
    await _dio.post('$baseUrl/api/auth/logout');
  }

  @override
  Future<UserInfoResponseDto> getUserInfo() async {
    final response = await _dio.get('$baseUrl/api/users/me');
    return UserInfoResponseDto.fromJson(response.data);
  }
}
