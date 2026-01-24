import { useState, useEffect } from "react";
import { useKeyboard } from "@opentui/react";
import BranchInfo from "./components/BranchInfo.tsx";
import FileList from "./components/FileList.tsx";
import FilePreview from "./components/FilePreview.tsx";
import StatusBar from "./components/StatusBar.tsx";
import HelpMenu from "./components/HelpMenu.tsx";
import { useGitStatus } from "./hooks/useGitStatus.ts";
import { useFileWatcher } from "./hooks/useFileWatcher.ts";
import { useConfig } from "./hooks/useConfig.ts";
import { useTheme } from "./theme/useTheme.ts";

interface AppProps {
  repoPath: string;
  configPath: string;
  watchMode: boolean;
}

export default function App({ repoPath, configPath, watchMode }: AppProps) {
  const config = useConfig(configPath);
  const theme = useTheme(config.ui.colorScheme);
  const { status, refresh, error } = useGitStatus(repoPath);
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const [showHelp, setShowHelp] = useState(false);
  const [showPreview, setShowPreview] = useState(
    config.display.showFilePreview,
  );
  const [selectedIndex, setSelectedIndex] = useState(0);

  // Watch for file changes if in watch mode
  useFileWatcher(repoPath, refresh, watchMode, config.ui.refreshDebounceMs);

  // Get all files for navigation
  const allFiles = [...status.stagedFiles, ...status.unstagedFiles];

  // Update selected file when index changes
  useEffect(() => {
    if (allFiles.length > 0 && selectedIndex < allFiles.length) {
      setSelectedFile(allFiles[selectedIndex].path);
    } else {
      setSelectedFile(null);
    }
  }, [selectedIndex, allFiles.length]);

  // Keyboard handling
  useKeyboard((key) => {
    // Close help menu first if open
    if (showHelp && (key.name === config.keybindings.help || key.name === "escape")) {
      setShowHelp(false);
      return;
    }

    if (key.name === config.keybindings.quit || key.name === "escape") {
      Deno.exit(0);
    }

    if (key.name === config.keybindings.refresh) {
      refresh();
    }

    if (key.name === config.keybindings.togglePreview) {
      setShowPreview(!showPreview);
    }

    if (key.name === config.keybindings.help) {
      setShowHelp(!showHelp);
    }

    // Navigation
    if (key.name === "down" || key.name === "j") {
      setSelectedIndex((prev) => Math.min(prev + 1, allFiles.length - 1));
    }

    if (key.name === "up" || key.name === "k") {
      setSelectedIndex((prev) => Math.max(prev - 1, 0));
    }

    if (key.name === "pagedown") {
      setSelectedIndex((prev) => Math.min(prev + 10, allFiles.length - 1));
    }

    if (key.name === "pageup") {
      setSelectedIndex((prev) => Math.max(prev - 10, 0));
    }
  });

  // Show error if repository check failed
  if (error) {
    return (
      <box
        flexDirection="column"
        justifyContent="center"
        alignItems="center"
        width="100%"
        height="100%"
        backgroundColor={theme.base}
      >
        <text fg={theme.error} bold={true}>Error</text>
        <text fg={theme.error}>{error}</text>
        <text> </text>
        <text fg={theme.subtext}>Press {config.keybindings.quit} or Esc to quit</text>
      </box>
    );
  }

  return (
    <box flexDirection="column" width="100%" height="100%" backgroundColor={theme.base}>
      {config.display.showBranchInfo && (
        <BranchInfo
          branch={status.branch}
          lastCommit={status.lastCommit}
          showLastCommit={config.display.showLastCommitInfo}
          theme={theme}
        />
      )}

      <FileList
        stagedFiles={status.stagedFiles}
        unstagedFiles={status.unstagedFiles}
        selectedFile={selectedFile}
        onSelect={setSelectedFile}
        config={config}
        theme={theme}
      />

      {showPreview && selectedFile && (
        <FilePreview
          file={selectedFile}
          repoPath={repoPath}
          maxLines={config.ui.maxPreviewLines}
          theme={theme}
        />
      )}

      <StatusBar
        keybindings={config.keybindings}
        isWatching={watchMode}
        theme={theme}
      />

      {showHelp && (
        <HelpMenu
          keybindings={config.keybindings}
          onClose={() => setShowHelp(false)}
          theme={theme}
        />
      )}
    </box>
  );
}
