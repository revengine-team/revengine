using Silk.NET.OpenGL;

namespace Revengine.Source.Engine.Render.Shaders
{
    public class ShaderProgram : IShaderProgram
    {
        private readonly GL _context;
        private readonly uint _shaderProgram;

        internal ShaderProgram(GL context, uint shaderProgram)
        {
            _shaderProgram = shaderProgram;
            _context = context;
        }
        public void Activate()
        {
            _context.UseProgram(_shaderProgram);
        }

        public void Dispose()
        {
            _context.DeleteProgram(_shaderProgram);
        }
    }
}