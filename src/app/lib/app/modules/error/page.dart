import 'package:flutter/material.dart';
import 'package:get/get.dart';

import 'controller.dart';

class ErrorPage extends GetView<ErrorController> {
  const ErrorPage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Text("error");
  }
}
