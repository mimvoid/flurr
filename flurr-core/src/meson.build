sources = files(
    'Application.vala',
    'widgets/Window.vala',
    'widgets/PinWindow.vala',
)

flurr_lib = shared_library(
    'flurr',
    sources,
    dependencies: dependencies,
    install: true,
    install_dir: [true, true, true]
)
flurr_dep = declare_dependency(include_directories: include_directories('.'), link_with: flurr_lib)
