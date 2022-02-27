import 'package:flutter/material.dart';

class IicPage extends StatefulWidget {
  IicPage({Key key}) : super(key: key);

  @override
  _IicPageState createState() => _IicPageState();
}

class IIcSettings {
  String addr;
}

class _IicPageState extends State<IicPage> {
  var isMaster = true;
  Widget build(BuildContext context) {
    var masterBtn = ChoiceChip(
        selected: isMaster,
        label: Text('Master'),
        onSelected: (bool selected) {
          setState(() {
            isMaster = true;
          });
        });

    var slaveBtn = ChoiceChip(
        selected: !isMaster,
        label: Text('Slave'),
        onSelected: (bool selected) {
          setState(() {
            isMaster = false;
          });
        });

    var header = Stack(
      alignment: Alignment.bottomRight,
      children: [
        Container(
          width: double.infinity,
          color: Colors.black12,
          child: Align(
            alignment: Alignment.topLeft,
            child: Text(
              "I2C Control",
              style: TextStyle(
                fontSize: 45,
                color: Colors.black87,
              ),
            ),
          ),
        ),
        Wrap(
          children: [
            masterBtn,
            const SizedBox(width: 8),
            slaveBtn,
          ],
        ),
      ],
    );

    var masterView = Container(
      height: 100,
      child: Column(
        children: [
          Row(
            children: [
              Text('地址' + ':'),
              TextField(
                decoration: InputDecoration(
                  border: OutlineInputBorder(),
                  labelText: '地址',
                ),
              ),
            ],
          )
        ],
      ),
    );

    var slaveView = Container(
      height: 100,
      child: Text("slaveView"),
    );

    return Container(
      height: 400,
      child: Column(
        children: [
          header,
          if (isMaster) masterView else slaveView,
        ],
      ),
    );
  }
}
