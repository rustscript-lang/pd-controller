import type { DragEvent } from "react";
import { ArrowLeft, Save, Trash2 } from "lucide-react";
import type { Connection, EdgeChange, NodeChange, ReactFlowInstance, Viewport } from "@xyflow/react";

import { ProgramComposerWorkspace } from "@/app/components/ProgramComposerWorkspace";
import type {
  FlowEdge,
  FlowNode,
  ProgramDetailResponse,
  SourceFlavor,
  UiBlockDefinition,
  UiSourceBundle
} from "@/app/types";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

type ProgramDetailViewProps = {
  selectedProgram: ProgramDetailResponse | null;
  programNameDraft: string;
  onProgramNameDraftChange: (value: string) => void;
  selectedVersion: number | null;
  onSelectVersion: (value: string) => void;
  renamingProgram: boolean;
  onRenameProgram: () => void;
  deletingProgram: boolean;
  onDeleteProgram: () => void;
  savingVersion: boolean;
  canSaveVersion: boolean;
  onSaveVersion: () => void;
  graphStatus: string;
  onBackToPrograms: () => void;
  isCodeEditMode: boolean;
  canExitCodeEditMode: boolean;
  onExitCodeEditMode: () => void;
  onEnterCodeEditMode: () => void;
  source: UiSourceBundle;
  activeFlavor: SourceFlavor;
  rendering: boolean;
  onFlavorChange: (value: SourceFlavor) => void;
  onSourceChange: (flavor: SourceFlavor, value: string) => void;
  selectedProgramId: string | null;
  graphCanvasRevision: number;
  nodes: FlowNode[];
  edges: FlowEdge[];
  onNodesChange: (changes: NodeChange<FlowNode>[]) => void;
  onEdgesChange: (changes: EdgeChange<FlowEdge>[]) => void;
  onConnect: (connection: Connection) => void;
  onInit: (instance: ReactFlowInstance<FlowNode, FlowEdge>) => void;
  onMoveEnd: (viewport: Viewport) => void;
  onCanvasDrop: (event: DragEvent<HTMLDivElement>) => void;
  selectedNodeCount: number;
  selectedEdgeCount: number;
  paletteMinimized: boolean;
  onTogglePaletteMinimized: () => void;
  codePanelMinimized: boolean;
  onToggleCodePanelMinimized: () => void;
  definitions: UiBlockDefinition[];
  search: string;
  onSearchChange: (value: string) => void;
  onPaletteDragStart: (event: DragEvent<HTMLDivElement>, blockId: string) => void;
  onAddNode: (blockId: string) => void;
};

