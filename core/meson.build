project('flurr', ['vala', 'c'], version: '0.1.0', license: 'MPL-2.0')

add_project_arguments('-w', language: 'c')
add_project_arguments('--target-glib=auto', '--abi-stability', '--vapi-comments', language: 'vala')

subdir('src')

if get_option('examples')
  subdir('examples')
endif
