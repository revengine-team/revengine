using Silk.NET.OpenGL;


namespace Revengine.Source.Engine.Render
{
    public class RenderService
    {
        private readonly GL _gl;

        public RenderService(GL gl)
        {
            _gl = gl;
        }
    }
}