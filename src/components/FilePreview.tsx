import { useState, useEffect } from "react";
import { GitService } from "../services/git.ts";

interface FilePreviewProps {
  file: string;
  repoPath: string;
  maxLines: number;
  staged?: boolean;
}

export default function FilePreview({
  file,
  repoPath,
  maxLines,
  staged = false,
}: FilePreviewProps) {
  const [diff, setDiff] = useState<string>("");

  useEffect(() => {
    const gitService = new GitService(repoPath);
    gitService.getFileDiff(file, staged).then(setDiff);
  }, [file, repoPath, staged]);

  const lines = diff.split("\n").slice(0, maxLines);

  return (
    <box
      flexDirection="column"
      border
      borderStyle="single"
      height={maxLines + 2}
      padding={{ left: 1, right: 1 }}
    >
      <text style={{ bold: true }}>PREVIEW: {file}</text>
      {lines.map((line, i) => (
        <text key={i} style={{ fg: getDiffLineColor(line) }}>
          {line}
        </text>
      ))}
    </box>
  );
}

function getDiffLineColor(line: string): string {
  if (line.startsWith("+")) return "#00FF00"; // Green
  if (line.startsWith("-")) return "#FF0000"; // Red
  if (line.startsWith("@@")) return "#00AAFF"; // Blue
  return "#FFFFFF"; // White
}
