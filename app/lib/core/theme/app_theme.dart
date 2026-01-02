import 'package:flutter/material.dart';

class AppTheme {
  static ThemeData light(int accentColor) =>
      _buildTheme(accentColor: accentColor, brightness: Brightness.light);

  static ThemeData dark(int accentColor) =>
      _buildTheme(accentColor: accentColor, brightness: Brightness.dark);

  static ThemeData _buildTheme({
    required int accentColor,
    required Brightness brightness,
  }) {
    final seedColor = Color(accentColor);
    final colorScheme = ColorScheme.fromSeed(
      seedColor: seedColor,
      brightness: brightness,
    );

    return ThemeData(useMaterial3: true, colorScheme: colorScheme);
  }
}
