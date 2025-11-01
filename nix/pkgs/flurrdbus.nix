{
  stdenv,
  lib,
  meson,
  ninja,
  pkg-config,
  gobject-introspection,
  vala,
  glib,
  introspection ? true,
}:

stdenv.mkDerivation (finalAttrs: {
  pname = "flurrdbus";
  version = "0.1.0";

  src = builtins.path {
    name = "${finalAttrs.pname}-${finalAttrs.version}";
    path = lib.cleanSource ../../core/subprojects/flurrdbus;
  };

  nativeBuildInputs = [
    meson
    ninja
    pkg-config
    vala
  ]
  ++ lib.optionals introspection [ gobject-introspection ];

  buildInputs = [ glib ];

  mesonFlags = [ (lib.mesonBool "introspection" introspection) ];

  meta = {
    license = lib.licenses.mpl20;
    maintainers = with lib.maintainers; [ mimvoid ];
    platforms = lib.platforms.linux;
  };
})
