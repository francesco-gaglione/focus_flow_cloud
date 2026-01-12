import 'package:flutter/material.dart';
import '../../../core/services/configuration_service.dart';
import '../../../main.dart'; // To access restartApp
import 'package:flutter_bloc/flutter_bloc.dart';
import '../../auth/cubit/auth_cubit.dart';

class BackendConfigPage extends StatefulWidget {
  final ConfigurationService configService;
  final bool isFirstRun;

  const BackendConfigPage({
    super.key,
    required this.configService,
    this.isFirstRun = false,
  });

  @override
  State<BackendConfigPage> createState() => _BackendConfigPageState();
}

class _BackendConfigPageState extends State<BackendConfigPage> {
  late TextEditingController _urlController;
  final _formKey = GlobalKey<FormState>();
  bool _isLoading = false;

  @override
  void initState() {
    super.initState();
    _urlController = TextEditingController(text: widget.configService.apiBaseUrl ?? '');
  }

  @override
  void dispose() {
    _urlController.dispose();
    super.dispose();
  }

  Future<void> _saveAndConnect() async {
    if (!_formKey.currentState!.validate()) return;

    setState(() => _isLoading = true);

    try {
      final url = _urlController.text.trim();
      
      // Basic validation
      final uri = Uri.tryParse(url);
      if (uri == null || !uri.hasScheme || !uri.hasAuthority) {
        throw Exception('Invalid URL format');
      }

      await widget.configService.setApiBaseUrl(url);

      // Force logout to clear tokens from the previous server
      if (mounted) {
        try {
          await context.read<AuthCubit>().logout();
        } catch (_) {
          // Ignore if AuthCubit is not found (e.g. during first run/onboarding)
        }
      }

      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Configuration saved! Restarting...')),
        );
        
        // Restart app flow
        restartApp(context);
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Error: ${e.toString()}')),
        );
      }
    } finally {
      if (mounted) {
        setState(() => _isLoading = false);
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Server Configuration'),
        automaticallyImplyLeading: !widget.isFirstRun,
      ),
      body: Center(
        child: SingleChildScrollView(
          padding: const EdgeInsets.all(24.0),
          child: Form(
            key: _formKey,
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                Icon(
                  Icons.cloud_sync,
                  size: 64,
                  color: Theme.of(context).colorScheme.primary,
                ),
                const SizedBox(height: 32),
                Text(
                  'Connect to Server',
                  style: Theme.of(context).textTheme.headlineSmall,
                  textAlign: TextAlign.center,
                ),
                const SizedBox(height: 16),
                Text(
                  'Enter the address of your Focus Flow server.',
                  style: Theme.of(context).textTheme.bodyMedium,
                  textAlign: TextAlign.center,
                ),
                const SizedBox(height: 32),
                TextFormField(
                  controller: _urlController,
                  decoration: InputDecoration(
                    labelText: 'Server URL',
                    hintText: 'http://192.168.1.5:3000',
                    border: OutlineInputBorder(),
                    prefixIcon: Icon(Icons.link),
                  ),
                  keyboardType: TextInputType.url,
                  validator: (value) {
                    if (value == null || value.isEmpty) {
                      return 'Please enter a URL';
                    }
                    if (!value.startsWith('http://') && !value.startsWith('https://')) {
                      return 'URL must start with http:// or https://';
                    }
                    return null;
                  },
                ),
                const SizedBox(height: 32),
                FilledButton(
                  onPressed: _isLoading ? null : _saveAndConnect,
                  child: Padding(
                    padding: const EdgeInsets.all(16.0),
                    child: _isLoading 
                      ? SizedBox(
                          height: 20, 
                          width: 20, 
                          child: CircularProgressIndicator(strokeWidth: 2)
                        )
                      : Text('Save & Connect'),
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
