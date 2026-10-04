# Genera le figure del PDF di L3 (python3 make_figs.py)
import math
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import Rectangle, FancyArrowPatch, Circle

BLUE, RED, GREY, LIGHT = "#2b6cb0", "#c53030", "#718096", "#e2ecf7"

def cell(ax, x, y, text, fc="white", ec="black", tc="black", w=1, h=1, bold=False):
    ax.add_patch(Rectangle((x, y), w, h, fc=fc, ec=ec, lw=1))
    ax.text(x + w/2, y + h/2, str(text), ha="center", va="center", color=tc,
            fontsize=11, fontweight="bold" if bold else "normal")

# ---------- 1) trace della deque ----------
A, k = [1, 3, 2, 1, 1, 2, 4, 3, 2], 3   # esempio della lezione
rows, q = [], []
for i, x in enumerate(A):
    front_pop = []
    while q and q[0] + k <= i:
        front_pop.append(q.pop(0))
    back_pop = []
    while q and A[i] >= A[q[-1]]:   # a lezione: rimuove gli elementi <= y
        back_pop.append(q.pop())
    q.append(i)
    rows.append((i, list(q), front_pop, back_pop, A[q[0]]))

fig, ax = plt.subplots(figsize=(10, 7.2))
n = len(A)
for r, (i, qs, fp, bp, mx) in enumerate(rows):
    y = -r * 1.5
    lo = max(0, i - k + 1)
    ax.text(-0.6, y + 0.5, f"p={i+1}", ha="right", va="center", fontsize=11)
    for j, v in enumerate(A):
        inwin = lo <= j <= i and i >= k - 1 or (i < k - 1 and j <= i)
        cell(ax, j, y, v, fc=LIGHT if inwin else "white",
             ec=BLUE if j == i else ("black" if inwin else "#cbd5e0"),
             tc="black" if j <= i else "#cbd5e0", bold=(j == i))
    # deque
    ax.text(n + 0.6, y + 0.5, "Q:", ha="left", va="center", fontsize=11)
    for t, idx in enumerate(qs):
        cell(ax, n + 1.5 + t * 1.6, y, f"<{A[idx]},{idx+1}>", w=1.5,
             fc="#fefcbf" if t == 0 else "white")
    notes = []
    if fp: notes.append("head out: " + ",".join(str(A[p]) for p in fp))
    if bp: notes.append("tail pop: " + ",".join(str(A[p]) for p in bp))
    ax.text(n + 6.6, y + 0.5, "; ".join(notes), ha="left", va="center", fontsize=9, color=GREY)
    if mx is not None:
        ax.text(n + 11.6, y + 0.5, f"max = {mx}" + ("  (partial)" if i < k-1 else ""), ha="left", va="center",
                fontsize=11, color=RED if i >= k-1 else GREY, fontweight="bold")
ax.set_xlim(-1.5, n + 13.5); ax.set_ylim(-len(rows) * 1.5 + 0.2, 1.3)
ax.text(0, 1.1, "array A (window shaded, new element outlined in blue)", fontsize=9, color=GREY)
ax.text(n + 1.5, 1.1, "deque of pairs <value, position> — head highlighted", fontsize=9, color=GREY)
ax.axis("off"); fig.tight_layout(); fig.savefig("deque_trace.pdf"); plt.close(fig)

# ---------- 2) right leaders ----------
W = [2, 9, 4, 7, 1, 6, 3, 5, 2]
leaders = [j for j in range(len(W)) if all(W[j] > W[t] for t in range(j+1, len(W)))]
fig, ax = plt.subplots(figsize=(8, 3.2))
for j, v in enumerate(W):
    c = RED if j in leaders else "#a0aec0"
    ax.bar(j, v, color=c, width=0.7)
    ax.text(j, v + 0.2, str(v), ha="center", fontsize=11, color=c,
            fontweight="bold" if j in leaders else "normal")
ax.set_xticks(range(len(W))); ax.set_xticklabels([f"{j}" for j in range(len(W))])
ax.set_yticks([]); ax.set_ylim(0, 10.8)
for s in ["top", "right", "left"]: ax.spines[s].set_visible(False)
ax.set_title("Right leaders (red) of one window: 9, 7, 6, 5, 2  = exactly the content of Q", fontsize=11)
fig.tight_layout(); fig.savefig("right_leaders.pdf"); plt.close(fig)

# ---------- 3) cicli di una permutazione ----------
perm = [4, 0, 6, 7, 1, 5, 9, 3, 2, 8]   # drawer d contiene perm[d]
seen, cycles = [False]*len(perm), []
for s in range(len(perm)):
    if not seen[s]:
        c, d = [], s
        while not seen[d]:
            seen[d] = True; c.append(d); d = perm[d]
        cycles.append(c)
