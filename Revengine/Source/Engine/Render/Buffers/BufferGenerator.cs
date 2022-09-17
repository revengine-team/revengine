using Silk.NET.OpenGL;

namespace Revengine.Source.Engine.Render.Buffers
{
    public class BufferGenerator<T> : IBufferGenerator<T> where T : unmanaged
    {
        private readonly GL _context;
        internal BufferGenerator(GL context)
        {
            _context = context;
        }

        public unsafe void Generate(BufferTargetARB bufferType, Span<T> data)
        {
            var id = _context.GenBuffer();
            _context.BindBuffer(bufferType, id);

            fixed(void* dataPointer = data)
            {
                _context.BufferData(bufferType, (nuint) (data.Length * sizeof(T)), dataPointer, BufferUsageARB.StaticDraw);
            }
        }
    }
}