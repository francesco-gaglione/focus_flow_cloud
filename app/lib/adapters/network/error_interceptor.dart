import 'package:dio/dio.dart';
import 'package:focus_flow_app/core/errors/api_exception.dart';

/// Global Dio interceptor that converts raw [DioException]s into [ApiException]s
/// with a user-readable message, extracted from:
///   1. The JSON response body (fields `message`, `error`, or `msg`)
///   2. The HTTP status code
///   3. The Dio exception type (timeout, connection error, etc.)
///
/// 401 errors are intentionally forwarded unchanged so [AuthInterceptor] can
/// handle token refresh before this interceptor processes the error.
class ErrorInterceptor extends Interceptor {
  @override
  void onError(DioException err, ErrorInterceptorHandler handler) {
    // Already a clean exception — pass through.
    if (err is ApiException) {
      return handler.next(err);
    }

    // Let AuthInterceptor handle 401 (token refresh / session expiry).
    if (err.response?.statusCode == 401) {
      return handler.next(err);
    }

    // Cancelled requests don't need a user-facing message.
    if (err.type == DioExceptionType.cancel) {
      return handler.next(err);
    }

    final message = _parseMessage(err);
    handler.reject(
      ApiException(
        readableMessage: message,
        requestOptions: err.requestOptions,
        response: err.response,
        type: err.type,
      ),
    );
  }

  String _parseMessage(DioException err) {
    // 1. Try to extract a message from the response body.
    final data = err.response?.data;
    if (data is Map) {
      for (final key in ['message', 'error', 'msg', 'detail']) {
        final value = data[key];
        if (value is String && value.isNotEmpty) return value;
      }
    }

    // 2. Fall back to status-code based messages.
    switch (err.response?.statusCode) {
      case 400:
        return 'Invalid request. Please check your input.';
      case 403:
        return 'Access denied.';
      case 404:
        return 'Resource not found.';
      case 409:
        return 'Conflict — the resource already exists.';
      case 422:
        return 'Validation error. Please check your input.';
      case 500:
      case 502:
      case 503:
        return 'Server error. Please try again later.';
      default:
        break;
    }

    // 3. Fall back to Dio exception type messages.
    switch (err.type) {
      case DioExceptionType.connectionTimeout:
      case DioExceptionType.sendTimeout:
      case DioExceptionType.receiveTimeout:
        return 'Request timed out. Please check your connection.';
      case DioExceptionType.connectionError:
        return 'Cannot reach the server. Please check your connection.';
      default:
        return 'An unexpected error occurred.';
    }
  }
}
