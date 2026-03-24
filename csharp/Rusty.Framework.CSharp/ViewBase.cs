namespace Rusty.Framework;

public class ViewBase {
    protected Signal<T> UseState<T>(T initial) => new Signal<T>(initial);
    public virtual object Build() => null;
}

public class Signal<T> {
    public T Value { get; set; }
    public Signal(T initial) => Value = initial;
}
