import 'package:focus_flow_app/domain/repositories/category_repository.dart';
import 'package:logger/web.dart';

class DeleteCategory {
  final Logger logger = Logger();
  final CategoryRepository categoryRepository;

  DeleteCategory({required this.categoryRepository});

  Future<DeleteCategoryResult> execute({required String id}) async {
    try {
      final exists = await categoryRepository.categoryExists(id);

      if (!exists) {
        logger.w('Category not found');
        return DeleteCategoryResult(
          success: false,
          error: 'Category not found',
          errorType: DeleteCategoryErrorType.notFound,
        );
      }

      logger.i('Deleting category with id $id');
      final deleted = await categoryRepository.deleteCategory(id);

      if (!deleted) {
        logger.e('Failed to delete category with id $id');
        return DeleteCategoryResult(
          success: false,
          error: 'Failed to delete category',
          errorType: DeleteCategoryErrorType.internal,
        );
      }

      return DeleteCategoryResult(success: true, deletedIds: [id]);
    } catch (e) {
      return DeleteCategoryResult(
        success: false,
        error: e.toString(),
        errorType: DeleteCategoryErrorType.internal,
      );
    }
  }
}

enum DeleteCategoryErrorType { notFound, internal }

class DeleteCategoryResult {
  final bool success;
  final List<String>? deletedIds;
  final String? error;
  final DeleteCategoryErrorType? errorType;

  DeleteCategoryResult({
    required this.success,
    this.deletedIds,
    this.error,
    this.errorType,
  });
}
