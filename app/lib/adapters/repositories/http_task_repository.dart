import 'package:dio/dio.dart';
import 'package:flutter/foundation.dart';
import 'package:logger/logger.dart';
import '../../domain/entities/task.dart';
import '../../domain/repositories/task_repository.dart';
import '../dtos/task_dtos.dart';

Task _taskFromDto(TaskResponseDto dto) {
  return Task(
    id: dto.id,
    name: dto.name,
    description: dto.description,
    categoryId: dto.categoryId,
    scheduledDate: dto.scheduledDate,
    scheduledEndDate: dto.scheduledEndDate,
    completedAt: dto.completedAt,
    createdAt: DateTime.now(),
    updatedAt: DateTime.now(),
  );
}

class HttpTaskRepository implements TaskRepository {
  final Dio _dio;
  final String baseUrl;
  final Logger _logger;

  HttpTaskRepository({required Dio dio, this.baseUrl = 'http://localhost:3000'})
    : _dio = dio,
      _logger = Logger(
        printer: SimplePrinter(printTime: true),
        level: kDebugMode ? Level.debug : Level.warning,
      );

  @override
  Future<List<Task>> getAllTasks() async {
    try {
      if (kDebugMode) _logger.d('GET $baseUrl/api/task');

      final response = await _dio.get('$baseUrl/api/task');
      final List<dynamic> data = response.data['tasks'];

      if (kDebugMode) {
        _logger.d('Response ${response.statusCode}: ${data.length} tasks');
      }

      return data
          .map(
            (json) => Task(
              id: json['id'],
              name: json['name'],
              description: json['description'],
              categoryId: json['categoryId'],
              scheduledDate: json['scheduledDate'],
              scheduledEndDate: json['scheduledEndDate'],
              completedAt: json['completedAt'],
              createdAt: DateTime.now(),
              updatedAt: DateTime.now(),
            ),
          )
          .toList();
    } catch (e, stackTrace) {
      _logger.e('Failed getAllTasks', error: e, stackTrace: kDebugMode ? stackTrace : null);
      rethrow;
    }
  }

  @override
  Future<Task?> getTaskById(String id) async {
    try {
      final tasks = await getAllTasks();
      return tasks.where((t) => t.id == id).firstOrNull;
    } catch (e, stackTrace) {
      _logger.e('Failed getTaskById: $id', error: e, stackTrace: kDebugMode ? stackTrace : null);
      return null;
    }
  }

  @override
  Future<List<Task>> getOrphanTasks() async {
    try {
      if (kDebugMode) _logger.d('GET $baseUrl/api/task/orphans');

      final response = await _dio.get('$baseUrl/api/task/orphans');
      final dto = OrphanTasksResponseDto.fromJson(response.data);

      if (kDebugMode) {
        _logger.d('Response ${response.statusCode}: ${dto.orphanTasks.length} orphan tasks');
      }

      return dto.orphanTasks.map(_taskFromDto).toList();
    } catch (e, stackTrace) {
      _logger.e('Failed getOrphanTasks', error: e, stackTrace: kDebugMode ? stackTrace : null);
      rethrow;
    }
  }

  @override
  Future<List<Task>> getScheduledTasks({
    bool? completed,
    int? from,
    int? to,
  }) async {
    try {
      final queryParams = <String, dynamic>{};
      if (completed != null) queryParams['completed'] = completed;
      if (from != null) queryParams['from'] = from;
      if (to != null) queryParams['to'] = to;

      if (kDebugMode) _logger.d('GET $baseUrl/api/task/scheduled params: $queryParams');

      final response = await _dio.get(
        '$baseUrl/api/task/scheduled',
        queryParameters: queryParams.isNotEmpty ? queryParams : null,
      );
      final dto = ScheduledTasksResponseDto.fromJson(response.data);

      if (kDebugMode) {
        _logger.d('Response ${response.statusCode}: ${dto.tasks.length} scheduled tasks');
      }

      return dto.tasks.map(_taskFromDto).toList();
    } catch (e, stackTrace) {
      _logger.e('Failed getScheduledTasks', error: e, stackTrace: kDebugMode ? stackTrace : null);
      rethrow;
    }
  }

