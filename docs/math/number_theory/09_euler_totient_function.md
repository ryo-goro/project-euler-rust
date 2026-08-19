# 09. オイラーのトーシェント函数

## 概要

オイラーのトーシェント函数 $\phi(n)$ を定義し, その性質を述べる.

---

## 定義 9.1

$n$ を 正整数とする.
$1$ 以上 $n$ 以下の整数で $n$ と互いに素であるものの個数を $\phi(n)$ と書く.

函数 $\phi: \mathbb{Z}_{>0} \to \mathbb{Z}$ を**オイラーのトーシェント函数**という.

## 補題 9.1

$n$ を正整数とし, $A$ を $n$ 未満の非負整数で $n$ と互いに素であるものの集合とする:

$$A = \{x \in \mathbb{Z} \mid 0 \le x < n, \, \gcd(x, n) = 1\}.$$

このとき $\phi(n) = |A|$ が成り立つ.
ただし $|A|$ は $A$ の要素数を表す.

### 証明

$A'$ を $1$ 以上 $n$ 以下の整数で $n$ と互いに素であるものの集合とする:

$$A' = \{x \in \mathbb{Z} \mid 1 \le x \le n, \, \gcd(x, n) = 1\}.$$

このとき $\phi(n)$ の定義から明らかに $\phi(n) = |A'|$.
一方定理 3.15 系, 3.18 (2) より $\gcd(0, n) = \gcd(n, n)$ だから,
$0 \in A \iff n \in A'$.
よって $|A| = |A'|$.
故に $\phi(n) = |A|$ が成り立つ.
$\blacksquare$

## 定理 9.2

以下が成り立つ.

(1) $\phi(1) = 1$.

(2) $p$ を素数とするとき, $\phi(p) = p - 1$.

(3) $p$ を素数, $k$ を正整数とするとき, $\phi(p^{k}) = p^{k} - p^{k - 1}$.

### 証明

#### (1)

定理 3.13 系より $1$ 以上 $1$ 以下の整数で $1$ と互いに素である整数は $1$ のみであるので,
定義により $\phi(1) = 1$ である.

#### (2)

いま $d$ を $1$ 以上 $p$ 以下の整数で, $p$ と互いに素ではないものとする.
このとき定理 5.2 より $p \mid d$ が成り立つ.
即ち $k \in \mathbb{Z}$ が存在して $d = kp$ となる.
ここで $1 \le d \le p$ だから, $k = 1, \, d = p$ となる.
故に $1$ 以上 $p$ 以下の整数は, $p$ を除いてすべて $p$ と互いに素だから, $\phi(p) = p - 1$ 個である.

#### (3)

$d$ を $1$ 以上 $p^{k}$ 以下の整数で, $p^{k}$ と互いに素ではないものとする.
このとき $g = \gcd(d, p^{k})$ と置けば $g > 1$ だから, 
定理 5.1 より $q \mid g$ となる素数 $q$ が存在する.
$g$ は $d$ と $p^{k}$ の公約数なので, 定理 2.5 より $q \mid d, \, q \mid p^{k}$ となる.
特にこの後者から, 定理 5.3 を繰り返し適用して $q \mid p$ が得られる.
$p, q$ は共に素数なので, 定理 5.2 系より $q = p$.
これと $q \mid d$ から, $l \in \mathbb{Z}$ が存在して $d = lp$ が成り立つ.
$p > 1, \, 1 \le d \le p^{k}$ だから, $1 \le l \le p^{k - 1}$ となる.

逆に $l$ を $1 \le l \le p^{k - 1}$ なる整数とするとき,
$p \mid lp, \, p \mid p^{k}$ であるから $p > 1$ は $lp$ と $p^{k}$ の公約数である.
よって定理 3.13 より $lp$ と $p^{k}$ は互いに素ではない.

