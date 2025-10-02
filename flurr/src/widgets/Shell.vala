[Flags]
public enum Flurr.Anchor {
  TOP,
  BOTTOM,
  LEFT,
  RIGHT,
}

public enum Flurr.Exclusion {
  NORMAL,
  EXCLUSIVE,
  NONE,
}

public struct Flurr.Edges {
  public int top;
  public int right;
  public int bottom;
  public int left;

  public Edges() {
    this.with_edges(0, 0, 0, 0);
  }
  public Edges.uniform(int size) {
    this.with_edges(size, size, size, size);
  }
  public Edges.block_inline(int vertical, int horizontal) {
    this.with_edges(vertical, horizontal, vertical, horizontal);
  }
  public Edges.with_edges(int top, int right, int bottom, int left) {
    this.top = top;
    this.right = right;
    this.bottom = bottom;
    this.left = left;
  }
}

public class Flurr.Shell : Gtk.ApplicationWindow {
  public string namespace {
    get { return GtkLayerShell.get_namespace(this); }
    set { GtkLayerShell.set_namespace(this, value); }
  }

  public GtkLayerShell.Layer layer {
    get { return GtkLayerShell.get_layer(this); }
    set { GtkLayerShell.set_layer(this, value); }
  }

  public Gdk.Monitor? monitor {
    get { return GtkLayerShell.get_monitor(this); }
    set { GtkLayerShell.set_monitor(this, value); }
  }

  public GtkLayerShell.KeyboardMode keyboard_mode {
    get { return GtkLayerShell.get_keyboard_mode(this); }
    set { GtkLayerShell.set_keyboard_mode(this, value); }
  }

  public Flurr.Exclusion exclusion {
    get {
      if (GtkLayerShell.auto_exclusive_zone_is_enabled(this)) {
        return Flurr.Exclusion.EXCLUSIVE;
      }
      if (GtkLayerShell.get_exclusive_zone(this) < 0) {
        return Flurr.Exclusion.NONE;
      }
      return Flurr.Exclusion.NORMAL;
    }
    set {
      switch (value) {
        case Flurr.Exclusion.NORMAL:
          GtkLayerShell.set_exclusive_zone(this, 0);
          break;
        case Flurr.Exclusion.EXCLUSIVE:
          GtkLayerShell.auto_exclusive_zone_enable(this);
          break;
        case Flurr.Exclusion.NONE:
          GtkLayerShell.set_exclusive_zone(this, -1);
          break;
      }
    }
  }

  public Flurr.Anchor anchor {
    get {
      var result = (GtkLayerShell.get_anchor(this, GtkLayerShell.Edge.TOP))
        ? Flurr.Anchor.TOP
        : 0;

      if (GtkLayerShell.get_anchor(this, GtkLayerShell.Edge.RIGHT)) {
        result |= Flurr.Anchor.RIGHT;
      }
      if (GtkLayerShell.get_anchor(this, GtkLayerShell.Edge.BOTTOM)) {
        result |= Flurr.Anchor.BOTTOM;
      }
      if (GtkLayerShell.get_anchor(this, GtkLayerShell.Edge.LEFT)) {
        result |= Flurr.Anchor.LEFT;
      }

      return result;
    }
    set {
      GtkLayerShell.set_anchor(this, GtkLayerShell.Edge.TOP, Flurr.Anchor.TOP in value);
      GtkLayerShell.set_anchor(this, GtkLayerShell.Edge.RIGHT, Flurr.Anchor.RIGHT in value);
      GtkLayerShell.set_anchor(this, GtkLayerShell.Edge.BOTTOM, Flurr.Anchor.BOTTOM in value);
      GtkLayerShell.set_anchor(this, GtkLayerShell.Edge.LEFT, Flurr.Anchor.LEFT in value);
    }
  }

  public new int margin_top {
    get { return GtkLayerShell.get_margin(this, GtkLayerShell.Edge.TOP); }
    set { GtkLayerShell.set_margin(this, GtkLayerShell.Edge.TOP, value); }
  }
  public new int margin_right {
    get { return GtkLayerShell.get_margin(this, GtkLayerShell.Edge.RIGHT); }
    set { GtkLayerShell.set_margin(this, GtkLayerShell.Edge.RIGHT, value); }
  }
  public new int margin_bottom {
    get { return GtkLayerShell.get_margin(this, GtkLayerShell.Edge.BOTTOM); }
    set { GtkLayerShell.set_margin(this, GtkLayerShell.Edge.BOTTOM, value); }
  }
  public new int margin_left {
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

  public Flurr.Edges get_margins() {
    return Flurr.Edges() {
      top = margin_top,
      right = margin_right,
      bottom = margin_bottom,
      left = margin_left,
    };
  }

  public void set_margins(Flurr.Edges margins) {
    margin_top = margins.top;
    margin_right = margins.right;
    margin_bottom = margins.bottom;
    margin_left = margins.left;
  }
}
