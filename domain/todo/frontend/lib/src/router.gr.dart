// dart format width=80
// GENERATED CODE - DO NOT MODIFY BY HAND

// **************************************************************************
// AutoRouterGenerator
// **************************************************************************

// ignore_for_file: type=lint
// coverage:ignore-file

part of 'router.dart';

/// generated route for
/// [HomeScreen]
class HomeRoute extends PageRouteInfo<void> {
  const HomeRoute({List<PageRouteInfo>? children})
    : super(HomeRoute.name, initialChildren: children);

  static const String name = 'HomeRoute';

  static PageInfo page = PageInfo(
    name,
    builder: (data) {
      return const HomeScreen();
    },
  );
}

/// generated route for
/// [TodoPage]
class TodoRoute extends PageRouteInfo<void> {
  const TodoRoute({List<PageRouteInfo>? children})
    : super(TodoRoute.name, initialChildren: children);

  static const String name = 'TodoRoute';

  static PageInfo page = PageInfo(
    name,
    builder: (data) {
      return const TodoPage();
    },
  );
}
