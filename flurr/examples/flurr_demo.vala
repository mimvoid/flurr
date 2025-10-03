public static int main(string[] args) {
  var app = new Flurr.Application();

  app.activate.connect(() => {
    var pin = new Flurr.PinShell(app) {
      title = "Example window",
    };

    var pin_box = new Gtk.Box(Gtk.Orientation.VERTICAL, 2);
    pin.overlay.child = pin_box;

    pin_box.append(new Gtk.Label("Flurr"));
    pin_box.append(new Gtk.Label("This is a widget."));

    var bar = new Flurr.Shell(app) {
      title = "Basic bar",
      anchor = Flurr.Anchor.TOP | Flurr.Anchor.LEFT | Flurr.Anchor.RIGHT,
      auto_exclusive_zone = true,
    };
    bar.set_margins(Flurr.Edges.block_inline(6, 12));

    var bar_box = new Gtk.Box(Gtk.Orientation.HORIZONTAL, 2);
    bar.child = bar_box;

    bar_box.append(new Gtk.Label("This is another widget."));

    pin.present();
    bar.present();
  });

  return app.run(args);
}
