import 'dart:async';
import 'dart:ffi';
import 'dart:io';

import 'package:flutter/services.dart';

import 'bridge_generated.dart';

class AppPlugin {
  static const MethodChannel _channel = MethodChannel('app_plugin');

  static Future<Map?> videoTextureRegister() async {
    try {
      Map? res = await _channel.invokeMethod('video_texture_register');
      return res;
    } catch (error) {
      return Future.error("VideoTextureRegister: call error($error)");
    }
  }

  static Future<void> deregisterTextureID(int textureID) async {
    await _channel.invokeMethod(
        'video_texture_deregister', <String, int>{'texture_id': textureID});
  }
}

DynamicLibrary _openLibrary() {
  if (Platform.isMacOS || Platform.isIOS) {
    return DynamicLibrary.executable();
  } else if (Platform.isWindows) {
    return DynamicLibrary.open("mirrorx_core.dll");
  } else {
    throw UnsupportedError("Not supported platform yet");
  }
}
