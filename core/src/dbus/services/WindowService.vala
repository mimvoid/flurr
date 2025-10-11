public class FlurrDBus.WindowService : FlurrDBus.Window, FlurrDBus.Service, Object {
  public weak Gtk.ApplicationWindow window { get; construct; }

  public WindowService(Gtk.ApplicationWindow window) {
    Object(window: window);
  }

  protected void on_dbus_connect(GLib.DBusConnection conn) {
    connect_window_dbus(window, conn, (FlurrDBus.Window) this);
  }

  // DBus

  public string name { owned get { return window.name; } }
  public bool visible {
    get { return window.visible; }
    set { window.visible = value; }
  }

  public async void toggle() throws DBusError, IOError {
    window.visible = !window.visible;
  }
}
