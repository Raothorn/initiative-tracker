namespace Server;

public class ServerState  
{
    private List<Client> _clients;     

    public ServerState() {
        _clients = new List<Client>();
    }

    public void AddClient(Client client) 
    {

    }

    public bool UsernameAvailable(string username)
    {
        return !_clients.Any(client => client.Username == username); 
    }
}
