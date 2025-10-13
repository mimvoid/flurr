namespace Flurr {
public interface DBusService : Object {
  public void own_name(string dest) {
    Bus.own_name(
      BusType.SESSION, dest, BusNameOwnerFlags.NONE, on_dbus_connect
    );
  }
  protected abstract void on_dbus_connect(DBusConnection conn);
}

public string make_object_path(Gtk.Application app) {
  return app.get_dbus_object_path() ?? "/" + app.application_id.replace(".", "/");
}

public void connect_window_dbus<T>(Gtk.ApplicationWindow window, DBusConnection conn, T interface) {
  var id = window.get_id();
  if (id == 0) {
    critical(@"Window \"$(window.name)\" is unregistered, cannot add to DBus.");
    return;
  }

  var obj_path = make_object_path(window.application);
  try {
    var registration_id = conn.register_object(@"$obj_path/window/$id", interface);
    window.close_request.connect(() => {
      conn.unregister_object(registration_id);
      return false;
    });
  } catch (Error err) {
    critical(err.message);
  }
}
}
