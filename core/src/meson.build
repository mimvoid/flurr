version_split = meson.project_version().split('.')
api_version = version_split[0] + '.' + version_split[1]

introspection = get_option('introspection')

flurrdbus_dep = dependency(
  'flurrdbus',
  fallback: ['flurrdbus', 'flurrdbus_dep'],
  default_options: ['introspection=' + introspection.to_string()],
)

dependencies = [
  dependency('glib-2.0'),
  dependency('gobject-2.0'),
  dependency('gio-2.0'),
  dependency('gtk4'),
  dependency('gtk4-layer-shell-0'),
  flurrdbus_dep,
]

sources = files(
  'Application.vala',
  'css.vala',
  'dbus/lib.vala',
  'dbus/services/ApplicationService.vala',
  'dbus/services/PinShellService.vala',
  'dbus/services/ShellService.vala',
  'dbus/services/utils.vala',
  'dbus/services/WindowService.vala',
  'widgets/property_types.vala',
  'widgets/PinShell.vala',
  'widgets/Shell.vala',
)

gir = 'Flurr-' + api_version + '.gir'
install_dir = [true, true, true]

if introspection
  install_dir += true
endif

flurr_lib = shared_library(
  meson.project_name(),
  sources,
  version: meson.project_version(),
  dependencies: dependencies,
  vala_gir: gir,
  install: true,
  install_dir: install_dir,
)

if introspection
  g_ir_compiler = find_program('g-ir-compiler')
  typelib = 'Flurr-' + api_version + '.typelib'

  custom_target(
    typelib,
    command: [
      g_ir_compiler,
      '--includedir', flurrdbus_dep.get_variable('build_dir'),
      '--output', '@OUTPUT@',
      meson.current_build_dir() / gir
    ],
    output: typelib,
    depends: flurr_lib,
    install: true,
    install_dir: get_option('libdir') / 'girepository-1.0',
  )
endif

flurr_dep = declare_dependency(
  link_with: flurr_lib,
  version: meson.project_version(),
  dependencies: dependencies,
  include_directories: include_directories('.'),
)
