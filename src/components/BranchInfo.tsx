import type { BranchInfo, CommitInfo } from "../types/git.ts";
import type { Theme } from "../theme/useTheme.ts";

interface BranchInfoProps {
  branch: BranchInfo;
  lastCommit?: CommitInfo;
  showLastCommit?: boolean;
  theme: Theme;
}

export default function BranchInfoComponent({
  branch,
  lastCommit,
  showLastCommit = true,
  theme,
}: BranchInfoProps) {
  const upstreamDisplay = branch.upstream ? ` → ${branch.upstream}` : "";

  return (
    <box
      backgroundColor={theme.surface}
      paddingLeft={2}
      paddingRight={2}
      paddingTop={1}
      paddingBottom={1}
      marginBottom={1}
    >
      <box flexDirection="row" gap={1}>
        {/* Branch icon and name */}
        <text fg={theme.subtext}></text>
        <text fg={theme.accent} bold={true}>
          {branch.name}
        </text>

        {/* Upstream info */}
        {branch.upstream && (
          <text fg={theme.subtext}>{upstreamDisplay}</text>
        )}

        {/* Ahead/Behind indicators */}
        {branch.ahead > 0 && (
          <text fg={theme.success}>↑{branch.ahead}</text>
        )}
        {branch.behind > 0 && (
          <text fg={theme.error}>↓{branch.behind}</text>
        )}

        {/* Separator */}
        {showLastCommit && lastCommit && (
          <text fg={theme.subtext}>│</text>
        )}

        {/* Last commit info */}
        {showLastCommit && lastCommit && (
          <>
            <text fg={theme.warning}>{lastCommit.hash}</text>
            <text fg={theme.subtext}>{lastCommit.subject}</text>
          </>
        )}
      </box>
    </box>
  );
}
