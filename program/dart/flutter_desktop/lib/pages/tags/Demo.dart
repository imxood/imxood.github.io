import 'package:flutter/material.dart';
import 'package:flutter_desktop/res/listData.dart';

class DemoPage extends StatefulWidget {
  DemoPage({Key key}) : super(key: key);

  @override
  _DemoPageState createState() => _DemoPageState();
}

class _DemoPageState extends State<DemoPage> {
  @override
  Widget build(BuildContext context) {
    return DemoLayout();
  }
}

class DemoLayout extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Row(
          children: [
            Expanded(
              child: Container(
                height: 200,
                color: Colors.black,
                child: Text(
                  '你好',
                  style: TextStyle(color: Colors.red),
                ),
                //   decoration: BoxDecoration(color: Colors.black),
              ),
            )
          ],
        ),
        SizedBox(height: 5),
        Row(
          children: [
            Expanded(
              flex: 2,
              child: Container(
                child: Image.network(
                  listData[0]["imageUrl"],
                  fit: BoxFit.cover,
                ),
              ),
            ),
            SizedBox(width: 5),
            Expanded(
              flex: 1,
              child: ListView(
                shrinkWrap: true,
                children: [
                  Container(
                    child: Image.network(
                      listData[0]["imageUrl"],
                      fit: BoxFit.fill,
                    ),
                  ),
                  Container(
                    child: Image.network(
                      listData[0]["imageUrl"],
                      fit: BoxFit.fill,
                    ),
                  ),
                ],
              ),
            )
          ],
        )
      ],
    );
  }
}
