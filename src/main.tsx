import { Command } from "@cliffy/command";
import { createCliRenderer } from "@opentui/core";
import { createRoot } from "@opentui/react";
import App from "./app.tsx";
import { GitService } from "./services/git.ts";
import { ConfigService } from "./services/config.ts";

async function main() {
  const { options, args } = await new Command()
    .name("gfm")
    .version("0.1.0")
    .description(
      "Git Changes Monitor - Real-time TUI for Git repository changes",
    )
    .arguments("[directory:string]")
    .option("-c, --config <path:string>", "Custom config file path")
    .option("--no-watch", "Single snapshot mode (no live updates)")
    .parse(Deno.args);

  // Determine repository path
  const repoPath = args[0] || Deno.cwd();

  // Validate it's a Git repository
  const gitService = new GitService(repoPath);
  const isGitRepo = await gitService.isGitRepo();

  if (!isGitRepo) {
    console.error(`Error: ${repoPath} is not a Git repository`);
    console.error(
      "\nPlease run this command from within a Git repository or provide a path to one.",
    );
    Deno.exit(1);
  }

  // Determine config path
  const configPath = options.config || ConfigService.getDefaultConfigPath();

  // Create renderer and render app
  const renderer = await createCliRenderer();

  createRoot(renderer).render(
    <App
      repoPath={repoPath}
      configPath={configPath}
      watchMode={options.watch !== false}
    />,
  );
}

// Run the application
if (import.meta.main) {
  main().catch((error) => {
    console.error("Fatal error:", error);
    Deno.exit(1);
  });
}
