import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:equatable/equatable.dart';
import 'package:dio/dio.dart';
import '../../../domain/usecases/login_user.dart';
import '../../../domain/usecases/logout_user.dart';
import '../../../core/services/token_service.dart';

part 'auth_state.dart';

class AuthCubit extends Cubit<AuthState> {
  final LoginUser loginUser;
  final LogoutUser logoutUser;
  final TokenService tokenService;

  AuthCubit({
    required this.loginUser,
    required this.logoutUser,
    required this.tokenService,
  }) : super(AuthInitial());

  Future<void> checkAuthStatus() async {
    final token = tokenService.getToken();
    if (token != null) {
      emit(AuthAuthenticated(token));
    } else {
      emit(AuthUnauthenticated());
    }
  }

  Future<void> login(String username, String password) async {
    emit(AuthLoading());
    try {
      final response = await loginUser(username, password);
      await tokenService.saveToken(response.token);
      await tokenService.saveRefreshToken(response.refreshToken);
      emit(AuthAuthenticated(response.token));
    } catch (e) {
       if (e is DioException && e.response?.statusCode == 401) {
            emit(const AuthError('Invalid credentials'));
        } else {
            emit(AuthError(e.toString()));
        }
    }
  }

  Future<void> logout() async {
    try {
        await logoutUser();
    } catch (_) {
        // Ignore logout errors, we want to clear local session anyway
    }
    await tokenService.clearToken();
    emit(AuthUnauthenticated());
  }
}
