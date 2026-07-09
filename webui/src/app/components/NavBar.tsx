import { Bug, FileCode2, Server, WandSparkles } from "lucide-react";

import { NavButton } from "@/app/components/NavButton";
import type { Section } from "@/app/types";

type NavBarProps = {
  section: Section;
  onSelectEdges: () => void;
  onSelectPrograms: () => void;
  onSelectDebugSessions: () => void;
};

export function NavBar({
  section,
  onSelectEdges,
  onSelectPrograms,
  onSelectDebugSessions
}: NavBarProps) {
  return (
    <aside className="w-[250px] border-r bg-card/80 p-4">
      <div className="mb-4 flex items-center gap-2">
        <WandSparkles className="h-5 w-5 text-primary" />
        <div className="text-sm font-semibold">pd-controller</div>
      </div>
      <div className="space-y-1">
        <NavButton active={section === "edges"} icon={<Server className="h-4 w-4" />} label="Edges" onClick={onSelectEdges} />
        <NavButton
          active={section === "programs"}
          icon={<FileCode2 className="h-4 w-4" />}
          label="Programs"
          onClick={onSelectPrograms}
        />
        <NavButton
          active={section === "debug_sessions"}
          icon={<Bug className="h-4 w-4" />}
          label="Debug Sessions"
          onClick={onSelectDebugSessions}
        />
      </div>
    </aside>
  );
}
