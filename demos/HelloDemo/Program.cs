using System.Diagnostics;

var root = Path.GetFullPath(Path.Combine(AppContext.BaseDirectory, "..", "..", "..", "..", ".."));
var psi = new ProcessStartInfo("cargo", "run")
{
    WorkingDirectory = root,
    UseShellExecute = false
};

Console.WriteLine("--- RUSTY NATIVE RUN ---");
Process.Start(psi)?.WaitForExit();
