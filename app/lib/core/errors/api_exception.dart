import 'package:dio/dio.dart';

/// A [DioException] subclass whose [toString] returns only the human-readable
/// [readableMessage], making it safe to surface directly in the UI.
class ApiException extends DioException {
  final String readableMessage;

  ApiException({
    required this.readableMessage,
    required super.requestOptions,
    super.response,
    super.type = DioExceptionType.badResponse,
  }) : super(message: readableMessage);

  @override
  String toString() => readableMessage;
}
