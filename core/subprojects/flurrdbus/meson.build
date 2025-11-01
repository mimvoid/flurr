project('flurrdbus', ['vala', 'c'], version: '0.1.0', license: 'MPL-2.0')

version_split = meson.project_version().split('.')
api_version = version_split[0] + '.' + version_split[1]

dependencies = [
  dependency('glib-2.0'),
  dependency('gobject-2.0'),
  dependency('gio-2.0'),
]

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
  install: true,
  install_dir: install_dir
)

if introspection
  g_ir_compiler = find_program('g-ir-compiler')
  typelib = gir.replace('gir', 'typelib')
  build_dir = meson.current_build_dir()

  custom_target(
    typelib,
    command: [g_ir_compiler, '--output', '@OUTPUT@', build_dir / gir],
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
  variables: ['build_dir=' + build_dir]
)
