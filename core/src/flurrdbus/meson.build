flurrdbus_lib = shared_library(
  'flurrdbus',
  files('flurrdbus.vala'),
  dependencies: [glib, gobject, gio],
  install: true,
  install_dir: [true, true, true]
)

flurrdbus_dep = declare_dependency(
  include_directories: include_directories('.'),
  link_with: flurrdbus_lib
)
