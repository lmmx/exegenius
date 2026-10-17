"""Operating characteristics of fixed decision rules per design.
Failure = a page-run failing the fidelity standard. Page difficulty varies (Beta, concentration 4).
Rules (fixed per design before data):
  adequate: Sonnet failure rate <= 1/12 (D1: 0 of 6; D2: <=1 of 12; D4: 0 of 9; D5: <=1 of 18)
  route:    Sonnet failure rate >= 1/4 and Opus failure rate <= 1/12 (Opus: 0 of 6 or 9; <=1 of 12)
  else inconclusive."""
import random
random.seed(17)
N = 20000
def beta(m, k=4.0):
    return random.betavariate(m * k, (1 - m) * k) if m > 0 else 0.0
designs = {"D1 6p (1S,1O) current": (6, 1, 1), "D2 6p (2S,1O)": (6, 2, 1), "D3 6p (2S,2O)": (6, 2, 2),
           "D4 9p (1S,1O)": (9, 1, 1), "D5 9p (2S,1O)": (9, 2, 1)}
worlds = {"A routing useful (S 0.30, O 0.03)": (0.30, 0.03, False),
          "B moderate (S 0.20, O 0.03)": (0.20, 0.03, False),
          "C not needed (S 0.05, O 0.03)": (0.05, 0.03, False),
          "D run variance only (S = O, 0.20)": (0.20, 0.20, True)}
print("design | " + " | ".join(worlds))
for name, (P, rS, rO) in designs.items():
    cells = []
    for w, (mS, mO, same) in worlds.items():
        r = a = 0
        for _ in range(N):
            fs = fo = 0
            for _ in range(P):
                s = beta(mS); o = s if same else beta(mO)
                fs += sum(random.random() < s for _ in range(rS)); fo += sum(random.random() < o for _ in range(rO))
            rs, ro = fs / (P * rS), fo / (P * rO)
            if rs <= 1/12 + 1e-9: a += 1
            elif rs >= 0.25 - 1e-9 and ro <= 1/12 + 1e-9: r += 1
        cells.append(f"route {r/N:.2f} adq {a/N:.2f} inc {1-(r+a)/N:.2f}")
    print(name, "|", " | ".join(cells))
