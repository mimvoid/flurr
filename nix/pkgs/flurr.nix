{
  stdenv,
  lib,
  meson,
  ninja,
  pkg-config,
  gobject-introspection,
  vala,
  glib,
  gtk4,
  gtk4-layer-shell,
  flurrdbus,
  introspection ? true,
  examples ? false,
}:

stdenv.mkDerivation (finalAttrs: {
  pname = "flurr";
  version = "0.1.0";

  src = builtins.path {
    name = "${finalAttrs.pname}-${finalAttrs.version}";
    path = lib.cleanSource ../../core;
  };

  nativeBuildInputs = [
    meson
    ninja
    pkg-config
    vala
  ]
  ++ lib.optionals introspection [ gobject-introspection ];

  buildInputs = [
    glib
    gtk4
    gtk4-layer-shell
    flurrdbus
  ];

  mesonFlags = [
    (lib.mesonBool "introspection" introspection)
    (lib.mesonBool "examples" examples)
  ];

  meta = {
    license = lib.licenses.mpl20;
    maintainers = with lib.maintainers; [ mimvoid ];
    platforms = lib.platforms.linux;
  };
})
