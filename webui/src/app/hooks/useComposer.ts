import { useCallback, useEffect, useMemo, useRef, useState, type DragEvent } from "react";
import {
  addEdge,
  applyEdgeChanges,
  applyNodeChanges,
  type Connection,
  type EdgeChange,
  type NodeChange,
  type ReactFlowInstance,
  type Viewport
} from "@xyflow/react";

import {
  applyConnectedInputs,
  defaultValues,
  graphPayload,
  normalizeFlowEdges
} from "@/app/helpers";
import type {
  FlowEdge,
  FlowNode,
  SourceFlavor,
  UiBlockDefinition,
  UiBlocksResponse,
  UiGraphNode,
  UiRenderResponse,
  UiSourceBundle
} from "@/app/types";
import { initialSource } from "@/app/types";

type UseComposerArgs = {
  onError: (message: string) => void;
};

const DEFAULT_FLOW_ZOOM = 1;

export type ComposerProgramApi = {
  activeFlavor: SourceFlavor;
  edges: FlowEdge[];
  getGraphSnapshot: () => { nodes: FlowNode[]; edges: FlowEdge[] };
  isCodeEditMode: boolean;
  nodes: FlowNode[];
  source: UiSourceBundle;
  loadBlocks: () => Promise<void>;
  setPaletteMinimized: (value: boolean | ((current: boolean) => boolean)) => void;
  setActiveFlavor: (value: SourceFlavor) => void;
  setSource: (value: UiSourceBundle) => void;
  setIsCodeEditMode: (value: boolean) => void;
  setGraphStatus: (value: string) => void;
  toFlowNodes: (graphNodes: UiGraphNode[]) => { loadedNodes: FlowNode[]; skippedNodes: number };
  hydrateGraph: (loadedNodes: FlowNode[], loadedEdges: FlowEdge[], isCurrent?: () => boolean) => void;
  clearHydrationState: () => void;
  clearGraphForCodeVersion: () => void;
  resetComposerToDraft: () => void;
};

