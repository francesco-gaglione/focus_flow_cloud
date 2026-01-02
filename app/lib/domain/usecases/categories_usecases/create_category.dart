import 'package:focus_flow_app/domain/entities/category.dart';
import 'package:focus_flow_app/domain/repositories/category_repository.dart';

class CreateCategory {
  final CategoryRepository categoryRepository;

  CreateCategory({required this.categoryRepository});

  Future<CreateCategoryResult> execute({
    required String name,
    String? color,
    String? description,
  }) async {
    try {
      // Validate inputs
      if (name.trim().isEmpty) {
        return CreateCategoryResult(
          success: false,
          error: 'Category name cannot be empty',
          errorType: CreateCategoryErrorType.validation,
        );
      }

      // Check if category already exists
      final exists = await categoryRepository.categoryExistsByName(name);
      if (exists) {
        return CreateCategoryResult(
          success: false,
          error: 'Category with this name already exists',
          errorType: CreateCategoryErrorType.conflict,
        );
      }

      // Set default color if not provided
      final categoryColor = color ?? '#6200EE';

      // Create category
      final category = await categoryRepository.createCategory(
        name: name,
        color: categoryColor,
        description: description,
      );

      return CreateCategoryResult(success: true, category: category);
    } catch (e) {
      return CreateCategoryResult(
        success: false,
        error: e.toString(),
        errorType: CreateCategoryErrorType.internal,
      );
    }
  }
}

enum CreateCategoryErrorType { validation, conflict, internal }

class CreateCategoryResult {
  final bool success;
  final Category? category;
  final String? error;
  final CreateCategoryErrorType? errorType;

  CreateCategoryResult({
    required this.success,
    this.category,
    this.error,
    this.errorType,
  });
}
