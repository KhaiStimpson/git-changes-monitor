import type { KeybindingsConfig } from "../types/config.ts";

interface StatusBarProps {
  keybindings: KeybindingsConfig;
  isWatching?: boolean;
}

export default function StatusBar({
  keybindings,
  isWatching = true,
}: StatusBarProps) {
  const shortcuts = [
    `↑↓: Navigate`,
    `${keybindings.togglePreview}: Preview`,
    `${keybindings.refresh}: Refresh`,
    `${keybindings.help}: Help`,
    `${keybindings.quit}: Quit`,
  ];

  const watchStatus = isWatching ? "● Watching" : "○ Paused";

  return (
    <box
      border={true}
      borderStyle="single"
      height={3}
      paddingLeft={1}
      paddingRight={1}
    >
      <text>
        {shortcuts.join(" | ")}
        <text fg="#888888"> | {watchStatus}</text>
      </text>
    </box>
  );
}
