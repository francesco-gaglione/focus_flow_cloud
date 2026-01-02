import 'package:flutter/material.dart';

class ColorPickerAdvanced extends StatelessWidget {
  final Color selectedColor;
  final ValueChanged<Color> onColorSelected;

  const ColorPickerAdvanced({
    super.key,
    required this.selectedColor,
    required this.onColorSelected,
  });

  // Predefined color palette with balanced, accessible Material Design 500 colors
  static const List<Color> _availableColors = [
    Color(0xFFEF5350), // Red
    Color(0xFFEC407A), // Pink
    Color(0xFFAB47BC), // Purple
    Color(0xFF7E57C2), // Deep Purple
    Color(0xFF5C6BC0), // Indigo
    Color(0xFF42A5F5), // Blue
    Color(0xFF29B6F6), // Light Blue
    Color(0xFF26C6DA), // Cyan
    Color(0xFF26A69A), // Teal
    Color(0xFF66BB6A), // Green
    Color(0xFF9CCC65), // Light Green
    Color(0xFFD4E157), // Lime
    Color(0xFFFFEE58), // Yellow
    Color(0xFFFFCA28), // Amber
    Color(0xFFFFA726), // Orange
    Color(0xFFFF7043), // Deep Orange
    Color(0xFF8D6E63), // Brown
    Color(0xFFBDBDBD), // Grey
    Color(0xFF78909C), // Blue Grey
    Color(0xFF424242), // Dark Grey
  ];

  @override
  Widget build(BuildContext context) {
    // Debug: print the selected color value
    debugPrint(
      'ColorPickerAdvanced - selectedColor: ${selectedColor.toARGB32().toRadixString(16)}',
    );

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      children: [
        Text('Choose a color', style: Theme.of(context).textTheme.titleSmall),
        const SizedBox(height: 16),

        // Color grid with responsive wrapping
        Wrap(
          spacing: 12,
          runSpacing: 12,
          children:
              _availableColors
                  .map((color) => _buildColorOption(context, color))
                  .toList(),
        ),
      ],
    );
  }

  /// Builds an individual color option with selection state and animations
  Widget _buildColorOption(BuildContext context, Color color) {
    // Compare color values directly
    final isSelected = selectedColor.toARGB32() == color.toARGB32();

    // Debug: print comparison
    if (isSelected) {
      debugPrint('Color matched! ${color.toARGB32().toRadixString(16)}');
    }

    return InkWell(
      onTap: () => onColorSelected(color),
      borderRadius: BorderRadius.circular(12),
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 200),
        // Slightly larger size for selected state to provide visual feedback
        width: isSelected ? 56 : 52,
        height: isSelected ? 56 : 52,
        decoration: BoxDecoration(
          color: color,
          borderRadius: BorderRadius.circular(12),
          border: Border.all(
            color:
                isSelected
                    ? Theme.of(context).colorScheme.primary
                    : Colors.transparent,
            width: 3,
          ),
          boxShadow: [
            if (isSelected)
              // Enhanced shadow for selected color to emphasize choice
              BoxShadow(
                color: color.withAlpha((255 * 0.5).round()),
                blurRadius: 8,
                spreadRadius: 2,
              )
            else
              // Subtle elevation for unselected colors
              BoxShadow(
                color: Colors.black.withAlpha((255 * 0.1).round()),
                blurRadius: 4,
                spreadRadius: 0,
                offset: const Offset(0, 2),
              ),
          ],
        ),
        // Checkmark icon for selected color with proper contrast
        child:
            isSelected
                ? Icon(Icons.check, color: _getContrastColor(color), size: 28)
                : null,
      ),
    );
  }

  /// Calculates appropriate contrast color (black or white) based on background luminance
  /// Ensures checkmark is always visible regardless of background color
  Color _getContrastColor(Color backgroundColor) {
    final luminance = backgroundColor.computeLuminance();
    return luminance > 0.5 ? Colors.black : Colors.white;
  }
}
