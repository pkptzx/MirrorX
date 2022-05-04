// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`.

// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, prefer_single_quotes

import 'dart:convert';
import 'dart:typed_data';

import 'dart:convert';
import 'dart:typed_data';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'dart:ffi' as ffi;

abstract class MirrorXCore {
  Future<void> init(
      {required String osName,
      required String osVersion,
      required String configDir,
      dynamic hint});

  Future<String?> configReadDeviceId({dynamic hint});

  Future<void> configSaveDeviceId({required String deviceId, dynamic hint});

  Future<int?> configReadDeviceIdExpiration({dynamic hint});

  Future<void> configSaveDeviceIdExpiration(
      {required int timeStamp, dynamic hint});

  Future<String?> configReadDevicePassword({dynamic hint});

  Future<void> configSaveDevicePassword(
      {required String devicePassword, dynamic hint});

  Future<void> socketDesktopConnect(
      {required String remoteDeviceId, dynamic hint});

  Future<bool> socketDesktopKeyExchangeAndPasswordVerify(
      {required String remoteDeviceId, required String password, dynamic hint});

  Future<StartMediaTransmissionReply> socketDesktopStartMediaTransmission(
      {required String remoteDeviceId, dynamic hint});

  Future<String> utilityGenerateDevicePassword({dynamic hint});
}

class StartMediaTransmissionReply {
  final String osName;
  final String osVersion;
  final String videoType;
  final String audioType;

  StartMediaTransmissionReply({
    required this.osName,
    required this.osVersion,
    required this.videoType,
    required this.audioType,
  });
}

class MirrorXCoreImpl extends FlutterRustBridgeBase<MirrorXCoreWire>
    implements MirrorXCore {
  factory MirrorXCoreImpl(ffi.DynamicLibrary dylib) =>
      MirrorXCoreImpl.raw(MirrorXCoreWire(dylib));

  MirrorXCoreImpl.raw(MirrorXCoreWire inner) : super(inner);

  Future<void> init(
          {required String osName,
          required String osVersion,
          required String configDir,
          dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_init(port_, _api2wire_String(osName),
            _api2wire_String(osVersion), _api2wire_String(configDir)),
        parseSuccessData: _wire2api_unit,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "init",
          argNames: ["osName", "osVersion", "configDir"],
        ),
        argValues: [osName, osVersion, configDir],
        hint: hint,
      ));

  Future<String?> configReadDeviceId({dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_config_read_device_id(port_),
        parseSuccessData: _wire2api_opt_String,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "config_read_device_id",
          argNames: [],
        ),
        argValues: [],
        hint: hint,
      ));

  Future<void> configSaveDeviceId({required String deviceId, dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) =>
            inner.wire_config_save_device_id(port_, _api2wire_String(deviceId)),
        parseSuccessData: _wire2api_unit,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "config_save_device_id",
          argNames: ["deviceId"],
        ),
        argValues: [deviceId],
        hint: hint,
      ));

  Future<int?> configReadDeviceIdExpiration({dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_config_read_device_id_expiration(port_),
        parseSuccessData: _wire2api_opt_box_autoadd_u32,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "config_read_device_id_expiration",
          argNames: [],
        ),
        argValues: [],
        hint: hint,
      ));

  Future<void> configSaveDeviceIdExpiration(
          {required int timeStamp, dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_config_save_device_id_expiration(
            port_, _api2wire_u32(timeStamp)),
        parseSuccessData: _wire2api_unit,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "config_save_device_id_expiration",
          argNames: ["timeStamp"],
        ),
        argValues: [timeStamp],
        hint: hint,
      ));

  Future<String?> configReadDevicePassword({dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_config_read_device_password(port_),
        parseSuccessData: _wire2api_opt_String,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "config_read_device_password",
          argNames: [],
        ),
        argValues: [],
        hint: hint,
      ));

  Future<void> configSaveDevicePassword(
          {required String devicePassword, dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_config_save_device_password(
            port_, _api2wire_String(devicePassword)),
        parseSuccessData: _wire2api_unit,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "config_save_device_password",
          argNames: ["devicePassword"],
        ),
        argValues: [devicePassword],
        hint: hint,
      ));

  Future<void> socketDesktopConnect(
          {required String remoteDeviceId, dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_socket_desktop_connect(
            port_, _api2wire_String(remoteDeviceId)),
        parseSuccessData: _wire2api_unit,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "socket_desktop_connect",
          argNames: ["remoteDeviceId"],
        ),
        argValues: [remoteDeviceId],
        hint: hint,
      ));

  Future<bool> socketDesktopKeyExchangeAndPasswordVerify(
          {required String remoteDeviceId,
          required String password,
          dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) =>
            inner.wire_socket_desktop_key_exchange_and_password_verify(port_,
                _api2wire_String(remoteDeviceId), _api2wire_String(password)),
        parseSuccessData: _wire2api_bool,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "socket_desktop_key_exchange_and_password_verify",
          argNames: ["remoteDeviceId", "password"],
        ),
        argValues: [remoteDeviceId, password],
        hint: hint,
      ));

  Future<StartMediaTransmissionReply> socketDesktopStartMediaTransmission(
          {required String remoteDeviceId, dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_socket_desktop_start_media_transmission(
            port_, _api2wire_String(remoteDeviceId)),
        parseSuccessData: _wire2api_start_media_transmission_reply,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "socket_desktop_start_media_transmission",
          argNames: ["remoteDeviceId"],
        ),
        argValues: [remoteDeviceId],
        hint: hint,
      ));

  Future<String> utilityGenerateDevicePassword({dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_utility_generate_device_password(port_),
        parseSuccessData: _wire2api_String,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "utility_generate_device_password",
          argNames: [],
        ),
        argValues: [],
        hint: hint,
      ));

  // Section: api2wire
  ffi.Pointer<wire_uint_8_list> _api2wire_String(String raw) {
    return _api2wire_uint_8_list(utf8.encoder.convert(raw));
  }

  int _api2wire_u32(int raw) {
    return raw;
  }

  int _api2wire_u8(int raw) {
    return raw;
  }

  ffi.Pointer<wire_uint_8_list> _api2wire_uint_8_list(Uint8List raw) {
    final ans = inner.new_uint_8_list(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

  // Section: api_fill_to_wire

}

// Section: wire2api
String _wire2api_String(dynamic raw) {
  return raw as String;
}

bool _wire2api_bool(dynamic raw) {
  return raw as bool;
}

int _wire2api_box_autoadd_u32(dynamic raw) {
  return raw as int;
}

String? _wire2api_opt_String(dynamic raw) {
  return raw == null ? null : _wire2api_String(raw);
}

int? _wire2api_opt_box_autoadd_u32(dynamic raw) {
  return raw == null ? null : _wire2api_box_autoadd_u32(raw);
}

StartMediaTransmissionReply _wire2api_start_media_transmission_reply(
    dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 4)
    throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
  return StartMediaTransmissionReply(
    osName: _wire2api_String(arr[0]),
    osVersion: _wire2api_String(arr[1]),
    videoType: _wire2api_String(arr[2]),
    audioType: _wire2api_String(arr[3]),
  );
}

