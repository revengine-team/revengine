using Silk.NET.OpenGL;

namespace Revengine.Source.Engine.Render.Buffers.ElementBuffer
{
    public class ElementBufferObjects : Buffer
    {
        internal ElementBufferObjects(GL context, uint id) : base(context, id, BufferTargetARB.ElementArrayBuffer)
        {
            
        }

        public override void Bind()
        {
            base.Bind();
        }

        public override void Unbind()
        {
            base.Unbind();
        }

        public override void Dispose()
        {
            base.Dispose();
        }
    }
}