namespace FlurrDBus {
[DBus(name = "io.flurr.Error")]
public errordomain FlurrError {
  [DBus(name = "WindowNotFound")]
  WINDOW_NOT_FOUND,
}

[DBus(name = "io.flurr.Application")]
public interface Application : Object {
  public abstract async ObjectPath get_window_path(string window_name)
    throws FlurrError, DBusError, IOError;
  public abstract async string[] list_window_names() throws DBusError, IOError;
  public abstract async uint[] list_window_ids() throws DBusError, IOError;
  public abstract async ObjectPath[] list_window_paths() throws DBusError, IOError;
  public abstract void quit() throws DBusError, IOError;
}

[DBus(name = "io.flurr.Window")]
public interface Window : Object {
  public abstract string name { owned get; }
  public abstract bool visible { get; set; }
  public abstract async void toggle() throws DBusError, IOError;
}

[DBus(name = "io.flurr.Shell")]
public interface Shell : Object {
  public abstract string namespace { owned get; }

  public abstract Flurr.Layer layer { get; set; }
  public abstract Flurr.KeyboardMode keyboard_mode { get; set; }
  public abstract Flurr.Anchor anchor { get; set; }
  public abstract int z_index { get; set; }
  public abstract bool auto_exclusive_zone { get; set; }

  public abstract int margin_top { get; set; }
  public abstract int margin_right { get; set; }
  public abstract int margin_bottom { get; set; }
  public abstract int margin_left { get; set; }
}

[DBus(name = "io.flurr.PinShell")]
public interface PinShell : Object {
  public abstract bool unlocked { get; set; }
}
}
