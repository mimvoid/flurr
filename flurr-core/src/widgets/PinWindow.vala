public class Flurr.PinWindow : Flurr.Window {
  public PinWindow(Gtk.Application app) {
    Object(application: app);
  }

  construct {
    GtkLayerShell.init_for_window(this);
    GtkLayerShell.auto_exclusive_zone_enable(this);
  }
}
