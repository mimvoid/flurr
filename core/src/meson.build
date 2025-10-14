if get_option('introspection')
  version_split = meson.project_version().split('.')
  api_version = version_split[0] + '.' + version_split[1]

  g_ir_compiler = find_program('g-ir-compiler')
endif

subdir('flurrdbus')
subdir('flurr')