  @override
  Future<Task> createTask({
    required String name,
    String? description,
    String? categoryId,
    int? scheduledDate,
    int? scheduledEndDate,
  }) async {
    try {
      final dto = CreateTaskDto(
        name: name,
        description: description,
        categoryId: categoryId,
        scheduledDate: scheduledDate,
        scheduledEndDate: scheduledEndDate,
      );

      if (kDebugMode) _logger.d('POST $baseUrl/api/task - name: $name');

      final response = await _dio.post('$baseUrl/api/task', data: dto.toJson());
      final responseDto = CreateTaskResponseDto.fromJson(response.data);

      if (kDebugMode) {
        _logger.d('Response ${response.statusCode}: task created ${responseDto.id}');
      }

      return Task(
        id: responseDto.id,
        name: name,
        description: description,
        categoryId: categoryId,
        scheduledDate: scheduledDate,
        scheduledEndDate: scheduledEndDate,
        completedAt: null,
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
      );
    } catch (e, stackTrace) {
      _logger.e('Failed createTask: $name', error: e, stackTrace: kDebugMode ? stackTrace : null);
      rethrow;
    }
  }

  @override
  Future<Task> updateTask({
    required String id,
    String? name,
    String? description,
    String? categoryId,
    int? scheduledDate,
    int? scheduledEndDate,
    int? completedAt,
  }) async {
    try {
      final dto = UpdateTaskDto(
        name: name,
        description: description,
        categoryId: categoryId,
        scheduledDate: scheduledDate,
        scheduledEndDate: scheduledEndDate,
        completedAt: completedAt,
      );

      if (kDebugMode) _logger.d('PUT $baseUrl/api/task/$id');

      final response = await _dio.put('$baseUrl/api/task/$id', data: dto.toJson());

      if (kDebugMode) _logger.d('Response ${response.statusCode}: task updated');

      // API returns {success: bool}, reconstruct task from params
      return Task(
        id: id,
        name: name ?? '',
        description: description,
        categoryId: categoryId,
        scheduledDate: scheduledDate,
        scheduledEndDate: scheduledEndDate,
        completedAt: completedAt,
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
      );
    } catch (e, stackTrace) {
      _logger.e('Failed updateTask: $id', error: e, stackTrace: kDebugMode ? stackTrace : null);
      rethrow;
    }
  }

  @override
  Future<List<String>> deleteTasks(List<String> taskIds) async {
    try {
      final dto = DeleteTasksDto(taskIds: taskIds);

      if (kDebugMode) _logger.d('DELETE $baseUrl/api/task - count: ${taskIds.length}');

      final response = await _dio.delete('$baseUrl/api/task', data: dto.toJson());
      final responseDto = DeleteTasksResponseDto.fromJson(response.data);

      if (kDebugMode) {
        _logger.d('Response ${response.statusCode}: deleted ${responseDto.deletedIds.length} tasks');
      }

      return responseDto.deletedIds;
    } catch (e, stackTrace) {
      _logger.e('Failed deleteTasks', error: e, stackTrace: kDebugMode ? stackTrace : null);
      rethrow;
    }
  }

  @override
  Future<bool> taskExistsByName(String name) async {
    try {
      final tasks = await getAllTasks();
      return tasks.any((task) => task.name.toLowerCase() == name.toLowerCase());
    } catch (e, stackTrace) {
      _logger.e('Failed taskExistsByName: $name', error: e, stackTrace: kDebugMode ? stackTrace : null);
      rethrow;
    }
  }

  @override
  Future<bool> taskExists(String id) async {
    try {
      final task = await getTaskById(id);
      return task != null;
    } catch (e, stackTrace) {
      _logger.e('Failed taskExists: $id', error: e, stackTrace: kDebugMode ? stackTrace : null);
      rethrow;
    }
  }

  @override
  Future<List<Task>> getTasksByIds(List<String> taskIds) async {
    try {
      if (kDebugMode) _logger.d('getTasksByIds count: ${taskIds.length}');

      final allTasks = await getAllTasks();
      return allTasks.where((task) => taskIds.contains(task.id)).toList();
    } catch (e, stackTrace) {
      _logger.e('Failed getTasksByIds', error: e, stackTrace: kDebugMode ? stackTrace : null);
      rethrow;
    }
  }
}
