import 'package:flutter/material.dart';
import 'package:flutter_rust/todo.dart';

class TodoScreen extends StatefulWidget {
  const TodoScreen({super.key});

  @override
  State<TodoScreen> createState() => _TodoScreenState();
}

class _TodoScreenState extends State<TodoScreen> {
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
