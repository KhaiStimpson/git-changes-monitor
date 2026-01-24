import { useState, useEffect } from "react";
import { GitService } from "../services/git.ts";
import type { Theme } from "../theme/useTheme.ts";

interface FilePreviewProps {
  file: string;
  repoPath: string;
  maxLines: number;
  staged?: boolean;
  theme: Theme;
}

export default function FilePreview({
  file,
  repoPath,
  maxLines,
  staged = false,
  theme,
}: FilePreviewProps) {
  const [diff, setDiff] = useState<string>("");

  useEffect(() => {
    const gitService = new GitService(repoPath);
    gitService.getFileDiff(file, staged).then(setDiff);
  }, [file, repoPath, staged]);

  const lines = diff.split("\n").slice(0, maxLines);

  const getDiffLineColor = (line: string): string => {
    if (line.startsWith("+++") || line.startsWith("---")) {
      return theme.subtext;
    }
    if (line.startsWith("+")) {
      return theme.success;
    }
    if (line.startsWith("-")) {
      return theme.error;
    }
    if (line.startsWith("@@")) {
      return theme.info;
    }
    if (line.startsWith("diff") || line.startsWith("index")) {
      return theme.subtext;
    }
    return theme.text;
  };

  const getDiffLinePrefix = (line: string): { prefix: string; color: string } | null => {
    if (line.startsWith("+") && !line.startsWith("+++")) {
      return { prefix: "+", color: theme.success };
    }
    if (line.startsWith("-") && !line.startsWith("---")) {
      return { prefix: "-", color: theme.error };
    }
    return null;
  };

  return (
    <box
      flexDirection="column"
      backgroundColor={theme.surface}
      height={maxLines + 3}
      paddingLeft={2}
      paddingRight={2}
      paddingTop={1}
      marginTop={1}
    >
      {/* Header */}
      <box paddingBottom={1}>
        <text>
          <text fg={theme.accent} bold={true}>PREVIEW</text>
          <text fg={theme.subtext}> │ </text>
          <text fg={theme.text}>{file}</text>
        </text>
      </box>

      {/* Diff content */}
      <box flexDirection="column">
        {lines.map((line, i) => {
          const prefixInfo = getDiffLinePrefix(line);
          const lineColor = getDiffLineColor(line);

          return (
            <text key={i}>
              {/* Line number gutter */}
              <text fg={theme.subtext}>
                {String(i + 1).padStart(3, " ")}
              </text>
              <text fg={theme.subtext}> │ </text>

              {/* Diff line content */}
              {prefixInfo ? (
                <>
                  <text fg={prefixInfo.color} bold={true}>
                    {prefixInfo.prefix}
                  </text>
                  <text fg={lineColor}>{line.slice(1)}</text>
                </>
              ) : (
                <text fg={lineColor}>{line}</text>
              )}
            </text>
          );
        })}

        {/* Show indicator if truncated */}
        {diff.split("\n").length > maxLines && (
          <text fg={theme.subtext}>
            {"    "}│ <text fg={theme.accent}>... {diff.split("\n").length - maxLines} more lines</text>
          </text>
        )}
      </box>
    </box>
  );
}
