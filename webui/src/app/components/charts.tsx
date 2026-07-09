import { useState } from "react";

import type { LineSeries } from "@/app/helpers";
import type { EdgeTrafficPoint } from "@/app/types";

const WIDTH = 520;
const HEIGHT = 180;
const PLOT_LEFT = 44;
const PLOT_RIGHT = 12;
const PLOT_TOP = 8;
const PLOT_BOTTOM = 30;

type AxisProps = {
  xAxisLabel?: string;
  yAxisLabel?: string;
};

type ChartCoordinate = {
  x: number;
  y: number;
};

function formatAxisTime(unixMs: number): string {
  return new Date(unixMs).toLocaleTimeString([], {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
    hour12: false
  });
}

function axisAndScales(points: EdgeTrafficPoint[], maxYInput: number) {
  const maxY = Number.isFinite(maxYInput) && maxYInput > 0 ? maxYInput : 1;
  const plotWidth = WIDTH - PLOT_LEFT - PLOT_RIGHT;
  const plotHeight = HEIGHT - PLOT_TOP - PLOT_BOTTOM;
  const xStep = points.length > 1 ? plotWidth / (points.length - 1) : 0;
  const ticksY = [maxY, maxY / 2, 0];
  return { maxY, plotWidth, plotHeight, xStep, ticksY };
}

function xFor(index: number, xStep: number): number {
  return PLOT_LEFT + index * xStep;
}

function yFor(value: number, maxY: number, plotHeight: number): number {
  return PLOT_TOP + (1 - value / maxY) * plotHeight;
}

function safeChartValue(value: number): number {
  if (!Number.isFinite(value) || value < 0) {
    return 0;
  }
  return value;
}

function formatAxisValue(value: number): string {
  if (!Number.isFinite(value)) {
    return "0";
  }
  const absValue = Math.abs(value);
  const maximumFractionDigits = absValue >= 100 ? 0 : absValue >= 10 ? 1 : 2;
  return new Intl.NumberFormat(undefined, { maximumFractionDigits }).format(value);
}

function pathForPoints(
  points: EdgeTrafficPoint[],
  valueFor: (point: EdgeTrafficPoint, index: number, points: EdgeTrafficPoint[]) => number,
  maxY: number,
  plotHeight: number,
  xStep: number
): string {
  return points
    .map((point, index) => {
      const x = xFor(index, xStep);
      const value = safeChartValue(valueFor(point, index, points));
      const y = yFor(value, maxY, plotHeight);
      return `${index === 0 ? "M" : "L"} ${x.toFixed(1)} ${y.toFixed(1)}`;
    })
    .join(" ");
}

function smoothValues(values: number[], alpha: number): number[] {
  if (values.length <= 2) {
    return values.slice();
  }
  const clampedAlpha = Math.min(Math.max(alpha, 0.05), 0.95);
  const output = new Array<number>(values.length);
  output[0] = values[0];
  for (let index = 1; index < values.length; index += 1) {
    output[index] = clampedAlpha * values[index] + (1 - clampedAlpha) * output[index - 1];
  }
  return output;
}

function coordinatesForValues(values: number[], maxY: number, plotHeight: number, xStep: number): ChartCoordinate[] {
  return values.map((value, index) => ({
    x: xFor(index, xStep),
    y: yFor(value, maxY, plotHeight)
  }));
}

function polylinePathForCoordinates(coordinates: ChartCoordinate[]): string {
  return coordinates
    .map((point, index) => `${index === 0 ? "M" : "L"} ${point.x.toFixed(1)} ${point.y.toFixed(1)}`)
    .join(" ");
}

function smoothPathForCoordinates(coordinates: ChartCoordinate[]): string {
  if (coordinates.length <= 2) {
    return polylinePathForCoordinates(coordinates);
  }

  let path = `M ${coordinates[0].x.toFixed(1)} ${coordinates[0].y.toFixed(1)}`;
  for (let index = 0; index < coordinates.length - 1; index += 1) {
    const p0 = coordinates[Math.max(0, index - 1)];
    const p1 = coordinates[index];
    const p2 = coordinates[index + 1];
    const p3 = coordinates[Math.min(coordinates.length - 1, index + 2)];
    const cp1x = p1.x + (p2.x - p0.x) / 6;
    const cp1y = p1.y + (p2.y - p0.y) / 6;
    const cp2x = p2.x - (p3.x - p1.x) / 6;
    const cp2y = p2.y - (p3.y - p1.y) / 6;
    path += ` C ${cp1x.toFixed(1)} ${cp1y.toFixed(1)} ${cp2x.toFixed(1)} ${cp2y.toFixed(1)} ${p2.x.toFixed(1)} ${p2.y.toFixed(1)}`;
  }
  return path;
}

