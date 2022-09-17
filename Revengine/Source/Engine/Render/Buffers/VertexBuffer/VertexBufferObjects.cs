using Silk.NET.OpenGL;
using Revengine.Source.Engine.Render.Buffers.Vertices;

namespace Revengine.Source.Engine.Render.Buffers.VertexBuffer
{
    public class VertexBufferObjects : Buffer
    {
        internal VertexBufferObjects(GL context, uint id) : base(context, id, BufferTargetARB.ArrayBuffer)
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