// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'statistics_dtos.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_$GetStatsByPeriodDtoImpl _$$GetStatsByPeriodDtoImplFromJson(
  Map<String, dynamic> json,
) => _$GetStatsByPeriodDtoImpl(
  startDate: (json['startDate'] as num).toInt(),
  endDate: (json['endDate'] as num?)?.toInt(),
);

Map<String, dynamic> _$$GetStatsByPeriodDtoImplToJson(
  _$GetStatsByPeriodDtoImpl instance,
) => <String, dynamic>{
  'startDate': instance.startDate,
  'endDate': instance.endDate,
};

_$CategoryDistributionDtoImpl _$$CategoryDistributionDtoImplFromJson(
  Map<String, dynamic> json,
) => _$CategoryDistributionDtoImpl(
  categoryId: json['categoryId'] as String,
  categoryName: json['categoryName'] as String,
  totalFocusTime: (json['totalFocusTime'] as num).toInt(),
  percentage: (json['percentage'] as num).toDouble(),
  taskDistribution:
      (json['taskDistribution'] as List<dynamic>)
          .map((e) => TaskDistributionDto.fromJson(e as Map<String, dynamic>))
          .toList(),
);

Map<String, dynamic> _$$CategoryDistributionDtoImplToJson(
  _$CategoryDistributionDtoImpl instance,
) => <String, dynamic>{
  'categoryId': instance.categoryId,
  'categoryName': instance.categoryName,
  'totalFocusTime': instance.totalFocusTime,
  'percentage': instance.percentage,
  'taskDistribution': instance.taskDistribution,
};

_$TaskDistributionDtoImpl _$$TaskDistributionDtoImplFromJson(
  Map<String, dynamic> json,
) => _$TaskDistributionDtoImpl(
  taskName: json['taskName'] as String,
  totalFocusTime: (json['totalFocusTime'] as num).toInt(),
  percentage: (json['percentage'] as num).toDouble(),
);

Map<String, dynamic> _$$TaskDistributionDtoImplToJson(
  _$TaskDistributionDtoImpl instance,
) => <String, dynamic>{
  'taskName': instance.taskName,
  'totalFocusTime': instance.totalFocusTime,
  'percentage': instance.percentage,
};

_$DailyActivityDistributionDtoImpl _$$DailyActivityDistributionDtoImplFromJson(
  Map<String, dynamic> json,
) => _$DailyActivityDistributionDtoImpl(
  categoryId: json['categoryId'] as String,
  categoryName: json['categoryName'] as String,
  totalFocusTime: (json['totalFocusTime'] as num).toInt(),
);

Map<String, dynamic> _$$DailyActivityDistributionDtoImplToJson(
  _$DailyActivityDistributionDtoImpl instance,
) => <String, dynamic>{
  'categoryId': instance.categoryId,
  'categoryName': instance.categoryName,
  'totalFocusTime': instance.totalFocusTime,
};

_$DailyActivityDtoImpl _$$DailyActivityDtoImplFromJson(
  Map<String, dynamic> json,
) => _$DailyActivityDtoImpl(
  date: (json['date'] as num).toInt(),
  categoryDistribution:
      (json['categoryDistribution'] as List<dynamic>)
          .map(
            (e) => DailyActivityDistributionDto.fromJson(
              e as Map<String, dynamic>,
            ),
          )
          .toList(),
);

Map<String, dynamic> _$$DailyActivityDtoImplToJson(
  _$DailyActivityDtoImpl instance,
) => <String, dynamic>{
  'date': instance.date,
  'categoryDistribution': instance.categoryDistribution,
};

_$GetStatsByPeriodResponseDtoImpl _$$GetStatsByPeriodResponseDtoImplFromJson(
  Map<String, dynamic> json,
) => _$GetStatsByPeriodResponseDtoImpl(
  totalSessions: (json['totalSessions'] as num).toInt(),
  totalBreaks: (json['totalBreaks'] as num).toInt(),
  totalFocusTime: (json['totalFocusTime'] as num).toInt(),
  totalBreakTime: (json['totalBreakTime'] as num).toInt(),
  mostConcentratedPeriod: json['mostConcentratedPeriod'] as String,
  lessConcentratedPeriod: json['lessConcentratedPeriod'] as String,
  concentrationDistribution:
      (json['concentrationDistribution'] as List<dynamic>)
          .map((e) => (e as num).toInt())
          .toList(),
  categoryDistribution:
      (json['categoryDistribution'] as List<dynamic>)
          .map(
            (e) => CategoryDistributionDto.fromJson(e as Map<String, dynamic>),
          )
          .toList(),
  dailyActivity:
      (json['dailyActivity'] as List<dynamic>)
          .map((e) => DailyActivityDto.fromJson(e as Map<String, dynamic>))
          .toList(),
);

Map<String, dynamic> _$$GetStatsByPeriodResponseDtoImplToJson(
  _$GetStatsByPeriodResponseDtoImpl instance,
) => <String, dynamic>{
  'totalSessions': instance.totalSessions,
  'totalBreaks': instance.totalBreaks,
  'totalFocusTime': instance.totalFocusTime,
  'totalBreakTime': instance.totalBreakTime,
  'mostConcentratedPeriod': instance.mostConcentratedPeriod,
  'lessConcentratedPeriod': instance.lessConcentratedPeriod,
  'concentrationDistribution': instance.concentrationDistribution,
  'categoryDistribution': instance.categoryDistribution,
  'dailyActivity': instance.dailyActivity,
};
