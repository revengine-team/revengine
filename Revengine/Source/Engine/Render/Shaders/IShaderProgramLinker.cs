namespace Revengine.Source.Engine.Render.Shaders
{
    public interface IShaderProgramLinker
    {
        IShaderProgram LinkProgram(IList<IShader> shaders);
    }
}