import { Trash2 } from "lucide-react";
import { Handle, Position, type NodeProps, type NodeTypes } from "@xyflow/react";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import type { FlowNode } from "@/app/types";

const CONNECTED_INPUT_PLACEHOLDER = "Value comes from connected output";

function BlockNode({ id, data }: NodeProps<FlowNode>) {
  return (
    <div className="min-w-[280px] rounded-lg border border-slate-700 bg-slate-900 text-slate-100 shadow-xl">
      <div className="flex items-center justify-between border-b border-slate-700 px-3 py-2">
        <div className="text-sm font-semibold">{data.definition.title}</div>
        <Badge className="border-slate-600 bg-slate-800 text-slate-200">{data.definition.category}</Badge>
      </div>

      <div className="space-y-2 px-3 py-3">
        {data.definition.accepts_flow ? (
          <div className="relative rounded-md bg-slate-800/70 p-2">
            <div className="text-xs text-slate-300">Flow In</div>
            <Handle
              type="target"
              id="__flow"
              position={Position.Left}
              className="!h-3 !w-3 !border-2 !border-slate-950 !bg-emerald-400"
              style={{ left: -8, top: "50%", transform: "translateY(-50%)" }}
            />
          </div>
        ) : null}

        {data.definition.inputs.map((input) => {
          const connected = data.connectedInputs[input.key] ?? false;
          const displayedValue = connected ? "" : (data.values[input.key] ?? "");
          const placeholder = connected ? CONNECTED_INPUT_PLACEHOLDER : input.placeholder;
          return (
            <div key={`${id}-${input.key}`} className="relative space-y-1 rounded-md bg-slate-800/70 p-2">
              <Label htmlFor={`${id}-${input.key}`} className="text-xs text-slate-300">
                {input.label}
              </Label>
              <Input
                id={`${id}-${input.key}`}
                type={input.input_type === "number" ? "number" : "text"}
                value={displayedValue}
                disabled={connected}
                className="h-8 border-slate-600 bg-slate-900 text-xs text-slate-100"
                placeholder={placeholder}
                onChange={(event) => data.onValueChange(id, input.key, event.target.value)}
              />
              {input.connectable ? (
                <Handle
                  type="target"
                  id={input.key}
                  position={Position.Left}
                  className="!h-3 !w-3 !border-2 !border-slate-950 !bg-cyan-400"
                  style={{ left: -8, top: "50%", transform: "translateY(-50%)" }}
                />
              ) : null}
            </div>
          );
        })}
      </div>

      {data.definition.outputs.length > 0 ? (
        <div className="border-t border-slate-700 px-3 py-2">
          <div className="text-[11px] uppercase tracking-wide text-slate-400">Outputs</div>
          <div className="mt-1 space-y-1">
            {data.definition.outputs.map((output) => (
              <div key={`${id}-${output.key}`} className="relative rounded-md bg-slate-800/70 px-2 py-1 text-xs text-slate-200">
                {output.label}
                <Handle
                  type="source"
                  id={output.key}
                  position={Position.Right}
                  className={`!h-3 !w-3 !border-2 !border-slate-950 ${
                    output.expr_from_input ? "!bg-amber-400" : "!bg-emerald-400"
                  }`}
                  style={{ right: -8, top: "50%", transform: "translateY(-50%)" }}
                />
              </div>
            ))}
          </div>
        </div>
      ) : null}

      <div className="flex justify-end border-t border-slate-700 px-3 py-2">
        <Button
          size="sm"
          variant="ghost"
          onClick={() => data.onDelete(id)}
          className="h-7 px-2 text-slate-200 hover:bg-slate-800"
        >
          <Trash2 className="h-3.5 w-3.5" />
        </Button>
      </div>
    </div>
  );
}

export const nodeTypes = {
  blockNode: BlockNode
} satisfies NodeTypes;
