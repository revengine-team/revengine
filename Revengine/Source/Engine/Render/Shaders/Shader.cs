using Silk.NET.OpenGL;


namespace Revengine.Source.Engine.Render.Shaders
{
    public abstract class Shader : IShader
    {
        private readonly String _shaderCode;
        private readonly ShaderType _shaderType;

        internal Shader(String shaderCode, ShaderType shaderType)
        {
            _shaderCode = shaderCode;
            _shaderType = shaderType;
        }

        public virtual uint Compile(GL context)
        {   
            uint shader = context.CreateShader(_shaderType);
            context.ShaderSource(shader, _shaderCode);
            context.CompileShader(shader);

            //infoLog = gl.GetShaderInfoLog(shader);
            //if (!string.IsNullOrWhiteSpace(infoLog))
            //{
                //throw new CustomException();
            //}

            return shader;
        }
    }
}