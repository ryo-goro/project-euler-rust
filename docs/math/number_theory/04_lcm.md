# 04. 最小公倍数

## 概要

最小公倍数の定義と性質について述べる.

---

## 定義 4.1 (公倍数)

$a_{1}, a_{2}, \cdots, a_{n}$ を $n$ 個の整数とする.
整数 $m$ がすべての $i = 1, 2, \cdots, n$ に対して
$a_{i} \mid m$ をみたすとき（即ち $m$ が $a_{1}, a_{2}, \cdots, a_{n}$ すべての倍数であるとき）,
$m$ を $a_{1}, a_{2}, \cdots, a_{n}$ の**公倍数**という.

## 定理 4.1

$a_{1}, a_{2}, \cdots, a_{n}$ を $n$ 個の整数とするとき, 以下が成り立つ.

(1) $m$ が $a_{1}, a_{2}, \cdots, a_{n}$ の公倍数ならば,
$-m$ は $a_{1}, a_{2}, \cdots, a_{n}$ の公倍数である.

(2) $m$ が $a_{1}, a_{2}, \cdots, a_{n}$ の公倍数である
$\iff$ $m$ が $|a_{1}|, |a_{2}|, \cdots, |a_{n}|$ の公倍数である.

### 証明

定理 2.2 によって明らか.
$\blacksquare$

## 定理 4.2

$a_{1}, a_{2}, \cdots, a_{n}$ を $n$ 個の整数とするとき, 以下が成り立つ.

(1) $0$ は $a_{1}, a_{2}, \cdots, a_{n}$ の公倍数である.

(2) $a_{1}, a_{2}, \cdots, a_{n}$ の少なくとも一つが $0$
$\iff$ $a_{1}, a_{2}, \cdots, a_{n}$ の公倍数は $0$ のみである.

(3) $1$ が $a_{1}, a_{2}, \cdots, a_{n}$ の公倍数となるのは,
$|a_{1}|, |a_{2}|, \cdots, |a_{n}|$ がすべて $1$ である場合に限る.

(4) $b \in \mathbb{Z}$ が $a_{1}, a_{2}, \cdots, a_{n}$ の公倍数である
$\iff$ $b$ が $a_{1}, a_{2}, \cdots, a_{n}, 1$ の公倍数である.

### 証明

(1), (3), (4) はいずれも定理 2.3 によって明らかであるから, (2) のみ証明を述べる.

(1) より $0$ は $a_{1}, a_{2}, \cdots, a_{n}$ の公倍数であるが,
$a_{1}, a_{2}, \cdots, a_{n}$ の少なくとも一つが $0$ ならば,
$a_{1}, a_{2}, \cdots, a_{n}$ の任意の公倍数は $0$ の倍数となり,
定理 2.3 (2) よりそれは $0$ である.
故にこのときは $a_{1}, a_{2}, \cdots, a_{n}$ の公倍数は $0$ のみである.

逆に $a_{1}, a_{2}, \cdots, a_{n}$ の公倍数が $0$ のみであるとき,
$a_{1}a_{2} \cdots a_{n}$ は $a_{1}, a_{2}, \cdots, a_{n}$ の公倍数だから,
$a_{1}a_{2} \cdots a_{n} = 0$.
したがって $a_{1}, a_{2}, \cdots, a_{n}$ の少なくとも一つは $0$ である.
$\blacksquare$

## 定理 4.3

$a_{1}, a_{2}, \cdots, a_{n}$ を $n$ 個の整数とするとき,
以下をすべてみたす整数 $l$ がただ一つ存在する.

(1) $l \ge 0$.

(2) $l$ は $a_{1}, a_{2}, \cdots, a_{n}$ の公倍数である.

(3) $a_{1}, a_{2}, \cdots, a_{n}$ の任意の公倍数は $l$ の倍数である.

### 証明

定理 3.5 より $a_{i}\mathbb{Z} \; (i = 1, 2, \cdots, n)$ はイデアルだから,
定理 3.7 を繰り返し適用することにより,
$a_{1}\mathbb{Z} \cap a_{2}\mathbb{Z} \cap \cdots \cap a_{n}\mathbb{Z}$ がイデアルであることがわかる.
よって定理 3.6 より, 整数 $l \ge 0$ が存在して

