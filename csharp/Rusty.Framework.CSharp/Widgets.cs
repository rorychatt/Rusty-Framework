#nullable disable

using System;
using System.Collections.Generic;

namespace Rusty.Framework
{
    public class Signal<T>
    {
        public T Value { get; set; }
        public string id;
        public Signal(string id, T initial) { this.id = id; Value = initial; }
        public void Set(T value) { Value = value; }
        public T get() => Value; // For transpiler compatibility if needed
    }

    public abstract class ViewBase
    {
        public abstract object Build();
        protected Signal<T> UseState<T>(T initial = default) => new Signal<T>("", initial);
        protected void UseEffect(Action action, params object[] dependencies) { }
    }

    public abstract class SampleBase : ViewBase
    {
        protected abstract object BuildSample();
        public override object Build() => BuildSample();
    }
}

namespace Rusty.Framework.Widgets
{
    public abstract class Widget
    {
        public static Layout operator |(Widget left, Widget right) => null;
        public static Layout operator |(Layout left, Widget right) => null;
        public static Layout operator |(Layout left, IEnumerable<Widget> right) => null;
    }

    public class Layout : Widget
    {
        public static Layout Center(params Widget[] children) => null;
        public static Layout Vertical(params Widget[] children) => null;
        public static Layout Horizontal(params Widget[] children) => null;
        public static Layout Grid(params Widget[] children) => null;
        public Layout Columns(int n) => this;
        public Layout Gap(double pixels) => this;
        public Layout Padding(double pixels) => this;
        public Layout Width(double pixels) => this;
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
        public static Widget H3(string text) => null;
        public static Widget P(string text) => null;
        public static Widget Monospaced(string text) => null;
        public static Widget Block(string text) => null;
        public static Widget Markdown(string text) => null;
    }

    public class Logo : Widget
    {
        public Logo(LogoType type) { }
    }
    public enum LogoType { Ivy }

    public class Separator : Widget { }
    public class Confetti : Widget
    {
        public Confetti(Widget child) { }
    }

    public enum BoolInputVariant { Checkbox, Switch, Toggle }

    public class BoolInput : Widget
    {
        public BoolInput(Signal<bool> value) { }
        public BoolInput Label(string text) => this;
        public BoolInput Description(string text) => this;
        public BoolInput Disabled() => this;
        public BoolInput Invalid(string text) => this;
        public BoolInput Loading(bool l) => this;
        public BoolInput Variant(BoolInputVariant v) => this;
        public BoolInput Icon(Icons i) => this;
        public BoolInput Small() => this;
        public BoolInput Large() => this;
        public BoolInput TestId(string id) => this;
    }

    public class TextInput : Widget
    {
        public TextInput(Signal<string> value) { }
        public TextInput Placeholder(string p) => this;
        public TextInput Label(string text) => this;
        public TextInput Description(string text) => this;
        public TextInput Disabled() => this;
        public TextInput Invalid(string text) => this;
        public TextInput Small() => this;
        public TextInput Large() => this;
        public TextInput Prefix(string p) => this;
        public TextInput Suffix(string s) => this;
        public TextInput Prefix(Icons i) => this;
        public TextInput Suffix(Icons i) => this;
        public TextInput MinLength(int n) => this;
        public TextInput MaxLength(int n) => this;
        public TextInput OnBlur(Action<string> a) => this;
        public TextInput OnSubmit(Action a) => this;
        public TextInput ShortcutKey(string key) => this;
    }

    public class Spacer : Widget
    {
        public Spacer Height(double pixels) => this;
        public Spacer Width(double pixels) => this;
    }

    public class Badge : Widget
    {
        public Badge(string text) { }
    }

    public class Box : Widget
    {
        public Box(string text) { }
    }

    public static class SignalExtensions
    {
        public static TextInput ToTextInput(this Signal<string> signal) => null;
        public static TextInput ToPasswordInput(this Signal<string> signal) => null;
        public static TextInput ToTextareaInput(this Signal<string> signal) => null;
        public static TextInput ToSearchInput(this Signal<string> signal) => null;
        public static BoolInput ToBoolInput(this Signal<bool> signal) => null;
        public static BoolInput ToBoolInput(this Signal<bool?> signal) => null;
        public static BoolInput ToSwitchInput(this Signal<bool> signal) => null;
        public static BoolInput ToToggleInput(this Signal<bool> signal, Icons icon) => null;
    }

    public static class Widgets
    {
        public static Widget Center(Widget child) => null;
    }
}
