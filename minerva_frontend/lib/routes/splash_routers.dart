import 'package:get/get_navigation/src/routes/get_route.dart';
import 'package:minerva_frontend/modules/splash/splash_page.dart';

class SplashRouters {
  SplashRouters._();

  static final routers = <GetPage>[
    GetPage(
      name: '/',
      page: () => const SplashPage(),
    )
  ];
}
