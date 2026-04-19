import 'package:auth_frontend/main.dart';
import 'package:todo_frontend/main.dart';
import 'package:auto_route/auto_route.dart';

import 'auth_guard.dart';

part 'router.gr.dart';

@AutoRouterConfig()
class AppRouter extends RootStackRouter {
  final AuthGuard authGuard;

  AppRouter({required this.authGuard});

  @override
  List<AutoRoute> get routes => [
    getAuthDomainRoutes(guards: [authGuard], rootPath: '/'),
    getTodoDomainRoutes(guards: [authGuard], rootPath: '/todo', initial: false),
  ];
}
