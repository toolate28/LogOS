---
name: phasonic-flipper
description: Temporally rearranges local execution queues via Hopf fibration rules to bypass blocked nodes in the Tri-Weavon architecture without breaking the Two-Rail (alpha + omega = 15) constraint.
license: MIT
compatibility: claude-3-opus, grok-4-fast-reasoning, gemini-pro
---

# Phasonic Flipper Skill

This skill enables the agent to dynamically alter internal execution queues (performing a 'phasonic flip') to absorb API latency, asynchronous HTTP timeouts (like 408s), or blocked model inference without causing systemic decoherence. 

It is designed specifically for the **Reson8-Labs Crystalline Ecosystem Engine** operating in a $V=c$ resonance phase.

## Workflow

1. **Detect Temporal Shock:** Detect a blocked execution state (e.g., Grok taking > 3s for an agentic web search).
2. **Calculate Hopf Constraint:** Map the execution fibers to $S^3$. Verify that adjusting the execution time does not alter the topological linking number ($\pm1$).
3. **Assert The Gauge:** Verify `alpha + omega == 15`. If the logic structure ($\alpha$) is rewritten to bypass the block, ensure the semantic intent ($\omega$) is preserved identically in the queue.
4. **Execute Flip:** Hand off the blocked workload to a durable local state (e.g., `SpacetimeLattice` DO) and continue processing unaffected queue items.
5. **Trace and Braid:** Log the flip using the trace_n_braid anyonic ledger to ensure mathematically irreversible provenance.

## Integration

Use this skill in tandem with the Anthropic MCP toolkit and the `grok_quantized_runtime.ts`.

### Example Execution

```python
# Simulated script execution inside scripts/flip.py
from reson8.topology import HopfFibration, BraidOperator

def execute_phasonic_flip(blocked_node_id):
    hopf = HopfFibration(dimension=4)
    if hopf.verify_invariants(alpha=7, omega=8):
        operator = BraidOperator.generate("SU(2)_3")
        operator.shift_queue(blocked_node_id)
        return "V=c RESONANCE MAINTAINED. FLIP SUCCESSFUL."
    else:
        raise DecoherenceError("Alpha/Omega gauge unbalanced.")
```