function AxisFrame({
  points,
  maxY,
  ticksY,
  xAxisLabel = "Time",
  yAxisLabel = "Value"
}: {
  points: EdgeTrafficPoint[];
  maxY: number;
  ticksY: number[];
} & AxisProps) {
  const plotHeight = HEIGHT - PLOT_TOP - PLOT_BOTTOM;
  return (
    <>
      <line x1={PLOT_LEFT} y1={PLOT_TOP} x2={PLOT_LEFT} y2={PLOT_TOP + plotHeight} stroke="#cbd5e1" strokeWidth={1} />
      <line
        x1={PLOT_LEFT}
        y1={PLOT_TOP + plotHeight}
        x2={WIDTH - PLOT_RIGHT}
        y2={PLOT_TOP + plotHeight}
        stroke="#cbd5e1"
        strokeWidth={1}
      />
      {ticksY.map((tick) => {
        const y = yFor(tick, maxY, plotHeight);
        return (
          <g key={`tick-${tick}`}>
            <line x1={PLOT_LEFT} y1={y} x2={WIDTH - PLOT_RIGHT} y2={y} stroke="#e2e8f0" strokeWidth={1} />
            <text x={PLOT_LEFT - 6} y={y + 4} textAnchor="end" fontSize={10} fill="#64748b">
              {formatAxisValue(tick)}
            </text>
          </g>
        );
      })}
      {points.length > 0 ? (
        <>
          <text x={PLOT_LEFT} y={HEIGHT - 8} textAnchor="start" fontSize={10} fill="#64748b">
            {formatAxisTime(points[0].unix_ms)}
          </text>
          <text x={WIDTH - PLOT_RIGHT} y={HEIGHT - 8} textAnchor="end" fontSize={10} fill="#64748b">
            {formatAxisTime(points[points.length - 1].unix_ms)}
          </text>
        </>
      ) : null}
      <text x={(PLOT_LEFT + WIDTH - PLOT_RIGHT) / 2} y={HEIGHT - 2} textAnchor="middle" fontSize={10} fill="#64748b">
        {xAxisLabel}
      </text>
      <text x={12} y={HEIGHT / 2} textAnchor="middle" fontSize={10} fill="#64748b" transform={`rotate(-90 12 ${HEIGHT / 2})`}>
        {yAxisLabel}
      </text>
    </>
  );
}

export function LineChart({
  points,
  valueFor,
  stroke,
  emptyLabel,
  xAxisLabel = "Time",
  yAxisLabel = "Value"
}: {
  points: EdgeTrafficPoint[];
  valueFor: (point: EdgeTrafficPoint, index: number, points: EdgeTrafficPoint[]) => number;
  stroke: string;
  emptyLabel: string;
} & AxisProps) {
  if (points.length === 0) {
    return (
      <div className="h-[180px] rounded-md border bg-background/70 p-3 text-sm text-muted-foreground">
        {emptyLabel}
      </div>
    );
  }

  const maxYRaw = Math.max(...points.map((point, index) => safeChartValue(valueFor(point, index, points))), 1);
  const { maxY, plotHeight, xStep, ticksY } = axisAndScales(points, maxYRaw);
  const path = pathForPoints(points, valueFor, maxY, plotHeight, xStep);
  const latestValue = safeChartValue(valueFor(points[points.length - 1], points.length - 1, points));

  return (
    <div className="rounded-md border bg-background/70 p-3">
      <svg viewBox={`0 0 ${WIDTH} ${HEIGHT}`} className="h-[180px] w-full">
        <AxisFrame points={points} maxY={maxY} ticksY={ticksY} xAxisLabel={xAxisLabel} yAxisLabel={yAxisLabel} />
        <path d={path} fill="none" stroke={stroke} strokeWidth={2.5} />
      </svg>
      <div className="mt-2 text-xs text-muted-foreground">
        latest={formatAxisValue(latestValue)} max={formatAxisValue(maxY)}
      </div>
    </div>
  );
}

