import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter/semantics.dart';

/// The destinations that a session shell exposes to both narrow and wide
/// layouts. Destination screens are intentionally owned by later packets.
class SessionDestination {
  const SessionDestination(this.label);

  final String label;
}

/// Synthetic session data used by the shell until the application state layer
/// is integrated. The destination list is shared by every adaptive layout.
class SessionShellModel {
  const SessionShellModel({required this.title, required this.destinations});

  final String title;
  final List<SessionDestination> destinations;
}

/// Presents the same session destinations with a phone or tablet structure.
class SessionShell extends StatefulWidget {
  const SessionShell({required this.session, super.key});

  final SessionShellModel session;

  @override
  State<SessionShell> createState() => _SessionShellState();
}

class _SessionShellState extends State<SessionShell> {
  static const double _tabletBreakpoint = 600;

  int _selectedIndex = 0;
  late final FocusNode _contentFocusNode;
  final Map<String, FocusNode> _navigationFocusNodes = <String, FocusNode>{};
  bool _contentHadFocus = false;
  bool? _lastTabletLayout;

  @override
  void initState() {
    super.initState();
    _contentFocusNode = FocusNode(debugLabel: 'Session content');
  }

  @override
  void dispose() {
    _contentFocusNode.dispose();
    for (final focusNode in _navigationFocusNodes.values) {
      focusNode.dispose();
    }
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final destinations = widget.session.destinations;
    if (destinations.isEmpty) {
      return Scaffold(
        appBar: AppBar(title: Text(widget.session.title)),
        body: const SizedBox.shrink(),
      );
    }

    final selectedIndex = _selectedIndex.clamp(0, destinations.length - 1);
    final selectedDestination = destinations[selectedIndex];

    return LayoutBuilder(
      builder: (context, constraints) {
        final isTablet = constraints.maxWidth >= _tabletBreakpoint;
        final largeText = _hasLargeText(context);
        final reducedMotion = MediaQuery.disableAnimationsOf(context);
        final useStaticNavigation = largeText || reducedMotion;

        if (_lastTabletLayout != null &&
            _lastTabletLayout != isTablet &&
            _contentHadFocus) {
          WidgetsBinding.instance.addPostFrameCallback((_) {
            if (mounted) {
              _contentFocusNode.requestFocus();
            }
          });
        }
        _lastTabletLayout = isTablet;

        final content = FocusTraversalOrder(
          order: const NumericFocusOrder(100),
          child: Focus(
            key: const ValueKey('session-content-focus'),
            focusNode: _contentFocusNode,
            onFocusChange: _handleContentFocusChange,
            child: _SessionContent(
              destination: selectedDestination,
              focused: _contentFocusNode.hasFocus,
            ),
          ),
        );

        final navigation = isTablet
            ? (useStaticNavigation
                  ? _StaticNavigation(
                      key: ValueKey(
                        reducedMotion
                            ? 'session-navigation-reduced-motion'
                            : 'session-navigation-large-text',
                      ),
                      axis: Axis.vertical,
                      destinations: destinations,
                      selectedIndex: selectedIndex,
                      onDestinationSelected: _selectDestination,
                    )
                  : _TabletNavigation(
                      destinations: destinations,
                      selectedIndex: selectedIndex,
                      onDestinationSelected: _selectDestination,
                    ))
            : (useStaticNavigation
                  ? _StaticNavigation(
                      key: ValueKey(
                        reducedMotion
                            ? 'session-navigation-reduced-motion'
                            : 'session-navigation-large-text',
                      ),
                      axis: Axis.horizontal,
                      destinations: destinations,
                      selectedIndex: selectedIndex,
                      onDestinationSelected: _selectDestination,
                    )
                  : _PhoneNavigation(
                      destinations: destinations,
                      selectedIndex: selectedIndex,
                      onDestinationSelected: _selectDestination,
                    ));

        final navigationWithOrder = FocusTraversalOrder(
          order: const NumericFocusOrder(0),
          child: navigation,
        );

        final shell = Scaffold(
          appBar: AppBar(title: Text(widget.session.title)),
          body: isTablet
              ? Row(
                  children: <Widget>[
                    navigationWithOrder,
                    const VerticalDivider(width: 1),
                    Expanded(child: content),
                  ],
                )
              : content,
          bottomNavigationBar: isTablet ? null : navigationWithOrder,
        );

        return FocusTraversalGroup(
          key: const ValueKey('session-navigation-focus-group'),
          policy: OrderedTraversalPolicy(),
          child: shell,
        );
      },
    );
  }

  void _selectDestination(int index) {
    if (index == _selectedIndex) {
      return;
    }

    setState(() => _selectedIndex = index);
    _contentFocusNode.requestFocus();
  }

  void _handleContentFocusChange(bool hasFocus) {
    _contentHadFocus = hasFocus;
    if (mounted) {
      setState(() {});
    }
  }

  bool _hasLargeText(BuildContext context) {
    return MediaQuery.textScalerOf(context).scale(1) > 1.25;
  }

  FocusNode _navigationFocusNodeFor(SessionDestination destination) {
    return _navigationFocusNodes.putIfAbsent(
      destination.label,
      () => FocusNode(debugLabel: 'Session destination ${destination.label}'),
    );
  }
}

class _SessionContent extends StatelessWidget {
  const _SessionContent({required this.destination, required this.focused});

  final SessionDestination destination;
  final bool focused;

