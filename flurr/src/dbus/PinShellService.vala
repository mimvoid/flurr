namespace FlurrDBus {
public class PinShellService : PinShell, Service, Object {
  public Flurr.PinShell shell { get; construct; }

  public PinShellService(Flurr.PinShell shell) {
    Object(shell: shell);
  }

  protected void on_dbus_connect(GLib.DBusConnection conn) {
    var id = shell.get_id();
    if (id == 0) {
      critical(@"Window \"$(shell.name)\" is unregistered, cannot add to DBus.");
      return;
    }

    try {
      conn.register_object(
        @"$(BASE_WINDOW_OBJECT_PATH)/$id", (FlurrDBus.PinShell) this
      );
    } catch (Error err) {
      critical(err.message);
    }
  }

  // DBus

  public bool unlocked {
    get { return shell.unlocked; }
    set { shell.unlocked = value; }
  }
}
}
