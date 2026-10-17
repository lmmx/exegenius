"""W3 design comparison: per-page fidelity failures (major error or <90% items faithful).
Pages differ in difficulty: each page's per-run failure probability is Beta with mean m and
concentration k (k=4: strongly heterogeneous). Designs vary pages P and runs per arm (rS, rO).
Statistic d = failure rate(S) - failure rate(O) over page-runs. The routing threshold is set so
that P(route) = 0.05 when both arms fail equally often (pure run and page variance)."""
import random, statistics
random.seed(20261017)
N = 20000
def beta(m, k):
    if m <= 0: return 0.0
    return random.betavariate(m * k, (1 - m) * k)
def sample(P, rS, rO, mS, mO, k, same):
    fs = fo = 0
    for _ in range(P):
        s = beta(mS, k)
        o = s if same else beta(mO, k)
        fs += sum(random.random() < s for _ in range(rS))
        fo += sum(random.random() < o for _ in range(rO))
    return fs / (P * rS), fo / (P * rO)
designs = {"D1 6p x (1S,1O) current": (6, 1, 1), "D2 6p x (2S,1O)": (6, 2, 1),
           "D3 6p x (2S,2O)": (6, 2, 2), "D4 9p x (1S,1O)": (9, 1, 1), "D5 9p x (2S,1O)": (9, 2, 1)}
k = 4
for name, (P, rS, rO) in designs.items():
    null = sorted(a - b for a, b in (sample(P, rS, rO, 0.25, 0.25, k, True) for _ in range(N)))
    t = null[int(0.95 * N)]
    t = max(t, 1e-9)
    pw = {}
    for lab, mS, mO in (("strong 0.35 vs 0.05", 0.35, 0.05), ("moderate 0.25 vs 0.05", 0.25, 0.05), ("weak 0.15 vs 0.05", 0.15, 0.05)):
        ds = [a - b for a, b in (sample(P, rS, rO, mS, mO, k, False) for _ in range(N))]
        pw[lab] = sum(d > t for d in ds) / N
    # adequacy: Sonnet failure rate <= 1/12 when Sonnet truly fails 0.05 vs 0.25
    ok_low = sum(sample(P, rS, rO, 0.05, 0.05, k, False)[0] <= 1/12 + 1e-9 for _ in range(N)) / N
    ok_high = sum(sample(P, rS, rO, 0.25, 0.05, k, False)[0] <= 1/12 + 1e-9 for _ in range(N)) / N
    print(f"{name:26s} route threshold d>{t:.3f} | power " + ", ".join(f"{l}: {v:.2f}" for l, v in pw.items())
          + f" | 'Sonnet adequate' (rate<=1/12): true 0.05 -> {ok_low:.2f}, true 0.25 -> {ok_high:.2f}")
