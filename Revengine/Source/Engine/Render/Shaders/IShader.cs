using Silk.NET.OpenGL;


namespace Revengine.Source.Engine.Render.Shaders
{
    public interface IShader
    {
        uint Compile(GL context);
    }
}