$$a_{1}\mathbb{Z} \cap a_{2}\mathbb{Z} \cap \cdots \cap a_{n}\mathbb{Z} = l\mathbb{Z} \tag{4.1}$$

が成り立つ.
$l \in l\mathbb{Z}$ だから, $(4.1)$ より
$l \in a_{i}\mathbb{Z} \; (i = 1, 2, \cdots, n)$.
よって $l$ は $a_{1}, a_{2}, \cdots, a_{n}$ の公倍数である.
また $m$ を $a_{1}, a_{2}, \cdots, a_{n}$ の任意の公倍数とするとき,
$m \in a_{1}\mathbb{Z} \cap a_{2}\mathbb{Z} \cap \cdots \cap a_{n}\mathbb{Z}$ だから
$(4.1)$ より $m \in l\mathbb{Z}$.
よって $m$ は $l$ の倍数である.
以上より, $l$ は (1), (2), (3) をすべてみたす.

いま $l' \in mathbb{Z}$ も (1), (2), (3)
(の $l$ を $l'$ に置き換えたもの) をすべてみたすとすれば,
(2), (3) より $l \mid l'$ かつ $l' \mid l$ となる.
よって定理 2.6 より $|l| = |l'|$ となるが, 1より $l, l' \ge 0$ だから $l = l'$ である.
$\blacksquare$

## 定義 4.2 (最小公倍数)

$a_{1}, a_{2}, \cdots, a_{n}$ を $n$ 個の整数とするとき,
定理 4.3 をみたすただ一つの整数 $l$ を $a_{1}, a_{2}, \cdots, a_{n}$ の**最小公倍数**といい,

$$\operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n})$$

と表す.

## 定理 4.4

$a_{1}, a_{2}, \cdots, a_{n}, m$ を整数とするとき, 次の (a) と (b) は同値である:

(a) $m$ は $a_{1}, a_{2}, \cdots, a_{n}$ の公倍数である.

(b) $m$ は $\operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n})$ の倍数である.

### 証明

以下見やすさのため, $\operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n})$ を $l$ と書く.

#### (a) $\implies$ (b)

最小公倍数の定義より明らか (定理4.3 (3)).

#### (b) $\implies$ (a)

最小公倍数の定義 (定理4.3 (2)) から, $a_{i} \mid l \; (i = 1, 2, \cdots, n)$.
故に $l \mid m$ ならば, 定理 2.5 より $a_{i} \mid m \; (i = 1, 2, \cdots, n)$ となる.
$\blacksquare$

## 定理 4.5

$a_{1}, a_{2}, \cdots, a_{n}$ を $n$ 個の整数とするとき, 以下が成り立つ.

(1) $\operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n}) = 0$
$\iff$ $a_{1}, a_{2}, \cdots, a_{n}$ のうち少なくとも一つは $0$.

(2) $\operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n}) > 0$
$\iff$ $a_{1}, a_{2}, \cdots, a_{n}$ のいずれも $0$ でない.

(3) $a_{1}, a_{2}, \cdots, a_{n}$ の $0$ でない任意の公倍数 $m$ に対して

$$\operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n}) \le |m|.$$

即ち, $a_{1}, a_{2}, \cdots, a_{n}$ のいずれも $0$ でないときは,
$\operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n})$ は
$a_{1}, a_{2}, \cdots, a_{n}$ の正の公倍数のうちで最小である.

### 証明

以下見やすさのため, $\operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n})$ を $l$ と書く.

#### (1)

$m$ を $a_{1}, a_{2}, \cdots, a_{n}$ の任意の公倍数とすると,
最小公倍数の定義 (定理 4.3 (3)) より $m$ は $l$ の倍数である.
そこで $l = 0$ ならば, 定理 2.3 (2) より $m = 0$.
故に定理 4.2 (2) より, $a_{1}, a_{2}, \cdots, a_{n}$ のうち少なくとも一つは $0$ である.

逆に $a_{1}, a_{2}, \cdots, a_{n}$ のうち少なくとも一つが $0$ であるとすると,
最小公倍数の定義 (定理 4.3 (2)) より $l$ は $0$ の倍数となるから,
定理 2.3 (2) より $l = 0$ となる.

#### (2)

$l \ge 0$ だから (1) によって明らか.

