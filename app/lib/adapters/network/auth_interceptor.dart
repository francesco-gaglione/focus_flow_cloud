import 'package:dio/dio.dart';
import '../../core/services/token_service.dart';
import '../../domain/repositories/auth_repository.dart';

class AuthInterceptor extends QueuedInterceptorsWrapper {
  final TokenService _tokenService;
  final AuthRepository _authRepository;
  final Dio _dio;
  final Future<void> Function()? onSessionExpired;

  AuthInterceptor({
    required TokenService tokenService,
    required AuthRepository authRepository,
    required Dio dio,
    this.onSessionExpired,
  })  : _tokenService = tokenService,
        _authRepository = authRepository,
        _dio = dio;

  @override
  void onRequest(RequestOptions options, RequestInterceptorHandler handler) {
    final token = _tokenService.getToken();
    if (token != null) {
      options.headers['Authorization'] = 'Bearer $token';
    }
    super.onRequest(options, handler);
  }

  @override
  Future<void> onError(DioException err, ErrorInterceptorHandler handler) async {
    if (err.type == DioExceptionType.cancel) {
      return super.onError(err, handler);
    }

      if (err.response?.statusCode == 401) {
        final path = err.requestOptions.path;
        if (path.contains('/api/auth/login') ||
            path.contains('/api/auth/refresh') ||
            path.contains('/api/auth/logout')) {
          return super.onError(err, handler);
        }

        final currentToken = _tokenService.getToken();
        final requestToken = err.requestOptions.headers['Authorization']
            ?.toString()
            .replaceAll('Bearer ', '');

        if (currentToken != null &&
            requestToken != null &&
            currentToken != requestToken) {
          return _retry(err.requestOptions, handler);
        }

        final refreshToken = _tokenService.getRefreshToken();
        if (refreshToken != null) {
          try {
            final response = await _authRepository.refreshToken(refreshToken);

            await _tokenService.saveToken(response.token);
            await _tokenService.saveRefreshToken(response.refreshToken);

            return _retry(err.requestOptions, handler);
          } catch (e) {
            await _tokenService.clearToken();
            onSessionExpired?.call();
          }
        } else {
          await _tokenService.clearToken();
          onSessionExpired?.call();
        }
      }

    super.onError(err, handler);
  }

  Future<void> _retry(RequestOptions requestOptions, ErrorInterceptorHandler handler) async {
    final newToken = _tokenService.getToken();
    final options = Options(
      method: requestOptions.method,
      headers: {
        ...requestOptions.headers,
        if (newToken != null) 'Authorization': 'Bearer $newToken',
      },
    );
    
    try {
      final response = await _dio.request(
        requestOptions.path,
        data: requestOptions.data,
        queryParameters: requestOptions.queryParameters,
        options: options,
      );
      handler.resolve(response);
    } catch (e) {
      if (e is DioException) {
        handler.next(e);
      } else {
         handler.reject(
           DioException(
             requestOptions: requestOptions,
             error: e,
             type: DioExceptionType.unknown,
           ),
         );
      }
    }
  }
}
