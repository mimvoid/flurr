public class FlurrDBus.ShellService : FlurrDBus.Shell, FlurrDBus.Service, Object {
  public Flurr.Shell shell { get; construct; }

  public ShellService(Flurr.Shell shell) {
    Object(shell: shell);
  }

  protected void on_dbus_connect(GLib.DBusConnection conn) {
    var id = shell.get_id();
    if (id == 0) {
      critical(@"Window \"$(shell.name)\" is unregistered, cannot add to DBus.");
      return;
    }

    try {
      conn.register_object(
        @"$(FlurrDBus.BASE_WINDOW_OBJECT_PATH)/$id", (FlurrDBus.Shell) this
      );
    } catch (Error err) {
      critical(err.message);
    }
  }

  // DBus

  public string name {
    owned get { return shell.name; }
    set { shell.name = value; }
  }
  public string namespace {
    owned get { return shell.namespace; }
    set { shell.namespace = value; }
  }

  public Flurr.Layer layer {
    get { return shell.layer; }
    set { shell.layer = value; }
  }
  public Flurr.KeyboardMode keyboard_mode {
    get { return shell.keyboard_mode; }
    set { shell.keyboard_mode = value; }
  }
  public Flurr.Anchor anchor {
    get { return shell.anchor; }
    set { shell.anchor = value; }
  }
  public int z_index {
    get { return shell.z_index; }
    set { shell.z_index = value; }
  }
  public bool auto_exclusive_zone {
    get { return shell.auto_exclusive_zone; }
    set { shell.auto_exclusive_zone = value; }
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

  public void show() throws DBusError, IOError {
    shell.visible = true;
  }
  public void hide() throws DBusError, IOError {
    shell.visible = false;
  }
  public void toggle() throws DBusError, IOError {
    shell.visible = !shell.visible;
  }
}
