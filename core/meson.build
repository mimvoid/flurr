project('flurr', ['vala', 'c'], version: '0.1.0', license: 'MPL-2.0')

add_project_arguments('-w', language: 'c')
add_project_arguments('--target-glib=auto', '--abi-stability', language: 'vala')

flurr_dep = dependency(
  'flurr',
  fallback: ['flurr', 'flurr_dep'],
  default_options: ['introspection=' + get_option('introspection').to_string()]
)

if get_option('examples')
  subdir('examples')
endif
