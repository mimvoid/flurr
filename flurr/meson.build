project('Flurr', ['vala', 'c'], version: '0.1.0', license: 'GPL3')

dependencies = [
    dependency('glib-2.0'),
    dependency('gobject-2.0'),
    dependency('gio-2.0'),
    dependency('gtk4'),
    dependency('gtk4-layer-shell-0'),
]

resources = import('gnome').compile_resources(
    'flurr_resources',
    'data/flurr.gresource.xml',
    source_dir: 'data'
)

subdir('src')

if get_option('examples')
    subdir('examples')
endif
