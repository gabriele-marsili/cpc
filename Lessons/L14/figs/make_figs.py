"""[extra] Figures for L14.pdf (not from the lecture).
l14_visited.pdf: for every n, the maximum number of nodes visited by the three-case RangeSum
over all ranges [l, r] (same tree as l14::range_sum, split in the middle), vs 4 * levels and vs n."""
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt


def visited(lo, hi, l, r):
    if hi < l or r < lo or (l <= lo and hi <= r):
        return 1
    mid = (lo + hi) // 2
    return 1 + visited(lo, mid, l, r) + visited(mid + 1, hi, l, r)


def levels(n):
    h = 1
    while (1 << (h - 1)) < n:
        h += 1
    return h


ns = list(range(1, 257))
mx = [max(visited(0, n - 1, l, r) for l in range(n) for r in range(l, n)) for n in ns]
fig, ax = plt.subplots(figsize=(6.2, 3.0))
ax.plot(ns, mx, color="#2B6CB0", lw=1.6, label="max visited nodes over all ranges")
ax.plot(ns, [4 * levels(n) for n in ns], color="#C53030", lw=1.2, ls="--", label=r"$4\cdot$levels $= 4(1+\lceil\log_2 n\rceil)$")
ax.plot(ns[:60], ns[:60], color="gray", lw=1, ls=":", label="$n$ (linear)")
ax.set_xlabel("$n$")
ax.set_ylabel("nodes")
ax.set_ylim(0, 45)
ax.spines[["top", "right"]].set_visible(False)
ax.legend(frameon=False, fontsize=8, loc="lower right")
fig.tight_layout()
fig.savefig("l14_visited.pdf")
print(mx[7], mx[255], [ (n, m) for n, m in zip(ns, mx) if n in (8, 16, 64, 256)])
