class Timer : Node
{
    public bool Paused { get; private set; } = true;
    public float TimeLeft { get; private set; } = 0.0f;

    public delegate void Timeout();

    // What to do with that null?
    // This is the Unity-style events.
    public static event Timeout? OnTimeout;


    public Timer() { }

    public void Start()
    {
        Paused = false;
    }

    /// <summary>
    /// Start timer with time in seconds
    /// </summary>
    public void Start(float time)
    {
        Paused = false;
        TimeLeft = time;
    }

    public void Stop()
    {
        Paused = true;
    }

    protected override void RenderProcess(float delta)
    {
        if (Paused)
        {
            TimeLeft -= delta;

            if (TimeLeft <= 0.0f && OnTimeout != null)
            {
                Paused = true;
                OnTimeout();
            }
        }
    }
}