using System.Numerics;

class StaticBody : PhysicsBody
{
    public Vector3 ConstantLinearVelocity { set; get; } = Vector3.Zero;
    public StaticBody() { }
}