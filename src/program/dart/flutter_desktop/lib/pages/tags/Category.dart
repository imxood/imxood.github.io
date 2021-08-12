import 'package:flutter/material.dart';
import 'package:flutter_desktop/pages/Form.dart';

class CategoryPage extends StatefulWidget {
  CategoryPage({Key key}) : super(key: key);

  @override
  _CategoryPageState createState() => _CategoryPageState();
}

class _CategoryPageState extends State<CategoryPage> {
  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        ElevatedButton(
          onPressed: () {
            Navigator.of(context).push(MaterialPageRoute(
              builder: (context) => FormPage(),
              settings: RouteSettings(arguments: "我是定制数据"),
            ));
          },
          child: Text("跳转到表单页面"),
        )
      ],
    );
  }
}
