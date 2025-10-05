public class FlurrDBus.WindowService : FlurrDBus.Window, FlurrDBus.Service, Object {
  public weak Gtk.ApplicationWindow window { get; construct; }

  public WindowService(Gtk.ApplicationWindow window) {
    Object(window: window);
  }

  protected void on_dbus_connect(GLib.DBusConnection conn) {
    connect_window_dbus(window, conn, (FlurrDBus.Window) this);
  }

  // DBus

  public string name {
    owned get { return window.name; }
    set { window.name = value; }
  }

  public void show() throws DBusError, IOError {
    window.visible = true;
  }
  public void hide() throws DBusError, IOError {
    window.visible = false;
  }
  public void toggle() throws DBusError, IOError {
    window.visible = !window.visible;
  }
}
