import 'package:get/get.dart';
import 'package:flutter_neumorphic/flutter_neumorphic.dart';
import 'package:minerva_frontend/routes/splash_routers.dart';

void main() {
  runApp(const MinervaMainApp());
}

class MinervaMainApp extends StatelessWidget {
  const MinervaMainApp({ Key? key }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return GetMaterialApp(
      title: 'Minerva System',
      getPages: [
        ...SplashRouters.routers
      ],
    );
  }
}

