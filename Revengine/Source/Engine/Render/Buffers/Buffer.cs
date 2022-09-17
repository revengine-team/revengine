using Silk.NET.OpenGL;

namespace Revengine.Source.Engine.Render.Buffers
{
    public abstract class Buffer : IBuffer
    {
        private readonly GL _context;
        private readonly uint _id;
        private readonly BufferTargetARB _bufferType;

        internal Buffer(GL context, uint id, BufferTargetARB bufferType)
        {
            _context = context;
            _id = id;
            _bufferType = bufferType;
        }
        public virtual void Bind()
        {
            _context.BindBuffer(_bufferType, _id);
        }

        public virtual void Dispose()
        {
            _context.DeleteBuffer(_id);
        }

        public virtual void Unbind()
        {
            _context.BindBuffer(_bufferType, 0);
        }
    }
}