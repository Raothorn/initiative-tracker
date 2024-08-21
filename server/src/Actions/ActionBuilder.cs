using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using Serilog;

namespace Server.Actions;

public class ActionBuilder 
{
    public IAction BuildAction(string rawMessage) 
    {
        ActionMessage? actionMessage = JsonConvert.DeserializeObject<ActionMessage>(rawMessage);
        
        if (actionMessage is not null) 
        {
            var payload  = actionMessage.payload;

            IAction action;

            switch (actionMessage.action_type) {
                case "login":
                    action = payload.ToObject<LoginAction>();
                    break;
            
                default:
                    action = new EmptyAction();
                    break;
            }

            return action ?? new EmptyAction();
        }
        else 
        {
            Log.Error("Unable to parse action");
            return new EmptyAction();
        }
    }

    class ActionMessage 
    {
        public string action_type { get; set; }
        public JObject payload { get; set; }
    }
}
