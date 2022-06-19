import 'package:get/get.dart';
import 'package:get/get_navigation/src/routes/get_route.dart';
import 'package:minerva_frontend/modules/main/main_page.dart';

class MainRouters {
  MainRouters._();

  static final routers = <GetPage>[
    GetPage(
      name: '/main',
      page: () => const MainPage(),
    ),
  ];
}