以上から, $1$ 以上 $p^{k}$ 以下の整数で, $p^{k}$ と互いに素ではないものは
$lp \; (1 \le l \le p^{k - 1})$ で尽くされる.
$p > 0$ よりこれらの整数のどの二つも等しくはないので, これらの個数は $p^{k - 1}$ である.
故に定義から $\phi(p^{k}) = p^{k} - p^{k - 1}$ となる.
$\blacksquare$

## 定理 9.3

$m, n$ を互いに素な正整数とする.
このとき

$$\phi(mn) = \phi(m)\phi(n) \tag{9.1}$$

が成り立つ.

### 証明

いま整数 $x$ に対し,
$A_{x}$ を $x$ 未満の非負整数で $x$ と互いに素であるもの全体の集合とする:

$$A_{x} = \{k \in \mathbb{Z} \mid 0 \le k < x, \, \gcd(k, x) = 1\}.$$

このとき補題 9.1 より, 任意の正整数 $x$ に対して

$$\phi(x) = |A_{x}| \tag{9.2}$$

が成り立つ.

さていま $A_{mn}$ から $\mathbb{Z} \times \mathbb{Z}$ への函数 $f$ を

$$f: x \longmapsto (x \bmod m, x \bmod n) \tag{9.3}$$

によって定義する.
このとき $f$ は $A_{mn}$ から $A_{m} \times A_{n}$ への全単射となる.
これが示せれば, $(9.2)$ より

$$\phi(mn) = |A_{mn}| = |A_{m} \times A_{n}| = |A_{m}||A_{n}| = \phi(m)\phi(n)$$

が成り立つので, 本定理は示されたことになる.

#### $f$ が $A_{m} \times A_{n}$ の中への写像であることの証明

任意の $x \in A_{mn}$ に対して $\gcd(x, mn) = 1$ だから,
定理 3.20 より $\gcd(x, m) = \gcd(x, n) = 1$ が成り立つ.
よって定理 3.19 系より

$$\gcd(x \bmod m, m) = \gcd(x \bmod n, n) = 1 \tag{9.4}$$

が成り立つ.
また剰余の定義より

$$0 \le x \bmod m < m, \, 0 \le x \bmod n < n \tag{9.5}$$

である.
$(9.4)$, $(9.5)$ より $x \bmod m \in A_{m}, \, x \bmod n \in A_{n}$ となる.
したがって $f$ は $A_{mn}$ から $A_{m} \times A_{n}$ への中への写像である.

#### $f$ が全射であることの証明

$(x, y) \in A_{m} \times A_{n}$ とするとき,
仮定より $m$ と $n$ は互いに素な正整数だから, 定理 8.2 より
$z \equiv x \pmod{m}$ と $z \equiv y \pmod{n}$ を共にみたす整数 $z$ で
$0 \le z < mn$ となるものが存在する.
ここで $x \in A_{m}, \, y \in A_{n}$ より
$0 \le x < m, \, 0 \le y < n$ だから, 定理 6.7 系 1 より
$z \bmod m = x, \, z \bmod n = y$ が成り立つ.

また $x \in A_{m}, \, y \in A_{n}$ より
$\gcd(x, m) = \gcd(y, n) = 1$ だから, これと定理 3.19 系より

$$
\begin{aligned}
    &\gcd(z, m) = \gcd(z \bmod m, m) = \gcd(x, m) = 1, \\
    &\gcd(z, n) = \gcd(z \bmod n, n) = \gcd(y, n) = 1
\end{aligned}
$$

が成り立つ.
よって定理 3.20 より $\gcd(z, mn) = 1$ となる.
故に $z \in A_{mn}$ で, $f(z) = (z \bmod m, z \bmod n) = (x, y)$ となる.
したがって $f$ は $A_{mn}$ から $A_{m} \times A_{n}$ への全射である.

#### $f$ が単射であることの証明

