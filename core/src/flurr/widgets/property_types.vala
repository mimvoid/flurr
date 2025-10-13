namespace Flurr {
[Flags]
public enum Anchor {
  TOP,
  BOTTOM,
  LEFT,
  RIGHT;

  /**
   * Check if only known bits for Flurr.Anchor are set.
   */
  public static bool is_valid(uint8 bits) {
    var stripped = bits & ~TOP & ~BOTTOM & ~LEFT & ~RIGHT;
    return stripped == 0;
  }
}

/**
 * Reexport of GtkLayerShell.Layer.
 */
public enum Layer {
  BACKGROUND = GtkLayerShell.Layer.BACKGROUND,
  BOTTOM = GtkLayerShell.Layer.BOTTOM,
  TOP = GtkLayerShell.Layer.TOP,
  OVERLAY = GtkLayerShell.Layer.OVERLAY;

  /**
   * Checks if the given value matches a member of Layer.
   */
  public static bool is_valid(uint8 num) {
    return (num == BACKGROUND || num == BOTTOM || num == TOP || num == OVERLAY);
  }
}

/**
 * Reexport of GtkLayerShell.KeyboardMode.
 */
public enum KeyboardMode {
  NONE = GtkLayerShell.KeyboardMode.NONE,
  EXCLUSIVE = GtkLayerShell.KeyboardMode.EXCLUSIVE,
  ON_DEMAND = GtkLayerShell.KeyboardMode.ON_DEMAND;

  /**
   * Checks if the given value matches a member of KeyboardMode.
   */
  public static bool is_valid(uint8 num) {
    return (num == NONE || num == EXCLUSIVE || num == ON_DEMAND);
  }
}

public struct Edges {
  int top;
  int right;
  int bottom;
  int left;

  public Edges() {
    this.with_edges(0, 0, 0, 0);
  }
  public Edges.uniform(int size) {
    this.with_edges(size, size, size, size);
  }
  public Edges.block_inline(int vertical, int horizontal) {
    this.with_edges(vertical, horizontal, vertical, horizontal);
  }
  public Edges.with_edges(int top, int right, int bottom, int left) {
    this.top = top;
    this.right = right;
    this.bottom = bottom;
    this.left = left;
  }
}
}
