import 'package:flutter/material.dart';
import 'package:flutter_desktop/pages/Form.dart';
import 'package:flutter_desktop/pages/Search.dart';

class HomePage extends StatefulWidget {
  HomePage({Key key}) : super(key: key);

  @override
  _HomePageState createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        ElevatedButton(
          onPressed: () {
            // Navigator.of(context)
            //     .push(MaterialPageRoute(builder: (context) => SearchPage()));
            // Navigator.pushNamed(context, '/search');
            Navigator.push(
              context,
              MaterialPageRoute(builder: (context) => SearchPage()),
            );
          },
          child: Text("跳转到搜索页面"),
        ),
        SizedBox(height: 10),
        ElevatedButton(
          onPressed: () {
            Navigator.of(context).push(MaterialPageRoute(
              builder: (context) => FormPage(),
              settings: RouteSettings(arguments: '我是定制表单名'),
            ));
            // Navigator.pushNamed(context, '/form');
          },
          child: Text("跳转到表单页面"),
        ),
      ],
    );
  }
}

class NewPageLayout extends StatelessWidget {
  const NewPageLayout({Key key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Navigator(
      pages: [
        MaterialPage(
          child: Scaffold(),
        ),
      ],
    );
  }
}
