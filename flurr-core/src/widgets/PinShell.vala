public class Flurr.PinShell : Flurr.Shell {
  public PinShell(Gtk.Application app) {
    Object(application: app);
  }

  construct {
    layer = GtkLayerShell.Layer.BOTTOM;
  }
}