export function MultiLineChart({
  points,
  series,
  emptyLabel,
  hideZeroSeries = false,
  smooth = false,
  smoothingAlpha = 0.28,
  showHoverValues = false,
  xAxisLabel = "Time",
  yAxisLabel = "Value"
}: {
  points: EdgeTrafficPoint[];
  series: LineSeries[];
  emptyLabel: string;
  hideZeroSeries?: boolean;
  smooth?: boolean;
  smoothingAlpha?: number;
  showHoverValues?: boolean;
} & AxisProps) {
  const [hoveredIndex, setHoveredIndex] = useState<number | null>(null);
  const visibleSeries = hideZeroSeries
    ? series.filter((item) =>
        points.some((point, index) => safeChartValue(item.valueFor(point, index, points)) > 0)
      )
    : series;
  if (points.length === 0 || visibleSeries.length === 0) {
    return (
      <div className="h-[180px] rounded-md border bg-background/70 p-3 text-sm text-muted-foreground">
        {emptyLabel}
      </div>
    );
  }

  const maxYRaw = Math.max(
    ...points.flatMap((point, index) => visibleSeries.map((item) => safeChartValue(item.valueFor(point, index, points)))),
    1
  );
  const { maxY, plotHeight, xStep, ticksY } = axisAndScales(points, maxYRaw);
  const lines = visibleSeries.map((item) => {
    const rawValues = points.map((point, index) => safeChartValue(item.valueFor(point, index, points)));
    const displayValues = smooth ? smoothValues(rawValues, smoothingAlpha) : rawValues;
    const coordinates = coordinatesForValues(displayValues, maxY, plotHeight, xStep);
    return {
      ...item,
      rawValues,
      coordinates,
      path: smooth ? smoothPathForCoordinates(coordinates) : polylinePathForCoordinates(coordinates)
    };
  });
  const hoverIndex = hoveredIndex !== null ? Math.max(0, Math.min(points.length - 1, hoveredIndex)) : null;
  const hoverX = hoverIndex !== null ? xFor(hoverIndex, xStep) : null;
  const hoverTimeLabel = hoverIndex !== null ? formatAxisTime(points[hoverIndex].unix_ms) : null;
  const hoverSeries = hoverIndex !== null
    ? lines.map((line) => ({
        key: line.key,
        stroke: line.stroke,
        value: line.rawValues[hoverIndex] ?? 0
      }))
    : [];
  const tooltipLines = hoverTimeLabel
    ? [hoverTimeLabel, ...hoverSeries.map((item) => `${item.key}: ${formatAxisValue(item.value)}`)]
    : [];
  const tooltipWidth = Math.max(120, ...tooltipLines.map((line) => line.length * 6 + 18));
  const tooltipHeight = Math.max(24, 10 + tooltipLines.length * 14);
  const tooltipX = hoverX === null
    ? PLOT_LEFT
    : Math.max(PLOT_LEFT, Math.min(WIDTH - PLOT_RIGHT - tooltipWidth, hoverX + 8));
  const tooltipY = PLOT_TOP + 8;

  return (
    <div className="rounded-md border bg-background/70 p-3">
      <svg
        viewBox={`0 0 ${WIDTH} ${HEIGHT}`}
        className="h-[180px] w-full"
        onMouseLeave={() => setHoveredIndex(null)}
        onMouseMove={(event) => {
          if (points.length === 0) {
            setHoveredIndex(null);
            return;
          }
          const rect = event.currentTarget.getBoundingClientRect();
          if (rect.width <= 0) {
            return;
          }
          const relativeX = (event.clientX - rect.left) / rect.width;
          const clampedX = Math.min(Math.max(relativeX, 0), 1);
          const plotWidth = WIDTH - PLOT_LEFT - PLOT_RIGHT;
          const xInPlot = clampedX * plotWidth;
          const index = points.length <= 1 ? 0 : Math.round(xInPlot / xStep);
          setHoveredIndex(Math.max(0, Math.min(points.length - 1, index)));
        }}
      >
        <AxisFrame points={points} maxY={maxY} ticksY={ticksY} xAxisLabel={xAxisLabel} yAxisLabel={yAxisLabel} />
        {hoverX !== null ? (
          <line
            x1={hoverX}
            y1={PLOT_TOP}
            x2={hoverX}
            y2={PLOT_TOP + plotHeight}
            stroke="#94a3b8"
            strokeWidth={1}
            strokeDasharray="3 3"
          />
        ) : null}
        {lines.map((line) => (
          <path key={line.key} d={line.path} fill="none" stroke={line.stroke} strokeWidth={2.2} strokeLinecap="round" strokeLinejoin="round" />
        ))}
        {hoverIndex !== null
          ? lines.map((line) => {
              const point = line.coordinates[hoverIndex];
              if (!point) {
                return null;
              }
              return (
                <circle
                  key={`${line.key}-hover-point`}
                  cx={point.x}
                  cy={point.y}
                  r={2.5}
                  fill={line.stroke}
                  stroke="#fff"
                  strokeWidth={0.8}
                />
              );
            })
          : null}
        {showHoverValues && hoverIndex !== null && hoverTimeLabel ? (
          <g pointerEvents="none">
            <rect
              x={tooltipX}
              y={tooltipY}
              width={tooltipWidth}
              height={tooltipHeight}
              rx={6}
              fill="#ffffff"
              fillOpacity={0.94}
              stroke="#cbd5e1"
              strokeWidth={1}
            />
            <text
              x={tooltipX + 8}
              y={tooltipY + 14}
              fontSize={10}
              fontWeight={700}
              fill="#0f172a"
            >
              {hoverTimeLabel}
            </text>
            {hoverSeries.map((item, index) => (
              <g key={`${item.key}-tooltip`} transform={`translate(${tooltipX + 8} ${tooltipY + 26 + index * 13})`}>
                <circle cx={0} cy={-3} r={2.8} fill={item.stroke} />
                <text x={8} y={0} fontSize={10} fill="#334155">
                  {item.key}: {formatAxisValue(item.value)}
                </text>
              </g>
            ))}
          </g>
        ) : null}
      </svg>
      <div className="mt-2 flex flex-wrap gap-3 text-xs text-muted-foreground">
        {lines.map((line) => (
          <div key={`${line.key}-legend`} className="inline-flex items-center gap-1">
            <span className="inline-block h-2.5 w-2.5 rounded-full" style={{ background: line.stroke }} />
            {line.key}
          </div>
        ))}
      </div>
    </div>
  );
}
