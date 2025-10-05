namespace Flurr {
public class PinShell : Shell {
  public bool unlocked { get; set; }

  private Gtk.MenuButton _menu_button = new Gtk.MenuButton() {
    visible = false
  };

  public Gtk.Overlay overlay {
    get;
    default = new Gtk.Overlay() {
      margin_start = 2,
      margin_end = 2,
      margin_top = 2,
      margin_bottom = 2,
    };
  }

  public PinShell(Gtk.Application app) {
    Object(application: app);
  }

  construct {
    add_css_class("pin-shell");
    remove_css_class("background");
    layer = Layer.BOTTOM;
    anchor = Anchor.TOP | Anchor.LEFT;
    child = overlay;

    notify["unlocked"].connect(() => {
      if (unlocked) add_css_class("unlocked");
      else remove_css_class("unlocked");
    });

    add_action_entries({
      { "toggle_lock", () => { unlocked = !unlocked; } },
    }, this);

    var _button_box = new Gtk.Box(Gtk.Orientation.HORIZONTAL, 2) {
      halign = Gtk.Align.END,
      valign = Gtk.Align.END,
      hexpand = true,
      vexpand = true,
    };
    _button_box.add_css_class("pin-shell-buttons");
    overlay.add_overlay(_button_box);
    overlay.set_measure_overlay(_button_box, false);

    bind_property("unlocked", _button_box, "visible", BindingFlags.SYNC_CREATE);

    _button_box.append(_menu_button);
    _button_box.append(new Gtk.Button.from_icon_name("changes-prevent-symbolic") {
      tooltip_text = "Lock",
      action_name = "win.toggle_lock",
      has_frame = false,
    });

    var model = new Menu();
    model.append("Unlock", "win.toggle_lock");
    _menu_button.popover = new Gtk.PopoverMenu.from_model(model) { has_arrow = false };

    _add_click_gesture();
    _add_gesture_drag();
  }

  private void _add_click_gesture() {
    var right_click = new Gtk.GestureClick() { button = Gdk.BUTTON_SECONDARY };

    right_click.pressed.connect((self, _n_pressed, x, y) => {
      var w = (PinShell) self.widget;
      if (w.unlocked)
        return;

      w._menu_button.popover.set_pointing_to(Gdk.Rectangle() {
        x = (int) x, y = (int) y, width = 1, height = 1
      });
      w._menu_button.popup();
    });

    ((Gtk.Widget) this).add_controller(right_click);
  }

  private void _add_gesture_drag() {
    var drag = new Gtk.GestureDrag();

    drag.drag_update.connect((self, offset_x, offset_y) => {
      var w = (PinShell) self.widget;
      if (!w.unlocked)
        return;

      self.set_state(Gtk.EventSequenceState.CLAIMED);

      var scale = w.get_surface().scale;
      w.margin_top += (int) (scale * offset_y);
      w.margin_left += (int) (scale * offset_x);
    });

    ((Gtk.Widget) this).add_controller(drag);
  }
}
}
