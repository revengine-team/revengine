using Silk.NET.OpenGL;


namespace Revengine.Source.Engine.Render.Shaders.VertexShaders
{
    public class VertexShader : Shader
    {
        internal VertexShader(String shaderSource) : base(shaderSource, ShaderType.VertexShader) 
        {

        }

        public override uint Compile(GL context)
        {
            return base.Compile(context);
        }
    }
}