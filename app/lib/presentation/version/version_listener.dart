import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'cubit/version_cubit.dart';

class VersionListener extends StatelessWidget {
  final Widget child;

  const VersionListener({required this.child, super.key});

  @override
  Widget build(BuildContext context) {
    return BlocListener<VersionCubit, VersionState>(
      listener: (context, state) {
        if (state is VersionIncompatible) {
          showDialog(
            context: context,
            builder: (context) => AlertDialog(
              title: const Text('Version Mismatch'), // TODO: Localize
              content: const Text(
                  'The app version is incompatible with the server. Please update the app.'), // TODO: Localize
              actions: [
                TextButton(
                  onPressed: () => Navigator.of(context).pop(),
                  child: const Text('OK'),
                ),
              ],
            ),
          );
        }
      },
      child: child,
    );
  }
}
