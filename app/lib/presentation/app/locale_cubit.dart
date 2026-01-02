import 'dart:ui';

import 'package:flutter_bloc/flutter_bloc.dart';

import '../../domain/usecases/get_saved_locale.dart';
import '../../domain/usecases/save_locale.dart';

class LocaleState {
  final Locale? locale;
  final bool isLoading;

  const LocaleState({this.locale, this.isLoading = false});

  LocaleState copyWith({Locale? locale, bool? isLoading}) {
    return LocaleState(
      locale: locale ?? this.locale,
      isLoading: isLoading ?? this.isLoading,
    );
  }
}

class LocaleCubit extends Cubit<LocaleState> {
  final GetSavedLocale _getSavedLocale;
  final SaveLocale _saveLocale;

  LocaleCubit({
    required GetSavedLocale getSavedLocale,
    required SaveLocale saveLocale,
  }) : _getSavedLocale = getSavedLocale,
       _saveLocale = saveLocale,
       super(const LocaleState(isLoading: true));

  Future<void> loadLocale() async {
    if (isClosed) return;
    emit(state.copyWith(isLoading: true));
    final locale = await _getSavedLocale();
    if (!isClosed) {
      emit(state.copyWith(locale: locale, isLoading: false));
    }
  }

  Future<void> setLocale(Locale locale) async {
    if (isClosed) return;
    emit(state.copyWith(isLoading: true));
    await _saveLocale(locale);
    if (!isClosed) {
      emit(state.copyWith(locale: locale, isLoading: false));
    }
  }
}
