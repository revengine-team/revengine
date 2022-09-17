using Silk.NET.OpenGL;

namespace Revengine.Source.Engine.Render.Shaders
{
    public class ShaderProgramLinker : IShaderProgramLinker
    {
        private readonly GL _context;
        internal ShaderProgramLinker(GL context)
        {
            _context = context;
        }
        public IShaderProgram LinkProgram(IList<IShader> shaders)
        {
            var shaderProgramId = _context.CreateProgram();
            
            foreach(var shader in shaders)
            {
                var shaderId = shader.Compile(_context);

                _context.AttachShader(shaderProgramId, shaderId);
            }

            _context.LinkProgram(shaderProgramId);

            //_context.GetProgram(shaderProgramId, GLEnum.LinkStatus, out int status);
            //if (status == 0)
            //{
                //throw new CustomException();
            //}

            return new ShaderProgram(_context, shaderProgramId);
        }
    }
}