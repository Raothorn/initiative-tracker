namespace Server;

public interface IServerState 
{
    List<IClient> Clients { get; }
    void AddClient(IClient client, string username);
    bool UsernameAvailable(string username);
    GameState GameState { get; }
}

public class ServerState : IServerState
{
    private List<IClient> _clients;     
    private GameState _gamestate;

    public ServerState() {
        _clients = new List<IClient>();
        _gamestate = new GameState();
    }

    public void AddClient(IClient client, string username) 
    {
        _clients.Add(client);
        client.Login(username, ClientType.Player);
    }

    public bool UsernameAvailable(string username)
    {
        return !_clients.Any(client => client.Username == username); 
    }


    public GameState GameState {
        get { return _gamestate; }
    }

    public List<IClient> Clients => _clients;
}
