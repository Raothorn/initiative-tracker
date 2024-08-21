namespace Server.Actions;

public interface IAction 
{
    ActionResult Execute(IServerState state, IClient client);
}

public class LoginAction : IAction
{
    public string username { get; set; }

    public LoginAction(string username = "Anonymous") 
    {
        this.username = username;
    }

    public ActionResult Execute(IServerState state, IClient client)
    {
        if (state.UsernameAvailable(username)) 
        {
            state.AddClient(client, username);
            return new ActionResult(true, $"Client successfully connected as {username}");
        }
        else 
        {
            return new ActionResult(false, $"Username {username} taken.");
        }
    }

}

public class AddCombatant : IAction
{
    public Combatant combatant { get; set; }

    // Constructor won't be used, since this is deserialized
    public AddCombatant(Combatant combatant)
    {
        this.combatant = combatant;
    }

    public ActionResult Execute(IServerState state, IClient client)
    {
        if (!state.GameState.encounter_started) 
        {
            state.GameState.AddCombatant(combatant);
            return new ActionResult(true); 
        }
        else 
        {
            return new ActionResult(false, "Cannot add combatants after encoutner already started.");
        }
    }
}

public class EmptyAction : IAction
{
    public ActionResult Execute(IServerState state, IClient client)
    {
        return new ActionResult(true, "Empty action.");
    }
}

public class ActionResult 
{
    public bool Succeeded { get; private set; }
    public string Message { get; private set; }

    public ActionResult(bool succeded, string message = "") {
        Succeeded = succeded;
        Message = message;
    }
}

