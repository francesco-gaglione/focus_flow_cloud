import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:focus_flow_app/core/di/service_locator.dart';
import 'package:focus_flow_app/presentation/notes/bloc/notes_bloc.dart';
import 'package:focus_flow_app/presentation/notes/bloc/notes_event.dart';
import 'package:focus_flow_app/presentation/notes/notes_view.dart';

class NotesPage extends StatelessWidget {
  const NotesPage({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create:
          (_) => NotesBloc(
            getSessionsWithFilters: sl(),
            updateSession: sl(),
            getCategoriesAndTasks: sl(),
          )..add(NotesStarted()),
      child: const NotesView(),
    );
  }
}
