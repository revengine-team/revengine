class Node
{
    protected virtual void Initialize() { }
    protected virtual void RenderProcess(float delta) { }
    protected virtual void PhysicsProcess(float delta) { }
    protected virtual void HandleInput(InputEvent inputEvent) { }
}