dependencies = [
    glib,
    gobject,
    gio,
    dependency('gtk4'),
    dependency('gtk4-layer-shell-0'),
    flurrdbus_dep
]

sources = files(
    'Application.vala',
    'widgets/property_types.vala',
    'widgets/Shell.vala',
    'widgets/PinShell.vala',
    'dbus/lib.vala',
    'dbus/services/utils.vala',
    'dbus/services/ApplicationService.vala',
    'dbus/services/WindowService.vala',
    'dbus/services/ShellService.vala',
    'dbus/services/PinShellService.vala',
)

flurr_lib = shared_library(
    'flurr',
    sources,
    dependencies: dependencies,
    install: true,
    install_dir: [true, true, true]
)
flurr_dep = declare_dependency(
    include_directories: include_directories('.'),
    link_with: flurr_lib
)
