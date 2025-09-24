sources = files(
    'Application.vala',
)

rimice_lib = shared_library(
    'rimice',
    sources,
    dependencies: dependencies,
    install: true,
    install_dir: [true, true, true]
)
rimice_dep = declare_dependency(include_directories: include_directories('.'), link_with: rimice_lib)
