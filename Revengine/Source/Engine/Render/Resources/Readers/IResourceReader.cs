namespace Revengine.Source.Engine.Render.Resources.Readers
{
    public interface IResourceReader<T>
    {
        T ReadFromFile(String fileName);
    }
}