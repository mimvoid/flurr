version_split = meson.project_version().split('.')
api_version = '.'.join(version_split[0], version_split[1])
filebase = '-'.join(meson.project_name(), api_version)

introspection = get_option('introspection')

flurrdbus_dep = dependency(
  'flurrdbus-0.1',
  fallback: ['flurrdbus', 'flurrdbus_dep'],
  default_options: ['introspection=' + introspection.to_string()],
)

dependencies = [
  dependency('glib-2.0'),
  dependency('gobject-2.0'),
  dependency('gio-2.0'),
  dependency('gtk4'),
  dependency('gtk4-layer-shell-0'),
]

config = configure_file(
  input: 'config.vala.in',
  output: 'config.vala',
  configuration: {'API_VERSION': api_version},
)
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
) + config

valadoc = find_program('valadoc', required: false)
valac_gir = introspection and not valadoc.found()
gir = 'Flurr-' + api_version + '.gir'

flurr_lib = shared_library(
  meson.project_name(),
  sources,
  version: meson.project_version(),
  dependencies: dependencies + flurrdbus_dep,
  vala_vapi: filebase + '.vapi',
  install: true,
  install_dir: valac_gir ? [true, true, true, true] : [true, true, true],
  kwargs: valac_gir ? {'vala_gir': gir} : {},
)

if not valac_gir
  pkgs = ['--pkg=flurrdbus-0.1']
  foreach dep : dependencies
    pkgs += ['--pkg', dep.name()]
  endforeach

  # valadoc can generate a GIR file with documentation
  valadoc_gir = custom_target(
    command: [
      valadoc,
      '-o', meson.current_build_dir() / 'docs',
      '--force',
      '--package-name', meson.project_name(),
      '--gir=@OUTPUT@',
      '--vapidir', flurrdbus_dep.get_variable('vapidir'),
      '@INPUT@'
    ] + pkgs,
    input: sources,
    output: gir,
    install: true,
    install_dir: get_option('datadir') / 'gir-1.0',
  )
endif

if introspection
  g_ir_compiler = find_program('g-ir-compiler')
  typelib = gir.replace('gir', 'typelib')
  lib_path = get_option('prefix') / get_option('libdir') / import('fs').name(flurr_lib.full_path())

  custom_target(
    command: [
      g_ir_compiler,
      '--includedir', flurrdbus_dep.get_variable('girdir'),
      '--output', '@OUTPUT@',
      '--shared-library', lib_path,
      meson.current_build_dir() / gir
    ],
    output: typelib,
    depends: valac_gir ? flurr_lib : valadoc_gir,
    install: true,
    install_dir: get_option('libdir') / 'girepository-1.0',
  )
endif

flurr_dep = declare_dependency(
  link_with: flurr_lib,
  version: meson.project_version(),
  dependencies: dependencies + flurrdbus_dep,
  include_directories: include_directories('.'),
)

import('pkgconfig').generate(
  flurr_lib,
  filebase: filebase,
  subdirs: meson.project_name(),
  requires: dependencies,
)