#### (3)

$a_{1}, a_{2}, \cdots, a_{n}$ の $0$ でない任意の公倍数 $m$ に対し,
最小公倍数の定義 (定理 4.3 (3)) より $l \mid m$ だから,
$k \in \mathbb{Z}$ が存在して $kl = m$ となる.
ここで $m \neq 0$ より $k \neq 0$.
よって $|k| \ge 1$ で, 

$$l = |l| \le |k||l| = |kl| = |m|$$

となる.
$\blacksquare$

## 定理 4.6

$a_{1}, a_{2}, \cdots, a_{n}$ を $n$ 個の整数とするとき, 次の (a) と (b) は同値である.

(a) $|a_{1}| = |a_{2}| = \cdots = |a_{n}| = 1$.

(b) $\operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n}) = 1$.

### 証明

以下見やすさのため, $\operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n})$ を $l$ と書く.

#### (a) $\implies$ (b)

$1 > 0$ である.
また (a) が成り立つとき, 定理 4.2 (3) より $1$ は $a_{1}, a_{2}, \cdots, a_{n}$ の公倍数となる.
また $a_{1}, a_{2}, \cdots, a_{n}$ の任意の公倍数 $m$ に対して,
定理 2.3 (3) より $1 \mid m$ である.
よって最小公倍数の定義から, $l = 1$ が成り立つ.

#### (b) $\implies$ (a)

最小公倍数の定義より $l$ は $a_{1}, a_{2}, \cdots, a_{n}$ の公倍数だから,
$l = 1$ ならば定理 4.2 (3) により (a) が成り立つ.
$\blacksquare$

## 定理 4.7

$a_{1}, a_{2}, \cdots, a_{n}$ を $n$ 個の整数とするとき,

$$
    \operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n})
    = \operatorname{lcm}(|a_{1}|, |a_{2}|, \cdots, |a_{n}|)
$$

が成り立つ.

### 証明

以下見やすさのため, $\operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n})$ を $l$,
$\operatorname{lcm}(|a_{1}|, |a_{2}|, \cdots, |a_{n}|)$ を $l'$ と書く.

まず $l' \ge 0$ である.
また $l'$ は $|a_{1}|, |a_{2}|, \cdots, |a_{n}|$ の公倍数だから,
定理 4.1 (2) より $l'$ は $a_{1}, a_{2}, \cdots, a_{n}$ の公倍数である.
また $m$ を $a_{1}, a_{2}, \cdots, a_{n}$ の任意の公倍数とするとき,
定理 4.1 (2) より $m$ は $|a_{1}|, |a_{2}|, \cdots, |a_{n}|$ の公倍数となるから,
$m$ は $l'$ の倍数となる.
故に, 最小公倍数の定義から $l' = l$ が成り立つ.
$\blacksquare$

## 定理 4.8

$a_{1}, a_{2}, \cdots, a_{n}$ を $n$ 個の整数とし,
$i$ を $1 \le i \le n$ なる整数とする.
このとき次の (a), (b), (c) は同値である.

(a) $a_{i}$ は $a_{1}, a_{2}, \cdots, a_{n}$ の公倍数である.

(b) $j \neq i, \, 1 \le j \le n$ なる任意の整数 $j$ に対して $a_{j} \mid a_{i}$.

(c) $\operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n}) = |a_{i}|$.

### 証明

#### (a) $\iff$ (b)

定理 2.4 より $a_{i} \mid a_{i}$ だから明らか.

#### (a) $\implies$ (c)

まず $|a_{i}| \ge 0$ である.

また (a) が成り立つとき, 定理 4.1 (1) より
$|a_{i}|$ は $a_{1}, a_{2}, \cdots, a_{n}$ の公倍数となる.

また $m$ を $a_{1}, a_{2}, \cdots, a_{n}$ の任意の公倍数とするとき,
特に $m$ は $a_{i}$ の倍数だから, 定理 2.2 より $m$ は $|a_{i}|$ の倍数となる.

以上から, 最小公倍数の定義より (c) が成り立つ.

#### (c) $\implies$ (a)

