using System;
using System.Collections.Generic;
using System.Linq;
using Rusty.Framework;
using Rusty.Framework.Widgets;

namespace Rusty.Framework.Samples;

public class HelloTextInput : SampleBase
{
    protected override object BuildSample()
    {
        var withoutValue = UseState((string)null);
        var withValue = UseState("Hello");

        var onChangedState = UseState("");
        var onChangeLabel = UseState("");
        UseEffect(() => { onChangeLabel.Set(string.IsNullOrEmpty(onChangedState.Value) ? "" : "Changed"); }, onChangedState);
        
        var onBlurState = UseState("");
        var onBlurLabel = UseState("");

        var stringState = UseState("");

        var dataBinding = Layout.Grid().Columns(3)
                          | Text.Monospaced("string")
                          | (Layout.Vertical()
                             | stringState.ToTextInput()
                             | stringState.ToTextareaInput()
                             | stringState.ToPasswordInput()
                             | stringState.ToSearchInput()
                          )
                          | stringState.Value
            ;

        return Layout.Vertical()
               | Text.H1("Text Inputs")
               | Text.H2("Sizes")
               | new HelloWorldTextInputSizes()
               | Text.H2("Variants")
               | (Layout.Grid().Columns(5)
                  | new Separator()
                  | Text.Monospaced("Empty")
                  | Text.Monospaced("With Value")
                  | Text.Monospaced("Disabled")
                  | Text.Monospaced("Invalid")

                  | Text.Monospaced("TextInputVariant.Text")
                  | withoutValue.ToTextInput().Placeholder("Placeholder")
                  | withValue.ToTextInput()
                  | withValue.ToTextInput().Disabled()
                  | withValue.ToTextInput().Invalid("Invalid input example")

                  | Text.Monospaced("TextInputVariant.Password")
                  | withoutValue.ToPasswordInput().Placeholder("Placeholder")
                  | withValue.ToPasswordInput()
                  | withValue.ToPasswordInput().Disabled()
                  | withValue.ToPasswordInput().Invalid("Invalid input example")

                  | Text.Monospaced("TextInputVariant.Textarea")
                  | withoutValue.ToTextareaInput().Placeholder("Placeholder")
                  | withValue.ToTextareaInput()
                  | withValue.ToTextareaInput().Disabled()
                  | withValue.ToTextareaInput().Invalid("Invalid input example")

                  | Text.Monospaced("TextInputVariant.Search")
                  | withoutValue.ToSearchInput().Placeholder("Placeholder").ShortcutKey("Ctrl+K")
                  | withValue.ToSearchInput()
                  | withValue.ToSearchInput().Disabled()
                  | withValue.ToSearchInput().Invalid("Invalid input example")
               )

               | Text.H2("Data Binding")
               | dataBinding

               | Text.H2("Events")
               | Text.H3("OnChange")
               | Layout.Horizontal(
                   onChangedState.ToTextInput(),
                   new TextBlock(onChangeLabel.Value)
                )
               | Text.H3("OnBlur")
               | Layout.Horizontal(
                   onBlurState.ToTextInput().OnBlur(e => onBlurLabel.Set("Blur")),
                   new TextBlock(onBlurLabel.Value)
               )
            ;
    }
}

public class HelloWorldTextInputSizes : ViewBase
{
    public override object Build()
    {
        var textState = UseState("Hello");

        return Layout.Grid().Columns(4)
               | Text.Monospaced("Variant")
               | Text.Monospaced("Small")
               | Text.Monospaced("Medium")
               | Text.Monospaced("Large")

               | Text.Monospaced("TextInputVariant.Text")
               | textState.ToTextInput().Small()
               | textState.ToTextInput()
               | textState.ToTextInput().Large();
    }
}

// Simplified TextBlock for internal use if needed, but we have Text.P etc.
public class TextBlock : Widget {
    public TextBlock(string content) {}
}