$x, y \in A_{mn}$ に対して $f(x) = f(y)$ であるとき,
$f$ の定義より $x \bmod m = y \bmod m, \, x \bmod n = y \bmod n$ だから,
定理 6.7 より $x \equiv y \pmod{m}, \, x \equiv y \pmod{n}$ が成り立つ.
これと $m, n$ が互いに素であることから, 定理 6.8 系により
$x \equiv y \pmod{mn}$ が成り立つ.
故に定理 6.7 より $x \bmod (mn) = y \bmod (mn)$ となるが,
$x, y \in A_{mn}$ より $0 \le x, y < mn$ だから

$$x = x \bmod (mn) = y \bmod (mn) = y$$

となる.
したがって $f$ は $A_{mn}$ から $A_{m} \times A_{n}$ への単射である.
$\blacksquare$

## 定理 9.4

$n$ を $1$ より大きい整数とし, $n$ の素因数分解を

$$n = \prod_{i = 1}^{m}p_{i}^{a_{i}}$$

とする ($m$ は正整数, $p_{1} < p_{2} < \cdots < p_{m}$ は素数, $a_{1}, a_{2}, \cdots, a_{m}$ は正整数).
このとき

$$\phi(n) = n \prod_{i = 1}^{m}\left(1 - \frac{1}{p_{i}}\right) \tag{9.6}$$

が成り立つ.

### 証明

いま $k$ を $1 < k \le m$ なる整数とし,
$i$ を $1 \le i < k$ なる整数とする.
このとき $p_{k} \neq p_{i}$ だから,
定理 5.2 系より $p_{k}$ と $p_{i}$ は互いに素である.
よって定理 3.22 (1) より $p_{k}^{a_{k}}$ と $p_{i}^{a_{i}}$ も互いに素となる.
そこで定理 3.20 を繰り返し適用することにより,
$p_{k}^{a_{k}}$ と $\displaystyle \prod_{i = 1}^{k - 1}p_{i}^{a_{i}}$ が互いに素となることがわかる.
よって定理 9.3 を繰り返し適用していき,

$$
\begin{aligned}
    \phi(n) &= \phi\left(\prod_{i = 1}^{m}p_{i}^{a_{i}}\right) \\
            &= \phi\left(\prod_{i = 1}^{m - 1}p_{i}^{a_{i}}\right)\phi(p_{m}^{a_{m}}) \\
            &= \phi\left(\prod_{i = 1}^{m - 2}p_{i}^{a_{i}}\right)\phi(p_{m - 1}^{a_{m - 1}})\phi(p_{m}^{a_{m}}) \\
            &= \cdots \\
            &= \phi(p_{1}^{a_{1}})\phi(p_{2}^{a_{2}}) \cdots \phi(p_{m}^{a_{m}}) \\
            &= \prod_{i = 1}^{m}\phi(p_{i}^{a_{i}}) \tag{9.7}
\end{aligned}
$$

を得る.
ここで定理 9.2 (3) より

$$
    \phi(p_{i}^{a_{i}}) = p_{i}^{a_{i}} - p_{i}^{a_{i} - 1}
                        = p_{i}^{a_{i}}\left(1 - \frac{1}{p_{i}}\right)
                        \quad (1 \le i \le m)
$$

であるから, これと $(9.7)$ から

$$
\begin{aligned}
    \phi(n) &= \prod_{i = 1}^{m}\phi(p_{i}^{a_{i}}) \\
            &= \prod_{i = 1}^{m}\left(p_{i}^{a_{i}}\left(1 - \frac{1}{p_{i}}\right)\right) \\
            &= \left(\prod_{i = 1}^{m}p_{i}^{a_{i}}\right)\prod_{i = 1}^{m}\left(1 - \frac{1}{p_{i}}\right) \\
            &= n \prod_{i = 1}^{m}\left(1 - \frac{1}{p_{i}}\right)
\end{aligned}
$$

を得る.
$\blacksquare$

## 定理 9.5 (オイラーの定理)

$a$ を整数, $n$ を正整数とする.
$a$ と $n$ が互いに素であるとき,

$$a^{\phi(n)} \equiv 1 \pmod{n} \tag{9.8}$$

が成り立つ.

### 証明

