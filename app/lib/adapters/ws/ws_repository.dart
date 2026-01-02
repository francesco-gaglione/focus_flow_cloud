import 'dart:async';
import 'dart:convert';

import 'package:focus_flow_app/adapters/dtos/ws_dtos.dart';
import 'package:logger/logger.dart';
import 'package:uuid/uuid.dart';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'package:focus_flow_app/core/services/token_service.dart';

class WebsocketRepository {
  final Logger logger = Logger();
  final String wsUrl;
  final TokenService _tokenService;
  final Uuid _uuid = const Uuid();

  WebSocketChannel? _ws;
  StreamSubscription? _subscription;
  bool _isConnected = false;

  // Stream controllers for different message types
  final _serverResponseController =
      StreamController<ServerResponse>.broadcast();
  final _broadcastEventController =
      StreamController<BroadcastEvent>.broadcast();
  final _pomodoroStateController =
      StreamController<UpdatePomodoroState>.broadcast();
  final _connectionStatusController = StreamController<bool>.broadcast();

  // Streams for listening to messages
  Stream<ServerResponse> get serverResponses =>
      _serverResponseController.stream;
  Stream<BroadcastEvent> get broadcastEvents =>
      _broadcastEventController.stream;
  Stream<UpdatePomodoroState> get pomodoroStateUpdates =>
      _pomodoroStateController.stream;
  Stream<bool> get connectionStatus => _connectionStatusController.stream;

  WebsocketRepository(this.wsUrl, this._tokenService);

  /// Connect to the WebSocket server
  Future<void> connect() async {
    try {
      // If already connected, do nothing
      if (_ws != null && _isConnected) {
        logger.d('Already connected to $wsUrl');
        _connectionStatusController.add(true);
        return;
      }

      logger.d('Connecting to $wsUrl...');
      final token = _tokenService.getToken();

      // Pass token as query parameter instead of header to avoid server 500 errors
      // during handshake if it doesn't handle headers well.
      String url = wsUrl;
      if (token != null) {
        final separator = url.contains('?') ? '&' : '?';
        url = '$url${separator}token=$token';
      }

      final uri = Uri.parse(url);
      _ws = WebSocketChannel.connect(uri);

      // Since WebSocketChannel.connect doesn't await the connection, we assume it's connecting.
      // The stream listen will handle errors and close events.
      logger.i('Connecting to $url');
      _isConnected = true;
      _connectionStatusController.add(true);

      // Start listening to incoming messages
      _subscription = _ws!.stream.listen(
        _handleMessage,
        onError: (error) {
          logger.e('WebSocket error: $error');
          _isConnected = false;
          _connectionStatusController.add(false);
          _ws = null;
        },
        onDone: () {
          logger.w('WebSocket connection closed');
          _isConnected = false;
          _connectionStatusController.add(false);
          _ws = null;
        },
      );

      // Attempt to wait for the connection to be established or fail?
      // WebSocketChannel doesn't expose readyState directly in a cross-platform way easily without casting.
      // We rely on the stream.
    } catch (e) {
      logger.e('Failed to connect to $wsUrl: $e');
      _isConnected = false;
      _connectionStatusController.add(false);
      _ws = null;
      // Don't throw, just log, so the app doesn't crash on reconnection attempts
    }
  }

  /// Handle incoming WebSocket messages
  void _handleMessage(dynamic data) {
    try {
      logger.d('Received message: $data');
      final jsonData = json.decode(data as String) as Map<String, dynamic>;

      // Check which type of message we received based on the keys
      if (jsonData.containsKey('success')) {
        // Success response
        final successData = jsonData['success'] as Map<String, dynamic>;
        final response = ServerResponse.success(
          message: successData['message'] as String,
          requestId: successData['requestId'] as String?,
        );
        _serverResponseController.add(response);
      } else if (jsonData.containsKey('error')) {
        // Error response
        final errorData = jsonData['error'] as Map<String, dynamic>;
        final response = ServerResponse.error(
          code: errorData['code'] as String,
          message: errorData['message'] as String,
          requestId: errorData['requestId'] as String?,
        );
        _serverResponseController.add(response);
      } else if (jsonData.containsKey('syncData')) {
        // Sync data response
        final syncData = jsonData['syncData'] as Map<String, dynamic>;
        final pomodoroState = UpdatePomodoroState.fromJson(syncData);
        final response = ServerResponse.syncData(pomodoroState);
        _serverResponseController.add(response);
        _pomodoroStateController.add(pomodoroState);
      } else if (jsonData.containsKey('pomodoroSessionUpdate')) {
        // Broadcast event
        final updateData =
            jsonData['pomodoroSessionUpdate'] as Map<String, dynamic>;
        final pomodoroState = UpdatePomodoroState.fromJson(updateData);
        final event = BroadcastEvent.pomodoroSessionUpdate(pomodoroState);
        _broadcastEventController.add(event);
        _pomodoroStateController.add(pomodoroState);
      } else {
        logger.w('Unknown message type received: $jsonData');
      }
    } catch (e, stackTrace) {
      logger.e('Error parsing message: $e\n$stackTrace');
    }
  }