int _wire2api_u32(dynamic raw) {
  return raw as int;
}

int _wire2api_u8(dynamic raw) {
  return raw as int;
}

Uint8List _wire2api_uint_8_list(dynamic raw) {
  return raw as Uint8List;
}

void _wire2api_unit(dynamic raw) {
  return;
}

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names

// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.

/// generated by flutter_rust_bridge
class MirrorXCoreWire implements FlutterRustBridgeWireBase {
  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  MirrorXCoreWire(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  MirrorXCoreWire.fromLookup(
      ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
          lookup)
      : _lookup = lookup;

  void wire_init(
    int port_,
    ffi.Pointer<wire_uint_8_list> os_name,
    ffi.Pointer<wire_uint_8_list> os_version,
    ffi.Pointer<wire_uint_8_list> config_dir,
  ) {
    return _wire_init(
      port_,
      os_name,
      os_version,
      config_dir,
    );
  }

  late final _wire_initPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>)>>('wire_init');
  late final _wire_init = _wire_initPtr.asFunction<
      void Function(int, ffi.Pointer<wire_uint_8_list>,
          ffi.Pointer<wire_uint_8_list>, ffi.Pointer<wire_uint_8_list>)>();

  void wire_config_read_device_id(
    int port_,
  ) {
    return _wire_config_read_device_id(
      port_,
    );
  }

  late final _wire_config_read_device_idPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_config_read_device_id');
  late final _wire_config_read_device_id =
      _wire_config_read_device_idPtr.asFunction<void Function(int)>();

  void wire_config_save_device_id(
    int port_,
    ffi.Pointer<wire_uint_8_list> device_id,
  ) {
    return _wire_config_save_device_id(
      port_,
      device_id,
    );
  }

