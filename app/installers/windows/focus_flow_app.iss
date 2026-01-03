[Setup]
AppId={{2A5C98DE-E0EE-417A-9372-6B4913758334}
AppName=Focus Flow
AppVersion={#MyAppVersion}
;AppVerName=Focus Flow {#MyAppVersion}
AppPublisher=Francesco Gaglione
AppPublisherURL=https://github.com/francesco-gaglione/focus_flow_cloud
AppSupportURL=https://github.com/francesco-gaglione/focus_flow_cloud
AppUpdatesURL=https://github.com/francesco-gaglione/focus_flow_cloud
DefaultDirName={autopf}\Focus Flow
DisableProgramGroupPage=yes
; Uncomment the following line to run in non administrative install mode (install for current user only.)
;PrivilegesRequired=lowest
OutputDir=..\..\..\
OutputBaseFilename=focus_flow_app-windows-setup
Compression=lzma
SolidCompression=yes
WizardStyle=modern

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "italian"; MessagesFile: "compiler:Languages\Italian.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "..\..\build\windows\x64\runner\Release\focus_flow_app.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\build\windows\x64\runner\Release\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs
; NOTE: Don't use "Flags: ignoreversion" on any shared system files

[Icons]
Name: "{autoprograms}\Focus Flow"; Filename: "{app}\focus_flow_app.exe"
Name: "{autodesktop}\Focus Flow"; Filename: "{app}\focus_flow_app.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\focus_flow_app.exe"; Description: "{cm:LaunchProgram,Focus Flow}"; Flags: nowait postinstall skipifsilent
