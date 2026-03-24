namespace Rusty.Framework.Widgets;

public static class Text {
    public static object H2(string content) => new object();
    public static object Block(string content) => new object();
    public static object Markdown(string content) => new object();
}

public static class Layout {
    public static LayoutBuilder Center() => new LayoutBuilder();
    public static LayoutBuilder Vertical() => new LayoutBuilder();
    public static LayoutBuilder Horizontal() => new LayoutBuilder();
}

public class LayoutBuilder {
    public static LayoutBuilder operator |(LayoutBuilder left, object right) => left;
    public LayoutBuilder Gap(int units) => this;
    public LayoutBuilder Padding(int units) => this;
    public LayoutBuilder Width(object size) => this;
}

public static class Size {
    public static string Units(double value) => "";
}

public class Card {
    public Card(object child) {}
    public Card Width(object size) => this;
}

public class Confetti {
    public Confetti(object child) {}
}

public class Logo {}
public class Separator {
    public object ToInput() => new object();
}
