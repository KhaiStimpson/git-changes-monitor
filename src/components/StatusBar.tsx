import type { KeybindingsConfig } from "../types/config.ts";
import type { Theme } from "../theme/useTheme.ts";

interface StatusBarProps {
  keybindings: KeybindingsConfig;
  isWatching?: boolean;
  theme: Theme;
}

export default function StatusBar({
  keybindings,
  isWatching = true,
  theme,
}: StatusBarProps) {
  const shortcuts = [
    { key: "↑↓", action: "Navigate" },
    { key: keybindings.togglePreview, action: "Preview" },
    { key: keybindings.refresh, action: "Refresh" },
    { key: keybindings.help, action: "Help" },
    { key: keybindings.quit, action: "Quit" },
  ];

  return (
    <box
      backgroundColor={theme.surface}
      paddingLeft={2}
      paddingRight={2}
      paddingTop={1}
      paddingBottom={1}
      marginTop={1}
    >
      <box flexDirection="row" justifyContent="space-between">
        {/* Keyboard shortcuts */}
        <box flexDirection="row" gap={0}>
          {shortcuts.map((shortcut, index) => (
            <text key={shortcut.key}>
              <text fg={theme.accent} bold={true}>{shortcut.key}</text>
              <text fg={theme.text}> {shortcut.action}</text>
              {index < shortcuts.length - 1 && (
                <text fg={theme.subtext}> │ </text>
              )}
            </text>
          ))}
        </box>

        {/* Watch status indicator */}
        <box>
          {isWatching ? (
            <text>
              <text fg={theme.success}>●</text>
              <text fg={theme.text}> Watching</text>
            </text>
          ) : (
            <text>
              <text fg={theme.subtext}>○</text>
              <text fg={theme.subtext}> Paused</text>
            </text>
          )}
        </box>
      </box>
    </box>
  );
}
