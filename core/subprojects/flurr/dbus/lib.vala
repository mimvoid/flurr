namespace Flurr {
public void register_dbus(Flurr.Application app) {
  var app_dbus = new Flurr.ApplicationService(app);
  app_dbus.own_name(app.application_id);

  foreach (var win in app.get_windows()) {
    on_window_added(app, win);
  }
  app.window_added.connect(on_window_added);
}

private void on_window_added(Gtk.Application app, Gtk.Window window) {
  if (!(window is Gtk.ApplicationWindow))
    return;

  var window_service = new Flurr.WindowService((Gtk.ApplicationWindow) window);
  window_service.own_name(app.application_id);

  if (!(window is Flurr.Shell))
    return;

  var shell_service = new Flurr.ShellService((Flurr.Shell) window);
  shell_service.own_name(app.application_id);

  if (window is Flurr.PinShell) {
    var pin_shell_service = new Flurr.PinShellService((Flurr.PinShell) window);
    pin_shell_service.own_name(app.application_id);
  }
}
}
