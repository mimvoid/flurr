namespace Flurr {
// NOTE: Although Gtk.StyleContext is deprecated, its static functions do not seem to be.

public bool load_css_string(string css, int priority = Gtk.STYLE_PROVIDER_PRIORITY_USER) {
  return load_css(priority, (provider) => {
    provider.load_from_string(css);
  });
}

public bool load_css_file(File file, int priority = Gtk.STYLE_PROVIDER_PRIORITY_USER) {
  return load_css(priority, (provider) => {
    provider.load_from_file(file);
  });
}

public bool load_css_path(string path, int priority = Gtk.STYLE_PROVIDER_PRIORITY_USER) {
  return load_css(priority, (provider) => {
    provider.load_from_path(path);
  });
}

delegate void CssProviderFunc(Gtk.CssProvider provider);

private bool load_css(int priority, CssProviderFunc callback) {
  var display = Gdk.Display.get_default();
  if (display == null)
    return false;

  var css_provider = new Gtk.CssProvider();
  callback(css_provider);
  Gtk.StyleContext.add_provider_for_display(
    display, css_provider, priority
  );

  return true;
}
}
