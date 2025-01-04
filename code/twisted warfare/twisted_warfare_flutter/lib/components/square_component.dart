import 'package:flame/components.dart';
import 'package:flutter/material.dart';

class SquareComponent extends PositionComponent {
  static const double defaultSize = 30.0;
  
  SquareComponent() : super(size: Vector2.all(defaultSize));

  @override
  void render(Canvas canvas) {
    canvas.drawRect(
      Rect.fromLTWH(0, 0, size.x, size.y),
      Paint()..color = Colors.blue,
    );
  }
}
