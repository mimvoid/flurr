/**
 * A special Shell for transparent floating windows that can be dragged and repositioned,
 * similar to Rainmeter widgets or Plasmoids.
 *
 * Its margins act as its position relative to the edges it is anchored to. By default, it's
 * anchored to the top and left edge, so the top and left margins can be set to position it.
 *
 * The real child is a Gtk.Overlay, which displays overlaid buttons when unlocked.
 */
public class Flurr.PinShell : Shell {
  /**
   * Whether the window can be dragged and repositioned.
   */
  public bool unlocked { get; set; }

  /**
   * Gets and sets the child of the overlay, which is the direct child of Flurr.PinShell.
   */
  public new Gtk.Widget child {
    get { return overlay.child; }
    set { overlay.child = value; }
  }

  private Gtk.MenuButton _menu_button = new Gtk.MenuButton() {
    visible = false
  };

  // TODO: Make any child of the window be the overlay's child
  private Gtk.Overlay overlay = new Gtk.Overlay() {
    margin_start = 2,
    margin_end = 2,
    margin_top = 2,
    margin_bottom = 2,
  };

  public PinShell(Gtk.Application app) {
    Object(application: app);
  }

  construct {
    add_css_class("pin-shell");
    remove_css_class("background");
    layer = Flurr.Layer.BOTTOM;
    anchor = Flurr.Anchor.TOP | Flurr.Anchor.LEFT;
    base.child = overlay;

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

  public Gtk.Widget? get_overlay_child() {
    return overlay.get_child();
  }
  public void set_overlay_child(Gtk.Widget? child) {
    overlay.set_child(child);
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
