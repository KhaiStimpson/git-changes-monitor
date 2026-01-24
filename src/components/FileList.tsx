import type { FileStatus } from "../types/git.ts";
import type { Config } from "../types/config.ts";
import type { Theme } from "../theme/useTheme.ts";

interface FileListProps {
  stagedFiles: FileStatus[];
  unstagedFiles: FileStatus[];
  selectedFile: string | null;
  onSelect: (file: string) => void;
  config: Config;
  theme: Theme;
}

export default function FileList({
  stagedFiles,
  unstagedFiles,
  selectedFile,
  config,
  theme,
}: FileListProps) {
  const getStatusColor = (status: string): string => {
    switch (status) {
      case "modified":
        return theme.warning;
      case "added":
        return theme.success;
      case "deleted":
        return theme.error;
      case "renamed":
        return theme.info;
      case "copied":
        return theme.info;
      case "untracked":
        return theme.untracked;
      case "unmerged":
        return theme.accentSecondary;
      default:
        return theme.text;
    }
  };

  const getStatusIcon = (status: string): string => {
    switch (status) {
      case "modified":
        return "M";
      case "added":
        return "A";
      case "deleted":
        return "D";
      case "renamed":
        return "R";
      case "copied":
        return "C";
      case "untracked":
        return "?";
      case "unmerged":
        return "U";
      default:
        return "?";
    }
  };

  const renderFile = (file: FileStatus, isSelected: boolean) => {
    const statusIcon = getStatusIcon(file.status);
    const statusColor = getStatusColor(file.status);

    const changeCount = config.display.showLineChangeCounts
      ? ` +${file.linesAdded} -${file.linesDeleted}`
      : "";

    return (
      <box
        key={file.path}
        backgroundColor={isSelected ? theme.selection : undefined}
        paddingLeft={1}
        paddingRight={1}
      >
        <text>
          {/* Selection indicator */}
          <text fg={isSelected ? theme.accent : theme.base}>
            {isSelected ? "▌" : " "}
          </text>
          {" "}
          {/* Staged indicator */}
          <text fg={file.staged ? theme.staged : theme.subtext}>
            {file.staged ? "●" : "○"}
          </text>
          {" "}
          {/* Status icon */}
          <text fg={statusColor} bold={true}>
            {statusIcon}
          </text>
          {" "}
          {/* File path */}
          <text fg={isSelected ? theme.selectionText : theme.text}>
            {file.path}
          </text>
          {/* Line changes */}
          {changeCount && (
            <text fg={theme.subtext}>{changeCount}</text>
          )}
        </text>
      </box>
    );
  };

  const renderSectionHeader = (title: string, count: number, isStaged: boolean) => (
    <box paddingLeft={1} paddingTop={1} paddingBottom={0}>
      <text>
        <text fg={isStaged ? theme.staged : theme.unstaged} bold={true}>
          {"█ "}
        </text>
        <text fg={theme.text} bold={true}>
          {title}
        </text>
        <text fg={theme.subtext}> ({count})</text>
      </text>
    </box>
  );

  return (
    <box flexDirection="column" flexGrow={1}>
      {/* Staged files section */}
      {config.display.showStagedVsUnstaged && stagedFiles.length > 0 && (
        <box flexDirection="column">
          {renderSectionHeader("STAGED CHANGES", stagedFiles.length, true)}
          <box flexDirection="column" paddingTop={0}>
            {stagedFiles.map((file) =>
              renderFile(file, file.path === selectedFile)
            )}
          </box>
        </box>
      )}

      {/* Unstaged files section */}
      {unstagedFiles.length > 0 && (
        <box flexDirection="column">
          {renderSectionHeader("UNSTAGED CHANGES", unstagedFiles.length, false)}
          <box flexDirection="column" paddingTop={0}>
            {unstagedFiles.map((file) =>
              renderFile(file, file.path === selectedFile)
            )}
          </box>
        </box>
      )}

      {/* Empty state */}
      {stagedFiles.length === 0 && unstagedFiles.length === 0 && (
        <box justifyContent="center" alignItems="center" flexGrow={1}>
          <box flexDirection="column" alignItems="center">
            <text fg={theme.subtext}>✓</text>
            <text fg={theme.subtext}>No changes detected</text>
            <text fg={theme.accent}>Working tree clean</text>
          </box>
        </box>
      )}
    </box>
  );
}
