using Silk.NET.OpenGL;


namespace Revengine.Source.Engine.Render.Shaders.FragmentShaders
{
    public class FragmentShader : Shader
    {
        internal FragmentShader(String shaderSource) : base(shaderSource, ShaderType.FragmentShader) 
        {

        }

        public override uint Compile(GL context)
        {
            return base.Compile(context);
        }
    }
}