import type { BranchInfo, CommitInfo } from "../types/git.ts";

interface BranchInfoProps {
  branch: BranchInfo;
  lastCommit?: CommitInfo;
  showLastCommit?: boolean;
}

export default function BranchInfoComponent({
  branch,
  lastCommit,
  showLastCommit = true,
}: BranchInfoProps) {
  const upstreamInfo = branch.upstream
    ? ` → ${branch.upstream}`
    : "";

  const aheadBehind = [];
  if (branch.ahead > 0) aheadBehind.push(`↑${branch.ahead}`);
  if (branch.behind > 0) aheadBehind.push(`↓${branch.behind}`);
  const trackingInfo = aheadBehind.length > 0 ? ` ${aheadBehind.join(" ")}` : "";

  const commitInfo = showLastCommit && lastCommit
    ? ` | ${lastCommit.subject} (${lastCommit.hash})`
    : "";

  return (
    <box
      border={true}
      borderStyle="single"
      paddingLeft={1}
      paddingRight={1}
      height={3}
    >
      <text>
        <text bold={true}>Branch:</text> {branch.name}
        {upstreamInfo}
        {trackingInfo}
        {commitInfo}
      </text>
    </box>
  );
}
