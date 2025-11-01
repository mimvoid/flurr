namespace Flurr {
// NOTE: Although Gtk.StyleContext is deprecated, its static functions do not seem to be.

public Gtk.CssProvider? load_css_string(string css, int priority = Gtk.STYLE_PROVIDER_PRIORITY_USER) {
  return load_css(priority, @"String: \"$css\"", (provider) => {
    provider.load_from_string(css);
  });
}

public Gtk.CssProvider? load_css_file(File file, int priority = Gtk.STYLE_PROVIDER_PRIORITY_USER) {
  return load_css(priority, "File: " + file.get_parse_name(), (provider) => {
    provider.load_from_file(file);
  });
}

public Gtk.CssProvider load_css_path(string path, int priority = Gtk.STYLE_PROVIDER_PRIORITY_USER) {
  return load_css(priority, "Path: " + path, (provider) => {
    provider.load_from_path(path);
  });
}

delegate void CssProviderFunc(Gtk.CssProvider provider);

private Gtk.CssProvider? load_css(int priority, string extra_err_info, CssProviderFunc callback) {
  var display = Gdk.Display.get_default();
  if (display == null) {
    warning(@"Couldn't get Gdk.Display to add the following CSS:\n$extra_err_info");
    return null;
  }

  var css_provider = new Gtk.CssProvider();
  css_provider.parsing_error.connect((section, err) => {
    warning(@"Failed to parse CSS: $(err.message)\nAt: $section\n$extra_err_info");
  });

  callback(css_provider);

  Gtk.StyleContext.add_provider_for_display(display, css_provider, priority);
  return css_provider;
}
}
