import 'package:auto_route/auto_route.dart';
import 'auth_screen.dart';

part './auth_router.gr.dart';

@AutoRouterConfig()
class AuthModule {}

AutoRoute getAuthDomainRoutes({
  required List<AutoRouteGuard> guards,
  required String rootPath,
  bool initial = false,
}) {
  return AutoRoute(
    page: AuthRoute.page,
    path: rootPath,
    guards: guards,
    initial: initial,
    children: [],
  );
}
