using Serilog;

namespace Server;

class Program
{
    static void Main(string[] args)
    {
        Log.Logger = new LoggerConfiguration()
            .WriteTo.Console()
            .MinimumLevel.Debug()
            .CreateLogger();

        Server server = new Server();
        server.Start();
        Console.ReadLine();
        server.Close();
    }
}
