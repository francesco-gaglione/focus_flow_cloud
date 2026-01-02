import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:focus_flow_app/presentation/category/bloc/category_bloc.dart';
import 'package:focus_flow_app/presentation/category/bloc/category_event.dart';
import 'package:focus_flow_app/presentation/category/category_view.dart';
import '../../core/di/service_locator.dart';

class CategoryPage extends StatelessWidget {
  const CategoryPage({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create:
          (_) => CategoryBloc(
            getCategoriesAndTasks: sl(),
            fetchOrphanTasks: sl(),
            createCategory: sl(),
            createTask: sl(),
            updateCategory: sl(),
            updateTask: sl(),
            deleteCategory: sl(),
            deleteTasks: sl(),
          )..add(InitState()),
      child: const CategoryView(),
    );
  }
}
