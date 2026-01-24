import type { KeybindingsConfig } from "../types/config.ts";
import type { Theme } from "../theme/useTheme.ts";

interface HelpMenuProps {
  keybindings: KeybindingsConfig;
  onClose: () => void;
  theme: Theme;
}

export default function HelpMenu({ keybindings, theme }: HelpMenuProps) {
  const sections = [
    {
      title: "Navigation",
      items: [
        { key: "↑/↓ or j/k", description: "Navigate file list" },
        { key: "PgUp/PgDn", description: "Page through files" },
      ],
    },
    {
      title: "Actions",
      items: [
        { key: keybindings.togglePreview, description: "Toggle file preview" },
        { key: keybindings.refresh, description: "Manual refresh" },
        { key: "Enter", description: "Select file" },
      ],
    },
    {
      title: "Other",
      items: [
        { key: keybindings.help, description: "Toggle this help menu" },
        { key: `${keybindings.quit} or Esc`, description: "Quit application" },
      ],
    },
  ];

  return (
    <box
      position="absolute"
      left={10}
      top={3}
      width={50}
      border={true}
      borderStyle="round"
      borderColor={theme.border}
      backgroundColor={theme.overlay}
      padding={2}
    >
      <box flexDirection="column" gap={1}>
        {/* Header */}
        <box paddingBottom={1}>
          <text fg={theme.accent} bold={true}>
            ⌨ Keyboard Shortcuts
          </text>
        </box>

        {/* Sections */}
        {sections.map((section) => (
          <box key={section.title} flexDirection="column">
            {/* Section title */}
            <text fg={theme.accentSecondary} bold={true}>
              {section.title}
            </text>

            {/* Section items */}
            {section.items.map((item) => (
              <box key={item.key} paddingLeft={2}>
                <text>
                  <text fg={theme.info}>{item.key.padEnd(14)}</text>
                  <text fg={theme.text}>{item.description}</text>
                </text>
              </box>
            ))}
          </box>
        ))}

        {/* Footer */}
        <box paddingTop={1}>
          <text fg={theme.subtext}>
            Press <text fg={theme.accent}>{keybindings.help}</text> or{" "}
            <text fg={theme.accent}>Esc</text> to close
          </text>
        </box>
      </box>
    </box>
  );
}
