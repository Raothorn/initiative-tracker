namespace Server;

using SocketIOSharp.Server.Client;

public interface IClient 
{
    void Login(string username, ClientType clientType);
    bool IsLoggedIn { get; }
    string Username { get; }
}

public class Client : IClient
{
    private SocketIOSocket? _socket;
    private string? _username = null;
    private ClientType? _clientType = null;

    public Client(SocketIOSocket? socket) 
    {
        _socket = socket;
    }

    public void Login(string username, ClientType clientType) 
    {
        _username = username;
        _clientType = clientType;
    }

    public bool IsLoggedIn 
    {
        get { return _clientType is not null && _username is not null; }
    }

    public string Username 
    {
        get { return _username ?? "Anonymous"; }
    }
}

/* [JsonConverter(typeof(StringEnumConverter))] */
public enum ClientType 
{ 
    Player,
    DungeonMaster 
}

