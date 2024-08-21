namespace Server;

using Actions;
using Newtonsoft.Json.Linq;
using Serilog;
using SocketIOSharp.Common;
using SocketIOSharp.Server;
using SocketIOSharp.Server.Client;

public class Server
{
    private SocketIOServer _server;
    private ServerState _serverState;

    public Server()
    {
        _server = new SocketIOServer(new SocketIOServerOption(9001));
        _serverState = new ServerState();
    }

    public void Start()
    {
        Log.Debug("Listening on 9001");
        var actionBuilder = new ActionBuilder();

        _server.OnConnection(
            (SocketIOSocket socket) =>
            {
                Log.Debug("Client connected");
                Client client = new Client(socket);

                socket.On(
                    "message",
                    (JToken[] data) =>
                    {
                        if (data.Any())
                        {
                            var payload = (string?) data[0];

                            Log.Debug($"Recieved raw message: {payload}");

                            if (payload is not null) 
                            {
                                IAction action = actionBuilder.BuildAction(payload);
                                Console.WriteLine(action);
                                var result = action.Execute(_serverState, client);

                                if (result.Succeeded) 
                                {
                                    Log.Debug(result.Message);
                                }
                                else 
                                {
                                    Log.Error(result.Message);
                                }
                            }

                        }
                        else
                        {
                            Log.Error("No action received.");
                        }
                    }

                );

                socket.On(
                    SocketIOEvent.DISCONNECT, () =>
                    {
                        Log.Debug("Client disconnected!");
                    }
                );
            }
        );

        _server.Start();
    }

    public void Close() 
    {
        Log.Debug("Closing server");
        _server.Stop();
    }
}
