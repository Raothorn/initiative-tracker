namespace Server.Actions;

public interface IAction 
{
    ActionResult Execute(ServerState state, Client client);
}

public class LoginAction : IAction
{
    public string username { get; set; } = "Anonymous";

    public ActionResult Execute(ServerState state, Client client)
    {
        if (state.UsernameAvailable(username)) 
        {
            client.Login(username, ClientType.Player);
            return new ActionResult(true, $"Client successfully connected as {username}");
        }
        else 
        {
            return new ActionResult(false, $"Username {username} taken.");
        }
    }

}

public class EmptyAction : IAction
{
    public ActionResult Execute(ServerState state, Client client)
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

