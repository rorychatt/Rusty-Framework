using Rusty.Framework.Widgets;

namespace Rusty.Framework
{
    public abstract class SampleBase : ViewBase
    {
        protected abstract object BuildSample();
        public override object Build() => BuildSample();
    }
}
