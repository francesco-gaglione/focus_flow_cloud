import 'package:flutter_bloc/flutter_bloc.dart';
import '../../../core/services/version_service.dart';

part 'version_state.dart';

class VersionCubit extends Cubit<VersionState> {
  final VersionService _versionService;

  VersionCubit(this._versionService) : super(VersionInitial());

  Future<void> checkVersion() async {
    emit(VersionChecking());
    final isCompatible = await _versionService.checkCompatibility();
    if (!isCompatible) {
      emit(VersionIncompatible());
    } else {
      emit(VersionCompatible());
    }
  }
}
