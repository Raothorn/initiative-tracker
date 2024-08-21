using Server;
using Server.Actions;

namespace server_test;

public class LoginTests
{

    [SetUp]
    public void Setup()
    {
    }

    [Test]
    public void TestLoginSucceeds()
    {
        var state = new ServerState();
        var username = "TestUser";

        var client = new Client(null);
        var loginAction = new LoginAction(username);

        var result = loginAction.Execute(state, client);

        Assert.That(result.Succeeded, Is.True);
        Assert.That(client.IsLoggedIn, Is.True);
    }

    [Test]
    public void TestLoginFailsIfUsernameExists()
    {
        var state = new ServerState();
        var username = "TestUser";

        var client1 = new Client(null);
        var client2 = new Client(null);

        var loginAction = new LoginAction(username);

        // Client 1 logs in
        var result1 = loginAction.Execute(state, client1);

        Assert.That(result1.Succeeded, Is.True);
        Assert.That(client1.IsLoggedIn, Is.True);

        state.Clients.Select(c => c.Username).ToList().ForEach(Console.WriteLine);

        // Client 2 attempts to log in with the same username
        var result2 = loginAction.Execute(state, client2);
        Assert.That(result2.Succeeded, Is.False);
        Assert.That(client2.IsLoggedIn, Is.False);
    }
}
