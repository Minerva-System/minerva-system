import 'package:flutter/material.dart';
import 'package:flutter_neumorphic/flutter_neumorphic.dart';

class SplashPage extends StatelessWidget {
  const SplashPage({ Key? key }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      child: Container(
        padding: const EdgeInsets.symmetric(vertical: 90.0, horizontal: 100.0),
        color: const Color(0xFFDADEEF),
        child: Neumorphic(
          style: NeumorphicStyle(
            shape: NeumorphicShape.flat,
            boxShape: NeumorphicBoxShape.roundRect(BorderRadius.circular(12)), 
            depth: 6,
            intensity: 0.6,
            lightSource: LightSource.topLeft,
            color: const Color(0xFFDFE0E7),
          ),
          padding: const EdgeInsets.symmetric(vertical: 10.0, horizontal: 50.0),
          child: Center(
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              crossAxisAlignment: CrossAxisAlignment.center,
              children: <Widget>[
                // Minerva logo
                Container(
                  child: Neumorphic(
                    style: NeumorphicStyle(
                      shape: NeumorphicShape.concave,
                      boxShape: const NeumorphicBoxShape.circle(),
                      depth: -10,
                      intensity: 0.9,
                      color: const Color(0xFFDEE1ED),
                    ),
                    padding: const EdgeInsets.all(15.0),
                    child: Image.asset('assets/images/logo.png'),
                  ),
                  padding: const EdgeInsets.only(bottom: 50.0),
                ),
                // Writable input
               Material(
                 color: const Color(0x00FFFFFF),
                  child: Container(
                    child: Neumorphic(
                      style: NeumorphicStyle(
                        shape: NeumorphicShape.flat,
                        boxShape: NeumorphicBoxShape.roundRect(
                            BorderRadius.circular(50)),
                        depth: -2,
                        intensity: 1.5,
                        color: const Color(0xFFDEE1ED),
                      ),
                      child: Padding(
                        padding: const EdgeInsets.symmetric(
                            horizontal: 20.0, vertical: 5.0),
                        child: TextField(
                          obscureText: false,
                          decoration: InputDecoration(
                            isDense: true,
                            labelText: "Login",
                            labelStyle:
                                const TextStyle(color: Color(0xFF9A9A9A)),
                            filled: false,
                            border: InputBorder.none,
                          ),
                        ),
                      ),
                    ),
                    padding: const EdgeInsets.only(bottom: 25.0),
                  ),
                ),
                // Writable input
              Container(
                  child: Neumorphic(
                    style: NeumorphicStyle(
                      shape: NeumorphicShape.flat,
                      boxShape: NeumorphicBoxShape.roundRect(
                          BorderRadius.circular(50)),
                      depth: -2,
                      intensity: 1.5,
                      color: const Color(0xFFDEE1ED),
                    ),
                    padding: const EdgeInsets.symmetric(
                        vertical: 25.0, horizontal: 200.0),
                  ),
                  padding: const EdgeInsets.only(bottom: 25.0),
                ),
                // Login button
                NeumorphicButton(
                  margin: EdgeInsets.only(top: 12),
                  onPressed: () {},
                  child: Icon(
                    Icons.arrow_forward,
                    size: 50,
                  ),
                  style: NeumorphicStyle(
                    shape: NeumorphicShape.concave,
                    boxShape: NeumorphicBoxShape.circle(),
                    depth: 5,
                    intensity: 0.6,
                    color: const Color(0xFFDFE0E6),
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
