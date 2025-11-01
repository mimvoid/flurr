{
  lib,
  rustPlatform,
  pkg-config,
  dbus,
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "flurrctl";
  version = "0.1.0";

  src = builtins.path {
    name = "${finalAttrs.pname}-${finalAttrs.version}";
    path = lib.cleanSource ../..;
  };

  cargoLock.lockFile = ../../Cargo.lock;
  buildAndTestSubdir = "crates/flurrctl";

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ dbus ];

  meta = {
    license = lib.licenses.mpl20;
    maintainers = with lib.maintainers; [ mimvoid ];
    platforms = lib.platforms.linux;
  };
})
