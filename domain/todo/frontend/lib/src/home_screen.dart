import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_rust/todo.dart';

@RoutePage()
class HomeScreen extends StatefulWidget {
  const HomeScreen({super.key});

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  @override
  void initState() {
    super.initState();
    dispatchLoadTodosCommand(
      command: LoadTodosCommand(limit: BigInt.from(10), offset: BigInt.from(0)),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Todo Domain (Rust Powered)')),
      body: Center(
        child: StreamBuilder<TodoList>(
          stream: watchTodoList(),
          builder: (context, snapshot) {
            final list = snapshot.data ?? const TodoList(items: []);
            if (list.items.isEmpty) {
              return const Text("No tasks yet. Hit the + button!");
            }
            return ListView.builder(
              itemCount: list.items.length,
              itemBuilder: (context, i) => ListTile(title: Text(list.items[i])),
            );
          },
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () =>
            dispatchAddTodoCommand(command: AddTodoCommand(text: "hi")),
        child: const Icon(Icons.add),
      ),
    );
  }
}
