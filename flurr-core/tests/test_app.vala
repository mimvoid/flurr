public static int main(string[] args) {
  var app = new Flurr.Application();

  app.activate.connect(() => {
    var window = new Flurr.PinWindow(app) {
      title = "Example window",
      default_width = 480,
      default_height = 360,
    };

    var box = new Gtk.Box(Gtk.Orientation.VERTICAL, 2);
    box.append(new Gtk.Label("Flurr"));
    box.append(new Gtk.Label("This is a widget."));

    window.child = box;
    window.present();
  });

  return app.run(args);
}
