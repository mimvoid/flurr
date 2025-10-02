[DBus(name = "com.Flurr.Application")]
public class Flurr.Application : Gtk.Application {
  public Application() {
    Object(application_id: "com.Flurr.flurr");
  }

  public Application.with_name(string instance_name) {
    Object(application_id: @"com.Flurr.$instance_name");
  }

  public override void activate() {
    var display = Gdk.Display.get_default();
    if (display == null)
      return;

    var css = new Gtk.CssProvider();
    css.load_from_resource("/com/Flurr/flurr/style.css");
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

  [DBus(visible=false)]
  public void toggle_window(string name) throws Error {
    var window = get_window_by_name(name);
    if (window == null) {
      throw new IOError.FAILED(@"Could not find a window named \"$name\"");
    }
    window.visible = !window.visible;
  }
}
