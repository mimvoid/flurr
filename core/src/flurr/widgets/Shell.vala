/**
 * An application window set up to behave as a desktop shell component.
 *
 * Under the hood, it uses gtk4-layer-shell to interact with the wlr-layer-shell protocol.
 * Most of these properties and methods are conveniences to call gtk4-layer-shell functions.
 */
public class Flurr.Shell : Gtk.ApplicationWindow {
  /**
   * Defines the purpose of the layer surface (e.g. "launcher").
   *
   * Some Wayland compositors like Hyprland can set layer rules for namespaces,
   * much like window rules.
   */
  public string namespace {
    get { return GtkLayerShell.get_namespace(this); }
    set { GtkLayerShell.set_namespace(this, value); }
  }

  public Gdk.Monitor? monitor {
    get { return GtkLayerShell.get_monitor(this); }
    set { GtkLayerShell.set_monitor(this, value); }
  }

  public Layer layer {
    get { return (Layer) GtkLayerShell.get_layer(this); }
    set { GtkLayerShell.set_layer(this, (GtkLayerShell.Layer) value); }
  }

  public KeyboardMode keyboard_mode {
    get { return (KeyboardMode) GtkLayerShell.get_keyboard_mode(this); }
    set { GtkLayerShell.set_keyboard_mode(this, (GtkLayerShell.KeyboardMode) value); }
  }

  public int z_index {
    get { return GtkLayerShell.get_exclusive_zone(this); }
    set { GtkLayerShell.set_exclusive_zone(this, value); }
  }

  public bool auto_exclusive_zone {
    get { return GtkLayerShell.auto_exclusive_zone_is_enabled(this); }
    set {
      if (value)
        GtkLayerShell.auto_exclusive_zone_enable(this);
      else
        GtkLayerShell.set_exclusive_zone(this, z_index);
    }
  }

  /**
   * Which edges of the screen, if any, to anchor the window to.
   */
  public Anchor anchor {
    get {
      var result = (GtkLayerShell.get_anchor(this, GtkLayerShell.Edge.TOP))
        ? Anchor.TOP
        : 0;

      if (GtkLayerShell.get_anchor(this, GtkLayerShell.Edge.RIGHT))
        result |= Anchor.RIGHT;

      if (GtkLayerShell.get_anchor(this, GtkLayerShell.Edge.BOTTOM))
        result |= Anchor.BOTTOM;

      if (GtkLayerShell.get_anchor(this, GtkLayerShell.Edge.LEFT))
        result |= Anchor.LEFT;

      return result;
    }
    set {
      GtkLayerShell.set_anchor(this, GtkLayerShell.Edge.TOP, Anchor.TOP in value);
      GtkLayerShell.set_anchor(this, GtkLayerShell.Edge.RIGHT, Anchor.RIGHT in value);
      GtkLayerShell.set_anchor(this, GtkLayerShell.Edge.BOTTOM, Anchor.BOTTOM in value);
      GtkLayerShell.set_anchor(this, GtkLayerShell.Edge.LEFT, Anchor.LEFT in value);
    }
  }

  public new int margin_top {
    get { return GtkLayerShell.get_margin(this, GtkLayerShell.Edge.TOP); }
    set { GtkLayerShell.set_margin(this, GtkLayerShell.Edge.TOP, value); }
  }
  public int margin_right {
    get { return GtkLayerShell.get_margin(this, GtkLayerShell.Edge.RIGHT); }
    set { GtkLayerShell.set_margin(this, GtkLayerShell.Edge.RIGHT, value); }
  }
  public new int margin_bottom {
    get { return GtkLayerShell.get_margin(this, GtkLayerShell.Edge.BOTTOM); }
    set { GtkLayerShell.set_margin(this, GtkLayerShell.Edge.BOTTOM, value); }
  }
  public int margin_left {
    get { return GtkLayerShell.get_margin(this, GtkLayerShell.Edge.LEFT); }
    set { GtkLayerShell.set_margin(this, GtkLayerShell.Edge.LEFT, value); }
  }

  public Shell(Gtk.Application app) {
    Object(application: app);
  }

  construct {
    GtkLayerShell.init_for_window(this);
    add_css_class("shell");
  }

  /**
   * Get all margins in a single struct.
   */
  public Edges get_margins() {
    return Edges() {
      top = margin_top,
      right = margin_right,
      bottom = margin_bottom,
      left = margin_left,
    };
  }

  /**
   * Set all margins with a single struct.
   */
  public void set_margins(Edges margins) {
    margin_top = margins.top;
    margin_right = margins.right;
    margin_bottom = margins.bottom;
    margin_left = margins.left;
  }
}
