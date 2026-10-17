# W3 design comparison (2026-10-17; simulation only, no model calls)

These are the scripts behind `calibration/plans/W3.md`, "Amendment 2". Each runs with `python3 -I <script>`.

- `sim.py`: power of a failure-rate difference test, with its threshold set for 5% false routing when both arms fail equally. Every design is weak on this test (0.29–0.51 for a strong effect), because thresholds on 6–9 pages are coarse.
- `oc.py`: operating characteristics of fixed rules for five designs in four worlds.
- `rules.py`: the 2026-10-15 11:06 secondary rule against rule R2 (two Sonnet runs per key-step page) and a stricter variant. This shows the 11:06 rule clears Sonnet wrongly 42–66% of the time when it fails 20–30% of page-runs.
