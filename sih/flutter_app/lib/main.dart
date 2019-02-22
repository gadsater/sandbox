import 'package:flutter/material.dart';

void main() => runApp(MyApp());

class MyApp extends StatelessWidget {
  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: Container(
        child: MainNavBar(),
      ),
    );
  }
}

class MainNavBar extends StatefulWidget {
  MainNavBar({Key key, this.title}) : super(key: key);

  final String title;

  @override
  _MainNavBar createState() => _MainNavBar();
}

class _MainNavBar extends State<MainNavBar> {
  int _counter = 0;

  void _incrementCounter() {
    setState(() {
      _counter++;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: 
    );
  }
}
