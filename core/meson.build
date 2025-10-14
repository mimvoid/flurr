project('Flurr', ['vala', 'c'], version: '0.1.0', license: 'MPL-2.0')

add_project_arguments('--target-glib=auto', '--Xcc=-w', '--abi-stability', language: 'vala')

glib = dependency('glib-2.0')
gobject = dependency('gobject-2.0')
gio = dependency('gio-2.0')

subdir('src')

if get_option('examples')
  subdir('examples')
endif