  late final _wire_config_save_device_idPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_uint_8_list>)>>('wire_config_save_device_id');
  late final _wire_config_save_device_id = _wire_config_save_device_idPtr
      .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_config_read_device_id_expiration(
    int port_,
  ) {
    return _wire_config_read_device_id_expiration(
      port_,
    );
  }

  late final _wire_config_read_device_id_expirationPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_config_read_device_id_expiration');
  late final _wire_config_read_device_id_expiration =
      _wire_config_read_device_id_expirationPtr
          .asFunction<void Function(int)>();

  void wire_config_save_device_id_expiration(
    int port_,
    int time_stamp,
  ) {
    return _wire_config_save_device_id_expiration(
      port_,
      time_stamp,
    );
  }

  late final _wire_config_save_device_id_expirationPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Uint32)>>(
          'wire_config_save_device_id_expiration');
  late final _wire_config_save_device_id_expiration =
      _wire_config_save_device_id_expirationPtr
          .asFunction<void Function(int, int)>();

  void wire_config_read_device_password(
    int port_,
  ) {
    return _wire_config_read_device_password(
      port_,
    );
  }

  late final _wire_config_read_device_passwordPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_config_read_device_password');
  late final _wire_config_read_device_password =
      _wire_config_read_device_passwordPtr.asFunction<void Function(int)>();

  void wire_config_save_device_password(
    int port_,
    ffi.Pointer<wire_uint_8_list> device_password,
  ) {
    return _wire_config_save_device_password(
      port_,
      device_password,
    );
  }

  late final _wire_config_save_device_passwordPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<wire_uint_8_list>)>>(
      'wire_config_save_device_password');
  late final _wire_config_save_device_password =
      _wire_config_save_device_passwordPtr
          .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_socket_desktop_connect(
    int port_,
    ffi.Pointer<wire_uint_8_list> remote_device_id,
  ) {
    return _wire_socket_desktop_connect(
      port_,
      remote_device_id,
    );
  }

  late final _wire_socket_desktop_connectPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_uint_8_list>)>>('wire_socket_desktop_connect');
  late final _wire_socket_desktop_connect = _wire_socket_desktop_connectPtr
      .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_socket_desktop_key_exchange_and_password_verify(
    int port_,
    ffi.Pointer<wire_uint_8_list> remote_device_id,
    ffi.Pointer<wire_uint_8_list> password,
  ) {
    return _wire_socket_desktop_key_exchange_and_password_verify(
      port_,
      remote_device_id,
      password,
    );
  }

  late final _wire_socket_desktop_key_exchange_and_password_verifyPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<wire_uint_8_list>,
                  ffi.Pointer<wire_uint_8_list>)>>(
      'wire_socket_desktop_key_exchange_and_password_verify');
  late final _wire_socket_desktop_key_exchange_and_password_verify =
      _wire_socket_desktop_key_exchange_and_password_verifyPtr.asFunction<
          void Function(int, ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>)>();

  void wire_socket_desktop_start_media_transmission(
    int port_,
    ffi.Pointer<wire_uint_8_list> remote_device_id,
  ) {
    return _wire_socket_desktop_start_media_transmission(
      port_,
      remote_device_id,
    );
  }

  late final _wire_socket_desktop_start_media_transmissionPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<wire_uint_8_list>)>>(
      'wire_socket_desktop_start_media_transmission');
  late final _wire_socket_desktop_start_media_transmission =
      _wire_socket_desktop_start_media_transmissionPtr
          .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_utility_generate_device_password(
    int port_,
  ) {
    return _wire_utility_generate_device_password(
      port_,
    );
  }

  late final _wire_utility_generate_device_passwordPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_utility_generate_device_password');
  late final _wire_utility_generate_device_password =
      _wire_utility_generate_device_passwordPtr
          .asFunction<void Function(int)>();

  ffi.Pointer<wire_uint_8_list> new_uint_8_list(
    int len,
  ) {
    return _new_uint_8_list(
      len,
    );
  }

  late final _new_uint_8_listPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_uint_8_list> Function(
              ffi.Int32)>>('new_uint_8_list');
  late final _new_uint_8_list = _new_uint_8_listPtr
      .asFunction<ffi.Pointer<wire_uint_8_list> Function(int)>();

  void free_WireSyncReturnStruct(
    WireSyncReturnStruct val,
  ) {
    return _free_WireSyncReturnStruct(
      val,
    );
  }

  late final _free_WireSyncReturnStructPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(WireSyncReturnStruct)>>(
          'free_WireSyncReturnStruct');
  late final _free_WireSyncReturnStruct = _free_WireSyncReturnStructPtr
      .asFunction<void Function(WireSyncReturnStruct)>();

  void store_dart_post_cobject(
    DartPostCObjectFnType ptr,
  ) {
    return _store_dart_post_cobject(
      ptr,
    );
  }

  late final _store_dart_post_cobjectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(DartPostCObjectFnType)>>(
          'store_dart_post_cobject');
  late final _store_dart_post_cobject = _store_dart_post_cobjectPtr
      .asFunction<void Function(DartPostCObjectFnType)>();
}

class wire_uint_8_list extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

typedef DartPostCObjectFnType = ffi.Pointer<
    ffi.NativeFunction<ffi.Uint8 Function(DartPort, ffi.Pointer<ffi.Void>)>>;
typedef DartPort = ffi.Int64;