  /// Send a client message to the server
  void _sendMessage(ClientMessage message, {String? requestId}) {
    if (_ws == null) {
      logger.w('Cannot send message: WebSocket is not connected');
      _connectionStatusController.add(false);
      return;
    }

    try {
      // Build JSON in the format expected by the server:
      // {"requestId": "...", "messageType": {payload}}
      // Example: {"requestId": "123", "requestSync": {}}
      final reqId = requestId ?? _uuid.v4();
      final Map<String, dynamic> jsonMessage = {'requestId': reqId};

      // Add the message type and payload based on the message variant
      message.when(
        requestSync: () {
          jsonMessage['requestSync'] = {};
        },
        startEvent: () {
          jsonMessage['startEvent'] = {};
        },
        breakEvent: () {
          jsonMessage['breakEvent'] = {};
        },
        terminateEvent: () {
          jsonMessage['terminateEvent'] = {};
        },
        updatePomodoroContext: (payload) {
          jsonMessage['updatePomodoroContext'] = payload.toJson();
        },
        updateNote: (payload) {
          jsonMessage['updateNote'] = payload.toJson();
        },
        updateConcentrationScore: (payload) {
          jsonMessage['updateConcentrationScore'] = payload.toJson();
        },
      );

      final jsonString = json.encode(jsonMessage);
      logger.d('Sending message: $jsonString');
      _ws!.sink.add(jsonString);
    } catch (e) {
      logger.e('Error sending message: $e');
    }
  }

  // -----------------------------------------------------------------------------
  // Client Message Methods
  // -----------------------------------------------------------------------------

  /// Request synchronization with the server
  void requestSync({String? requestId}) {
    _sendMessage(const ClientMessage.requestSync(), requestId: requestId);
  }

  /// Send start event (start a focus session)
  void sendStartEvent({String? requestId}) {
    _sendMessage(const ClientMessage.startEvent(), requestId: requestId);
  }

  /// Send break event (start a break)
  void sendBreakEvent({String? requestId}) {
    _sendMessage(const ClientMessage.breakEvent(), requestId: requestId);
  }

  /// Send terminate event (end current session)
  void sendTerminateEvent({String? requestId}) {
    _sendMessage(const ClientMessage.terminateEvent(), requestId: requestId);
  }

  /// Update the pomodoro context (category and task)
  void updatePomodoroContext({
    String? categoryId,
    String? taskId,
    String? requestId,
  }) {
    final payload = UpdatePomodoroContext(
      categoryId: categoryId,
      taskId: taskId,
    );
    _sendMessage(
      ClientMessage.updatePomodoroContext(payload),
      requestId: requestId,
    );
  }

  /// Update the session note
  void updateNote(String note, {String? requestId}) {
    final payload = NoteUpdate(newNote: note);
    _sendMessage(ClientMessage.updateNote(payload), requestId: requestId);
  }

  /// Update the concentration score
  void updateConcentrationScore(int score, {String? requestId}) {
    final payload = UpdateConcentrationScore(concentrationScore: score);
    _sendMessage(
      ClientMessage.updateConcentrationScore(payload),
      requestId: requestId,
    );
  }

  // -----------------------------------------------------------------------------
  // Legacy Methods (for backward compatibility)
  // -----------------------------------------------------------------------------

  /// Register a raw listener (deprecated - use typed streams instead)
  @Deprecated(
    'Use serverResponses, broadcastEvents, or pomodoroStateUpdates streams instead',
  )
  void registerListener(Function(dynamic) callback) {
    if (_ws != null) {
      _subscription = _ws!.stream.listen(callback);
    } else {
      logger.w('Cannot register listener: WebSocket is not initialized');
    }
  }

  /// Send raw string message (deprecated - use typed methods instead)
  @Deprecated(
    'Use typed message methods like requestSync(), sendStartEvent(), etc.',
  )
  void send(String message) {
    if (_ws != null) {
      logger.d('Sending raw message: $message');
      _ws!.sink.add(message);
    } else {
      logger.w('Cannot send message: WebSocket is not connected');
    }
  }

  // -----------------------------------------------------------------------------
  // Connection Management
  // -----------------------------------------------------------------------------

  /// Check if WebSocket is connected
  bool isConnected() {
    return _isConnected;
  }

  /// Disconnect from the WebSocket server
  Future<void> disconnect() async {
    if (_ws != null) {
      await _subscription?.cancel();
      await _ws!.sink.close();
      _ws = null;
      _isConnected = false;
      logger.i('Disconnected from websocket');
      _connectionStatusController.add(false);
    }

    // Close stream controllers
    await _serverResponseController.close();
    await _broadcastEventController.close();
    await _pomodoroStateController.close();
    await _connectionStatusController.close();
  }
}
