import 'package:flame/game.dart';
import 'package:flame/components.dart';
import '../components/square_component.dart';

class TwistedWarfareGame extends FlameGame {
  @override
  Future<void> onLoad() async {
    await super.onLoad();
    add(SquareComponent()
      ..position = size / 2
      ..anchor = Anchor.center);
  }
}
