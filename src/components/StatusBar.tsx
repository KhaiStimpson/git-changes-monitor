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
      border
      borderStyle="single"
      height={3}
      padding={{ left: 1, right: 1 }}
    >
      <text>
        {shortcuts.join(" | ")}
        <span style={{ fg: "#888888" }}> | {watchStatus}</span>
      </text>
    </box>
  );
}
