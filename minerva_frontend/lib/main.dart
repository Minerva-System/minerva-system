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
    return GetMaterialApp(
      initialRoute: '/',
      title: 'Minerva System',
      getPages: [
        ...LoginRouters.routers,
        ...MainRouters.routers,
      ],
    );
  }
}