最小公倍数の定義より, $\operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n})$ は
$a_{1}, a_{2}, \cdots, a_{n}$ の公倍数だから,
(c) が成り立つとき $|a_{i}|$ は $a_{1}, a_{2}, \cdots, a_{n}$ の公倍数である.
よって定理 4.1 (1) より, $a_{i}$ は $a_{1}, a_{2}, \cdots, a_{n}$ の公倍数となる.
$\blacksquare$

## 定理 4.8 系

$a_{1}, a_{2}, \cdots, a_{n}$ を $n$ 個の整数とし,
$|a_{1}| = |a_{2}| = \cdots = |a_{n}|$ であるとする.
このとき

$$\operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n}) = |a_{1}|$$

が成り立つ.

### 証明

このとき定理 2.2, 2.4 からわかるように
$a_{1}$ は $a_{1}, a_{2}, \cdots, a_{n}$ の公倍数となるから,
定理 4.8 より $\operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n}) = |a_{1}|$ となる.
$\blacksquare$

## 定理 4.9
$a_{1}, a_{2}, \cdots, a_{n}, b \in \mathbb{Z}$ とするとき,

$$
    \operatorname{lcm}(ba_{1}, ba_{2}, \cdots, ba_{n})
    = |b|\operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n}) \tag{4.2}
$$

が成り立つ.

### 証明

$b = 0$ ならば定理 4.5 (1) より $(4.2)$ の両辺 $= 0$ となって成り立つから, 以下 $b \neq 0$ とする.
また以下見やすさのため, $\operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n})$ を $l$ と書く.

まず $l \ge 0$ だから, $|b|l \ge 0$ である.

また $l$ は $a_{1}, a_{2}, \cdots, a_{n}$ の公倍数だから,
定理 2.2, 2.7 (2) より $|b|l$ は $ba_{1}, ba_{2}, \cdots, ba_{n}$ の公倍数となる.

また $m \in \mathbb{Z}$ を $ba_{1}, ba_{2}, \cdots, ba_{n}$ の任意の公倍数とすれば,
$ba_{i} \mid m \; (i = 1, 2, \cdots, n)$ だから, $k_{i} \in \mathbb{Z}$ が存在して

$$m = k_{i}ba_{i} \tag{4.3}$$

となる ($i = 1, 2, \cdots, n$).
また定理 2.7 (4) より $b \mid m$ だから, $k \in \mathbb{Z}$ が存在して

$$m = kb \tag{4.4}$$

となる.
$(4.3)$, $(4.4)$ と $b \neq 0$ より $k_{i}a_{i} = k \; (i = 1, 2, \cdots, n)$ となるから,
$k$ は $a_{1}, a_{2}, \cdots, a_{n}$ の公倍数である.
よって最小公倍数の定義 (定理 4.3 (3)) より $l \mid k$.
よって定理 2.7 (2) より $bl \mid bk$, 即ち $bk = m$ は $bl$ の倍数となるから,
定理 2.2 より $m$ は $|b|l$ の倍数となる.

以上から, 最小公倍数の定義より
$\operatorname{lcm}(ba_{1}, ba_{2}, \cdots, ba_{n}) = |b|l$ である.
$\blacksquare$

## 定理 4.10
$a_{1}, a_{2}, \cdots, a_{n}, b \in \mathbb{Z}$ とするとき,

$$
    \operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n}, b)
    = \operatorname{lcm}(\operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n}), b)
$$

が成り立つ.

### 証明

以下見やすさのため, $\operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n})$ を $l$,
$\operatorname{lcm}(l, b)$ を $l'$ と書く.

まず $l' \ge 0$ である.

また $l'$ は $l$ と $b$ の公倍数だから, 定理 4.4 より
$l'$ は $a_{1}, a_{2}, \cdots, a_{n}, b$ の公倍数である.

また $m \in \mathbb{Z}$ を $a_{1}, a_{2}, \cdots, a_{n}, b$ の任意の公倍数とするとき,
最小公倍数の定義 (定理 4.3 (3)) より $m$ は $l$ の倍数だから, $m$ は $l$ と $b$ の公倍数となり,
再び最小公倍数の定義 (定理 4.3 (3)) より, $m$ は $l'$ の倍数である.

以上から, 最小公倍数の定義より
$l' = \operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n}, b)$ である.
$\blacksquare$

---

