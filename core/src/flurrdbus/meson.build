if get_option('introspection')
  gir = 'FlurrDBus-' + api_version + '.gir'
  typelib = 'FlurrDBus-' + api_version + '.typelib'
  flurrdbus_dir = meson.current_build_dir()

  flurrdbus_lib = shared_library(
    'flurrdbus',
    'flurrdbus.vala',
    dependencies: [glib, gobject, gio],
    vala_gir: gir,
    install: true,
    install_dir: [true, true, true, true],
  )

  custom_target(
    typelib,
    command: [g_ir_compiler, '--output', '@OUTPUT@', flurrdbus_dir / gir],
    depends: flurrdbus_lib,
    output: typelib,
    install: true,
    install_dir: get_option('libdir') / 'girepository-1.0',
  )
else
  flurrdbus_lib = shared_library(
    'flurrdbus',
    'flurrdbus.vala',
    dependencies: [glib, gobject, gio],
    install: true,
    install_dir: [true, true, true],
  )
endif

flurrdbus_dep = declare_dependency(
  include_directories: include_directories('.'),
  link_with: flurrdbus_lib,
)