$A$ を $n$ 未満の非負整数で $n$ と互いに素であるものの集合とする:

$$A = \{x \in \mathbb{Z} \mid 0 \le x < n, \, \gcd(x, n) = 1\}.$$

このとき補題 9.1 より $\phi(n) = |A|$ が成り立つ.
また $A$ から $\mathbb{Z}$ への函数 $f$ を

$$f: x \longmapsto ax \bmod n$$

によって定義する.
このとき $f$ は $A$ から $A$ への全単射となる (本証明の後半で示す).

さて $f$ が $A$ から $A$ への全単射であることから,
$A$ のすべての元の積 $\displaystyle \prod_{x \in A}x$ は
$\displaystyle \prod_{x \in A}f(x) = \prod_{x \in A}(ax \bmod n)$ と一致する.
よって

$$\left(\prod_{x \in A}(ax \bmod n)\right) \bmod n = \left(\prod_{x \in A}x\right) \bmod n$$

が成り立つ.
ここで定理 1.10 (2) より,
左辺は $\displaystyle \left(\prod_{x \in A}ax\right) \bmod n$ となる.
さらに $\phi(n) = |A|$ であるから,
これは $\displaystyle \left(a^{\phi(n)}\prod_{x \in A}x\right) \bmod n$ に等しい.
よって

$$\left(a^{\phi(n)}\prod_{x \in A}x\right) \bmod n = \left(\prod_{x \in A}x\right) \bmod n$$

が成り立つ.
故に定理 6.7 より

$$a^{\phi(n)}\prod_{x \in A}x \equiv \prod_{x \in A}x \pmod{n}$$

が成り立つ.
いま $A$ の元はいずれも $n$ と互いに素であるから, 定理 3.20 を繰り返し適用することにより
$\displaystyle \prod_{x \in A}x$ も $n$ と互いに素であることがわかる.
よって定理 6.5 により $a^{\phi(n)} \equiv 1 \pmod{n}$ が成り立つ.

#### $f$ が $A$ から $A$ への全単射であることの証明

$x \in A$ とすると, 剰余の定義から

$$0 \le f(x) = ax \bmod n < n \tag{9.9}$$

である.
また仮定より $\gcd(a, n) = 1$ であり, $x \in A$ より $\gcd(x, n) = 1$ だから,
定理 3.20 より $\gcd(ax, n) = 1$ が成り立つ.
よって定理 3.19 系より

$$\gcd(f(x), n) = \gcd(ax \bmod n, n) = 1 \tag{9.10}$$

が成り立つ.
$(9.9)$, $(9.10)$ から $f(x) \in A$.
したがって $f$ は $A$ から $A$ への写像である.

また $x, y \in A$ に対して $f(x) = f(y)$ であるとすると,
$ax \bmod n = ay \bmod n$ だから定理 6.7 より $ax \equiv ay \pmod{n}$.
ここで $a$ と $n$ は互いに素だから, 定理 6.5 より $x \equiv y \pmod{n}$ となる.
故に再び定理 6.7 より

$$x \bmod n = y \bmod n$$

となるが, $x, y \in A$ より $0 \le x, y < n$ だから剰余の定義より
$x \bmod n = x, \, y \bmod n = y$ である.
故に $x = y$.
したがって $f$ は $A$ から $A$ への単射である.

また $x \in A$ とする.
いま $a$ と $n$ が互いに素であることから, 定理 3.11 系 1 より

$$ab + nc = 1 \tag{9.11}$$

をみたす整数 $b, c$ が存在する.
そこでいま $y = bx \bmod n$ とおく.
このとき剰余の定義から

$$0 \le y < n \tag{9.12}$$

である.
また $x \in A$ より $\gcd(x, n) = 1$ であり,
$(9.11)$ と定理 3.11 系 1 より $\gcd(b, n) = 1$ だから,
定理 3.20 より $\gcd(bx, n) = 1$ となる.
よって定理 3.19 系より

$$\gcd(y, n) = \gcd(bx \bmod n, n) = 1 \tag{9.13}$$

