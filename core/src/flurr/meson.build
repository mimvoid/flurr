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

if get_option('introspection')
    gir = 'Flurr-' + api_version + '.gir'
    typelib = 'Flurr-' + api_version + '.typelib'

    flurr_lib = shared_library(
        'flurr',
        sources,
        dependencies: dependencies,
        vala_gir: gir,
        install: true,
        install_dir: [true, true, true, true]
    )

    custom_target(
        typelib,
        command: [
            g_ir_compiler,
            '--includedir', flurrdbus_dir,
            '--output', '@OUTPUT@',
            meson.current_build_dir() / gir
        ],
        output: typelib,
        depends: flurr_lib,
        install: true,
        install_dir: get_option('libdir') / 'girepository-1.0'
    )
else
    flurr_lib = shared_library(
        'flurr',
        sources,
        dependencies: dependencies,
        install: true,
        install_dir: [true, true, true]
    )
endif

flurr_dep = declare_dependency(
    include_directories: include_directories('.'),
    link_with: flurr_lib
)
