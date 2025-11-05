project('flurrdbus', ['vala', 'c'], version: '0.1.0', license: 'MPL-2.0')

add_project_arguments('-w', language: 'c')
add_project_arguments('--target-glib=auto', '--abi-stability', '--vapi-comments', language: 'vala')

version_split = meson.project_version().split('.')
api_version = '.'.join(version_split[0], version_split[1])
filebase = '-'.join(meson.project_name(), api_version)

dependencies = [
  dependency('glib-2.0'),
  dependency('gobject-2.0'),
  dependency('gio-2.0'),
]

config = configure_file(
  input: 'config.vala.in',
  output: 'config.vala',
  configuration: {'API_VERSION': api_version},
)
sources = files('flurrdbus.vala') + config

introspection = get_option('introspection')
valadoc = find_program('valadoc', required: false)
valac_gir = introspection and not valadoc.found()
gir = 'FlurrDBus-' + api_version + '.gir'

flurrdbus_lib = shared_library(
  meson.project_name(),
  sources,
  version: meson.project_version(),
  dependencies: dependencies,
  vala_vapi: filebase + '.vapi',
  install: true,
  install_dir: valac_gir ? [true, true, true, true] : [true, true, true],
  kwargs: valac_gir ? {'vala_gir': gir} : {},
)

build_dir = meson.current_build_dir()

if not valac_gir
  pkgs = []
  foreach dep : dependencies
    pkgs += ['--pkg', dep.name()]
  endforeach

  # valadoc can generate a GIR file with documentation
  valadoc_gir = custom_target(
    command: [
      valadoc,
      '-o', build_dir / 'docs',
      '--force',
      '--package-name', meson.project_name(),
      '--gir=@OUTPUT@',
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
  lib_path = get_option('prefix') / get_option('libdir') / import('fs').name(flurrdbus_lib.full_path())

  custom_target(
    command: [g_ir_compiler, '--output', '@OUTPUT@', '--shared-library', lib_path, build_dir / gir],
    output: typelib,
    depends: valac_gir ? flurrdbus_lib : valadoc_gir,
    install: true,
    install_dir: get_option('libdir') / 'girepository-1.0',
  )
endif

flurrdbus_dep = declare_dependency(
  link_with: flurrdbus_lib,
  version: meson.project_version(),
  dependencies: dependencies,
  include_directories: include_directories('.'),
  variables: {'girdir': build_dir, 'vapidir': build_dir},
)

import('pkgconfig').generate(
  flurrdbus_lib,
  filebase: filebase,
  subdirs: meson.project_name(),
  requires: dependencies,
  variables: [
    'datadir=${prefix}' / get_option('datadir'),
    'girdir=${datadir}' / 'gir-1.0',
    'vapidir=${datadir}' / 'vala' / 'vapi'
  ],
)
