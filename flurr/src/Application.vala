[DBus(name = "io.flurr.Application")]
public class Flurr.Application : Gtk.Application {
  private DBusConnection dbus_conn;

  public Application() {
    Object(application_id: "io.flurr.Flurr");
  }

  public Application.with_name(string instance_name) {
    Object(application_id: @"io.flurr.$instance_name");
  }

  [DBus(visible=false)]
  protected override void activate() {
    Bus.own_name(
      BusType.SESSION,
      application_id,
      BusNameOwnerFlags.NONE,
      (conn) => {
        try {
          this.dbus_conn = conn;
          conn.register_object("/io/flurr/Application", this);
        } catch (Error err) {
          critical(err.message);
        }
      }
    );

    var display = Gdk.Display.get_default();
    if (display == null)
      return;

    var css = new Gtk.CssProvider();
    css.load_from_resource("/io/flurr/Core/style.css");
    Gtk.StyleContext.add_provider_for_display(display, css, Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION);
  }

  [DBus(visible=false)]
  public Gtk.Window? get_window_by_name(string name) {
    foreach (var window in get_windows()) {
      if (window.name == name) {
        return window;
      }
    }
    return null;
  }

  public void toggle_window(string name) throws Error {
    var window = get_window_by_name(name);
    if (window == null) {
      throw new IOError.FAILED(@"Could not find a window named \"$name\"");
    }
    window.visible = !window.visible;
  }
}
