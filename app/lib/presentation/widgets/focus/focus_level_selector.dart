import 'package:easy_localization/easy_localization.dart' hide TextDirection;
import 'package:flutter/material.dart';

class FocusLevelSelector extends StatefulWidget {
  final ValueChanged<int> onFocusLevelChanged;
  final int? initialLevel;

  const FocusLevelSelector({
    super.key,
    required this.onFocusLevelChanged,
    this.initialLevel,
  });

  @override
  State<FocusLevelSelector> createState() => _FocusLevelSelectorState();
}

class _FocusLevelSelectorState extends State<FocusLevelSelector> {
  int? selectedLevel;

  @override
  void initState() {
    super.initState();
    selectedLevel = widget.initialLevel;
  }

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;

    return Container(
      decoration: BoxDecoration(
        color: colorScheme.surfaceContainerHighest.withAlpha(
          (255 * 0.3).round(),
        ),
        borderRadius: BorderRadius.circular(24),
        border: Border.all(
          color: colorScheme.outlineVariant.withAlpha((255 * 0.2).round()),
        ),
      ),
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Container(
                  padding: const EdgeInsets.all(8),
                  decoration: BoxDecoration(
                    color: colorScheme.primary.withAlpha((255 * 0.1).round()),
                    shape: BoxShape.circle,
                  ),
                  child: Icon(Icons.psychology, color: colorScheme.primary),
                ),
                const SizedBox(width: 12),
                Text(
                  context.tr('focus.level_title'),
                  style: Theme.of(context).textTheme.titleMedium?.copyWith(
                    fontWeight: FontWeight.bold,
                    letterSpacing: 0.5,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 24),

            // Level selector grid
            Center(
              child: Wrap(
                spacing: 16,
                runSpacing: 16,
                children: List.generate(5, (index) {
                  final level = index + 1;
                  final isSelected = selectedLevel == level;

                  return InkWell(
                    onTap:
                        () => {
                          widget.onFocusLevelChanged(level),
                          setState(() => selectedLevel = level),
                        },
                    borderRadius: BorderRadius.circular(16),
                    child: AnimatedContainer(
                      duration: const Duration(milliseconds: 300),
                      curve: Curves.easeOutBack,
                      width: isSelected ? 56 : 48,
                      height: isSelected ? 56 : 48,
                      decoration: BoxDecoration(
                        gradient:
                            isSelected
                                ? LinearGradient(
                                  colors: [
                                    colorScheme.primary,
                                    colorScheme.tertiary,
                                  ],
                                  begin: Alignment.topLeft,
                                  end: Alignment.bottomRight,
                                )
                                : null,
                        color:
                            isSelected
                                ? null
                                : colorScheme.surfaceContainerHigh.withAlpha(
                                  (255 * 0.5).round(),
                                ),
                        borderRadius: BorderRadius.circular(16),
                        border: Border.all(
                          color:
                              isSelected
                                  ? Colors.transparent
                                  : colorScheme.outline.withAlpha(
                                    (255 * 0.2).round(),
                                  ),
                          width: 1,
                        ),
                        boxShadow:
                            isSelected
                                ? [
                                  BoxShadow(
                                    color: colorScheme.primary.withAlpha(
                                      (255 * 0.4).round(),
                                    ),
                                    blurRadius: 12,
                                    offset: const Offset(0, 4),
                                  ),
                                ]
                                : [],
                      ),
                      child: Center(
                        child: Text(
                          '$level',
                          style: Theme.of(
                            context,
                          ).textTheme.titleLarge?.copyWith(
                            fontWeight: FontWeight.bold,
                            color:
                                isSelected
                                    ? colorScheme.onPrimary
                                    : colorScheme.onSurface,
                          ),
                        ),
                      ),
                    ),
                  );
                }),
              ),
            ),

            if (selectedLevel != null) ...[
              const SizedBox(height: 24),
              Container(
                width: double.infinity,
                padding: const EdgeInsets.all(12),
                decoration: BoxDecoration(
                  color: colorScheme.surface.withAlpha((255 * 0.5).round()),
                  borderRadius: BorderRadius.circular(12),
                  border: Border.all(
                    color: colorScheme.outlineVariant.withAlpha(
                      (255 * 0.2).round(),
                    ),
                  ),
                ),
                child: Text(
                  _getLevelDescription(selectedLevel!),
                  textAlign: TextAlign.center,
                  style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                    color: colorScheme.onSurface,
                    fontWeight: FontWeight.w500,
                  ),
                ),
              ),
            ],
          ],
        ),
      ),
    );
  }

  String _getLevelDescription(int level) {
    if (level <= 1) return context.tr('focus.level_low');
    if (level <= 3) return context.tr('focus.level_medium');
    if (level <= 4) return context.tr('focus.level_high');
    return context.tr('focus.level_maximum');
  }
}
