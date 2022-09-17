using Silk.NET.OpenGL;

namespace Revengine.Source.Engine.Render.Buffers.VertexArray
{
    public class VertexArrayObject
    {
        private readonly GL _context;
        private readonly uint _id;

        public VertexArrayObject(GL context, uint id)
        {
            _context = context;
            _id = id;
        }

        public void Bind()
        {
            _context.BindVertexArray(_id);
        }

        public void Unbind()
        {
            _context.BindVertexArray(0);
        }

        public void Dispose()
        {
            _context.DeleteVertexArray(_id);
        }
    }
}