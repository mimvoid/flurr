/**
 * Flurr's special Application class that handles instance names, registering DBus objects,
 * and loading the minimal default CSS.
 *
 * It automatically sets a valid ``application_id`` for Flurr on construction. You can set a
 * custom ``application_id`` that doesn't start with "io.flurr.", but note that it won't be
 * found with tools like flurrctl.
 */
public class Flurr.Application : Gtk.Application {
  private bool enable_dbus = true; // TODO: set on construction
  private List<Gtk.CssProvider> css_providers = new List<Gtk.CssProvider>();

  /**
   * Construct a Flurr application with the default instance name, "Flurr".
   */
  public Application() {
    Object(application_id: "io.flurr.Flurr");
  }

  /**
   * Construct a Flurr application with a custom instance name.
   *
   * The ``application_id`` will be formatted like "io.flurr.instance_name".
   */
  public Application.with_name(string instance_name) {
    Object(application_id: @"io.flurr.$instance_name");
  }

  protected override void activate() {
    if (enable_dbus) {
      Flurr.register_dbus(this);
    }

    load_css_string("
      .pin-shell {
        border-radius: 4px;
        border: 1px solid transparent;
      }
      .pin-shell.unlocked:not(.dragging) {
        border-color: white;
      }
      .pin-shell-buttons {
        -gtk-icon-size: 12px;
      }
      .pin-shell-buttons button,
      .pin-shell-buttons menubutton {
        padding: 0;
      }
    ", Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION);
  }

  /**
   * Searches for a window with the given name.
   *
   * @param name The name of the window
   * @return The window with that name, if any
   */
  public Gtk.Window? get_window_by_name(string name) {
    foreach (var window in get_windows()) {
      if (window.name == name) {
        return window;
      }
    }
    return null;
  }

  /**
   * Get all windows known to the application and return all non-empty names.
   */
  public string[] get_window_names() {
    var names = new string[0];
    foreach (var win in get_windows()) {
      var name = win.name;
      if (name != "") {
        names += win.name;
      }
    }
    return names;
  }

  /**
   * Helper function that's the same as get_window_by_name(), but throws an IOError if
   * no window is found.
   */
  private Gtk.Window check_window_name(string name) throws IOError {
    var window = get_window_by_name(name);
    if (window == null) {
      throw new IOError.FAILED(@"Could not find a window named \"$name\"");
    }
    return window;
  }

  /**
   * Shows a window if it is not visible, and hides it if it is visible.
   *
   * @param name The name of the window
   * @throws IOError If no window with the name is found
   */
  public void toggle_window(string name) throws IOError {
    var window = check_window_name(name);
    window.visible = !window.visible;
  }

  // CSS
  // NOTE: GTK 4.20 added prefers-color-scheme and prefers-contrast to CSS Providers,
  // which would be nice to include here

  private Gtk.CssProvider? add_css(Gtk.CssProvider? provider) {
    if (provider != null) {
      css_providers.append(provider);
    }
    return provider;
  }

  /**
   * Parses a CSS string, and if successful, adds it to the application's list of CSS Providers.
   */
  public Gtk.CssProvider? load_css_string(
    string css, int priority = Gtk.STYLE_PROVIDER_PRIORITY_USER
  ) {
    return add_css(Flurr.load_css_string(css, priority));
  }

  /**
   * Parses a CSS file, and if successful, adds it to the application's list of CSS Providers.
   */
  public Gtk.CssProvider? load_css_file(
    File file, int priority = Gtk.STYLE_PROVIDER_PRIORITY_USER
  ) {
    return add_css(Flurr.load_css_file(file, priority));
  }

  /**
   * Parses CSS from a file path, and if successful, adds it to the application's list of
   * CSS Providers.
   */
  public Gtk.CssProvider? load_css_path(
    string path, int priority = Gtk.STYLE_PROVIDER_PRIORITY_USER
  ) {
    return add_css(Flurr.load_css_path(path, priority));
  }

  /**
   * Removes all known CSS Providers known to the application from the default Gdk.Display.
   */
  public void reset_css() {
    var display = Gdk.Display.get_default();
    if (display == null) {
      warning(@"Couldn't get the default Gdk.Display to reset CSS.");
      return;
    }

    foreach (var provider in css_providers) {
      Gtk.StyleContext.remove_provider_for_display(display, provider);
    }
  }
}
