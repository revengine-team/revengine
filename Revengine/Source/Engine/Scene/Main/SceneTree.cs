class SceneTree : GameLoop
{
    // TODO idk is it supposed to be here
    public bool IsRenderFrame { get; private set; } = false;
    public bool IsPhysicsFrame { get; private set; } = false;
    public float RenderDelta { get; private set; } = 0.0f;
    public float PhysicsDelta { get; private set; } = 0.0f;

    public SceneTree()
    {

    }
}