public class FlurrDBus.ApplicationService : FlurrDBus.Application, FlurrDBus.Service, Object {
  public Flurr.Application app { get; construct; }

  public ApplicationService(Flurr.Application app) {
    Object(app: app);
  }

  protected void on_dbus_connect(GLib.DBusConnection conn) {
    var obj_path = app.get_dbus_object_path();
    try {
      conn.register_object(obj_path, (FlurrDBus.Application) this);
    } catch (Error err) {
      critical(err.message);
    }
  }

  // DBus

  public void toggle_window(string name) throws DBusError, IOError {
    app.toggle_window(name);
  }

  public string[] list_window_names() throws DBusError, IOError {
    unowned var windows = app.get_windows();
    var names = new string[(int) windows.length()];

    foreach (var win in windows) {
      names += win.name;
    }

    return names;
  }

  public uint[] list_window_ids() throws DBusError, IOError {
    unowned var windows = app.get_windows();
    var ids = new uint[(int) windows.length()];

    foreach (var win in windows) {
      if (win is Gtk.ApplicationWindow) {
        ids += ((Gtk.ApplicationWindow) win).get_id();
      }
    }

    return ids;
  }

  public ObjectPath[] list_window_paths() throws DBusError, IOError {
    unowned var windows = app.get_windows();
    var paths = new ObjectPath[(int) windows.length()];

    foreach (var win in windows) {
      if (!(win is Gtk.ApplicationWindow))
        continue;

      var id = ((Gtk.ApplicationWindow) win).get_id();
      if (id != 0) {
        paths += new ObjectPath(app.get_dbus_object_path() + @"/window/$id");
      }
    }

    return paths;
  }

  public void quit() throws DBusError, IOError {
    app.quit();
  }
}
