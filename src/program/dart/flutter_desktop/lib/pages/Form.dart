import 'package:flutter/material.dart';

class FormPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final arguments = ModalRoute.of(context).settings.arguments;
    var title;
    if (arguments != null) {
      title = arguments;
    }
    print("arguments: $arguments");
    return Container(
      child: Scaffold(
        floatingActionButton: FloatingActionButton(
          child: Text("返回"),
          onPressed: () {
            Navigator.of(context).pop();
          },
        ),
        appBar: AppBar(
          title: Text(title),
        ),
        body: ListView(
          children: [
            ListTile(title: Text("我是表单")),
            ListTile(title: Text("我是表单")),
            ListTile(title: Text("我是表单")),
            ListTile(title: Text("我是表单")),
            ListTile(title: Text("我是表单")),
            ListTile(title: Text("我是表单")),
            ListTile(title: Text("我是表单")),
            ListTile(title: Text("我是表单")),
            ListTile(title: Text("我是表单")),
          ],
        ),
      ),
    );
  }
}
