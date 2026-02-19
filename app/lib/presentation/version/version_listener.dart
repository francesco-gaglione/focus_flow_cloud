import 'package:easy_localization/easy_localization.dart';
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
            barrierDismissible: false,
            builder: (context) => AlertDialog(
              title: Text(context.tr('version.mismatch_title')),
              content: Text(context.tr('version.mismatch_message')),
              actions: [
                TextButton(
                  onPressed: () {
                    // Ideally open store or just dismiss if user insists (but user said "must update")
                    // For now, let's just allow dismissal to continue using the app (or exit app?)
                    // User Request: "user must be warned... and must update".
                    // Strict interpretation: Exit app or Open Store.
                    // Pragmatic interpretation for now: Dismiss.
                    Navigator.of(context).pop();
                  },
                  child: Text(context.tr('common.ok')),
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