となる.
$(9.12)$, $(9.13)$ から $y \in A$ である.
そこでこの $y$ に対して $f(y)$ を求めると, 定理 1.6 (2), 1.9 より

$$
\begin{aligned}
    f(y) &= ay \bmod n \\
         &= (a \bmod n)(y \bmod n) \bmod n \\
         &= (a \bmod n)((bx \bmod n) \bmod n) \bmod n \\
         &= (a \bmod n)(bx \bmod n) \bmod n \\
         &= abx \bmod n \tag{9.14}
\end{aligned}
$$

となる.
ここで $(9.11)$ より $abx = (1 - nc)x = x - ncx$ だから,
定理 1.5 より

$$
    abx \bmod n = (x - ncx) \bmod n = x \bmod n \tag{9.15}
$$

が成り立つ.
また $x \in A$ より $0 \le x < n$ だから, 剰余の定義より

$$x \bmod n = x \tag{9.16}$$

である.
$(9.14)$, $(9.15)$, $(9.16)$ から, $f(y) = x$ を得る.
したがって $f$ は $A$ から $A$ への全射である.

以上で $f$ が $A$ から $A$ への全単射であることが示された.
$\blacksquare$

## 定理 9.6

$n$ を正整数とするとき,

$$\sum_{d \mid n}\phi(d) = n \tag{9.17}$$

が成り立つ.
ただし和は $n$ のすべての正の約数 $d$ についてとるものとする.

### 証明

正整数 $d$ に対し, $A_{d}$ を $1$ 以上 $d$ 以下の整数で $d$ と互いに素であるものの集合とする:

$$A_{d} = \{x \in \mathbb{Z} \mid 1 \le x \le d, \, \gcd(x, d) = 1\}.$$

このとき $\phi(d)$ の定義より $\phi(d) = |A_{d}|$ である.
また正整数 $d$ に対して集合 $T_{d}$ を

$$T_{d} = \{(d, x) \mid x \in A_{d}\}$$

と定義し, 集合 $T$ を

$$T = \bigcup_{d \mid n}T_{d}$$

と定義する.
ただしこの和集合は $n$ のすべての正の約数 $d$ についてとる.
このとき明らかに

$$|T_{d}| = |A_{d}| = \phi(d) \tag{9.18}$$

