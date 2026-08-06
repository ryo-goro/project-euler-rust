# 01. 除法の原理とその性質

## 概要

整数の商・剰余の定義とそれらの基本的な性質について述べる.

---

## 定理 1.1 (除法の原理)

$a, b \in \mathbb{Z}, \, b \neq 0$ とする.
このとき

$$a = bq + r \quad (0 \le r < |b|) \tag{1.1}$$

をみたす整数 $q, r$ がただ一組存在する.

### 証明

まず $q, r$ の存在について示す.

#### $a \ge 0, b > 0$ の場合

$A = \{n \in \mathbb{N} \mid bn > a\}$ とおく.
このとき $a + 1 \in \mathbb{N}$ かつ

$$
\begin{aligned}
    b(a + 1) &= ab + b \\
             &> ab \\
             &\ge a &(\because a \ge 0, b > 0) \\
\end{aligned}
$$

であるから, $a + 1 \in A$.
即ち $A$ は $\mathbb{N}$ の空でない部分集合である.
したがって $m = \min A$ が存在する.
$m \in A$ より

$$bm > a \ge 0 \tag{1.2}$$

だから, これと $b > 0$ から $m > 0$.
故に

$$m - 1 \in \mathbb{N}. \tag{1.3}$$

また $m - 1 < m = \min A$ であるので

$$m - 1 \notin A. \tag{1.4}$$

$(1.3)$, $(1.4)$ から,

$$b(m - 1) \le a. \tag{1.5}$$

そこで $q = m - 1$ とおけば, $(1.2)$, $(1.3)$, $(1.5)$ から, $q \in \mathbb{Z}$ かつ

$$bq \le a < b(q + 1) \tag{1.6}$$

が成り立つ.
ここで $r = a - bq$ とおけば, $r \in \mathbb{Z}$, $a = bq + r$ で,
$(1.6)$ より $0 \le r < b = |b|$ である.

#### $a < 0, b > 0$ の場合

$-a > 0$ だから, いま示したことから

