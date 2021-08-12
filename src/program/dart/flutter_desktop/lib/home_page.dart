import 'package:flutter/material.dart';
import './iic_page.dart';
import './spi_page.dart';
import './log_page.dart';

class HomePage extends StatelessWidget {
  const HomePage({Key key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(
        child: Column(
          children: [
            Container(
              width: double.infinity,
              child: Row(
                children: [
                  Expanded(
                    child: IicPage(),
                  ),
                  Expanded(
                    child: SpiPage(),
                  ),
                ],
              ),
            ),
            LogPage()
          ],
        ),
      ),
    );
  }
}
