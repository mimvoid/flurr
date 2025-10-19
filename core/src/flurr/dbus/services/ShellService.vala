public class Flurr.ShellService : FlurrDBus.Shell, Flurr.DBusService, Object {
  public weak Flurr.Shell shell { get; construct; }

  public ShellService(Flurr.Shell shell) {
    Object(shell: shell);
  }

  protected void on_dbus_connect(GLib.DBusConnection conn) {
    connect_window_dbus(shell, conn, (FlurrDBus.Shell) this);
  }

  // DBus

  public string namespace { owned get { return shell.namespace; } }

  public uint8 layer {
    get { return shell.layer; }
    set {
      if (Flurr.Layer.is_valid(value))
        shell.layer = (Flurr.Layer) value;
    }
  }
  public uint8 keyboard_mode {
    get { return shell.keyboard_mode; }
    set {
      if (Flurr.KeyboardMode.is_valid(value))
        shell.keyboard_mode = (Flurr.KeyboardMode) value;
    }
  }
  public uint8 anchor {
    get { return shell.anchor; }
    set {
      if (Flurr.Anchor.is_valid(value))
        shell.anchor = (Flurr.Anchor) value;
    }
  }
  public int exclusion {
    get { return shell.exclusion; }
    set { shell.exclusion = value; }
  }
  public bool auto_exclusion {
    get { return shell.auto_exclusion; }
    set { shell.auto_exclusion = value; }
  }

  public int margin_top {
    get { return shell.margin_top; }
    set { shell.margin_top = value; }
  }
  public int margin_right {
    get { return shell.margin_right; }
    set { shell.margin_right = value; }
  }
  public int margin_bottom {
    get { return shell.margin_bottom; }
    set { shell.margin_bottom = value; }
  }
  public int margin_left {
    get { return shell.margin_left; }
    set { shell.margin_left = value; }
  }
}
