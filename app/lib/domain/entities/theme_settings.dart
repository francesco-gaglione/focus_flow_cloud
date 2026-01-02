class ThemeSettings {
  static const int defaultAccentColor = 0xFF6750A4;

  final bool isDarkMode;
  final int accentColor;

  const ThemeSettings({required this.isDarkMode, required this.accentColor});

  const ThemeSettings.initial()
    : isDarkMode = false,
      accentColor = defaultAccentColor;

  ThemeSettings copyWith({bool? isDarkMode, int? accentColor}) {
    return ThemeSettings(
      isDarkMode: isDarkMode ?? this.isDarkMode,
      accentColor: accentColor ?? this.accentColor,
    );
  }
}
