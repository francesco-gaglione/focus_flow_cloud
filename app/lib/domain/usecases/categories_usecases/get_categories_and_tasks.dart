import 'package:focus_flow_app/domain/entities/category_with_tasks.dart';
import 'package:focus_flow_app/domain/repositories/category_repository.dart';
import 'package:logger/logger.dart';

class GetCategoriesAndTasks {
  Logger logger = Logger();

  final CategoryRepository categoryRepository;

  GetCategoriesAndTasks({required this.categoryRepository});

  Future<GetCategoriesAndTasksResult> execute() async {
    try {
      logger.i('Executing getAllCategories');
      final categoriesWithTasks = await categoryRepository.getAllCategories();
      return GetCategoriesAndTasksResult(
        success: true,
        categoriesWithTasks: categoriesWithTasks,
      );
    } catch (e) {
      return GetCategoriesAndTasksResult(success: false, error: e.toString());
    }
  }
}

class GetCategoriesAndTasksResult {
  final bool success;
  final List<CategoryWithTasks>? categoriesWithTasks;
  final String? error;

  GetCategoriesAndTasksResult({
    required this.success,
    this.categoriesWithTasks,
    this.error,
  });
}
