public class Flurr.PinShell : Flurr.Shell {
  Gtk.PopoverMenu _popover_menu;
  Gtk.MenuButton _menu_button;
  int _saved_margin_top = 0;
  int _saved_margin_left = 0;

  public bool unlocked { get; private set; default = false; }

  public PinShell(Gtk.Application app) {
    Object(application: app);
  }

  construct {
    add_css_class("pin-shell");
    layer = GtkLayerShell.Layer.BOTTOM;
    anchor = Flurr.Anchor.TOP | Flurr.Anchor.LEFT;
    exclusion = Flurr.Exclusion.NONE;

    add_action_entries({
      { "toggle_lock", this.toggle_lock },
    }, this);

    var model = new Menu();
    model.append("Toggle lock", "win.toggle_lock");

    _popover_menu = new Gtk.PopoverMenu.from_model(model) { has_arrow = false };
    _menu_button = new Gtk.MenuButton() { popover = _popover_menu };
    titlebar = _menu_button;

    var right_click = new Gtk.GestureClick() { button = Gdk.BUTTON_SECONDARY };

    right_click.pressed.connect((self, _n_pressed, x, y) => {
      var w = (Flurr.PinShell) self.widget;
      w._popover_menu.set_pointing_to(Gdk.Rectangle() {
        x = (int) x, y = (int) y, width = 1, height = 1
      });
      w._menu_button.popup();
    });

    ((Gtk.Widget) this).add_controller(right_click);

    var drag = new Gtk.GestureDrag();

    drag.drag_begin.connect((self, _x, _y) => {
      var w = (Flurr.PinShell) self.widget;
      if (!w.unlocked)
        return;

      w._saved_margin_top = w.margin_top;
      w._saved_margin_left = w.margin_left;
    });

    drag.drag_update.connect((self, offset_x, offset_y) => {
      var w = (Flurr.PinShell) self.widget;
      if (!w.unlocked)
        return;

      w.margin_top = w._saved_margin_top + (int) offset_y;
      w.margin_left = w._saved_margin_left + (int) offset_x;
    });

    ((Gtk.Widget) this).add_controller(drag);
  }

  public void toggle_lock() {
    if (unlocked) {
      unlocked = false;
      remove_css_class("unlocked");
    } else {
      unlocked = true;
      add_css_class("unlocked");
    }
  }
}
