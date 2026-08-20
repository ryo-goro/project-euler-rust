# 08. 中国剰余定理

## 概要

中国剰余定理を示す.

---

## 補題 8.1

$a, b, m$ を整数とし, $a \equiv b \pmod{m}$ であるとする.
このとき $m \mid n, \, n \neq 0$ なる任意の $n \in \mathbb{Z}$ に対して

$$a \bmod n \equiv b \pmod{m} \tag{8.1}$$

が成り立つ.

### 証明

仮定より $k \in \mathbb{Z}$ が存在して $a - b = mk$ と書ける.
また $q = a / n, \, r = a \bmod n$ とおけば $q, r \in \mathbb{Z}$ であり,
$a = nq + r$ となる.
よって

$$r - b = a - nq - b = a - b - nq = mk - nq$$

となる.
ここで $m \mid n$ だから, 定理 2.4, 2.7 (1), (2) により
$m \mid (mk - nq)$ が成り立つ.
故に $m \mid (r - b)$, 即ち $a \bmod n \equiv b \pmod{m}$ が成り立つ.
$\blacksquare$

## 定理 8.2

$a, b$ を整数とする.
また $m$ と $n$ を互いに素な正整数とする.
このとき

$$
\begin{aligned}
    &x \equiv a \pmod{m}, \\
    &x \equiv b \pmod{n}
\end{aligned}
$$

をともにみたす整数 $x$ で, $0 \le x < mn$ となるものがただ一つ存在する. 

### 証明

$m$ と $n$ が互いに素であることから,
定理 3.11 系 1 により

$$mu + nv = 1 \tag{8.2}$$

をみたす整数 $u, v$ が存在する.
ここで

$$x_{0} = bmu + anv$$

とおけば $x_{0} \in \mathbb{Z}$ であり, $(8.2)$ より

$$x_{0} - a = bmu + a(nv - 1) = bmu - amu = (b - a)u \cdot m$$

となる.
よって

$$x_{0} \equiv a \pmod{m} \tag{8.3}$$

が成り立つ.
同様に, $(8.2)$ より

$$x_{0} - b = b(mu - 1) + anv = -bnv + anv = (a - b)v \cdot n$$

だから,

$$x_{0} \equiv b \pmod{n} \tag{8.4}$$

も成り立つ.
そこで

$$x = x_{0} \bmod (mn)$$

と置けば, $x \in \mathbb{Z}, \, 0 \le x < mn$ であり, $(8.3)$, $(8.4)$ と補題 8.1 により
$x \equiv a \pmod{m}$ と $x \equiv b \pmod{n}$ が共に成り立つ.

いま $y \in \mathbb{Z}$ も
$0 \le y < mn, \, y \equiv a \pmod{m}, \, y \equiv b \pmod{n}$ をみたすとすれば,
定理 6.1 (2), (3) により $x \equiv y \pmod{m}, \, x \equiv y \pmod{n}$ が成り立つ.
これと $m, n$ が互いに素であることから, 定理 6.8 系 (2) により
$x \equiv y \pmod{mn}$ が成り立つ.
故に定理 6.7 より $x \bmod (mn) = y \bmod (mn)$ となるが,
$0 \le x, y < mn$ より $x \bmod (mn) = x, \, y \bmod(mn) = y$ だから,
$x = y$ である.
$\blacksquare$

## 定理 8.3 (中国剰余定理)

$a_{1}, a_{2}, \cdots, a_{n}$ を $n$ 個の整数とする.
また $m_{1}, m_{2}, \cdots, m_{n}$ を $n$ 個の正整数とし,
これらのうちどの二つも互いに素
($1 \le i, j \le n, \, i \neq j \implies \gcd(m_{i}, m_{j}) = 1$)
であるとする.

このとき

$$
\begin{aligned}
    x \equiv a_{1} &\pmod{m_{1}}, \\
    x \equiv a_{2} &\pmod{m_{2}}, \\
    &\cdots \\
    x \equiv a_{n} &\pmod{m_{n}} \tag{8.5}
\end{aligned}
$$

をみたす整数 $x$ で, $0 \le x < m_{1}m_{2} \cdots m_{n}$ となるものがただ一つ存在する.

### 証明

