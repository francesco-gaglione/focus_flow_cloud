import 'package:easy_localization/easy_localization.dart' hide TextDirection;
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

class MainLayout extends StatelessWidget {
  final Widget child;
  final String currentPath;

  const MainLayout({super.key, required this.child, required this.currentPath});

  @override
  Widget build(BuildContext context) {
    final isDesktop = MediaQuery.of(context).size.width > 800;

    if (isDesktop) {
      return Scaffold(
        body: Row(
          children: [
            NavigationRail(
              selectedIndex: _getSelectedIndexForRail(),
              onDestinationSelected:
                  (index) => _onDestinationSelected(context, index),
              labelType: NavigationRailLabelType.none,
              groupAlignment: 0.0,
              backgroundColor: Theme.of(context).colorScheme.surface,
              elevation: 1,
              useIndicator: true,
              indicatorColor: Theme.of(context).colorScheme.secondaryContainer,
              selectedIconTheme: IconThemeData(
                color: Theme.of(context).colorScheme.onSecondaryContainer,
                size: 28,
              ),
              unselectedIconTheme: IconThemeData(
                color: Theme.of(context).colorScheme.onSurfaceVariant,
                size: 24,
              ),
              trailing: Expanded(
                child: Align(
                  alignment: Alignment.bottomCenter,
                  child: Padding(
                    padding: const EdgeInsets.only(bottom: 16),
                    child: IconButton(
                      icon: Icon(
                        currentPath.startsWith('/settings')
                            ? Icons.settings
                            : Icons.settings_outlined,
                      ),
                      onPressed: () => context.go('/settings'),
                      tooltip: context.tr('navigation.settings'),
                      color:
                          currentPath.startsWith('/settings')
                              ? Theme.of(context).colorScheme.primary
                              : Theme.of(context).colorScheme.onSurfaceVariant,
                    ),
                  ),
                ),
              ),
              destinations: const [
                NavigationRailDestination(
                  icon: Icon(Icons.play_circle_outline),
                  selectedIcon: Icon(Icons.play_circle),
                  label: Text(""),
                ),
                NavigationRailDestination(
                  icon: Icon(Icons.category_outlined),
                  selectedIcon: Icon(Icons.category),
                  label: Text(''),
                ),
                NavigationRailDestination(
                  icon: Icon(Icons.bar_chart_outlined),
                  selectedIcon: Icon(Icons.bar_chart),
                  label: Text(''),
                ),
              ],
            ),
            const VerticalDivider(thickness: 1, width: 1),
            Expanded(child: child),
          ],
        ),
      );
    }

    // Mobile Layout - Solo icone, centrate verticalmente
    return Scaffold(
      body: child,
      bottomNavigationBar: Container(
        height: 60,
        decoration: BoxDecoration(
          color: Theme.of(context).colorScheme.surface,
          boxShadow: [
            BoxShadow(
              color: Colors.black.withAlpha((255 * 0.05).round()),
              blurRadius: 8,
              offset: const Offset(0, -2),
            ),
          ],
        ),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.spaceEvenly,
          crossAxisAlignment: CrossAxisAlignment.center, // Aggiungi questo!
          children: [
            _buildNavItem(
              context,
              icon: Icons.play_circle_outline,
              selectedIcon: Icons.play_circle,
              isSelected: currentPath.startsWith('/focus'),
              onTap: () => context.go('/focus'),
            ),
            _buildNavItem(
              context,
              icon: Icons.category_outlined,
              selectedIcon: Icons.category,
              isSelected: currentPath.startsWith('/categories'),
              onTap: () => context.go('/categories'),
            ),
            _buildNavItem(
              context,
              icon: Icons.bar_chart_outlined,
              selectedIcon: Icons.bar_chart,
              isSelected: currentPath.startsWith('/stats'),
              onTap: () => context.go('/stats'),
            ),
            _buildNavItem(
              context,
              icon: Icons.settings_outlined,
              selectedIcon: Icons.settings,
              isSelected: currentPath.startsWith('/settings'),
              onTap: () => context.go('/settings'),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildNavItem(
    BuildContext context, {
    required IconData icon,
    required IconData selectedIcon,
    required bool isSelected,
    required VoidCallback onTap,
  }) {
    return Expanded(
      child: InkWell(
        onTap: onTap,
        splashColor: Theme.of(
          context,
        ).colorScheme.primary.withAlpha((255 * 0.1).round()),
        highlightColor: Theme.of(
          context,
        ).colorScheme.primary.withAlpha((255 * 0.05).round()),
        child: SizedBox(
          height: double.infinity, // Occupa tutta l'altezza
          child: Center(
            // Center garantisce il centraggio verticale
            child: Icon(
              isSelected ? selectedIcon : icon,
              color:
                  isSelected
                      ? Theme.of(context).colorScheme.primary
                      : Theme.of(context).colorScheme.onSurfaceVariant,
              size: 28,
            ),
          ),
        ),
      ),
    );
  }

  int _getSelectedIndexForRail() {
    if (currentPath.startsWith('/focus')) return 0;
    if (currentPath.startsWith('/categories')) return 1;
    if (currentPath.startsWith('/stats')) return 2;
    return 0;
  }

  void _onDestinationSelected(BuildContext context, int index) {
    switch (index) {
      case 0:
        context.go('/focus');
        break;
      case 1:
        context.go('/categories');
        break;
      case 2:
        context.go('/stats');
        break;
      case 3:
        context.go('/settings');
        break;
    }
  }
}