が成り立つ.
また $n$ の相異なる正の約数 $d, d'$ に対し, 明らかに $T_{d} \cap T_{d'}$ が空集合となる
(それぞれの要素の第一成分が $d, d'$ で異なるため) から,

$$|T| = \sum_{d \mid n}|T_{d}| \tag{9.19}$$

が成り立つ.
$(9.18)$, $(9.19)$ から

$$|T| = \sum_{d \mid n}\phi(d) \tag{9.20}$$

を得る.

さていま集合 $N$ を

$$N = \{k \in \mathbb{Z} \mid 1 \le k \le n\} = \{1, 2, \cdots, n\}$$

と定義し, $N$ から $\mathbb{Z} \times \mathbb{Z}$ への函数 $f$ を

$$f: x \longmapsto (n / \gcd(n, x), x / \gcd(n, x))$$

によって定義する ($n > 0$ だから定理 3.12 (2) より $\gcd(n, x) > 0$ であることに注意).
このとき $f$ は $N$ から $T$ への全単射となる.
これが示せれば, $|T| = |N| = n$ が成り立つから, これと $(9.20)$ から $(9.17)$ が得られる.

#### $f$ が $N$ から $T$ の中への写像であることの証明

$x \in N$ とする.
$\gcd(n, x)$ を $g$ と書くとき, 最大公約数の定義より $g$ は $n$ と $x$ の公約数だから,
$n', x' \in \mathbb{Z}$ が存在して $n = gn', \, x = gx'$ と書ける.
故に定理 1.4 から

$$f(x) = (n / g, x / g) = (gn' / g, gx' / g) = (n', x')$$

となる.
ここで $n, g > 0$ より $n'$ は $n$ の正の約数だから,

$$T_{n'} \subset \bigcup_{d \mid n}T_{d} = T$$

である.
また $x \in N$ より $1 \le x \le n$ だから, $1 \le gx' \le gn'$.
よって $g > 0$ より

$$1 \le x' \le n'. \tag{9.21}$$

また $g > 0$ だから, 定理 3.16 より

$$g = \gcd(n, x) = \gcd(gn', gx') = g\gcd(n', x')$$

が成り立つ.
よってこの最左辺と最右辺を $g$ で割って

$$\gcd(n', x') = 1 \tag{9.22}$$

を得る.
$(9.21)$, $(9.22)$ より $x' \in A_{n'}$.
故に

$$f(x) = (n', x') \in T_{n'} \subset T$$

となる.
したがって $f$ は $N$ から $T$ の中への写像である.

#### $f$ が全射であることの証明
$\displaystyle t \in T = \bigcup_{d \mid n}T_{d}$ とすると,
$n$ の正の約数 $d$ が存在して $t \in T_{d}$ となるから,
$x \in A_{d}$ が存在して $t = (d, x)$ となる.
いま $d \mid n$ より $n' \in \mathbb{Z}$ が存在して $n = n'd$ と書けるが,
$n, d > 0$ だから $n' > 0$.
また $x \in A_{d}$ より $1 \le x \le d$ だから,
$1 \le n' \le n'x \le n'd = n$ である.
よって $n'x \in N$.
また $x \in A_{d}$ より $\gcd(x, d) = 1$ だから,
定理3.16 より

$$\gcd(n, n'x) = \gcd(n'd, n'x) = n'\gcd(d, x) = n'$$

となる.
故に定理 1.4 より

$$f(n'x) = (n / \gcd(n, n'x), n'x / \gcd(n, n'x)) = (n'd / n', n'x / n') = (d, x) = t$$

となる.
したがって $f$ は $N$ から $T$ への全射である.

#### $f$ が単射であることの証明

$x, y \in N$ に対して $f(x) = f(y)$ であるとすると, $f$ の定義より

$$
\begin{aligned}
    &n / \gcd(n, x) = n / \gcd(n, y), \\
    &x / \gcd(n, x) = y / \gcd(n, y) \tag{9.23}
\end{aligned}
$$

である.
最大公約数の定義より $\gcd(n, x), \gcd(n, y)$ はそれぞれ $n$ と $x$, $n$ と $y$ の公約数だから,
$n', n'', x', y' \in \mathbb{Z}$ が存在して

$$
\begin{aligned}
    &n = \gcd(n, x)n' = \gcd(n, y)n'', \\
    &x = \gcd(n, x)x', \\
    &y = \gcd(n, y)y'
\end{aligned}
$$

となる.
よって定理 1.4 より

$$
\begin{aligned}
    &n' = \gcd(n, x)n' / \gcd(n, x) = n / \gcd(n, x), \\
    &n'' = \gcd(n, y)n'' / \gcd(n, y) = n / \gcd(n, y), \\
    &x' = \gcd(n, x)x' / \gcd(n, x) = x / \gcd(n, x), \\
    &y' = \gcd(n, y)y' / \gcd(n, y) = y / \gcd(n, y) \tag{9.24}
\end{aligned}
$$

が成り立つ.
$(9.23)$, $(9.24)$ より

$$n' = n'', \tag{9.25}$$

$$x' = y' \tag{9.26}$$

が成り立つ.
$(9.25)$ から, 定理 1.4 より

$$\gcd(n, x) = \gcd(n, x)n' / n' = n / n' = n / n'' = \gcd(n, y)n'' / n'' = \gcd(n, y)$$

となる ($n > 0$ だから $n', n'' \neq 0$ であることに注意) から, これと $(9.26)$ から

$$x = \gcd(n, x)x' = \gcd(n, y)y' = y$$

を得る.
したがって $f$ は $N$ から $T$ への単射である.
$\blacksquare$
