import { useMemo, useState, type DragEvent } from "react";
import { Blocks, ChevronDown, ChevronRight, Maximize2, Minimize2, Plus } from "lucide-react";

import type { UiBlockDefinition } from "@/app/types";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

type ProgramPaletteProps = {
  floating?: boolean;
  minimized?: boolean;
  definitions: UiBlockDefinition[];
  search: string;
  onSearchChange: (value: string) => void;
  onPaletteDragStart: (event: DragEvent<HTMLDivElement>, blockId: string) => void;
  onAddNode: (blockId: string) => void;
  onToggleMinimized?: () => void;
};

export function ProgramPalette({
  floating = false,
  minimized = false,
  definitions,
  search,
  onSearchChange,
  onPaletteDragStart,
  onAddNode,
  onToggleMinimized
}: ProgramPaletteProps) {
  const minimizedFloating = floating && minimized;
  const [collapsed, setCollapsed] = useState<Record<string, boolean>>({});
  const hasSearch = search.trim().length > 0;
  const groupedDefinitions = useMemo(() => {
    const grouped = new Map<string, UiBlockDefinition[]>();
    for (const definition of definitions) {
      const existing = grouped.get(definition.category);
      if (existing) {
        existing.push(definition);
      } else {
        grouped.set(definition.category, [definition]);
      }
    }
    return Array.from(grouped.entries()).map(([category, items]) => ({
      category,
      items
    }));
  }, [definitions]);

  if (minimizedFloating) {
    return (
      <Card className="pointer-events-auto h-10 w-10 border-slate-700 bg-white/95 text-slate-900 shadow-lg backdrop-blur">
        <div className="flex h-full items-center justify-center">
          <Button
            size="sm"
            variant="ghost"
            className="h-8 w-8 px-0"
            onClick={onToggleMinimized}
            aria-label="Expand palette"
          >
            <Blocks className="h-4 w-4" />
          </Button>
        </div>
      </Card>
    );
  }

  return (
    <Card
      className={
        floating
          ? "pointer-events-auto flex h-full min-h-0 w-[320px] flex-col overflow-hidden border-slate-700 bg-white/95 text-slate-900 backdrop-blur transition-[height,transform,box-shadow] duration-300 ease-out"
          : "h-fit"
      }
    >
      <CardHeader className="pb-3">
        <div className="flex items-start justify-between gap-3">
          <div>
            <CardTitle>Palette</CardTitle>
            <CardDescription>Drag blocks onto the canvas</CardDescription>
          </div>
          {floating ? (
            <Button
              size="sm"
              variant="ghost"
              className="h-7 w-7 px-0"
              onClick={onToggleMinimized}
              aria-label={minimized ? "Expand palette" : "Minimize palette"}
            >
              {minimized ? (
                <Maximize2 className="h-3.5 w-3.5 transition-transform duration-300 ease-out" />
              ) : (
                <Minimize2 className="h-3.5 w-3.5 transition-transform duration-300 ease-out" />
              )}
            </Button>
          ) : null}
        </div>
      </CardHeader>
      <div
        className={
          floating
            ? "grid min-h-0 flex-1 grid-rows-[1fr] transition-[grid-template-rows,opacity] duration-300 ease-out"
            : "grid grid-rows-[1fr]"
        }
      >
        <div className="h-full min-h-0 overflow-hidden">
          <CardContent className={floating ? "h-full space-y-3 overflow-auto" : "space-y-3"}>
            <div className="sticky top-0 z-10 -mx-4 mb-1 space-y-1 border-b bg-white/95 px-4 pb-3 pt-2 backdrop-blur">
              <Label htmlFor={floating ? "block-search" : "block-search-mobile"}>Search blocks</Label>
              <Input
                id={floating ? "block-search" : "block-search-mobile"}
                value={search}
                onChange={(event) => onSearchChange(event.target.value)}
                placeholder="if, header, rate, set..."
              />
            </div>
            {groupedDefinitions.map(({ category, items }) => {
              const isCollapsed = hasSearch ? false : collapsed[category] ?? false;
              return (
                <div key={`category-${category}`} className="space-y-2">
                  <Button
                    type="button"
                    variant="ghost"
                    className="flex w-full items-center justify-between gap-2 px-2"
                    onClick={() =>
                      setCollapsed((current) => ({
                        ...current,
                        [category]: !isCollapsed
                      }))
                    }
                  >
                    <div className="flex items-center gap-2 text-left">
                      {isCollapsed ? <ChevronRight className="h-4 w-4" /> : <ChevronDown className="h-4 w-4" />}
                      <span className="text-sm font-semibold">{category}</span>
                    </div>
                    <Badge>{items.length}</Badge>
                  </Button>
                  {!isCollapsed ? (
                    <div className="space-y-3">
                      {items.map((definition) => (
                        <div
                          key={floating ? definition.id : `mobile-${definition.id}`}
                          className="cursor-grab rounded-md border bg-muted/40 p-3 active:cursor-grabbing"
                          draggable
                          onDragStart={(event) => onPaletteDragStart(event, definition.id)}
                        >
                          <div className="mb-1 flex items-center justify-between gap-2">
                            <div className="text-sm font-semibold">{definition.title}</div>
                            <Badge>{definition.category}</Badge>
                          </div>
                          <p className="mb-2 text-xs text-muted-foreground">{definition.description}</p>
                          <Button
                            size="sm"
                            variant="secondary"
                            className="w-full"
                            onClick={() => onAddNode(definition.id)}
                          >
                            <Plus className="mr-1 h-3.5 w-3.5" />
                            Add to canvas
                          </Button>
                        </div>
                      ))}
                    </div>
                  ) : null}
                </div>
              );
            })}
          </CardContent>
        </div>
      </div>
    </Card>
  );
}
