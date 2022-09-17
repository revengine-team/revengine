using Revengine.Source.Engine.Render.Resources.Readers;

namespace Revengine.Source.Engine.Render.Resources
{
    public interface IResource<T>
    {
        T Read(IResourceReader<T> resourceReader);
    }
}