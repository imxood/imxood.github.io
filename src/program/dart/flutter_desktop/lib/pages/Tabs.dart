import 'package:flutter/material.dart';
import 'package:flutter_desktop/pages/tags/Category.dart';
import 'package:flutter_desktop/pages/tags/Demo.dart';
import 'package:flutter_desktop/pages/tags/Home.dart';
import 'package:flutter_desktop/pages/tags/Setting.dart';

class Tabs extends StatefulWidget {
  Tabs({Key key}) : super(key: key);

  @override
  _TabsState createState() => _TabsState();
}

class _TabsState extends State<Tabs> {
  var _currentIndex = 0;
  var _pageList = <Widget>[
    HomePage(),
    CategoryPage(),
    DemoPage(),
    SettingsPage(),
  ];
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('Material App Bar')),
      body: this._pageList[_currentIndex],
      bottomNavigationBar: BottomNavigationBar(
        iconSize: 40, // 按钮大小
        fixedColor: Colors.red, // 点击时的颜色
        currentIndex: this._currentIndex, // 当前选中项的索引
        onTap: (index) {
          setState(() {
            _currentIndex = index;
            print("index: $index");
          });
        },
        items: [
          BottomNavigationBarItem(
            icon: Icon(
              Icons.home,
              color: Colors.black,
            ),
            label: "首页",
          ),
          BottomNavigationBarItem(
            icon: Icon(
              Icons.category,
              color: Colors.black,
            ),
            label: "分类",
          ),
          BottomNavigationBarItem(
            icon: Icon(
              Icons.ac_unit,
              color: Colors.black,
            ),
            label: "示例",
          ),
          BottomNavigationBarItem(
            icon: Icon(
              Icons.settings,
              color: Colors.black,
            ),
            label: "设置",
          ),
        ],
      ),
    );
  }
}
