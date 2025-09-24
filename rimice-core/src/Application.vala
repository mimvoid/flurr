[DBus(name = "com.Rimice.Application")]
public class Rimice.Application : Gtk.Application {
  public Application() {
    Object(application_id: "com.Rimice.rimice");
  }

  public Application.with_name(string instance_name) {
    Object(application_id: @"com.Rimice.$instance_name");
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

  [DBus(visible=false)]
  public void toggle_window(string name) throws Error {
    var window = get_window_by_name(name);
    if (window == null) {
      throw new IOError.FAILED(@"Could not find a window named \"$name\"");
    }
    window.visible = !window.visible;
  }
}
