# -*- coding: utf-8 -*-
"""Pipeline topology endpoint (#1451).

Exposes the scheduled data-processing pipeline - its stages, their schedules
and logical dependencies, and each stage's latest run - so the backend can ask
the service what runs, in what order, and whether a stage is stale, instead of
reading ``src/scheduler.py``.
"""

from __future__ import annotations

from typing import List, Optional

from fastapi import APIRouter
from pydantic import BaseModel

from src.pipeline_run_registry import get_default_registry
from src.pipeline_topology import build_topology, generated_at

router = APIRouter(prefix="/api/pipeline", tags=["Pipeline"])


class StageStatus(BaseModel):
    """One pipeline stage plus the outcome of its most recent run."""

    id: str
    name: str
    schedule: str
    depends_on: List[str]
    # never_run | running | success | failed
    status: str
    last_run: Optional[str] = None
    duration_seconds: Optional[float] = None
    outcome: Optional[str] = None
    error: Optional[str] = None


class PipelineTopologyResponse(BaseModel):
    """The full pipeline topology response."""

    generated_at: str
    stages: List[StageStatus]


@router.get("/topology", response_model=PipelineTopologyResponse)
async def get_pipeline_topology() -> PipelineTopologyResponse:
    """
    Return the data-processing pipeline topology.

    Each stage reports its schedule, the stages it logically depends on, and
    its latest run (last run timestamp, duration and outcome). A stage that has
    never run reports ``status = "never_run"`` with null run fields, which is
    how a failed stage is distinguished from one that has not run yet.

    The stage set and schedules are derived from the scheduler configuration
    (``AnalyticsScheduler`` in ``src/scheduler.py``); run status is recorded by
    the scheduler process and read here through the pipeline run registry.
    """
    runs = get_default_registry().get_runs()
    stages = [StageStatus(**stage) for stage in build_topology(runs)]
    return PipelineTopologyResponse(generated_at=generated_at(), stages=stages)
