namespace Flurr {
[Flags]
public enum Anchor {
  TOP,
  BOTTOM,
  LEFT,
  RIGHT,
}

/**
 * Reexport of GtkLayerShell.Layer.
 */
public enum Layer {
  BACKGROUND = GtkLayerShell.Layer.BACKGROUND,
  BOTTOM = GtkLayerShell.Layer.BOTTOM,
  TOP = GtkLayerShell.Layer.TOP,
  OVERLAY = GtkLayerShell.Layer.OVERLAY,
}

/**
 * Reexport of GtkLayerShell.KeyboardMode.
 */
public enum KeyboardMode {
  NONE = GtkLayerShell.KeyboardMode.NONE,
  EXCLUSIVE = GtkLayerShell.KeyboardMode.EXCLUSIVE,
  ON_DEMAND = GtkLayerShell.KeyboardMode.ON_DEMAND,
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