fig, ax = plt.subplots(figsize=(9, 3.4))
cx, colors = 0, [BLUE, RED, "#2f855a", "#b7791f"]
for ci, c in enumerate(cycles):
    L = len(c); R = 0.35 + 0.22 * L
    cx += R + 0.4
    pos = {}
    for t, d in enumerate(c):
        a = math.pi/2 - 2*math.pi*t/L
        pos[d] = (cx + R*math.cos(a), R*math.sin(a))
    for d in c:
        (x1, y1), (x2, y2) = pos[d], pos[perm[d]]
        if d == perm[d]:
            ax.add_patch(FancyArrowPatch((x1-0.12, y1+0.2), (x1+0.12, y1+0.2),
                         connectionstyle="arc3,rad=-2.2", arrowstyle="-|>", mutation_scale=12, color=colors[ci % 4]))
        else:
            ax.add_patch(FancyArrowPatch((x1, y1), (x2, y2), arrowstyle="-|>", mutation_scale=12,
                         shrinkA=13, shrinkB=13, color=colors[ci % 4], connectionstyle="arc3,rad=0.15"))
    for d, (x, y) in pos.items():
        ax.add_patch(Circle((x, y), 0.2, fc="white", ec=colors[ci % 4], lw=1.5))
        ax.text(x, y, str(d), ha="center", va="center", fontsize=10)
    ax.text(cx, -R - 0.3, f"length {L}", ha="center", va="top", fontsize=9, color=colors[ci % 4])
    cx += R
ax.set_xlim(0, cx + 0.4); ax.set_ylim(-2.1, 1.9); ax.set_aspect("equal"); ax.axis("off")
ax.set_title("drawers = " + str(perm) + "   (arrow d → content of drawer d)", fontsize=10)
fig.tight_layout(); fig.savefig("perm_cycles.pdf"); plt.close(fig)
print(cycles)

# ---------- 4) P(successo) al variare di n ----------
ns = list(range(2, 201, 2))
ps = [1 - sum(1/l for l in range(n//2+1, n+1)) for n in ns]
fig, ax = plt.subplots(figsize=(7, 2.8))
ax.plot(ns, ps, color=BLUE, lw=2, label="cycle-following strategy")
ax.axhline(1 - math.log(2), color=RED, ls="--", lw=1, label="1 − ln 2 ≈ 0.307")
ax.scatter([100], [ps[ns.index(100)]], color=BLUE, zorder=3)
ax.annotate("n = 100: 0.312", (100, ps[ns.index(100)]), xytext=(115, 0.36), fontsize=9,
            arrowprops=dict(arrowstyle="-", color=GREY))
ax.set_xlabel("number of prisoners n"); ax.set_ylabel("P(all succeed)")
ax.set_ylim(0.28, 0.52); ax.legend(frameon=False, fontsize=9)
for s in ["top", "right"]: ax.spines[s].set_visible(False)
fig.tight_layout(); fig.savefig("prisoners_prob.pdf"); plt.close(fig)

# ---------- 5) trapping rain water ----------
Hh=[6,2,0,4,0,1,0,5,0,3]
n=len(Hh); ML=[max(Hh[:i+1]) for i in range(n)]; MR=[max(Hh[i:]) for i in range(n)]
Wt=[min(ML[i],MR[i])-Hh[i] for i in range(n)]
fig, ax = plt.subplots(figsize=(7,3.6))
for i,h in enumerate(Hh):
    ax.add_patch(Rectangle((i,0),1,h,fc="#cbd5e0",ec="black"))
    if Wt[i]: ax.add_patch(Rectangle((i,h),1,Wt[i],fc="#90cdf4",ec=BLUE,lw=0.5))
    ax.text(i+.5,-.6,str(h),ha="center",fontsize=10)
    ax.text(i+.5,-1.4,str(MR[i]),ha="center",fontsize=9,color=BLUE)
    ax.text(i+.5,-2.2,str(Wt[i]),ha="center",fontsize=9,color=RED)
ax.text(-.3,-.6,"H",ha="right",fontsize=10); ax.text(-.3,-1.4,"MR",ha="right",fontsize=9,color=BLUE)
ax.text(-.3,-2.2,"water",ha="right",fontsize=9,color=RED)
ax.set_title(f"H = {Hh}:  total water = {sum(Wt)}",fontsize=10)
ax.set_xlim(-1.2,n+.2); ax.set_ylim(-2.6,6.5); ax.set_aspect("equal"); ax.axis("off")
fig.tight_layout(); fig.savefig("trw.pdf"); plt.close(fig)
