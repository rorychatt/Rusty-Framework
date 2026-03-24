using System;
using Rusty.Framework;
using Rusty.Framework.Widgets;

namespace Rusty.Framework.Samples;

public class HelloBoolInput : SampleBase
{
    protected override object BuildSample()
    {
        var checkboxState = UseState(false);
        var switchState = UseState(false);
        var toggleState = UseState(false);

        return Layout.Vertical()
               | Text.H1("Boolean Inputs")
               | Text.H2("Checkbox")
               | Layout.Horizontal(
                   checkboxState.ToBoolInput().Label("Checkbox Label").Description("This is a checkbox"),
                   new TextBlock($"Value: {checkboxState.Value}")
                 )
               | Text.H2("Switch")
               | Layout.Horizontal(
                   switchState.ToSwitchInput().Label("Switch Label"),
                   new TextBlock($"Value: {switchState.Value}")
                 )
               | Text.H2("Toggle")
               | Layout.Horizontal(
                   toggleState.ToToggleInput(Icons.Magnet).Label("Toggle Label"),
                   new TextBlock($"Value: {toggleState.Value}")
                 )
               | Text.H2("Variants")
               | (Layout.Grid().Columns(3).Gap(20)
                  | checkboxState.ToBoolInput().Label("Default")
                  | checkboxState.ToBoolInput().Label("Disabled").Disabled()
                  | checkboxState.ToBoolInput().Label("Invalid").Invalid("Required field")
                  
                  | switchState.ToSwitchInput().Label("Small").Small()
                  | switchState.ToSwitchInput().Label("Medium")
                  | switchState.ToSwitchInput().Label("Large").Large()
                 );
    }
}
