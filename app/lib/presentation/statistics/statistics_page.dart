import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:focus_flow_app/domain/repositories/statistics_repository.dart';
import 'package:focus_flow_app/domain/usecases/categories_usecases/get_categories_and_tasks.dart';
import 'package:focus_flow_app/presentation/statistics/bloc/statistics_bloc.dart';
import 'package:focus_flow_app/presentation/statistics/statistics_view.dart';
import 'package:get_it/get_it.dart';

class StatisticsPage extends StatelessWidget {
  const StatisticsPage({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create:
          (context) => StatisticsBloc(
            statisticsRepository: GetIt.I<StatisticsRepository>(),
            getCategoriesAndTasks: GetIt.I<GetCategoriesAndTasks>(),
          ),
      child: const StatisticsView(),
    );
  }
}
