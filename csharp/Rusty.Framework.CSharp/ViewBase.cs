namespace Rusty.Framework;

public class ViewBase {
    public static Signal<T> UseState<T>(T initialValue) => new Signal<T>(initialValue);
    public virtual object Build() => new object();
}

public class Signal<T> {
    public T Value { get; set; }
    public Signal(T initial) => Value = initial;
}
