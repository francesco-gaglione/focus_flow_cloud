import '../entities/statistics.dart';
import '../repositories/statistics_repository.dart';

class CalculateStatsByPeriod {
  final StatisticsRepository statisticsRepository;

  CalculateStatsByPeriod({required this.statisticsRepository});

  Future<CalculateStatsByPeriodResult> execute({
    required int startDate,
    int? endDate,
  }) async {
    try {
      // Validate date range if endDate is provided
      if (endDate != null && startDate > endDate) {
        return CalculateStatsByPeriodResult(
          success: false,
          error: 'Start date cannot be after end date',
          errorType: CalculateStatsByPeriodErrorType.validation,
        );
      }

      // Validate that startDate is not in the future
      final now = DateTime.now().millisecondsSinceEpoch;
      if (startDate > now) {
        return CalculateStatsByPeriodResult(
          success: false,
          error: 'Start date cannot be in the future',
          errorType: CalculateStatsByPeriodErrorType.validation,
        );
      }

      // Calculate statistics for the period
      final statistics = await statisticsRepository.calculateStatsByPeriod(
        startDate: startDate,
        endDate: endDate,
      );

      return CalculateStatsByPeriodResult(
        success: true,
        statistics: statistics,
      );
    } catch (e) {
      return CalculateStatsByPeriodResult(
        success: false,
        error: e.toString(),
        errorType: CalculateStatsByPeriodErrorType.internal,
      );
    }
  }
}

enum CalculateStatsByPeriodErrorType { validation, internal }

class CalculateStatsByPeriodResult {
  final bool success;
  final PeriodStatistics? statistics;
  final String? error;
  final CalculateStatsByPeriodErrorType? errorType;

  CalculateStatsByPeriodResult({
    required this.success,
    this.statistics,
    this.error,
    this.errorType,
  });
}
