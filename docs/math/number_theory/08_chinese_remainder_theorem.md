# 08. 中国剰余定理

## 概要

中国剰余定理を示す.

---

## 補題 8.1

$a, b, m$ を整数とし, $a \equiv b \pmod{m}$ であるとする.
このとき $m \mid n, \, n \neq 0$ なる任意の $n \in \mathbb{Z}$ に対して

$$(a \bmod n) \equiv b \pmod{m} \tag{8.1}$$

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
故に $m \mid (r - b)$, 即ち $(a \bmod n) \equiv b \pmod{m}$ が成り立つ.
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
これと $m, n$ が互いに素であることから, 定理 6.8 系により
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
    x \equiv a_{n} &\pmod{m_{n}}
\end{aligned}
$$

をみたす整数 $x$ で, $0 \le x < m_{1}m_{2} \cdots m_{n}$ となるものがただ一つ存在する.

### 証明

まずこのような $x$ の存在について, $n$ に関する数学的帰納法によって証明する.

$n = 1$ のとき, $x = a_{1} \bmod m_{1}$ と置けば $0 \le x < m_{1}$ である.
また定理 2.4 より $m_{1} \mid m_{1}$ であり,
定理 6.1 (1) より $a_{1} \equiv a_{1} \pmod{m_{1}}$ だから,
補題 8.1 より $x \equiv a_{1} \pmod{m_{1}}$ が成り立つ.

いま $k$ を正整数とし, $n = k$ のときには本定理をみたすような $x$ が存在するとする.
そして $a_{1}, \cdots, a_{k}, a_{k + 1}$ を整数,
$m_{1}, \cdots, m_{k}, m_{k + 1}$ を正整数とし,
これらのうちどの二つも互いに素
($1 \le i, j \le k + 1, \, i \neq j \implies \gcd(m_{i}, m_{j}) = 1$)
であるとする.
このとき仮定より,

$$
\begin{aligned}
    x' \equiv a_{1} &\pmod{m_{1}}, \\
    &\cdots \\
    x' \equiv a_{k} &\pmod{m_{k}}
\end{aligned}
$$

をみたす整数 $x'$ で, $0 \le x' < m_{1} \cdots m_{k}$ となるもの存在する.



$\blacksquare$