$x_{1} = a_{1} \bmod m_{1}$ と置けば $0 \le x_{1} < m_{1}$ である.
またこれより $x_{1} \bmod m_{1} = x_{1} = a_{1} \bmod m_{1}$ となる (剰余の一意性) から,
定理 6.7 より $x_{1} \equiv a_{1} \pmod{m_{1}}$ が成り立つ.

いま $k$ を $1 \le k < n$ なる整数とし,

$$
\begin{aligned}
    x_{k} \equiv a_{1} &\pmod{m_{1}}, \\
    x_{k} \equiv a_{2} &\pmod{m_{2}}, \\
    &\cdots \\
    x_{k} \equiv a_{k} &\pmod{m_{k}} \tag{8.6}
\end{aligned}
$$

をみたす整数 $x_{k}$ で, $0 \le x_{k} < m_{1}m_{2} \cdots m_{k}$ となるものが存在するとする.
このとき仮定より $m_{k + 1}$ は $m_{1}, m_{2}, \cdots, m_{k}$ のどの一つとも互いに素となるから,
定理 3.20 を繰り返し適用することにより, 
$m_{k + 1}$ が $m_{1}m_{2} \cdots m_{k}$ と互いに素となることがわかる.
よって定理 8.2 により,

$$
\begin{align}
    &x_{k + 1} \equiv x_{k} \pmod{m_{1}m_{2} \cdots m_{k}}, \tag{8.7}\\
    &x_{k + 1} \equiv a_{k + 1} \pmod{m_{k + 1}} \tag{8.8}
\end{align}
$$

を共にみたす整数 $x_{k + 1}$ で,
$0 \le x_{k + 1} < m_{1}m_{2} \cdots m_{k}m_{k + 1}$ となるものが存在する.
$(8.7)$ から, $i = 1, 2, \cdots, k$ に対して定理 6.8 系 (1) より
$x_{k + 1} \equiv x_{k} \pmod{m_{i}}$ が成り立つから,
これと $(8.6)$ から定理 6.1 (3) により $x_{k + 1} \equiv a_{i} \pmod{m_{i}}$ が成り立つ.
このことと $(8.8)$ から, $x_{k + 1}$ は

$$
\begin{aligned}
    x_{k + 1} \equiv a_{1} &\pmod{m_{1}}, \\
    x_{k + 1} \equiv a_{2} &\pmod{m_{2}}, \\
    &\cdots \\
    x_{k + 1} \equiv a_{k} &\pmod{m_{k}}, \\
    x_{k + 1} \equiv a_{k + 1} &\pmod{m_{k + 1}}
\end{aligned}
$$

をすべてみたす.

以上から, 数学的帰納法により $(8.5)$ をみたす整数 $x$ で
$0 \le x < m_{1}m_{2} \cdots m_{n}$ となるものが存在することが示された.

いま $y \in \mathbb{Z}$ も $(8.5)$ (の $x$ をすべて $y$ に置き換えたもの) をみたし,
$0 \le y < m_{1}m_{2} \cdots m_{n}$ であるとする.
このとき定理 6.1 (2), (3) により

$$
\begin{aligned}
    x \equiv y &\pmod{m_{1}}, \\
    x \equiv y &\pmod{m_{2}}, \\
    &\cdots \\
    x \equiv y &\pmod{m_{n}} \tag{8.9}
\end{aligned}
$$

が成り立つ.
いま $m_{1}, m_{2}, \cdots, m_{n}$ はどの二つも互いに素だから,
定理 3.20 を繰り返し適用することにより,
$k = 2, 3, \cdots, n$ に対して $m_{k}$ が
$m_{1}m_{2} \cdots m_{k - 1}$ と互いに素となることがわかる.
このことと $(8.9)$ から, 定理 6.8 系 (2) を繰り返し適用することにより,

$$x \equiv y \pmod{m_{1}m_{2} \cdots m_{n}}$$

が成り立つことがわかる.
故に定理 6.7 より

$$x \bmod (m_{1}m_{2} \cdots m_{n}) = y \bmod (m_{1}m_{2} \cdots m_{n})$$

となるが,
$0 \le x, y < m_{1}m_{2} \cdots m_{n}$ より
$x \bmod (m_{1}m_{2} \cdots m_{n}) = x, \, y \bmod(m_{1}m_{2} \cdots m_{n}) = y$
だから $x = y$ である.
これで $x$ の一意性についても示された.
$\blacksquare$
