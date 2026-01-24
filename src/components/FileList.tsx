import type { FileStatus } from "../types/git.ts";
import type { Config } from "../types/config.ts";

interface FileListProps {
  stagedFiles: FileStatus[];
  unstagedFiles: FileStatus[];
  selectedFile: string | null;
  onSelect: (file: string) => void;
  config: Config;
}

export default function FileList({
  stagedFiles,
  unstagedFiles,
  selectedFile,
  onSelect,
  config,
}: FileListProps) {
  const renderFile = (file: FileStatus, isSelected: boolean) => {
    const statusIcon = getStatusIcon(file.status);
    const stagedMark = file.staged ? "✓ " : "  ";
    const selectedMark = isSelected ? "> " : "  ";

    const changeCount = config.display.showLineChangeCounts
      ? ` +${file.linesAdded} -${file.linesDeleted}`
      : "";

    return (
      <text key={file.path}>
        {selectedMark}
        {stagedMark}
        <span style={{ fg: getStatusColor(file.status) }}>
          {statusIcon}
        </span>{" "}
        {file.path}
        <span style={{ fg: "#888888" }}>{changeCount}</span>
      </text>
    );
  };

  return (
    <box flexDirection="column" flexGrow={1}>
      {config.display.showStagedVsUnstaged && stagedFiles.length > 0 && (
        <box flexDirection="column">
          <text style={{ bold: true }}>
            STAGED CHANGES ({stagedFiles.length})
          </text>
          {stagedFiles.map((file) =>
            renderFile(file, file.path === selectedFile)
          )}
          <text> </text>
        </box>
      )}

      {unstagedFiles.length > 0 && (
        <box flexDirection="column">
          <text style={{ bold: true }}>
            UNSTAGED CHANGES ({unstagedFiles.length})
          </text>
          {unstagedFiles.map((file) =>
            renderFile(file, file.path === selectedFile)
          )}
        </box>
      )}

      {stagedFiles.length === 0 && unstagedFiles.length === 0 && (
        <box justifyContent="center" alignItems="center" flexGrow={1}>
          <text style={{ fg: "#888888" }}>No changes detected</text>
        </box>
      )}
    </box>
  );
}

function getStatusIcon(status: string): string {
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
}

function getStatusColor(status: string): string {
  switch (status) {
    case "modified":
      return "#FFAA00"; // Orange
    case "added":
      return "#00FF00"; // Green
    case "deleted":
      return "#FF0000"; // Red
    case "renamed":
      return "#00AAFF"; // Blue
    case "copied":
      return "#00AAFF"; // Blue
    case "untracked":
      return "#888888"; // Gray
    case "unmerged":
      return "#FF00FF"; // Magenta
    default:
      return "#FFFFFF"; // White
  }
}