$$-a = bq' + r' \quad (0 \le r' < b)$$

となる整数 $q', r'$ が存在する.
ここで $r' = 0$ ならば, $q = -q', r = 0$ として

$$a = -bq' - r' = bq + r \quad (0 \le r < b)$$

が成り立つ.
$r' > 0$ ならば, $q = -q' - 1, r = b - r'$ とおけば

$$a = -bq' - r' = b(-q' - 1) + b - r' = bq + r \quad (0 \le r < b)$$

が成り立つ.

#### $b < 0$ の場合

$-b > 0$ だから, 上で示したことから整数 $q', r$ が存在して

$$a = -bq' + r \quad (0 \le r < -b = |b|)$$

が成り立つ.
そこで $q = -q'$ とおけば,

$$a = -bq' + r = bq + r \quad (0 \le r < |b|)$$

である.

次に $q, r$ の一意性について示す.
いま $q, r, q', r' \in \mathbb{Z}$ が

$$a = bq + r = bq' + r' \quad (0 \le r, r' < |b|)$$

をみたすとする.
このとき

$$|b||q - q'| = |bq - bq'| = |r' - r| < |b|$$

であり, $b \neq 0$ より $|b| > 0$ だから,

$$|q - q'| < 1$$

となり, $q, q'$ がともに整数であることから $q = q'$ を得る.
これから $r = r'$ も従う.
$\blacksquare$

## 定義 1.1

$a, b \in \mathbb{Z}, b \neq 0$ とする.
このとき定理 1.1 により, $(1.1)$ をみたす整数 $q, r$ がただ一組存在する.
これらをそれぞれ $a$ を $b$ で割った**商**, **余り** (または**剰余**) といい,
$q = a \div b, \, r = a \bmod b$ と表す.

$a \div b$ については, 分数との混同のおそれのない限り $a / b$ とも書く.

## 定理 1.2

$a, b \in \mathbb{Z}, \, b \neq 0$ とするとき,

$$
\begin{aligned}
    &a / (-b)    = -(a / b), \\
    &a \bmod (-b) = a \bmod b
\end{aligned}
$$

が成り立つ.

### 証明

$a / b = q$, $a \bmod b = r$ とおくとき,

$$a = bq + r = (-b)(-q) + r$$

である.
ここで $-q, r \in \mathbb{Z}$ であり, $0 \le r < |b| = |-b|$ だから, 商と剰余の一意性より
$a / (-b) = -q = -(a / b)$, $a \bmod (-b) = r = a \bmod b$ である.
$\blacksquare$

## 定理 1.3

$a \in \mathbb{Z}$ とするとき, 以下が成り立つ.

(1) $a \neq 0$ のとき, $0 / a = 0, 0 \bmod a = 0$.

(2) $a / 1 = a, a \bmod 1 = 0$.

(3) $a \neq 0$ のとき, $a / a = 1, a \bmod a = 0$.

### 証明

#### (1)

$0 = a \cdot 0 + 0$ だから商と剰余の一意性により明らか.

#### (2)

$a = 1 \cdot a + 0$ だから商と剰余の一意性により明らか.

#### (3)

$a = a \cdot 1 + 0$ だから商と剰余の一意性により明らか.
$\blacksquare$

## 定理 1.4

$a, b \in \mathbb{Z}, \, a \neq 0$ とするとき,

$$
\begin{aligned}
    &ab / a    = b, \\
    &ab \bmod a = 0
\end{aligned}
$$

が成り立つ.

### 証明
$ab = ab + 0$ だから商と剰余の一意性によって明らか.
$\blacksquare$

## 定理 1.5

$a, b, c \in \mathbb{Z}, \, a \neq 0$ とするとき,

$$
\begin{aligned}
    &(ab + c) / a    = b + c /a, \\
    &(ab + c) \bmod a = c \bmod a
\end{aligned}
$$

が成り立つ.

### 証明

$c / a = q$, $c \bmod a = r$ とおくとき, $c = aq + r$ だから

$$ab + c = ab + aq + r = a(b + q) + r$$

となる.
ここで $b + q, r \in \mathbb{Z}$ であり, $0 \le r < |a|$ だから, 商と剰余の一意性より
$(ab + c) / a = b + q = b + c / a$, $(ab + c) \bmod a = r = c \bmod a$ である.
$\blacksquare$

## 定理 1.6

$a, b, c \in \mathbb{Z}, \, c \neq 0$ とするとき, 以下が成り立つ.

(1) $(a \pm b) \bmod c = (a \bmod c \pm b \bmod c) \bmod c$.

(2) $ab \bmod c = (a \bmod c)(b \bmod c) \bmod c$.

### 証明

$q = a / c, r = a \bmod c, q' = b / c, r' = b \bmod c$ とおくと
$q, r, q', r' \in \mathbb{Z}$ であり,
$a = cq + r, b = cq' + r'$ である.
よって

$$
\begin{aligned}
    &a \pm b = (cq + r) \pm (cq' + r') = c(q \pm q') + r \pm r', \\
    &ab = (cq + r)(cq' + r') = c(cqq' + qr' + q'r) + rr'
\end{aligned}
$$

であるから, 定理 1.5 より

$$
\begin{aligned}
    &(a \pm b) \bmod c = (r \pm r') \bmod c = (a \bmod c \pm b \bmod c) \bmod c, \\
    &ab \bmod c = rr' \bmod c = (a \bmod c)(b \bmod c) \bmod c
\end{aligned}
$$

となる.
$\blacksquare$

## 定理 1.7

$a, b, c \in \mathbb{Z}, b \neq 0, c > 0$ とするとき,

$$
\begin{aligned}
    &(ac) / (bc) = a / b, \\
    &(ac) \bmod (bc) = c(a \bmod b)
\end{aligned}
$$

が成り立つ.

### 証明

$q = a / b, r = a \bmod b$ とおけば, $q, r \in \mathbb{Z}$ であり,

$$a = bq + r \quad (0 \le r < |b|)$$

が成り立つ.
よって

$$ac = bcq + cr.$$

ここで $cr \in \mathbb{Z}$ であり,

$$
\begin{aligned}
    0 \le cr &< c|b| &(\because c > 0, \, r < |b|) \\
             &= |bc|
\end{aligned}
$$

であるから, 商と剰余の一意性により
$(ac) / (bc) = q = a / b, \, (ac) \bmod (bc) = cr = c(a \bmod b)$.
$\blacksquare$

## 定理 1.8

$a, b, c \in \mathbb{Z}, b > 0, c > 0$ とするとき,
$a / (bc) = (a / b) / c$ が成り立つ.

### 証明

$q = a / b, \, r = a \bmod b, \, q' = q / c, \, r' = q \bmod c$ とおけば,
$q, r, q', r' \in \mathbb{Z}$ であり,

$$
\begin{aligned}
    &a = bq + r \quad &(0 \le r < b), \\
    &q = cq' + r' \quad &(0 \le r' < c)
\end{aligned}
$$

が成り立つ.
よって

$$a = b(cq' + r') + r = bcq' + br' + r.$$

ここで $q', br' + r \in \mathbb{Z}$ であり,

$$
\begin{aligned}
    0 \le br' + r &\le b(c - 1) + r &(\because b > 0, \, r' \le c - 1) \\
                  &< b(c - 1) + b   &(\because r < b) \\
                  &= bc
\end{aligned}
$$

であるから,商の一意性により $a / (bc) = q' = q / c = (a / b) / c$.
$\blacksquare$

## 定理 1.9

$a, b, c \in \mathbb{Z}, \, a \le b$ とするとき, 以下が成り立つ.

1) $c > 0$ ならば $a / c \le b / c$.
2) $c < 0$ ならば $a / c \ge b / c$.

### 証明

$q = a / c, \, r = a \bmod c, \, q' = b / c, \, r' = b \bmod c$ とおけば,
$q, r, q', r' \in \mathbb{Z}$ であり,

$$
\begin{aligned}
    &a = cq + r \quad &(0 \le r < |c|), \\
    &b = cq' + r' \quad &(0 \le r' < |c|)
\end{aligned}
$$

が成り立つ.
いま $a \le b$, 即ち $cq + r \le cq' + r'$ だから,

$$
\begin{aligned}
    c(q - q') = cq - cq' &\le r' - r \\
                         &\le r' &(\because r \ge 0) \\
                         &< |c| &(\because r' < |c|) \tag{7}
\end{aligned}
$$

となる.

#### 1.

$c > 0$ のとき, $(7)$ より $c(q - q') < c$ となるから,
$q - q' < 1$, 即ち $q - q' \le 0$ となる.
したがって $a / c = q \le q' = b / c$ である.

#### 2.

$c < 0$ のとき, $(7)$ より $c(q - q') < -c$ となるから,
$q - q' > -1$, 即ち $q - q' \ge 0$ となる.
したがって $a / c = q \ge q' = b / c$ である.
$\blacksquare$
