import 'package:easy_localization/easy_localization.dart';
import 'package:fl_chart/fl_chart.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:focus_flow_app/domain/entities/statistics.dart';
import 'package:focus_flow_app/presentation/statistics/bloc/statistics_bloc.dart';
import 'package:focus_flow_app/presentation/statistics/bloc/statistics_event.dart';
import 'package:focus_flow_app/presentation/statistics/bloc/statistics_state.dart';
import 'package:logger/logger.dart';

class StatisticsView extends StatefulWidget {
  const StatisticsView({super.key});

  @override
  State<StatisticsView> createState() => _StatisticsViewState();
}

class _StatisticsViewState extends State<StatisticsView> {
  Logger logger = Logger();

  @override
  void initState() {
    super.initState();
    context.read<StatisticsBloc>().add(
      const ChangeTimeRange(StatisticsTimeRange.day),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: BlocBuilder<StatisticsBloc, StatisticsState>(
        builder: (context, state) {
          if (state is StatisticsLoading) {
            return const Center(child: CircularProgressIndicator());
          }

          if (state is StatisticsError) {
            return Center(child: Text(state.message));
          }

          if (state is StatisticsLoaded) {
            return CustomScrollView(
              slivers: [
                SliverAppBar.large(
                  title: Text(context.tr('statistics.title')),
                  centerTitle: false,
                ),
                SliverToBoxAdapter(
                  child: Padding(
                    padding: const EdgeInsets.all(16.0),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        _buildTimeRangeSelector(context, state.timeRange),
                        const SizedBox(height: 24),
                        _buildSummaryCards(context, state.statistics),
                        const SizedBox(height: 32),
                        _buildActivitySection(
                          context,
                          state.statistics,
                          state.categoryColors,
                          state.timeRange,
                        ),
                        const SizedBox(height: 32),
                        _buildSectionTitle(
                          context,
                          context.tr('statistics.categories'),
                        ),
                        const SizedBox(height: 16),
                        _buildCategoryDistributionChart(
                          context,
                          state.statistics,
                          state.categoryColors,
                        ),
                        const SizedBox(height: 32),
                        _buildSectionTitle(
                          context,
                          context.tr('statistics.concentration_score'),
                        ),
                        const SizedBox(height: 16),
                        _buildConcentrationChart(context, state.statistics),
                        const SizedBox(height: 32),
                      ],
                    ),
                  ),
                ),
              ],
            );
          }

          return const SizedBox.shrink();
        },
      ),
    );
  }

  Widget _buildSectionTitle(BuildContext context, String title) {
    return Text(
      title,
      style: Theme.of(
        context,
      ).textTheme.titleLarge?.copyWith(fontWeight: FontWeight.bold),
    );
  }

  String _formatDuration(Duration duration) {
    if (duration.inMinutes <= 60) {
      return '${duration.inMinutes}m';
    } else {
      final hours = duration.inHours;
      final minutes = duration.inMinutes.remainder(60);
      return '${hours}h ${minutes}m';
    }
  }

  Widget _buildTimeRangeSelector(
    BuildContext context,
    StatisticsTimeRange currentRange,
  ) {
    return SingleChildScrollView(
      scrollDirection: Axis.horizontal,
      child: Row(
        children: [
          _buildTimeRangeChip(
            context,
            StatisticsTimeRange.day,
            context.tr('statistics.day'),
            currentRange,
          ),
          const SizedBox(width: 8),
          _buildTimeRangeChip(
            context,
            StatisticsTimeRange.yesterday,
            context.tr('statistics.yesterday'),
            currentRange,
          ),
          const SizedBox(width: 8),
          _buildTimeRangeChip(
            context,
            StatisticsTimeRange.week,
            context.tr('statistics.week'),
            currentRange,
          ),
          const SizedBox(width: 8),
          _buildTimeRangeChip(
            context,
            StatisticsTimeRange.lastWeek,
            context.tr('statistics.last_week'),
            currentRange,
          ),
          const SizedBox(width: 8),
          _buildTimeRangeChip(
            context,
            StatisticsTimeRange.last7Days,
            context.tr('statistics.last_7_days'),
            currentRange,
          ),
          const SizedBox(width: 8),
          _buildTimeRangeChip(
            context,
            StatisticsTimeRange.month,
            context.tr('statistics.month'),
            currentRange,
          ),
          const SizedBox(width: 8),
          _buildTimeRangeChip(
            context,
            StatisticsTimeRange.last30Days,
            context.tr('statistics.last_30_days'),
            currentRange,
          ),
        ],
      ),
    );
  }

  Widget _buildTimeRangeChip(
    BuildContext context,
    StatisticsTimeRange range,
    String label,
    StatisticsTimeRange currentRange,
  ) {
    final isSelected = range == currentRange;
    return ChoiceChip(
      label: Text(label),
      selected: isSelected,
      onSelected: (bool selected) {
        if (selected) {
          context.read<StatisticsBloc>().add(ChangeTimeRange(range));
        }
      },
      showCheckmark: false,
      labelStyle: TextStyle(
        color:
            isSelected
                ? Theme.of(context).colorScheme.onPrimary
                : Theme.of(context).colorScheme.onSurface,
        fontWeight: isSelected ? FontWeight.bold : FontWeight.normal,
      ),
      selectedColor: Theme.of(context).colorScheme.primary,
      backgroundColor: Theme.of(
        context,
      ).colorScheme.surfaceContainerHighest.withAlpha((255 * 0.3).round()),
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(20),
        side: BorderSide(
          color:
              isSelected
                  ? Colors.transparent
                  : Theme.of(
                    context,
                  ).colorScheme.outlineVariant.withAlpha((255 * 0.5).round()),
        ),
      ),
    );
  }

  Widget _buildSummaryCards(BuildContext context, PeriodStatistics stats) {
    final duration = Duration(seconds: stats.totalFocusTime);
    final breakDuration = Duration(seconds: stats.totalBreakTime);
    final avgSession =
        stats.totalSessions > 0
            ? Duration(seconds: stats.totalFocusTime ~/ stats.totalSessions)
            : Duration.zero;

    final items = [
      _SummaryItem(
        title: context.tr('statistics.focus_time'),
        value: _formatDuration(duration),
        icon: Icons.timer_outlined,
        color: Colors.blue,
      ),
      _SummaryItem(
        title: context.tr('statistics.sessions'),
        value: stats.totalSessions.toString(),
        icon: Icons.check_circle_outline,
        color: Colors.green,
      ),
      _SummaryItem(
        title: context.tr('statistics.break_time'),
        value: _formatDuration(breakDuration),
        icon: Icons.coffee_outlined,
        color: Colors.orange,
      ),
      _SummaryItem(
        title: context.tr('statistics.average_session'),
        value: _formatDuration(avgSession),
        icon: Icons.timelapse_outlined,
        color: Colors.purple,
      ),
      _SummaryItem(
        title: context.tr('statistics.most_productive'),
        value:
            stats.mostConcentratedPeriod == ConcentrationPeriod.morning
                ? context.tr('statistics.morning')
                : context.tr('statistics.afternoon'),
        icon: Icons.wb_sunny_outlined,
        color: Colors.amber,
      ),
      _SummaryItem(
        title: context.tr('statistics.least_productive'),
        value:
            stats.lessConcentratedPeriod == ConcentrationPeriod.morning
                ? context.tr('statistics.morning')
                : context.tr('statistics.afternoon'),
        icon: Icons.nightlight_outlined,
        color: Colors.indigo,
      ),
    ];

    return LayoutBuilder(
      builder: (context, constraints) {
        final isDesktop = constraints.maxWidth > 600;

        if (isDesktop) {
          int crossAxisCount = 3;
          if (constraints.maxWidth > 1100) {
            crossAxisCount = 5;
          } else if (constraints.maxWidth > 800) {
            crossAxisCount = 4;
          }

          return GridView.builder(
            shrinkWrap: true,
            physics: const NeverScrollableScrollPhysics(),
            gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
              crossAxisCount: crossAxisCount,
              crossAxisSpacing: 12,
              mainAxisSpacing: 12,
              childAspectRatio: 2.8,
            ),
            itemCount: items.length,
            itemBuilder: (context, index) {
              return _buildSummaryCard(context, items[index]);
            },
          );
        }

        return SizedBox(
          height: 80,
          child: ListView.separated(
            scrollDirection: Axis.horizontal,
            itemCount: items.length,
            separatorBuilder: (context, index) => const SizedBox(width: 12),
            itemBuilder: (context, index) {
              return SizedBox(
                width: 150,
                child: _buildSummaryCard(context, items[index]),
              );
            },
          ),
        );
      },
    );
  }

  Widget _buildSummaryCard(BuildContext context, _SummaryItem item) {
    final colorScheme = Theme.of(context).colorScheme;
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
      decoration: BoxDecoration(
        color: colorScheme.surfaceContainerHighest.withAlpha(
          (255 * 0.5).round(),
        ), // Glass-like opacity
        borderRadius: BorderRadius.circular(16),
        border: Border.all(
          color: colorScheme.outlineVariant.withAlpha((255 * 0.2).round()),
        ),
      ),
      child: Row(
        children: [
          Container(
            padding: const EdgeInsets.all(8),
            decoration: BoxDecoration(
              color: item.color.withAlpha((255 * 0.1).round()),
              shape: BoxShape.circle,
            ),
            child: Icon(item.icon, color: item.color, size: 20),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Text(
                  item.value,
                  style: Theme.of(context).textTheme.titleMedium?.copyWith(
                    fontWeight: FontWeight.bold,
                  ),
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                ),
                Text(
                  item.title,
                  style: Theme.of(context).textTheme.bodySmall?.copyWith(
                    color: colorScheme.onSurfaceVariant,
                    fontSize: 10,
                  ),
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildNoDataWidget(
    BuildContext context,
    String message,
    IconData icon,
  ) {
    return Container(
      padding: const EdgeInsets.all(24),
      alignment: Alignment.center,
      decoration: BoxDecoration(
        color: Theme.of(
          context,
        ).colorScheme.surfaceContainerHighest.withAlpha((255 * 0.3).round()),
        borderRadius: BorderRadius.circular(24),
      ),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(
            icon,
            size: 48,
            color: Theme.of(
              context,
            ).colorScheme.outline.withAlpha((255 * 0.5).round()),
          ),
          const SizedBox(height: 16),
          Text(
            message,
            textAlign: TextAlign.center,
            style: Theme.of(context).textTheme.bodyLarge?.copyWith(
              color: Theme.of(context).colorScheme.onSurfaceVariant,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildActivitySection(
    BuildContext context,
    PeriodStatistics stats,
    Map<String, Color> categoryColors,
    StatisticsTimeRange timeRange,
  ) {
    if (stats.dailyActivity.isEmpty) {
      return const SizedBox.shrink();
    }

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        _buildSectionTitle(context, context.tr('statistics.activity')),
        const SizedBox(height: 16),
        _buildActivityChart(context, stats, categoryColors, timeRange),
      ],
    );
  }

  Widget _buildActivityChart(
    BuildContext context,
    PeriodStatistics stats,
    Map<String, Color> categoryColors,
    StatisticsTimeRange timeRange,
  ) {
    final dailyActivity = _fillMissingDates(stats.dailyActivity, timeRange);

    if (stats.dailyActivity.isEmpty) {
      return SizedBox(
        height: 240,
        child: _buildNoDataWidget(
          context,
          context.tr('statistics.no_activity_data'),
          Icons.bar_chart_outlined,
        ),
      );
    }

    final yAxisParams = _calculateYAxisParams(dailyActivity);

    return Container(
      height: 240,
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: Theme.of(
          context,
        ).colorScheme.surfaceContainerHighest.withAlpha((255 * 0.3).round()),
        borderRadius: BorderRadius.circular(24),
      ),
      child: BarChart(
        BarChartData(
          alignment: BarChartAlignment.spaceAround,
          maxY: yAxisParams.maxY,
          barTouchData: BarTouchData(
            touchTooltipData: BarTouchTooltipData(
              getTooltipColor:
                  (group) =>
                      Theme.of(context).colorScheme.surfaceContainerHighest,
              getTooltipItem: (group, groupIndex, rod, rodIndex) {
                if (groupIndex >= dailyActivity.length) return null;
                final activity = dailyActivity[groupIndex];
                final date = DateTime.fromMillisecondsSinceEpoch(
                  activity.date * 1000,
                );

                final textStyles = Theme.of(context).textTheme;

                return BarTooltipItem(
                  DateFormat('EEE, d MMM').format(date),
                  textStyles.titleSmall!.copyWith(fontWeight: FontWeight.bold),
                  children: [
                    const TextSpan(text: '\n'),
                    ...activity.categoryDistribution.map((dist) {
                      final duration = Duration(seconds: dist.totalFocusTime);
                      return TextSpan(
                        text:
                            '\n${dist.categoryName}: ${_formatDuration(duration)}',
                        style: textStyles.bodySmall,
                      );
                    }),
                    if (activity.categoryDistribution.isEmpty)
                      TextSpan(
                        text: '\n${context.tr('statistics.no_activity')}',
                        style: textStyles.bodySmall,
                      ),
                  ],
                );
              },
            ),
          ),
          titlesData: FlTitlesData(
            show: true,
            bottomTitles: AxisTitles(
              sideTitles: SideTitles(
                showTitles: true,
                getTitlesWidget: (double value, TitleMeta meta) {
                  if (value.toInt() >= 0 &&
                      value.toInt() < dailyActivity.length) {
                    final date = DateTime.fromMillisecondsSinceEpoch(
                      dailyActivity[value.toInt()].date * 1000,
                    );

                    if (timeRange == StatisticsTimeRange.month ||
                        timeRange == StatisticsTimeRange.last30Days) {
                      final isMobile = MediaQuery.of(context).size.width < 900;
                      if (isMobile && date.day != 1 && date.day % 5 != 0) {
                        return const SizedBox.shrink();
                      }
                      return Padding(
                        padding: const EdgeInsets.only(top: 8.0),
                        child: Text(
                          '${date.day}',
                          style: const TextStyle(
                            fontSize: 10,
                            fontWeight: FontWeight.w500,
                          ),
                        ),
                      );
                    }

                    return Padding(
                      padding: const EdgeInsets.only(top: 8.0),
                      child: Text(
                        DateFormat('E').format(date),
                        style: const TextStyle(
                          fontSize: 10,
                          fontWeight: FontWeight.w500,
                        ),
                      ),
                    );
                  }
                  return const SizedBox.shrink();
                },
              ),
            ),
            leftTitles: AxisTitles(
              sideTitles: SideTitles(
                showTitles: true,
                reservedSize: 40,
                interval: yAxisParams.interval,
                getTitlesWidget: (double value, TitleMeta meta) {
                  if (value == 0) return const SizedBox.shrink();

                  String text;
                  if (value < 3600) {
                    text = '${(value / 60).round()}m';
                  } else {
                    if (value % 3600 == 0) {
                      text = '${(value / 3600).round()}h';
                    } else {
                      text = '${(value / 3600).toStringAsFixed(1)}h';
                    }
                  }

                  return Padding(
                    padding: const EdgeInsets.only(right: 8.0),
                    child: Text(
                      text,
                      style: const TextStyle(
                        fontSize: 10,
                        color: Colors.grey,
                        fontWeight: FontWeight.w500,
                      ),
                      textAlign: TextAlign.right,
                    ),
                  );
                },
              ),
            ),
            topTitles: const AxisTitles(
              sideTitles: SideTitles(showTitles: false),
            ),
            rightTitles: const AxisTitles(
              sideTitles: SideTitles(showTitles: false),
            ),
          ),
          gridData: FlGridData(
            show: true,
            drawVerticalLine: false,
            horizontalInterval: yAxisParams.interval,
            getDrawingHorizontalLine:
                (value) => FlLine(
                  color: Theme.of(
                    context,
                  ).colorScheme.outlineVariant.withAlpha((255 * 0.2).round()),
                  strokeWidth: 1,
                  dashArray: [5, 5],
                ),
          ),
          borderData: FlBorderData(show: false),
          barGroups:
              dailyActivity.asMap().entries.map((entry) {
                final index = entry.key;
                final activity = entry.value;

                final rods = <BarChartRodStackItem>[];
                double currentY = 0;

                for (final dist in activity.categoryDistribution) {
                  final color =
                      categoryColors[dist.categoryId] ??
                      Theme.of(context).colorScheme.primary;
                  rods.add(
                    BarChartRodStackItem(
                      currentY,
                      currentY + dist.totalFocusTime,
                      color,
                    ),
                  );
                  currentY += dist.totalFocusTime;
                }

                return BarChartGroupData(
                  x: index,
                  barRods: [
                    BarChartRodData(
                      toY: currentY,
                      width:
                          (timeRange == StatisticsTimeRange.month ||
                                  timeRange == StatisticsTimeRange.last30Days)
                              ? 6
                              : 12,
                      borderRadius: BorderRadius.circular(6),
                      rodStackItems: rods,
                      color: Colors.transparent,
                    ),
                  ],
                );
              }).toList(),
        ),
      ),
    );
  }

  ({double maxY, double interval}) _calculateYAxisParams(
    List<DailyActivity> activities,
  ) {
    if (activities.isEmpty) return (maxY: 3600.0, interval: 900.0);

    final maxVal =
        activities
            .map(
              (e) => e.categoryDistribution.fold(
                0,
                (sum, item) => sum + item.totalFocusTime,
              ),
            )
            .fold(0, (max, current) => current > max ? current : max)
            .toDouble();

    if (maxVal == 0) return (maxY: 3600.0, interval: 900.0);

    final niceIntervals = [
      900.0, // 15m
      1800.0, // 30m
      3600.0, // 1h
      7200.0, // 2h
      14400.0, // 4h
      28800.0, // 8h
      43200.0, // 12h
    ];

    double targetInterval = maxVal / 4;

    double interval = niceIntervals.firstWhere(
      (i) => i >= targetInterval,
      orElse: () {
        return (targetInterval / 3600).ceil() * 3600.0;
      },
    );

    double maxY = (maxVal / interval).ceil() * interval;
    if (maxY == maxVal) {
      maxY += interval;
    }

    return (maxY: maxY, interval: interval);
  }

  List<DailyActivity> _fillMissingDates(
    List<DailyActivity> rawActivity,
    StatisticsTimeRange timeRange,
  ) {
    final now = DateTime.now();
    DateTime startDate;
    int daysCount;

    if (timeRange == StatisticsTimeRange.day) {
      return rawActivity;
    } else if (timeRange == StatisticsTimeRange.yesterday) {
      return rawActivity;
    } else if (timeRange == StatisticsTimeRange.week) {
      startDate = DateTime(now.year, now.month, now.day - (now.weekday - 1));
      daysCount = 7;
    } else if (timeRange == StatisticsTimeRange.lastWeek) {
      startDate = DateTime(
        now.year,
        now.month,
        now.day - (now.weekday - 1) - 7,
      );
      daysCount = 7;
    } else if (timeRange == StatisticsTimeRange.last7Days) {
      startDate = DateTime(now.year, now.month, now.day - 6);
      daysCount = 7;
    } else if (timeRange == StatisticsTimeRange.month) {
      startDate = DateTime(now.year, now.month, 1);
      daysCount = DateTime(now.year, now.month + 1, 0).day;
    } else if (timeRange == StatisticsTimeRange.last30Days) {
      startDate = DateTime(now.year, now.month, now.day - 29);
      daysCount = 30;
    } else {
      return rawActivity;
    }

    final filledList = <DailyActivity>[];
    for (int i = 0; i < daysCount; i++) {
      final currentDate = startDate.add(Duration(days: i));
      final startOfDay = DateTime(
        currentDate.year,
        currentDate.month,
        currentDate.day,
      );

      final existingIndex = rawActivity.indexWhere((element) {
        final date = DateTime.fromMillisecondsSinceEpoch(element.date * 1000);
        return date.year == currentDate.year &&
            date.month == currentDate.month &&
            date.day == currentDate.day;
      });

      if (existingIndex != -1) {
        filledList.add(rawActivity[existingIndex]);
      } else {
        filledList.add(
          DailyActivity(
            date: startOfDay.millisecondsSinceEpoch ~/ 1000,
            categoryDistribution: [],
          ),
        );
      }
    }
    return filledList;
  }

  Widget _buildCategoryDistributionChart(
    BuildContext context,
    PeriodStatistics stats,
    Map<String, Color> categoryColors,
  ) {
    final categories = stats.categoryDistribution;
    if (categories.isEmpty) {
      return _buildNoDataWidget(
        context,
        context.tr('statistics.no_category_data'),
        Icons.pie_chart_outline,
      );
    }

    final sortedCategories = List<CategoryDistribution>.from(categories)
      ..sort((a, b) => b.percentage.compareTo(a.percentage));

    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: Theme.of(
          context,
        ).colorScheme.surfaceContainerHighest.withAlpha((255 * 0.3).round()),
        borderRadius: BorderRadius.circular(24),
      ),
      child: Column(
        children:
            sortedCategories.map((category) {
              final color =
                  categoryColors[category.categoryId] ??
                  Theme.of(context).colorScheme.primary;
              final hasTasks = category.taskDistribution.isNotEmpty;

              return Padding(
                padding: const EdgeInsets.only(bottom: 16.0),
                child: Theme(
                  data: Theme.of(
                    context,
                  ).copyWith(dividerColor: Colors.transparent),
                  child: ExpansionTile(
                    tilePadding: EdgeInsets.zero,
                    childrenPadding: const EdgeInsets.only(top: 8, bottom: 8),
                    enabled: hasTasks,
                    trailing:
                        hasTasks
                            ? Icon(
                              Icons.keyboard_arrow_down,
                              size: 20,
                              color:
                                  Theme.of(
                                    context,
                                  ).colorScheme.onSurfaceVariant,
                            )
                            : const SizedBox.shrink(),
                    title: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Row(
                          mainAxisAlignment: MainAxisAlignment.spaceBetween,
                          children: [
                            Row(
                              children: [
                                Container(
                                  width: 12,
                                  height: 12,
                                  decoration: BoxDecoration(
                                    color: color,
                                    shape: BoxShape.circle,
                                  ),
                                ),
                                const SizedBox(width: 8),
                                Text(
                                  category.categoryName,
                                  style: Theme.of(context).textTheme.bodyMedium
                                      ?.copyWith(fontWeight: FontWeight.w600),
                                ),
                              ],
                            ),
                            Text(
                              '${category.percentage.toStringAsFixed(1)}% - ${_formatDuration(Duration(seconds: category.totalFocusTime))}',
                              style: Theme.of(
                                context,
                              ).textTheme.bodySmall?.copyWith(
                                color:
                                    Theme.of(
                                      context,
                                    ).colorScheme.onSurfaceVariant,
                              ),
                            ),
                          ],
                        ),
                        const SizedBox(height: 8),
                        ClipRRect(
                          borderRadius: BorderRadius.circular(8),
                          child: LinearProgressIndicator(
                            value: category.percentage / 100,
                            backgroundColor: color.withAlpha(
                              (255 * 0.1).round(),
                            ),
                            color: color,
                            minHeight: 12,
                          ),
                        ),
                      ],
                    ),
                    children:
                        category.taskDistribution.map((task) {
                          return Padding(
                            padding: const EdgeInsets.only(
                              left: 20.0,
                              top: 8.0,
                              bottom: 8.0,
                              right: 16.0,
                            ),
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Row(
                                  mainAxisAlignment:
                                      MainAxisAlignment.spaceBetween,
                                  children: [
                                    Expanded(
                                      child: Text(
                                        task.taskName,
                                        style:
                                            Theme.of(
                                              context,
                                            ).textTheme.bodySmall,
                                        maxLines: 1,
                                        overflow: TextOverflow.ellipsis,
                                      ),
                                    ),
                                    const SizedBox(width: 8),
                                    Text(
                                      '${task.percentage.toStringAsFixed(1)}% (${_formatDuration(Duration(seconds: task.totalFocusTime))})',
                                      style: Theme.of(
                                        context,
                                      ).textTheme.bodySmall?.copyWith(
                                        color:
                                            Theme.of(
                                              context,
                                            ).colorScheme.onSurfaceVariant,
                                        fontSize: 10,
                                      ),
                                    ),
                                  ],
                                ),
                                const SizedBox(height: 4),
                                ClipRRect(
                                  borderRadius: BorderRadius.circular(4),
                                  child: LinearProgressIndicator(
                                    value: task.percentage / 100,
                                    backgroundColor: color.withAlpha(
                                      (255 * 0.1).round(),
                                    ),
                                    color: color.withAlpha(
                                      (255 * 0.7).round(),
                                    ), // Slightly lighter/transparent than category
                                    minHeight: 8,
                                  ),
                                ),
                              ],
                            ),
                          );
                        }).toList(),
                  ),
                ),
              );
            }).toList(),
      ),
    );
  }

  Widget _buildConcentrationChart(
    BuildContext context,
    PeriodStatistics stats,
  ) {
    final distribution = stats.concentrationDistribution;

    if (distribution.every((element) => element == 0)) {
      return SizedBox(
        height: 240,
        child: _buildNoDataWidget(
          context,
          context.tr('statistics.no_activity_data'),
          Icons.show_chart_outlined,
        ),
      );
    }

    return Container(
      height: 240,
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: Theme.of(
          context,
        ).colorScheme.surfaceContainerHighest.withAlpha((255 * 0.3).round()),
        borderRadius: BorderRadius.circular(24),
      ),
      child: BarChart(
        BarChartData(
          alignment: BarChartAlignment.spaceAround,
          maxY: distribution.reduce((a, b) => a > b ? a : b).toDouble() * 1.2,
          barTouchData: BarTouchData(
            touchTooltipData: BarTouchTooltipData(
              getTooltipColor:
                  (group) =>
                      Theme.of(context).colorScheme.surfaceContainerHighest,
            ),
          ),
          titlesData: FlTitlesData(
            show: true,
            bottomTitles: AxisTitles(
              sideTitles: SideTitles(
                showTitles: true,
                getTitlesWidget: (double value, TitleMeta meta) {
                  if (value.toInt() >= 0 &&
                      value.toInt() < distribution.length) {
                    return Padding(
                      padding: const EdgeInsets.only(top: 8.0),
                      child: Text(
                        '${value.toInt() + 1}',
                        style: const TextStyle(
                          fontSize: 14,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                    );
                  }
                  return const SizedBox.shrink();
                },
              ),
            ),
            leftTitles: const AxisTitles(
              sideTitles: SideTitles(showTitles: false),
            ),
            topTitles: const AxisTitles(
              sideTitles: SideTitles(showTitles: false),
            ),
            rightTitles: const AxisTitles(
              sideTitles: SideTitles(showTitles: false),
            ),
          ),
          gridData: FlGridData(show: false),
          borderData: FlBorderData(show: false),
          barGroups:
              distribution.asMap().entries.map((entry) {
                final index = entry.key;
                final count = entry.value;

                final color =
                    HSVColor.fromAHSV(1.0, index * 30.0, 0.8, 0.9).toColor();

                return BarChartGroupData(
                  x: index,
                  barRods: [
                    BarChartRodData(
                      toY: count.toDouble(),
                      color: color,
                      width: 32,
                      borderRadius: BorderRadius.circular(8),
                      backDrawRodData: BackgroundBarChartRodData(
                        show: true,
                        toY:
                            distribution
                                .reduce((a, b) => a > b ? a : b)
                                .toDouble() *
                            1.2,
                        color: color.withAlpha((255 * 0.1).round()),
                      ),
                    ),
                  ],
                );
              }).toList(),
        ),
      ),
    );
  }
}

class _SummaryItem {
  final String title;
  final String value;
  final IconData icon;
  final Color color;

  _SummaryItem({
    required this.title,
    required this.value,
    required this.icon,
    required this.color,
  });
}
