namespace Revengine.Source.Engine.Render.Buffers
{
    public interface IBuffer : IDisposable
    {
        void Bind();
        void Unbind();
    }
}