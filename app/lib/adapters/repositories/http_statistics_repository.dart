import 'package:dio/dio.dart';
import '../../domain/entities/statistics.dart';
import '../../domain/repositories/statistics_repository.dart';
import '../dtos/statistics_dtos.dart';

class HttpStatisticsRepository implements StatisticsRepository {
  final Dio _dio;
  final String baseUrl;

  HttpStatisticsRepository({
    required Dio dio,
    this.baseUrl = 'http://localhost:3000',
  }) : _dio = dio;

  @override
  Future<PeriodStatistics> calculateStatsByPeriod({
    required int startDate,
    int? endDate,
  }) async {
    final queryParams = <String, dynamic>{'startDate': startDate};
    if (endDate != null) queryParams['endDate'] = endDate;

    final response = await _dio.get(
      '$baseUrl/api/stats/period',
      queryParameters: queryParams,
    );

    final dto = GetStatsByPeriodResponseDto.fromJson(response.data);

    return PeriodStatistics(
      totalSessions: dto.totalSessions,
      totalBreaks: dto.totalBreaks,
      totalFocusTime: dto.totalFocusTime,
      totalBreakTime: dto.totalBreakTime,
      mostConcentratedPeriod: ConcentrationPeriod.fromString(
        dto.mostConcentratedPeriod,
      ),
      lessConcentratedPeriod: ConcentrationPeriod.fromString(
        dto.lessConcentratedPeriod,
      ),
      concentrationDistribution: dto.concentrationDistribution,
      categoryDistribution:
          dto.categoryDistribution
              .map(
                (dist) => CategoryDistribution(
                  categoryId: dist.categoryId,
                  categoryName: dist.categoryName,
                  totalFocusTime: dist.totalFocusTime,
                  percentage: dist.percentage,
                  taskDistribution:
                      dist.taskDistribution
                          .map(
                            (taskDist) => TaskDistribution(
                              taskName: taskDist.taskName,
                              totalFocusTime: taskDist.totalFocusTime,
                              percentage: taskDist.percentage,
                            ),
                          )
                          .toList(),
                ),
              )
              .toList(),
      dailyActivity:
          dto.dailyActivity
              .map(
                (activity) => DailyActivity(
                  date: activity.date,
                  categoryDistribution:
                      activity.categoryDistribution
                          .map(
                            (dist) => DailyActivityDistribution(
                              categoryId: dist.categoryId,
                              categoryName: dist.categoryName,
                              totalFocusTime: dist.totalFocusTime,
                            ),
                          )
                          .toList(),
                ),
              )
              .toList(),
    );
  }

  @override
  Future<int> getTotalFocusTime(int startDate, int? endDate) async {
    final stats = await calculateStatsByPeriod(
      startDate: startDate,
      endDate: endDate,
    );
    return stats.totalFocusTime;
  }

  @override
  Future<int> getTotalBreakTime(int startDate, int? endDate) async {
    final stats = await calculateStatsByPeriod(
      startDate: startDate,
      endDate: endDate,
    );
    return stats.totalBreakTime;
  }

  @override
  Future<int> getTotalSessions(int startDate, int? endDate) async {
    final stats = await calculateStatsByPeriod(
      startDate: startDate,
      endDate: endDate,
    );
    return stats.totalSessions;
  }

  @override
  Future<int> getTotalBreaks(int startDate, int? endDate) async {
    final stats = await calculateStatsByPeriod(
      startDate: startDate,
      endDate: endDate,
    );
    return stats.totalBreaks;
  }

  @override
  Future<List<CategoryDistribution>> getCategoryDistribution(
    int startDate,
    int? endDate,
  ) async {
    final stats = await calculateStatsByPeriod(
      startDate: startDate,
      endDate: endDate,
    );
    return stats.categoryDistribution;
  }

  @override
  Future<List<DailyActivity>> getDailyActivity(
    int startDate,
    int? endDate,
  ) async {
    final stats = await calculateStatsByPeriod(
      startDate: startDate,
      endDate: endDate,
    );
    return stats.dailyActivity;
  }

  @override
  Future<List<int>> getConcentrationDistribution(
    int startDate,
    int? endDate,
  ) async {
    final stats = await calculateStatsByPeriod(
      startDate: startDate,
      endDate: endDate,
    );
    return stats.concentrationDistribution;
  }

  @override
  Future<ConcentrationPeriod> getMostConcentratedPeriod(
    int startDate,
    int? endDate,
  ) async {
    final stats = await calculateStatsByPeriod(
      startDate: startDate,
      endDate: endDate,
    );
    return stats.mostConcentratedPeriod;
  }

  @override
  Future<ConcentrationPeriod> getLessConcentratedPeriod(
    int startDate,
    int? endDate,
  ) async {
    final stats = await calculateStatsByPeriod(
      startDate: startDate,
      endDate: endDate,
    );
    return stats.lessConcentratedPeriod;
  }
}
