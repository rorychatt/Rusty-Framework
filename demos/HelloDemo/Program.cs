using System.Diagnostics;

var root = Path.GetFullPath(Path.Combine(AppContext.BaseDirectory, "..", "..", "..", "..", ".."));
var psi = new ProcessStartInfo("cargo", "run") {
    WorkingDirectory = root,
    UseShellExecute = false
};

Process.Start(psi)?.WaitForExit();
