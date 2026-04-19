import 'package:auto_route/auto_route.dart';
import 'todo_page.dart';
import 'home_screen.dart';

part 'router.gr.dart';

@AutoRouterConfig()
class TodoModule {}

AutoRoute getTodoDomainRoutes({
  required List<AutoRouteGuard> guards,
  required String rootPath,
  bool initial = false,
}) {
  return AutoRoute(
    page: TodoRoute.page,
    path: rootPath,
    guards: guards,
    initial: initial,
    children: [AutoRoute(page: HomeRoute.page, path: '')],
  );
}
