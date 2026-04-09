import 'package:flutter/material.dart';
import 'package:flutter_rust/todo.dart';
import 'package:state_machine_flutter/flutter.dart';

import 'home.dart';

class TodoFeature extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return FractalWidget(
      onInit: () => initTodoSystem(
        config: TodoSystemConfig(baseUrl: "http://127.0.0.1:50051"),
      ),
      child: const TodoScreen(),
      onDispose: () => disposeTodoSystem(),
    );
  }
}