export function useComposer({ onError }: UseComposerArgs) {
  const [definitions, setDefinitions] = useState<UiBlockDefinition[]>([]);
  const [search, setSearch] = useState("");
  const [nodes, setNodes] = useState<FlowNode[]>([]);
  const [edges, setEdges] = useState<FlowEdge[]>([]);
  const [source, setSourceState] = useState<UiSourceBundle>(initialSource);
  const [activeFlavorState, setActiveFlavorState] = useState<SourceFlavor>("rustscript");
  const [rendering, setRendering] = useState(false);
  const [idSequence, setIdSequence] = useState(0);
  const [graphStatusState, setGraphStatusState] = useState("");
  const [graphCanvasRevision, setGraphCanvasRevision] = useState(0);
  const [paletteMinimized, setPaletteMinimized] = useState(true);
  const [codePanelMinimized, setCodePanelMinimized] = useState(true);
  const [isCodeEditModeState, setIsCodeEditModeState] = useState(false);

  const hydratingGraphRef = useRef(false);
  const pendingFitViewRef = useRef(false);
  const flowZoomRef = useRef(DEFAULT_FLOW_ZOOM);
  const hydrationTimerRef = useRef<number | null>(null);
  const rfInstanceRef = useRef<ReactFlowInstance<FlowNode, FlowEdge> | null>(null);
  const definitionMapRef = useRef<Map<string, UiBlockDefinition>>(new Map());
  const nodesRef = useRef<FlowNode[]>([]);
  const edgesRef = useRef<FlowEdge[]>([]);

  const definitionMap = useMemo(() => {
    const map = new Map<string, UiBlockDefinition>();
    for (const definition of definitions) {
      map.set(definition.id, definition);
    }
    return map;
  }, [definitions]);

  useEffect(() => {
    definitionMapRef.current = definitionMap;
  }, [definitionMap]);

  useEffect(() => {
    nodesRef.current = nodes;
  }, [nodes]);

  useEffect(() => {
    edgesRef.current = edges;
  }, [edges]);

  const filteredDefinitions = useMemo(() => {
    const term = search.trim().toLowerCase();
    if (!term) {
      return definitions;
    }
    return definitions.filter((definition) =>
      `${definition.title} ${definition.category} ${definition.description}`.toLowerCase().includes(term)
    );
  }, [definitions, search]);

  const clearHydrationTimer = useCallback(() => {
    if (hydrationTimerRef.current !== null) {
      window.clearTimeout(hydrationTimerRef.current);
      hydrationTimerRef.current = null;
    }
  }, []);

  const bumpGraphCanvasRevision = useCallback(() => {
    setGraphCanvasRevision((value) => value + 1);
  }, []);

  const clearHydrationState = useCallback(() => {
    clearHydrationTimer();
    hydratingGraphRef.current = false;
  }, [clearHydrationTimer]);

  const clearGraphForCodeVersion = useCallback(() => {
    clearHydrationState();
    pendingFitViewRef.current = false;
    setNodes([]);
    setEdges([]);
    flowZoomRef.current = DEFAULT_FLOW_ZOOM;
    rfInstanceRef.current?.setViewport({ x: 0, y: 0, zoom: DEFAULT_FLOW_ZOOM });
    bumpGraphCanvasRevision();
  }, [bumpGraphCanvasRevision, clearHydrationState]);

  const resetComposerToDraft = useCallback(() => {
    clearGraphForCodeVersion();
    setSourceState(initialSource);
    setIsCodeEditModeState(false);
  }, [clearGraphForCodeVersion]);

  const hydrateGraph = useCallback(
    (loadedNodes: FlowNode[], loadedEdges: FlowEdge[], isCurrent?: () => boolean) => {
      hydratingGraphRef.current = true;
      pendingFitViewRef.current = true;
      setNodes(loadedNodes);
      setEdges(loadedEdges);
      bumpGraphCanvasRevision();
      clearHydrationTimer();
      hydrationTimerRef.current = window.setTimeout(() => {
        if (isCurrent && !isCurrent()) {
          return;
        }
        hydratingGraphRef.current = false;
        const instance = rfInstanceRef.current;
        if (instance && pendingFitViewRef.current) {
          instance.fitView({ padding: 0.35, duration: 140 });
          pendingFitViewRef.current = false;
        }
        hydrationTimerRef.current = null;
      }, 80);
    },
    [bumpGraphCanvasRevision, clearHydrationTimer]
  );

  useEffect(() => {
    return () => {
      clearHydrationState();
    };
  }, [clearHydrationState]);

  const loadBlocks = useCallback(async () => {
    const response = await fetch("/v1/ui/blocks");
    if (!response.ok) {
      throw new Error(`failed to load blocks (${response.status})`);
    }
    const data = (await response.json()) as UiBlocksResponse;
    const nextMap = new Map<string, UiBlockDefinition>();
    for (const definition of data.blocks) {
      nextMap.set(definition.id, definition);
    }
    definitionMapRef.current = nextMap;
    setDefinitions(data.blocks);
  }, []);

  const updateSourceText = useCallback((flavor: SourceFlavor, value: string) => {
    setSourceState((curr) => ({ ...curr, [flavor]: value }));
  }, []);

  const setSource = useCallback((value: UiSourceBundle) => {
    setSourceState(value);
  }, []);

  const setActiveFlavor = useCallback((value: SourceFlavor) => {
    setActiveFlavorState(value);
  }, []);

  const setIsCodeEditMode = useCallback((value: boolean) => {
    setIsCodeEditModeState(value);
  }, []);

  const setGraphStatus = useCallback((value: string) => {
    setGraphStatusState(value);
  }, []);

  const removeNode = useCallback((nodeId: string) => {
    setNodes((curr) => curr.filter((node) => node.id !== nodeId));
    setEdges((curr) => curr.filter((edge) => edge.source !== nodeId && edge.target !== nodeId));
  }, []);

  const updateNodeValue = useCallback((nodeId: string, key: string, value: string) => {
    setNodes((curr) =>
      curr.map((node) =>
        node.id === nodeId
          ? {
              ...node,
              data: {
                ...node.data,
                values: { ...node.data.values, [key]: value }
              }
            }
          : node
      )
    );
  }, []);

  const toFlowNodes = useCallback(
    (graphNodes: UiGraphNode[]) => {
      const loadedNodes: FlowNode[] = [];
      let maxId = 0;
      let skippedNodes = 0;
      const currentDefinitionMap = definitionMapRef.current;
      for (let index = 0; index < graphNodes.length; index += 1) {
        const graphNode = graphNodes[index];
        const definition = currentDefinitionMap.get(graphNode.block_id);
        if (!definition) {
          skippedNodes += 1;
          continue;
        }
        const mergedValues = { ...defaultValues(definition), ...graphNode.values };
        loadedNodes.push({
          id: graphNode.id,
          type: "blockNode",
          position: graphNode.position ?? { x: 120 + (index % 4) * 72, y: 120 + Math.floor(index / 4) * 140 },
          data: {
            blockId: definition.id,
            definition,
            values: mergedValues,
            connectedInputs: {},
            onValueChange: updateNodeValue,
            onDelete: removeNode
          }
        });
        const numeric = Number.parseInt(graphNode.id.replace("node-", ""), 10);
        if (!Number.isNaN(numeric)) {
          maxId = Math.max(maxId, numeric);
        }
      }
      setIdSequence(maxId);
      return { loadedNodes, skippedNodes };
    },
    [removeNode, updateNodeValue]
  );

  useEffect(() => {
    setNodes((curr) => applyConnectedInputs(curr, edges));
  }, [edges]);

  useEffect(() => {
    if (isCodeEditModeState) {
      return;
    }
    const payload = graphPayload(nodes, edges);
    const controller = new AbortController();
    const timer = setTimeout(async () => {
      setRendering(true);
      try {
        const response = await fetch("/v1/ui/render", {
          method: "POST",
          headers: { "content-type": "application/json" },
          body: JSON.stringify(payload),
          signal: controller.signal
        });
        const text = await response.text();
        if (!response.ok) {
          throw new Error(text || `render failed (${response.status})`);
        }
        const rendered = JSON.parse(text) as UiRenderResponse;
        setSourceState(rendered.source);
      } catch (err) {
        if ((err as { name?: string }).name !== "AbortError") {
          onError(err instanceof Error ? err.message : "render failed");
        }
      } finally {
        setRendering(false);
      }
    }, 220);
    return () => {
      controller.abort();
      clearTimeout(timer);
    };
  }, [edges, isCodeEditModeState, nodes, onError]);

  const getViewportCenterPosition = useCallback(() => {
    const instance = rfInstanceRef.current;
    if (!instance) {
      return null;
    }
    const flowRoot = document.querySelector(".react-flow");
    if (!(flowRoot instanceof HTMLElement)) {
      return null;
    }
    const rect = flowRoot.getBoundingClientRect();
    if (rect.width <= 0 || rect.height <= 0) {
      return null;
    }
    return instance.screenToFlowPosition({
      x: rect.left + rect.width / 2,
      y: rect.top + rect.height / 2
    });
  }, []);

  const addNode = useCallback(
    (blockId: string, position?: { x: number; y: number }) => {
      const definition = definitionMap.get(blockId);
      if (!definition) {
        return;
      }
      const nextId = idSequence + 1;
      setIdSequence(nextId);
      const id = `node-${nextId}`;
      const fallback = { x: 130 + ((nextId - 1) % 4) * 56, y: 120 + ((nextId - 1) % 4) * 56 };
      const viewportCenter = position ? null : getViewportCenterPosition();
      const resolvedPosition = position ?? viewportCenter ?? fallback;
      const created: FlowNode = {
        id,
        type: "blockNode",
        position: resolvedPosition,
        data: {
          blockId: definition.id,
          definition,
          values: defaultValues(definition),
          connectedInputs: {},
          onValueChange: updateNodeValue,
          onDelete: removeNode
        }
      };
      setNodes((curr) => [...curr, created]);

      // "Add to canvas" has no explicit position; center the node's visual box after measurement.
      if (!position && viewportCenter) {
        const seedX = resolvedPosition.x;
        const seedY = resolvedPosition.y;
        const recenterAfterMeasure = (attempt: number) => {
          const instance = rfInstanceRef.current;
          if (!instance) {
            return;
          }
          const internalNode = instance.getInternalNode(id);
          const measuredWidth = internalNode?.measured?.width;
          const measuredHeight = internalNode?.measured?.height;

          if (!measuredWidth || !measuredHeight) {
            if (attempt < 8) {
              window.requestAnimationFrame(() => recenterAfterMeasure(attempt + 1));
            }
            return;
          }

          const currentNode = nodesRef.current.find((node) => node.id === id);
          if (!currentNode) {
            return;
          }

          const movedSinceCreate =
            Math.abs(currentNode.position.x - seedX) > 1 || Math.abs(currentNode.position.y - seedY) > 1;
          if (movedSinceCreate) {
            return;
          }

          const centeredTopLeft = {
            x: viewportCenter.x - measuredWidth / 2,
            y: viewportCenter.y - measuredHeight / 2
          };
          setNodes((curr) => curr.map((node) => (node.id === id ? { ...node, position: centeredTopLeft } : node)));
        };

        window.requestAnimationFrame(() => recenterAfterMeasure(0));
      }
    },
    [definitionMap, getViewportCenterPosition, idSequence, removeNode, updateNodeValue]
  );

  const onNodesChange = useCallback((changes: NodeChange<FlowNode>[]) => {
    if (hydratingGraphRef.current) {
      const allowedHydrationChanges = changes.filter((change) => change.type === "dimensions");
      if (allowedHydrationChanges.length > 0) {
        setNodes((curr) => applyNodeChanges(allowedHydrationChanges, curr));
      }
      return;
    }
    setNodes((curr) => applyNodeChanges(changes, curr));
  }, []);

  const onEdgesChange = useCallback((changes: EdgeChange<FlowEdge>[]) => {
    if (hydratingGraphRef.current) {
      return;
    }
    setEdges((curr) => normalizeFlowEdges(applyEdgeChanges(changes, curr)));
  }, []);

  const onConnect = useCallback(
    (connection: Connection) => {
      if (!connection.source || !connection.target || !connection.sourceHandle || !connection.targetHandle) {
        return;
      }
      const sourceNode = nodes.find((node) => node.id === connection.source);
      const targetNode = nodes.find((node) => node.id === connection.target);
      if (!sourceNode || !targetNode) {
        return;
      }
      const sourceHandle = connection.sourceHandle;
      const targetHandle = connection.targetHandle;
      const sourceOutput = sourceNode.data.definition.outputs.find((output) => output.key === sourceHandle);
      if (!sourceOutput) {
        return;
      }
      if (sourceOutput.expr_from_input === null) {
        if (targetHandle !== "__flow" || !targetNode.data.definition.accepts_flow) {
          onError("flow outputs must connect to Flow In");
          return;
        }
      } else {
        const targetInput = targetNode.data.definition.inputs.find((input) => input.key === targetHandle);
        if (!targetInput || !targetInput.connectable) {
          onError("data outputs must connect to connectable input");
          return;
        }
      }

      setEdges((curr) =>
        normalizeFlowEdges(
          addEdge(
            {
              ...connection,
              id: `${connection.source}:${sourceHandle}->${connection.target}:${targetHandle}`,
              data: {
                source_output: sourceHandle,
                target_input: targetHandle
              },
              type: "default",
              animated: true,
              style: { stroke: "#22d3ee", strokeWidth: 2 }
            },
            curr.filter((edge) => !(edge.target === connection.target && edge.targetHandle === targetHandle))
          )
        )
      );
      onError("");
    },
    [nodes, onError]
  );

  const onFlowInit = useCallback(
    (instance: ReactFlowInstance<FlowNode, FlowEdge>) => {
      rfInstanceRef.current = instance;
      if (!pendingFitViewRef.current) {
        instance.setViewport({ x: 0, y: 0, zoom: flowZoomRef.current });
      }
      if (pendingFitViewRef.current) {
        window.requestAnimationFrame(() => {
          if (rfInstanceRef.current !== instance || !pendingFitViewRef.current) {
            return;
          }
          instance.fitView({ padding: 0.35, duration: 140 });
          pendingFitViewRef.current = false;
        });
      }
    },
    []
  );

  const onFlowMoveEnd = useCallback((viewport: Viewport) => {
    flowZoomRef.current = viewport.zoom;
  }, []);

  const onPaletteDragStart = useCallback((event: DragEvent<HTMLDivElement>, blockId: string) => {
    event.dataTransfer.setData("application/x-pd-block", blockId);
    event.dataTransfer.effectAllowed = "move";
  }, []);

  const onCanvasDrop = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      event.preventDefault();
      const blockId = event.dataTransfer.getData("application/x-pd-block");
      const instance = rfInstanceRef.current;
      if (!blockId || !instance) {
        return;
      }
      const position = instance.screenToFlowPosition({ x: event.clientX, y: event.clientY });
      addNode(blockId, position);
    },
    [addNode]
  );

  const getGraphSnapshot = useCallback(() => {
    const instance = rfInstanceRef.current;
    if (!instance) {
      return { nodes: nodesRef.current, edges: edgesRef.current };
    }
    const instanceNodes = instance.getNodes() as FlowNode[];
    const instanceEdges = instance.getEdges() as FlowEdge[];
    if (instanceEdges.length === 0) {
      return { nodes: instanceNodes, edges: edgesRef.current };
    }
    const fallbackById = new Map(edgesRef.current.map((edge) => [edge.id, edge]));
    const mergedEdges = instanceEdges.map((edge) => {
      const fallback = fallbackById.get(edge.id);
      const sourceHandle =
        edge.sourceHandle ??
        edge.data?.source_output ??
        fallback?.sourceHandle ??
        fallback?.data?.source_output ??
        null;
      const targetHandle =
        edge.targetHandle ??
        edge.data?.target_input ??
        fallback?.targetHandle ??
        fallback?.data?.target_input ??
        null;
      return {
        ...(fallback ?? {}),
        ...edge,
        sourceHandle: sourceHandle ?? undefined,
        targetHandle: targetHandle ?? undefined,
        data: {
          ...(fallback?.data ?? {}),
          ...(edge.data ?? {}),
          ...(sourceHandle ? { source_output: sourceHandle } : {}),
          ...(targetHandle ? { target_input: targetHandle } : {})
        }
      } as FlowEdge;
    });
    const normalizedEdges = normalizeFlowEdges(mergedEdges);
    const edges = normalizedEdges.length === 0 ? edgesRef.current : normalizedEdges;
    return {
      nodes: instanceNodes,
      edges
    };
  }, []);

  const selectedNodeCount = nodes.filter((node) => node.selected).length;
  const selectedEdgeCount = edges.filter((edge) => edge.selected).length;

  return {
    definitions,
    filteredDefinitions,
    search,
    setSearch,
    nodes,
    edges,
    getGraphSnapshot,
    source,
    activeFlavor: activeFlavorState,
    rendering,
    graphStatus: graphStatusState,
    graphCanvasRevision,
    paletteMinimized,
    setPaletteMinimized,
    codePanelMinimized,
    setCodePanelMinimized,
    isCodeEditMode: isCodeEditModeState,
    setIsCodeEditMode,
    setSource,
    setActiveFlavor,
    setGraphStatus,
    selectedNodeCount,
    selectedEdgeCount,
    loadBlocks,
    updateSourceText,
    addNode,
    onNodesChange,
    onEdgesChange,
    onConnect,
    onFlowInit,
    onFlowMoveEnd,
    onPaletteDragStart,
    onCanvasDrop,
    toFlowNodes,
    hydrateGraph,
    clearHydrationState,
    clearGraphForCodeVersion,
    resetComposerToDraft
  };
}
