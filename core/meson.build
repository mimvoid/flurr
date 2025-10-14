project('Flurr', ['vala', 'c'], version: '0.1.0', license: 'MPL-2.0')

glib = dependency('glib-2.0')
gobject = dependency('gobject-2.0')
gio = dependency('gio-2.0')

subdir('src')

if get_option('examples')
  subdir('examples')
endif
