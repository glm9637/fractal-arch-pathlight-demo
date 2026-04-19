import 'package:flutter/material.dart';
import 'package:flutter_rust/todo.dart';
import 'package:state_machine_flutter/flutter.dart';
import 'package:auto_route/auto_route.dart';
import 'home_screen.dart';

@RoutePage()
class TodoPage extends StatelessWidget {
  const TodoPage({super.key});

  @override
  Widget build(BuildContext context) {
    return FractalWidget(
      onInit: () => initTodoSystem(
        config: TodoSystemConfig(baseUrl: "http://127.0.0.1:50051"),
      ),
      child: const HomeScreen(),
      onDispose: () => disposeTodoSystem(),
    );
  }
}
