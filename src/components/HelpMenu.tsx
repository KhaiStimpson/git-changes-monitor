import type { KeybindingsConfig } from "../types/config.ts";

interface HelpMenuProps {
  keybindings: KeybindingsConfig;
  onClose: () => void;
}

export default function HelpMenu({ keybindings, onClose }: HelpMenuProps) {
  return (
    <box
      position="absolute"
      left={10}
      top={5}
      width={60}
      height={20}
      border={true}
      borderStyle="double"
      backgroundColor="#000000"
      padding={2}
    >
      <box flexDirection="column">
        <text bold={true}>Keyboard Shortcuts</text>
        <text> </text>
        <text>Navigation:</text>
        <text>  ↑/↓ or j/k - Navigate file list</text>
        <text>  PgUp/PgDn - Scroll preview</text>
        <text> </text>
        <text>Actions:</text>
        <text>  {keybindings.togglePreview} - Toggle file preview</text>
        <text>  {keybindings.refresh} - Manual refresh</text>
        <text>  Enter - Select file</text>
        <text> </text>
        <text>Other:</text>
        <text>  {keybindings.help} - Toggle this help menu</text>
        <text>  {keybindings.quit} or Esc - Quit application</text>
        <text> </text>
        <text fg="#888888">
          Press {keybindings.help} or Esc to close
        </text>
      </box>
    </box>
  );
}