  @override
  Widget build(BuildContext context) {
    return Semantics(
      container: true,
      liveRegion: true,
      focusable: true,
      focused: focused,
      label: 'Current editor destination: ${destination.label}',
      child: Center(child: ExcludeSemantics(child: Text(destination.label))),
    );
  }
}

class _PhoneNavigation extends StatelessWidget {
  const _PhoneNavigation({
    required this.destinations,
    required this.selectedIndex,
    required this.onDestinationSelected,
  });

  final List<SessionDestination> destinations;
  final int selectedIndex;
  final ValueChanged<int> onDestinationSelected;

  @override
  Widget build(BuildContext context) {
    return Semantics(
      container: true,
      explicitChildNodes: true,
      label: 'Editor destination navigation',
      child: BottomNavigationBar(
        currentIndex: selectedIndex,
        onTap: onDestinationSelected,
        showUnselectedLabels: true,
        items: [
          for (final destination in destinations)
            BottomNavigationBarItem(
              icon: const SizedBox.shrink(),
              label: destination.label,
              tooltip: destination.label,
            ),
        ],
      ),
    );
  }
}

class _TabletNavigation extends StatelessWidget {
  const _TabletNavigation({
    required this.destinations,
    required this.selectedIndex,
    required this.onDestinationSelected,
  });

  final List<SessionDestination> destinations;
  final int selectedIndex;
  final ValueChanged<int> onDestinationSelected;

  @override
  Widget build(BuildContext context) {
    return Semantics(
      container: true,
      explicitChildNodes: true,
      label: 'Editor destination navigation',
      child: NavigationRail(
        selectedIndex: selectedIndex,
        onDestinationSelected: onDestinationSelected,
        labelType: NavigationRailLabelType.all,
        destinations: [
          for (var index = 0; index < destinations.length; index += 1)
            NavigationRailDestination(
              icon: const SizedBox.shrink(),
              selectedIcon: const SizedBox.shrink(),
              label: Semantics(
                container: true,
                label: destinations[index].label,
                button: true,
                selected: index == selectedIndex,
                onTap: () => onDestinationSelected(index),
                child: ExcludeSemantics(child: Text(destinations[index].label)),
              ),
            ),
        ],
      ),
    );
  }
}

class _StaticNavigation extends StatelessWidget {
  const _StaticNavigation({
    required this.axis,
    required this.destinations,
    required this.selectedIndex,
    required this.onDestinationSelected,
    super.key,
  });

  final Axis axis;
  final List<SessionDestination> destinations;
  final int selectedIndex;
  final ValueChanged<int> onDestinationSelected;

  @override
  Widget build(BuildContext context) {
    final children = <Widget>[
      for (var index = 0; index < destinations.length; index += 1)
        FocusTraversalOrder(
          order: NumericFocusOrder(index.toDouble()),
          child: _StaticDestination(
            destination: destinations[index],
            selected: index == selectedIndex,
            sortOrder: index.toDouble(),
            focusNode: _focusNodeFor(context, destinations[index]),
            onSelected: () => onDestinationSelected(index),
          ),
        ),
    ];

    final Widget layout = axis == Axis.horizontal
        ? LayoutBuilder(
            builder: (context, constraints) {
              return Wrap(
                alignment: WrapAlignment.center,
                spacing: 4,
                runSpacing: 4,
                children: [
                  for (final child in children)
                    ConstrainedBox(
                      constraints: const BoxConstraints(
                        minWidth: 96,
                        maxWidth: 180,
                      ),
                      child: child,
                    ),
                ],
              );
            },
          )
        : Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: children,
          );

    final Widget boundedLayout = axis == Axis.vertical
        ? ConstrainedBox(
            constraints: const BoxConstraints(minWidth: 96, maxWidth: 180),
            child: layout,
          )
        : layout;

    return Semantics(
      container: true,
      explicitChildNodes: true,
      label: 'Editor destination navigation',
      child: boundedLayout,
    );
  }

  FocusNode _focusNodeFor(
    BuildContext context,
    SessionDestination destination,
  ) {
    final shellState = context.findAncestorStateOfType<_SessionShellState>();
    assert(shellState != null);
    return shellState!._navigationFocusNodeFor(destination);
  }
}

class _StaticDestination extends StatelessWidget {
  const _StaticDestination({
    required this.destination,
    required this.selected,
    required this.sortOrder,
    required this.focusNode,
    required this.onSelected,
  });

  final SessionDestination destination;
  final bool selected;
  final double sortOrder;
  final FocusNode focusNode;
  final VoidCallback onSelected;

  @override
  Widget build(BuildContext context) {
    return Focus(
      focusNode: focusNode,
      onKeyEvent: (node, event) {
        if (event is KeyDownEvent &&
            (event.logicalKey == LogicalKeyboardKey.enter ||
                event.logicalKey == LogicalKeyboardKey.space)) {
          onSelected();
          return KeyEventResult.handled;
        }
        return KeyEventResult.ignored;
      },
      child: Semantics(
        container: true,
        label: destination.label,
        button: true,
        selected: selected,
        focusable: true,
        sortKey: OrdinalSortKey(sortOrder),
        onTap: onSelected,
        child: ExcludeSemantics(
          child: GestureDetector(
            behavior: HitTestBehavior.opaque,
            onTap: onSelected,
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
              child: Text(
                destination.label,
                textAlign: TextAlign.center,
                softWrap: true,
              ),
            ),
          ),
        ),
      ),
    );
  }
}