定理 4.10 によって, n個の整数の最小公倍数を求める問題は結局2個の整数の最小公倍数を求める問題に帰着する.

1個または2個の整数の最小公倍数を求める具体的な方法は, 次の定理 4.11, 12 による.

## 定理 4.11

$a$ を整数とするとき, 以下が成り立つ.

(1) $\operatorname{lcm}(a) = |a|$.

(2) $\operatorname{lcm}(a, 1) = |a|$.

### 証明

#### (1)

定理 4.8 系における $n = 1$ の場合である.

#### (2)

定理 2.3 (3) より $1 \mid a$ だから,
定理 4.8 により $\operatorname{lcm}(a, 1) = |a|$ が成り立つ.
$\blacksquare$

## 定理 4.11 系

$a_{1}, a_{2}, \cdots, a_{n} \in \mathbb{Z}$ とするとき,

$$
    \operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n}, 1)
    = \operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n})
$$

が成り立つ.

### 証明

定理 4.10, 4.11 (2) と $\operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n}) \ge 0$ より

$$
\begin{aligned}
    \operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n}, 1)
    &= \operatorname{lcm}(\operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n}), 1) \\
    &= |\operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n})| \\
    &= \operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n})
\end{aligned}
$$

となる.
$\blacksquare$

## 定理 4.12

$a, b$ を整数とするとき,

$$|ab| = \gcd(a, b)\operatorname{lcm}(a, b)$$

が成り立つ.

### 証明

以下見やすさのため, $\gcd(a, b)$ を $g$, $\operatorname{lcm}(a, b)$ を $l$ と書く.

$a = 0$ または $b = 0$ のときは定理4.5.1より $l = 0$ だから, $|ab| = gl = 0$ が成り立つ.

そこで以下では $a \neq 0, \, b \neq 0$ とする.
このとき定理 3.12 (2), 4.5 (2) より $g, l > 0$ である.
いま $g$ は $a$ と $b$ の公約数であるから, $a', b' \in \mathbb{Z}$ が存在して

$$a = ga', \; b = gb' \tag{4.5}$$

と書ける.
ここで

$$\gcd(a', b') = 1 \tag{4.6}$$

である.
実際 $(4.5)$ と定理 3.16 より

$$g = \gcd(ga', gb') = |g|\gcd(a', b') = g\gcd(a', b')$$

だから, 最左辺と最右辺を $g > 0$ で割って $(4.6)$ を得る.

さていま

$$g|a'b'| = l \tag{4.7}$$

が成り立つ.

実際 $g|a'b'| \ge 0$.

また $(4.5)$ と $g > 0$ より
$g|a'b'| = |ga'||b'| = |a||b'|, \, g|a'b'| = |a'||gb'| = |a'||b|$ だから, 
$g|a'b'|$ は $|a|$ と $|b|$ の公倍数で,
したがって定理 2.2 より $a$ と $b$ の公倍数である.

また $m$ を$a$ と $b$ の任意の公倍数とするとき, $x, y \in \mathbb{Z}$ が存在して

$$m = ax = by \tag{4.8}$$

と書ける.
これと $(4.5)$ より $ga'x = gb'y$ となるから, 両辺を $g > 0$ で割って $a'x = b'y$.
故に $b' \mid a'x$.
そこで $(4.6)$ と定理 3.26 系より $b' \mid x$ となる.
即ち $z \in \mathbb{Z}$ が存在して

$$x = b'z \tag{4.9}$$

となる.
$(4.5)$, $(4.8)$, $(4.9)$ から,

$$m = ax = ga'b'z$$

となるから, $m$ は $ga'b'$ の倍数である.
故に $g > 0$ と 定理 2.2 より, $m$ は $|ga'b'| = g|a'b'|$ の倍数である.

以上から, 最小公倍数の定義より $(4.7)$ が成り立つ.

$(4.5)$, $(4.7)$ から,

$$gl = g^2|a'b'| = |ga' \cdot gb'| = |ab|$$

である.
$\blacksquare$

## 定理 4.12 系

$a, b \in \mathbb{Z}$ が互いに素であるとき,

$$|ab| = \operatorname{lcm}(a, b)$$

が成り立つ.

### 証明

定理 4.12 より明らか.
$\blacksquare$

## 補題 4.13

$a, b$ を整数, $n$ を非負整数とするとき,

