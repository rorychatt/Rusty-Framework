#nullable disable

using System;

namespace Rusty.Framework
{
    public class Signal<T>
    {
        public T Value { get; set; }
        public Signal(T initial) { Value = initial; }
    }

    public abstract class ViewBase
    {
        public abstract object Build();
    }
}

namespace Rusty.Framework.Widgets
{
    public abstract class Widget
    {
        public static Layout operator |(Widget left, Widget right) => null;
        public static Layout operator |(Layout left, Widget right) => null;
    }

    public class Layout : Widget
    {
        public static Layout Center() => null;
        public static Layout Vertical() => null;
        public Layout Gap(double pixels) => this;
        public Layout Padding(double pixels) => this;
    }

    public class Card : Widget
    {
        public Card(Widget child) { }
        public Card Width(double pixels) => this;
    }

    public static class Text
    {
        public static Widget H1(string text) => null;
        public static Widget H2(string text) => null;
        public static Widget Block(string text) => null;
        public static Widget Markdown(string text) => null;
    }

    public class Logo : Widget { }
    public class Separator : Widget { }
    public class Confetti : Widget 
    {
        public Confetti(Widget child) { }
    }
    public class TextInput : Widget {
        public TextInput(Signal<string> value) { }
        public TextInput Placeholder(string p) => this;
    }

    public static class SignalExtensions {
        public static Widget ToInput(this Signal<string> signal, string placeholder = "") => null;
    }

    public static class Widgets
    {
        public static Widget Center(Widget child) => null;
    }
}
