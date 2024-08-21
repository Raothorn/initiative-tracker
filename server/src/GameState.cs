namespace Server;

public class GameState
{
    public List<Combatant> encounter { get; set; }
    public bool encounter_started { get; set; }

    public GameState()
    {
        encounter = new List<Combatant>();
        encounter_started = false;
    }

    public void AddCombatant(Combatant combatant) 
    {
        encounter.Add(combatant);
    }
}

public class Combatant
{
    public string name { get; set; } = string.Empty;
    public int max_hitpoints { get; set; }
    public int current_hitpoints { get; set; }
    public int? initiative { get; set; } = null;
}