$$\operatorname{lcm}(a^{n}, b^{n}) = \operatorname{lcm}(a, b)^{n} \tag{4.10}$$

が成り立つ.

### 証明

定理 4.11 (2) より

$$
    \operatorname{lcm}(a^{0}, b^{0}) = \operatorname{lcm}(1, 1) = |1| = 1
    = \operatorname{lcm}(a, b)^{0}
$$

だから, $n = 0$ のときは $(4.10)$ が成り立つ.

そこで以下 $n \ge 1$ とする.
$a = b = 0$ のときは, 定理4.5.1より

$$
\begin{aligned}
    &\operatorname{lcm}(a^{n}, b^{n}) = \operatorname{lcm}(0^{n}, 0^{n}) = \operatorname{lcm}(0, 0) = 0, \\
    &\operatorname{lcm}(a, b)^{n} = \operatorname{lcm}(0, 0)^{n} = 0^{n} = 0
\end{aligned}
$$

だから $(4.10)$ が成り立つ.
よって以下 $a, b$ のうち少なくとも一方は $0$ でないとする.
このとき定理 3.12 (2) より, $\gcd(a, b) > 0$.
また定理 4.12 より $|ab| = \gcd(a, b)\operatorname{lcm}(a, b)$ だから,

$$
    |ab|^{n} = (\gcd(a, b)\operatorname{lcm}(a, b))^{n}
             = \gcd(a, b)^{n}\operatorname{lcm}(a, b)^{n} \tag{4.11}
$$

が成り立つ.
また同じく定理 4.12 より

$$|ab|^{n} = |a^{n}b^{n}| = \gcd(a^{n}, b^{n})\operatorname{lcm}(a^{n}, b^{n})$$

が成り立ち, 定理 3.24 より

$$\gcd(a^{n}, b^{n}) = \gcd(a, b)^{n}$$

が成り立つから,

$$|ab|^{n} = \gcd(a, b)^{n}\operatorname{lcm}(a^{n}, b^{n}) \tag{4.12}$$

が成り立つ.
よって $(4.11)$, $(4.12)$ から,

$$
    \gcd(a, b)^{n}\operatorname{lcm}(a^{n}, b^{n})
    = \gcd(a, b)^{n}\operatorname{lcm}(a, b)^{n}
$$

となり, 両辺を $\gcd(a, b)^{n} > 0$ で割って $(4.10)$ を得る.
$\blacksquare$

## 定理 4.14

$a_{1}, a_{2}, \cdots, a_{n}$ を整数とし, $m$ を非負整数とする.
このとき

$$
    \operatorname{lcm}(a_{1}^{m}, a_{2}^{m}, \cdots, a_{n}^{m})
    = \operatorname{lcm}(a_{1}, a_{2}, \cdots, a_{n})^{m} \tag{4.13}
$$

が成り立つ.

### 証明

定理 4.11 (1) より
$\operatorname{lcm}(a_{1}^{m}) = |a_{1}^{m}| = |a_{1}|^m = \operatorname{lcm}(a_{1})^m$ となる.

いま $1 \le k < n$ とし,

$$
    \operatorname{lcm}(a_{1}^{m}, \cdots, a_{k}^{m})
    = \operatorname{lcm}(a_{1}, \cdots, a_{k})^{m}
$$

が成り立つとする.
このとき定理 4.10, 補題 4.13 により,

$$
\begin{aligned}
    \operatorname{lcm}(a_{1}^{m}, \cdots, a_{k}^{m}, a_{k + 1}^{m}) 
    &= \operatorname{lcm}(\operatorname{lcm}(a_{1}^{m}, \cdots, a_{k}^{m}), a_{k + 1}^{m}) \\
    &= \operatorname{lcm}(\operatorname{lcm}(a_{1}, \cdots, a_{k})^{m}, a_{k + 1}^{m}) \\
    &= \operatorname{lcm}(\operatorname{lcm}(a_{1}, \cdots, a_{k}), a_{k + 1})^{m} \\
    &= \operatorname{lcm}(a_{1}, \cdots, a_{k}, a_{k + 1})^{m}
\end{aligned}
$$

が成り立つ.

以上から, 数学的帰納法により $(4.13)$ が成り立つ.
$\blacksquare$
