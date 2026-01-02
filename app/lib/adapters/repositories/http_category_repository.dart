import 'package:dio/dio.dart';
import 'package:focus_flow_app/domain/entities/category_with_tasks.dart';
import 'package:logger/web.dart';
import '../../domain/entities/category.dart';
import '../../domain/entities/task.dart';
import '../../domain/repositories/category_repository.dart';
import '../dtos/category_dtos.dart';

class HttpCategoryRepository implements CategoryRepository {
  final Logger logger = Logger();
  final Dio _dio;
  final String baseUrl;

  HttpCategoryRepository({
    required Dio dio,
    this.baseUrl = 'http://localhost:3000',
  }) : _dio = dio;

  @override
  Future<List<CategoryWithTasks>> getAllCategories() async {
    final response = await _dio.get('$baseUrl/api/category');
    final dto = GetCategoriesResponseDto.fromJson(response.data);
    return dto.categories.map((catDto) {
      final category = Category(
        id: catDto.id,
        name: catDto.name,
        color: catDto.color,
        description: catDto.description,
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
      );
      final tasks =
          catDto.tasks.map((taskDto) {
            return Task(
              id: taskDto.id,
              name: taskDto.name,
              description: taskDto.description,
              categoryId: taskDto.categoryId,
              scheduledDate: taskDto.scheduledDate,
              completedAt: taskDto.completedAt,
              createdAt: DateTime.now(),
              updatedAt: DateTime.now(),
            );
          }).toList();
      return CategoryWithTasks(category: category, tasks: tasks);
    }).toList();
  }

  @override
  Future<Category?> getCategoryById(String id) async {
    try {
      logger.d('Fetching category with ID: $id');
      final response = await _dio.get('$baseUrl/api/category/$id');
      logger.d('Fetched category with ID: $id');
      final cat = CategoryDto.fromJson(response.data);
      logger.d('Parsed category with ID: $id');
      return Category(
        id: cat.id,
        name: cat.name,
        color: cat.color,
        description: cat.description,
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
      );
    } on DioException catch (e) {
      if (e.response?.statusCode == 404) return null;
      rethrow;
    }
  }

  @override
  Future<Category> createCategory({
    required String name,
    required String color,
    String? description,
  }) async {
    final dto = CreateCategoryDto(
      name: name,
      color: color,
      description: description,
    );
    await _dio.post('$baseUrl/api/category', data: dto.toJson());
    // Return created category (assuming API returns it or generate ID)
    return Category(
      id: DateTime.now().millisecondsSinceEpoch.toString(),
      name: name,
      color: color,
      description: description,
      createdAt: DateTime.now(),
      updatedAt: DateTime.now(),
    );
  }

  @override
  Future<Category> updateCategory({
    required String id,
    String? name,
    String? color,
    String? description,
  }) async {
    final dto = UpdateCategoryDto(
      name: name,
      color: color,
      description: description,
    );
    final response = await _dio.put(
      '$baseUrl/api/category/$id',
      data: dto.toJson(),
    );
    final updated = UpdateCategoryResponseDto.fromJson(response.data);
    return Category(
      id: updated.updatedCategory.id,
      name: updated.updatedCategory.name,
      color: updated.updatedCategory.color,
      description: updated.updatedCategory.description,
      createdAt: DateTime.now(),
      updatedAt: DateTime.now(),
    );
  }

  @override
  Future<bool> deleteCategory(String id) async {
    try {
      await _dio.delete('$baseUrl/api/category/$id');
      return true;
    } catch (e) {
      return false;
    }
  }

  @override
  Future<bool> categoryExistsByName(String name) async {
    final categoriesWithTasks = await getAllCategories();
    return categoriesWithTasks.any(
      (cat) => cat.category.name.toLowerCase() == name.toLowerCase(),
    );
  }

  @override
  Future<bool> categoryExists(String id) async {
    logger.d('Checking if category exists with ID: $id');
    final category = await getCategoryById(id);
    logger.d('Category found: ${category != null}');
    return category != null;
  }
}
