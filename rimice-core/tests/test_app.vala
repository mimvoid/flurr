public static int main(string[] args) {
  var app = new Rimice.Application();

  app.activate.connect(() => {
    var window = new Gtk.ApplicationWindow(app) {
      title = "Example window",
      default_width = 480,
      default_height = 360,
    };

    window.present();
  });

  return app.run(args);
}
