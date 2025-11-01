project('flurrdbus', ['vala', 'c'], version: '0.1.0', license: 'MPL-2.0')

add_project_arguments('-w', language: 'c')
add_project_arguments('--target-glib=auto', '--abi-stability', language: 'vala')

version_split = meson.project_version().split('.')
api_version = version_split[0] + '.' + version_split[1]

dependencies = [
  dependency('glib-2.0'),
  dependency('gobject-2.0'),
  dependency('gio-2.0'),
]

filebase = meson.project_name() + '-' + api_version
gir = 'FlurrDBus-' + api_version + '.gir'
install_dir = [true, true, true]

introspection = get_option('introspection')
if introspection
  install_dir += true
endif

flurrdbus_lib = shared_library(
  meson.project_name(),
  'flurrdbus.vala',
  version: meson.project_version(),
  dependencies: dependencies,
  vala_gir: gir,
  vala_vapi: filebase + '.vapi',
  install: true,
  install_dir: install_dir
)

build_dir = meson.current_build_dir()

if introspection
  g_ir_compiler = find_program('g-ir-compiler')
  typelib = gir.replace('gir', 'typelib')
  lib_path = get_option('prefix') / get_option('libdir') / import('fs').name(flurrdbus_lib.full_path())

  custom_target(
    typelib,
    command: [g_ir_compiler, '--output', '@OUTPUT@', '--shared-library', lib_path, build_dir / gir],
    depends: flurrdbus_lib,
    output: typelib,
    install: true,
    install_dir: get_option('libdir') / 'girepository-1.0',
  )
endif

flurrdbus_dep = declare_dependency(
  link_with: flurrdbus_lib,
  version: meson.project_version(),
  dependencies: dependencies,
  include_directories: include_directories('.'),
  variables: ['girdir=' + build_dir]
)

import('pkgconfig').generate(
  flurrdbus_lib,
  filebase: filebase,
  subdirs: meson.project_name(),
  requires: dependencies,
  variables: ['girdir=${prefix}' / get_option('datadir') / 'gir-1.0']
)
