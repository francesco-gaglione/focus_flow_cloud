import '../entities/statistics.dart';

abstract class StatisticsRepository {
  /// Calculate statistics for a given period
  Future<PeriodStatistics> calculateStatsByPeriod({
    required int startDate,
    int? endDate,
  });

  /// Get total focus time for a period
  Future<int> getTotalFocusTime(int startDate, int? endDate);

  /// Get total break time for a period
  Future<int> getTotalBreakTime(int startDate, int? endDate);

  /// Get total number of sessions for a period
  Future<int> getTotalSessions(int startDate, int? endDate);

  /// Get total number of breaks for a period
  Future<int> getTotalBreaks(int startDate, int? endDate);

  /// Get category distribution for a period
  Future<List<CategoryDistribution>> getCategoryDistribution(
    int startDate,
    int? endDate,
  );

  /// Get task distribution for a period
  Future<List<TaskDistribution>> getTaskDistribution(
    int startDate,
    int? endDate,
  );

  /// Get daily activity for a period
  Future<List<DailyActivity>> getDailyActivity(int startDate, int? endDate);

  /// Get concentration distribution (array of 5 elements for scores 1-5)
  Future<List<int>> getConcentrationDistribution(int startDate, int? endDate);

  /// Get most concentrated period (morning or afternoon)
  Future<ConcentrationPeriod> getMostConcentratedPeriod(
    int startDate,
    int? endDate,
  );

  /// Get less concentrated period (morning or afternoon)
  Future<ConcentrationPeriod> getLessConcentratedPeriod(
    int startDate,
    int? endDate,
  );
}
