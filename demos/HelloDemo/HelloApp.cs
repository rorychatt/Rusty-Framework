using Rusty.Framework;
using Rusty.Framework.Widgets;

namespace HelloDemo;

public class HelloApp : ViewBase {
    protected Signal<string> nameState = new Signal<string>("");

    public override object Build() =>
        Layout.Center()
        | (new Card(
            Layout.Vertical().Gap(6).Padding(2)
            | new Confetti(new Logo())
            | Text.H2($"Hello {(string.IsNullOrEmpty(nameState.Value) ? "there" : nameState.Value)}!")
            | Text.Block("Welcome to the fantastic world of Rusty. Let's build something amazing together!")
            | new Separator()
            | new Separator()
            | Text.Markdown("You'd be a hero to us if you could ⭐ us on [Github](https://github.com/Ivy-Interactive/Ivy-Framework)")
          ).Width(500));
}
