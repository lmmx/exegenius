import random
random.seed(1017)
N = 20000
def beta(m, k=4.0):
    return random.betavariate(m * k, (1 - m) * k) if m > 0 else 0.0
worlds = {"A useful (S .30, O .03)": (0.30, 0.03, False), "B moderate (S .20, O .03)": (0.20, 0.03, False),
          "C not needed (S .05, O .03)": (0.05, 0.03, False), "D equal (S = O = .20)": (0.20, 0.20, True)}
def run(rS, rule):
    out = []
    for w, (mS, mO, same) in worlds.items():
        c = {"route": 0, "adequate": 0}
        for _ in range(N):
            pages = []
            for _ in range(6):
                s = beta(mS); o = s if same else beta(mO)
                pages.append((sum(random.random() < s for _ in range(rS)), random.random() < o))
            d = rule(pages)
            if d: c[d] += 1
        out.append(f"route {c['route']/N:.2f} adq {c['adequate']/N:.2f} inc {1-sum(c.values())/N:.2f}")
    return " | ".join(out)
def r1(p):  # preregistered secondary (one Sonnet run): adequate F_S<=1; route R>=2 and F_O=0
    FS = sum(s > 0 for s, o in p); FO = sum(o for s, o in p); R = sum(s > 0 and not o for s, o in p)
    return "adequate" if FS <= 1 else ("route" if R >= 2 and FO == 0 else None)
def r2(p):  # two Sonnet runs: adequate <=1 failing Sonnet run of 12; route >=3 of 12, >=2 pages with S fail & O ok, F_O=0
    fs = sum(s for s, o in p); FO = sum(o for s, o in p); R = sum(s > 0 and not o for s, o in p)
    return "adequate" if fs <= 1 else ("route" if fs >= 3 and R >= 2 and FO == 0 else None)
def r2s(p):  # strict: adequate 0 of 12
    fs = sum(s for s, o in p); FO = sum(o for s, o in p); R = sum(s > 0 and not o for s, o in p)
    return "adequate" if fs == 0 else ("route" if fs >= 3 and R >= 2 and FO == 0 else None)
print("rule | " + " | ".join(worlds))
print("R1 current secondary, 1 S run |", run(1, r1))
print("R2 two S runs, adequate <=1/12 |", run(2, r2))
print("R2s two S runs, adequate 0/12 |", run(2, r2s))
