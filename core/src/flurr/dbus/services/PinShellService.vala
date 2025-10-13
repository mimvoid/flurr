namespace Flurr {
public class PinShellService : FlurrDBus.PinShell, Flurr.DBusService, Object {
  public weak Flurr.PinShell shell { get; construct; }

  public PinShellService(Flurr.PinShell shell) {
    Object(shell: shell);
  }

  protected void on_dbus_connect(GLib.DBusConnection conn) {
    connect_window_dbus(shell, conn, (FlurrDBus.PinShell) this);
  }

  // DBus

  public bool unlocked {
    get { return shell.unlocked; }
    set { shell.unlocked = value; }
  }
}
}
