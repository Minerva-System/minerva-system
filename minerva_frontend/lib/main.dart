import 'package:flutter/foundation.dart';
import 'package:get/get.dart';
import 'package:flutter_neumorphic/flutter_neumorphic.dart';
import 'package:minerva_frontend/routes/login_routers.dart';
import 'package:minerva_frontend/routes/main_routes.dart';

void main() {
  runApp(const MinervaMainApp());
}

class MinervaMainApp extends StatelessWidget {
  const MinervaMainApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    // Start normally at root, getting a 404.
    // Otherwise use the 'minerva' tenant
    var initialRoute = '/';
    if (kDebugMode) {
      initialRoute = '/minerva/';
    }
    
    return GetMaterialApp(
      initialRoute: initialRoute,
      title: 'Minerva System',
      getPages: [
        ...LoginRouters.routers,
        ...MainRouters.routers,
      ],
    );
  }
}
