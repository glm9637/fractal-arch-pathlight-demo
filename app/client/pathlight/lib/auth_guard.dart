import 'package:auth_frontend/main.dart';
import 'package:auto_route/auto_route.dart';
import 'package:todo_frontend/main.dart';

class AuthGuard extends AutoRouteGuard {
  final AuthStateNotifier authState;

  AuthGuard(this.authState);

  @override
  void onNavigation(NavigationResolver resolver, StackRouter router) {
    final isLoggedIn = authState.isLoggedIn;
    final isGoingToLogin = resolver.route.name == AuthRoute.name;

    if (!isLoggedIn && !isGoingToLogin) {
      router.push(AuthRoute());
    } else if (isLoggedIn && isGoingToLogin) {
      router.push(TodoRoute());
    } else {
      resolver.next(true);
    }
  }
}
