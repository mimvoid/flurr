/**
 * DBus object to wrap and expose a Flurr application's methods to DBus.
 */
public class FlurrDBus.ApplicationService : FlurrDBus.Application, FlurrDBus.Service, Object {
  public weak Flurr.Application app { get; construct; }

  public ApplicationService(Flurr.Application app) {
    Object(app: app);
  }

  protected void on_dbus_connect(GLib.DBusConnection conn) {
    try {
      conn.register_object(make_object_path(app), (FlurrDBus.Application) this);
    } catch (Error err) {
      critical(err.message);
    }
  }

  // DBus

  public ObjectPath get_window_path(string window_name) throws DBusError, IOError {
    var win = app.get_window_by_name(window_name);
    if (win == null) {
      throw new IOError.FAILED(@"Could not find a window named \"$window_name\"");
    }

    if (!(win is Gtk.ApplicationWindow)) {
      throw new IOError.FAILED(
        @"Window \"$window_name\" is not a Gtk.ApplicationWindow, does not have a known object path"
      );
    }

    var obj_path_base = make_object_path(app) + "/window/";
    return new ObjectPath(
      obj_path_base + ((Gtk.ApplicationWindow) win).get_id().to_string()
    );
  }

  public string[] list_window_names() throws DBusError, IOError {
    return app.get_window_names();
  }

  public uint[] list_window_ids() throws DBusError, IOError {
    var ids = new uint[0];

    foreach (var win in app.get_windows()) {
      if (win is Gtk.ApplicationWindow) {
        ids += ((Gtk.ApplicationWindow) win).get_id();
      }
    }

    return ids;
  }

  public ObjectPath[] list_window_paths() throws DBusError, IOError {
    var obj_path_base = make_object_path(app) + "/window/";
    var ids = list_window_ids();
    var paths = new ObjectPath[ids.length];

    for (uint i = 0; i < ids.length; i++) {
      paths[i] = new ObjectPath(obj_path_base + ids[i].to_string());
    }

    return paths;
  }

  public void quit() throws DBusError, IOError {
    app.quit();
  }
}
