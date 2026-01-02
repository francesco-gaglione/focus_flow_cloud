import 'package:easy_localization/easy_localization.dart' hide TextDirection;
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import '../app/locale_cubit.dart';
import '../app/theme_cubit.dart';
import '../../domain/usecases/get_app_version.dart';
import '../../core/di/service_locator.dart';
import 'cubit/account_cubit.dart';
import 'cubit/account_state.dart';
import '../auth/cubit/auth_cubit.dart';
import 'bloc/settings_bloc.dart';
import 'bloc/settings_event.dart';
import 'bloc/settings_state.dart';
import '../../domain/entities/note_template.dart';
import '../widgets/common/markdown_editor_input.dart';

class SettingsPage extends StatefulWidget {
  const SettingsPage({super.key});

  @override
  State<SettingsPage> createState() => _SettingsPageState();
}

class _SettingsPageState extends State<SettingsPage> {
  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create: (context) => sl<SettingsBloc>()..add(LoadSettings()),
      child: Scaffold(
      appBar: AppBar(title: Text(context.tr('settings.title'))),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          Card(
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Icon(
                        Icons.palette_outlined,
                        color: Theme.of(context).colorScheme.primary,
                      ),
                      const SizedBox(width: 12),
                      Text(
                        context.tr('settings.appearance'),
                        style: Theme.of(context).textTheme.titleLarge,
                      ),
                    ],
                  ),
                  const SizedBox(height: 16),
                  BlocBuilder<ThemeCubit, ThemeState>(
                    builder: (context, state) {
                      return Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          SwitchListTile(
                            title: Text(context.tr('settings.dark_mode')),
                            subtitle: Text(
                              state.isDarkMode
                                  ? context.tr('settings.switch_to_light')
                                  : context.tr('settings.switch_to_dark'),
                            ),
                            value: state.isDarkMode,
                            onChanged:
                                state.isLoading
                                    ? null
                                    : (_) {
                                      context.read<ThemeCubit>().toggleTheme();
                                    },
                            secondary: Icon(
                              state.isDarkMode
                                  ? Icons.dark_mode
                                  : Icons.light_mode,
                              color: Theme.of(context).colorScheme.primary,
                            ),
                          ),
                          ListTile(
                            title: Text(context.tr('settings.language')),
                            subtitle: Text(
                              context.locale.languageCode == 'en'
                                  ? context.tr('settings.english')
                                  : context.tr('settings.italian'),
                            ),
                            leading: Icon(
                              Icons.language,
                              color: Theme.of(context).colorScheme.primary,
                            ),
                            onTap: () {
                              showDialog(
                                context: context,
                                builder:
                                    (context) => SimpleDialog(
                                      title: Text(
                                        context.tr('settings.select_language'),
                                      ),
                                      children: [
                                        SimpleDialogOption(
                                          onPressed: () {
                                            const locale = Locale('en');
                                            context
                                                .read<LocaleCubit>()
                                                .setLocale(locale);
                                            Navigator.pop(context);
                                          },
                                          child: Text(
                                            context.tr('settings.english'),
                                          ),
                                        ),
                                        SimpleDialogOption(
                                          onPressed: () {
                                            const locale = Locale('it');
                                            context
                                                .read<LocaleCubit>()
                                                .setLocale(locale);
                                            Navigator.pop(context);
                                          },
                                          child: Text(
                                            context.tr('settings.italian'),
                                          ),
                                        ),
                                      ],
                                    ),
                              );
                            },
                          ),
                        ],
                      );
                    },
                  ),
                ],
              ),
            ),
          ),
          const SizedBox(height: 16),
          _buildNoteTemplateSection(context),
          const SizedBox(height: 16),
          _buildAccountSection(context),
          const SizedBox(height: 16),
          Card(
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Icon(
                        Icons.info_outline,
                        color: Theme.of(context).colorScheme.primary,
                      ),
                      const SizedBox(width: 12),
                      Text(
                        context.tr('settings.about'),
                        style: Theme.of(context).textTheme.titleLarge,
                      ),
                    ],
                  ),
                  const SizedBox(height: 16),
                  FutureBuilder<String>(
                    future: sl<GetAppVersion>()(),
                    builder: (context, snapshot) {
                      return ListTile(
                        title: Text(context.tr('settings.version')),
                        subtitle: Text(snapshot.data ?? '...'),
                        leading: const Icon(Icons.code),
                      );
                    },
                  ),
                  ListTile(
                    title: Text(context.tr('settings.app_name')),
                    subtitle: Text(context.tr('settings.app_description')),
                    leading: const Icon(Icons.timer),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
      ),
    );
  }

  Widget _buildAccountSection(BuildContext context) {
    return BlocProvider(
      create: (context) => sl<AccountCubit>()..loadUserInfo(),
      child: BlocConsumer<AccountCubit, AccountState>(
        listener: (context, state) {
          if (state is AccountSuccess) {
            ScaffoldMessenger.of(
              context,
            ).showSnackBar(SnackBar(content: Text(state.message)));
          } else if (state is AccountError) {
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: Text(state.message),
                backgroundColor: Colors.red,
              ),
            );
          }
        },
        builder: (context, state) {
          String? currentUsername;
          bool isAdmin = false;
          if (state is AccountLoaded) {
            currentUsername = state.userInfo.username;
            isAdmin = state.userInfo.role == 'Admin';
          }

          return Card(
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Icon(
                        Icons.account_circle_outlined,
                        color: Theme.of(context).colorScheme.primary,
                      ),
                      const SizedBox(width: 12),
                      Text(
                        '${context.tr('settings.account')}${currentUsername != null ? ' ($currentUsername)' : ''}',
                        style: Theme.of(context).textTheme.titleLarge,
                      ),
                    ],
                  ),
                  const SizedBox(height: 16),
                  if (state is AccountLoading && currentUsername == null)
                    const Center(child: CircularProgressIndicator())
                  else ...[
                    ListTile(
                      title: Text(context.tr('settings.change_username')),
                      leading: const Icon(Icons.person),
                      onTap:
                          state is AccountLoading
                              ? null
                              : () => _showChangeUsernameDialog(
                                context,
                                currentUsername,
                              ),
                    ),
                    ListTile(
                      title: Text(context.tr('settings.change_password')),
                      leading: const Icon(Icons.lock),
                      onTap:
                          state is AccountLoading
                              ? null
                              : () => _showChangePasswordDialog(context),
                    ),
                    if (isAdmin)
                      ListTile(
                        title: Text(context.tr('settings.create_user')),
                        leading: const Icon(Icons.person_add),
                        onTap:
                            state is AccountLoading
                                ? null
                                : () => _showCreateUserDialog(context),
                      ),
                    const Divider(),
                    ListTile(
                      title: Text(
                        context.tr('settings.logout'),
                        style: const TextStyle(color: Colors.red),
                      ),
                      leading: const Icon(Icons.logout, color: Colors.red),
                      onTap: () {
                        context.read<AuthCubit>().logout();
                        // Router will handle redirect
                      },
                    ),
                  ],
                ],
              ),
            ),
          );
        },
      ),
    );
  }

  Future<void> _showCreateUserDialog(BuildContext context) async {
    final usernameController = TextEditingController();
    final passwordController = TextEditingController();

    await showDialog(
      context: context,
      builder:
          (dialogContext) => BlocProvider.value(
            value: context.read<AccountCubit>(),
            child: AlertDialog(
              title: Text(context.tr('settings.create_user')),
              content: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  TextField(
                    controller: usernameController,
                    decoration: InputDecoration(
                      labelText: context.tr('settings.username'),
                    ),
                  ),
                  TextField(
                    controller: passwordController,
                    decoration: InputDecoration(
                      labelText: context.tr('settings.password'),
                    ),
                    obscureText: true,
                  ),
                ],
              ),
              actions: [
                TextButton(
                  onPressed: () => Navigator.pop(dialogContext),
                  child: Text(context.tr('settings.cancel')),
                ),
                TextButton(
                  onPressed: () {
                    context.read<AccountCubit>().createUser(
                      usernameController.text,
                      passwordController.text,
                    );
                    Navigator.pop(dialogContext);
                  },
                  child: Text(context.tr('settings.create')),
                ),
              ],
            ),
          ),
    );
  }

  Future<void> _showChangeUsernameDialog(
    BuildContext context,
    String? currentUsername,
  ) async {
    final controller = TextEditingController(text: currentUsername);
    await showDialog(
      context: context,
      builder:
          (dialogContext) => BlocProvider.value(
            value: context.read<AccountCubit>(),
            child: AlertDialog(
              title: Text(context.tr('settings.change_username')),
              content: TextField(
                controller: controller,
                decoration: InputDecoration(
                  labelText: context.tr('settings.new_username'),
                ),
              ),
              actions: [
                TextButton(
                  onPressed: () => Navigator.pop(dialogContext),
                  child: Text(context.tr('settings.cancel')),
                ),
                TextButton(
                  onPressed: () {
                    context.read<AccountCubit>().changeUsername(
                      controller.text,
                    );
                    Navigator.pop(dialogContext);
                  },
                  child: Text(context.tr('settings.save')),
                ),
              ],
            ),
          ),
    );
  }

  Future<void> _showChangePasswordDialog(BuildContext context) async {
    final oldPassController = TextEditingController();
    final newPassController = TextEditingController();

    await showDialog(
      context: context,
      builder:
          (dialogContext) => BlocProvider.value(
            value: context.read<AccountCubit>(),
            child: AlertDialog(
              title: Text(context.tr('settings.change_password')),
              content: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  TextField(
                    controller: oldPassController,
                    decoration: InputDecoration(
                      labelText: context.tr('settings.old_password'),
                    ),
                    obscureText: true,
                  ),
                  TextField(
                    controller: newPassController,
                    decoration: InputDecoration(
                      labelText: context.tr('settings.new_password'),
                    ),
                    obscureText: true,
                  ),
                ],
              ),
              actions: [
                TextButton(
                  onPressed: () => Navigator.pop(dialogContext),
                  child: Text(context.tr('settings.cancel')),
                ),
                TextButton(
                  onPressed: () {
                    context.read<AccountCubit>().changePassword(
                      oldPassController.text,
                      newPassController.text,
                    );
                    Navigator.pop(dialogContext);
                  },
                  child: Text(context.tr('settings.save')),
                ),
              ],
            ),
          ),
    );
  }
  Widget _buildNoteTemplateSection(BuildContext context) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Row(
                  children: [
                    Icon(
                      Icons.note_alt_outlined,
                      color: Theme.of(context).colorScheme.primary,
                    ),
                    const SizedBox(width: 12),
                    Text(
                      context.tr('settings.note_templates'),
                      style: Theme.of(context).textTheme.titleLarge,
                    ),
                  ],
                ),
                IconButton(
                  icon: const Icon(Icons.add),
                  onPressed: () => _showEditTemplateDialog(context, null),
                ),
              ],
            ),
            const SizedBox(height: 16),
            BlocBuilder<SettingsBloc, SettingsState>(
              builder: (context, state) {
                if (state.isLoading) {
                  return const Center(child: CircularProgressIndicator());
                }

                if (state.noteTemplates.isEmpty) {
                   return Center(
                     child: Text(
                       context.tr('settings.no_templates'),
                       style: TextStyle(fontStyle: FontStyle.italic),
                     )
                   );
                }

                return ListView.builder(
                  shrinkWrap: true,
                  physics: const NeverScrollableScrollPhysics(),
                  itemCount: state.noteTemplates.length,
                  itemBuilder: (context, index) {
                    final template = state.noteTemplates[index];
                    return ListTile(
                      title: Text(template.name),
                      subtitle: Text(
                        template.content,
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis
                      ),
                      leading: const Icon(Icons.notes),
                      trailing: Row(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          IconButton(
                            icon: const Icon(Icons.edit),
                            onPressed: () => _showEditTemplateDialog(context, template),
                          ),
                          IconButton(
                            icon: const Icon(Icons.delete),
                             onPressed: () {
                               context.read<SettingsBloc>().add(DeleteNoteTemplate(template.id));
                             },
                          ),
                        ],
                      ),
                    );
                  },
                );
              },
            ),
          ],
        ),
      ),
    );
  }

  Future<void> _showEditTemplateDialog(
    BuildContext context,
    NoteTemplate? template,
  ) async {
    final nameController = TextEditingController(text: template?.name ?? '');
    final contentController = TextEditingController(text: template?.content ?? '');
    
    // Capture the bloc context before showing dialog
    final settingsBloc = context.read<SettingsBloc>();

    await showDialog(
      context: context,
      builder:
          (dialogContext) => AlertDialog(
            title: Text(template == null ? context.tr('settings.add_template') : context.tr('settings.edit_template')),
            content: SizedBox(
              width: double.maxFinite,
              child: SingleChildScrollView(
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    TextField(
                      controller: nameController,
                      decoration: InputDecoration(
                        labelText: context.tr('settings.template_name'),
                      ),
                    ),
                    const SizedBox(height: 16),
                    MarkdownEditorInput(
                      controller: contentController,
                      label: context.tr('settings.template_content'),
                    ),
                  ],
                ),
              ),
            ),
            actions: [
              TextButton(
                onPressed: () => Navigator.pop(dialogContext),
                child: Text(context.tr('settings.cancel')),
              ),
              TextButton(
                onPressed: () {
                   if (nameController.text.isEmpty) return;

                   if (template != null) {
                     settingsBloc.add(
                       EditNoteTemplate(
                         template.copyWith(
                           name: nameController.text,
                           content: contentController.text,
                         )
                       )
                     );
                   } else {
                     final newId = DateTime.now().millisecondsSinceEpoch.toString();
                     settingsBloc.add(
                       AddNoteTemplate(
                         NoteTemplate(
                           id: newId,
                           name: nameController.text,
                           content: contentController.text,
                         )
                       )
                     );
                   }
                   Navigator.pop(dialogContext);
                },
                child: Text(context.tr('settings.save')),
              ),
            ],
          ),
    );
  }
}
