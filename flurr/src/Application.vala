public class Flurr.Application : Gtk.Application {
  public Application() {
    Object(application_id: "io.flurr.Flurr");
  }

  public Application.with_name(string instance_name) {
    Object(application_id: @"io.flurr.$instance_name");
  }

  protected override void activate() {
    var app_dbus = new FlurrDBus.ApplicationService(this);
    app_dbus.own_name(application_id);

    get_windows().foreach(window_added_dbus);
    window_added.connect(window_added_dbus);

    var display = Gdk.Display.get_default();
    if (display == null)
      return;

    var css = new Gtk.CssProvider();
    css.load_from_resource("/io/flurr/Core/style.css");
    Gtk.StyleContext.add_provider_for_display(display, css, Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION);
  }

  public Gtk.Window? get_window_by_name(string name) {
    foreach (var window in get_windows()) {
      if (window.name == name) {
        return window;
      }
    }
    return null;
  }

  public string[] get_window_names() {
    var names = new string[0];
    foreach (var win in get_windows()) {
      names += win.name;
    }
    return names;
  }

  public void toggle_window(string name) throws IOError {
    var window = get_window_by_name(name);
    if (window == null) {
      throw new IOError.FAILED(@"Could not find a window named \"$name\"");
    }
    window.visible = !window.visible;
  }

  private void window_added_dbus(Gtk.Window window) {
    if (!(window is Flurr.Shell))
      return;

    var shell_service = new FlurrDBus.ShellService((Flurr.Shell) window);
    shell_service.own_name(application_id);

    if (window is Flurr.PinShell) {
      var pin_shell_service = new FlurrDBus.PinShellService((Flurr.PinShell) window);
      pin_shell_service.own_name(application_id);
    }
  }
}