export function ProgramDetailView({
  selectedProgram,
  programNameDraft,
  onProgramNameDraftChange,
  selectedVersion,
  onSelectVersion,
  renamingProgram,
  onRenameProgram,
  deletingProgram,
  onDeleteProgram,
  savingVersion,
  canSaveVersion,
  onSaveVersion,
  graphStatus,
  onBackToPrograms,
  isCodeEditMode,
  canExitCodeEditMode,
  onExitCodeEditMode,
  onEnterCodeEditMode,
  source,
  activeFlavor,
  rendering,
  onFlavorChange,
  onSourceChange,
  selectedProgramId,
  graphCanvasRevision,
  nodes,
  edges,
  onNodesChange,
  onEdgesChange,
  onConnect,
  onInit,
  onMoveEnd,
  onCanvasDrop,
  selectedNodeCount,
  selectedEdgeCount,
  paletteMinimized,
  onTogglePaletteMinimized,
  codePanelMinimized,
  onToggleCodePanelMinimized,
  definitions,
  search,
  onSearchChange,
  onPaletteDragStart,
  onAddNode
}: ProgramDetailViewProps) {
  return (
    <div className="space-y-4">
      <Card>
        <CardHeader>
          <div className="flex flex-wrap items-center justify-between gap-3">
            <div>
              <div className="text-xs uppercase tracking-[0.24em] text-slate-500">Workflow Registry</div>
              <div className="mt-1 text-2xl font-semibold tracking-tight">Program Detail</div>
              <div className="mt-1 text-sm text-muted-foreground">
                {selectedProgram ? `Edit ${selectedProgram.name}` : "Program composer"}
              </div>
              {selectedProgram ? (
                <div className="mt-1 font-mono text-xs text-muted-foreground">UUID: {selectedProgram.program_id}</div>
              ) : null}
            </div>
            <Button
              variant="outline"
              onClick={onBackToPrograms}
              className="inline-flex items-center gap-1"
            >
              <ArrowLeft className="h-4 w-4" />
              Back To Programs
            </Button>
          </div>
        </CardHeader>
        <CardContent className="space-y-4">
          {selectedProgram ? (
            <>
              <div className="grid grid-cols-1 gap-2 md:grid-cols-[1fr_180px_auto]">
                <div className="space-y-1">
                  <Label htmlFor="program-name">Program Name</Label>
                  <Input
                    id="program-name"
                    value={programNameDraft}
                    onChange={(event) => onProgramNameDraftChange(event.target.value)}
                  />
                </div>
                <div className="space-y-1">
                  <Label htmlFor="version-select">Version</Label>
                  <select
                    id="version-select"
                    value={selectedVersion !== null ? String(selectedVersion) : ""}
                    onChange={(event) => onSelectVersion(event.target.value)}
                    className="h-9 w-full rounded-md border bg-background px-2 text-sm"
                  >
                    {selectedProgram.versions.length === 0 ? <option value="0">v0 (draft)</option> : null}
                    {selectedProgram.versions
                      .slice()
                      .sort((a, b) => b.version - a.version)
                      .map((version) => (
                        <option key={version.version} value={String(version.version)}>
                          v{version.version} ({version.flavor}{version.flow_synced ? "" : ", code"})
                        </option>
                      ))}
                  </select>
                </div>
                <div className="flex items-end gap-2">
                  <Button variant="secondary" onClick={onRenameProgram} disabled={renamingProgram}>
                    {renamingProgram ? "Renaming" : "Rename"}
                  </Button>
                  <Button
                    variant="outline"
                    className="border-rose-300 text-rose-700 hover:bg-rose-50"
                    onClick={onDeleteProgram}
                    disabled={deletingProgram}
                  >
                    <Trash2 className="mr-1 h-4 w-4" />
                    {deletingProgram ? "Deleting" : "Delete"}
                  </Button>
                  <Button onClick={onSaveVersion} disabled={savingVersion || !canSaveVersion}>
                    <Save className="mr-1 h-4 w-4" />
                    {savingVersion ? "Saving" : "Save Version"}
                  </Button>
                </div>
              </div>
              {graphStatus ? <div className="text-xs text-muted-foreground">{graphStatus}</div> : null}
            </>
          ) : (
            <div className="text-sm text-muted-foreground">Select a program from the Programs table first.</div>
          )}
        </CardContent>
      </Card>

      <div className="space-y-4">
        <ProgramComposerWorkspace
          isCodeEditMode={isCodeEditMode}
          onExitCodeEditMode={onExitCodeEditMode}
          showExitCodeEditButton={canExitCodeEditMode}
          onEnterCodeEditMode={onEnterCodeEditMode}
          source={source}
          activeFlavor={activeFlavor}
          rendering={rendering}
          onFlavorChange={onFlavorChange}
          onSourceChange={onSourceChange}
          selectedProgramId={selectedProgramId}
          selectedVersion={selectedVersion}
          graphCanvasRevision={graphCanvasRevision}
          nodes={nodes}
          edges={edges}
          onNodesChange={onNodesChange}
          onEdgesChange={onEdgesChange}
          onConnect={onConnect}
          onInit={onInit}
          onMoveEnd={onMoveEnd}
          onCanvasDrop={onCanvasDrop}
          selectedNodeCount={selectedNodeCount}
          selectedEdgeCount={selectedEdgeCount}
          paletteMinimized={paletteMinimized}
          onTogglePaletteMinimized={onTogglePaletteMinimized}
          codePanelMinimized={codePanelMinimized}
          onToggleCodePanelMinimized={onToggleCodePanelMinimized}
          definitions={definitions}
          search={search}
          onSearchChange={onSearchChange}
          onPaletteDragStart={onPaletteDragStart}
          onAddNode={onAddNode}
        />
      </div>
    </div>
  );
}
