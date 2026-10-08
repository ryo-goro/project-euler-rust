# 02. 約数と倍数

## 概要

約数・倍数の定義とそれらの基本的な性質について述べる.

---

## 定義 2.1 (約数と倍数)

$a, b \in \mathbb{Z}$ とする.
整数 $k$ が存在して $b = ak$ と表せるとき,
$a$ は $b$ の**約数**である, または $b$ は $a$ の**倍数**であるといい,
$a \mid b$ と書く.
$a \mid b$ の否定は $a \nmid b$ と書く.

## 定理 2.1

$a, b \in \mathbb{Z}, \, b \neq 0$ とするとき,
次の (a) と (b) は同値である.

(a) $b \mid a$.

(b) $a \bmod b = 0$.

### 証明

#### (a) $\implies$ (b)

$b \mid a$ のとき, 整数 $k$ が存在して $a = bk = bk + 0$ と表せるから,
剰余の一意性により $a \bmod b = 0$ である.

#### (b) $\implies$ (a)

$a \bmod b = 0$ のとき, $q = a / b$ とすれば $q \in \mathbb{Z}$ であり,
$a = bq$ となるから, $b \mid a$ である.
$\blacksquare$

## 定理 2.2

$a, b \in \mathbb{Z}$ に対し, 次の (a), (b), (c) はすべて同値である.

(a) $a \mid b$.

(b) $a \mid (-b)$.

(c) $(-a) \mid b$.

### 証明

#### (a) $\implies$ (b)

$a \mid b$ のとき, $k \in \mathbb{Z}$ が存在して $b = ka$ と書ける.
このとき $-b = (-k)a, \, -k \in \mathbb{Z}$ であるから, $a \mid (-b)$.

#### (b) $\implies$ (c)

$a \mid (-b)$ のとき, $k \in \mathbb{Z}$ が存在して $-b = ka$ と書ける.
このとき $b = k(-a)$ だから, $(-a) \mid b$.

#### (c) $\implies$ (a)

$(-a) \mid b$ のとき, $k \in \mathbb{Z}$ が存在して $b = k(-a)$ と書ける.
このとき $b = (-k)a, \, -k \in \mathbb{Z}$ であるから, $a \mid b$.
$\blacksquare$

## 定理 2.3

$a \in \mathbb{Z}$ に対して以下が成り立つ.

(1) $a \mid 0$.

(2) $0 \mid a \iff a = 0$.

(3) $1 \mid a$.

(4) $a \mid 1 \iff |a| = 1$.

### 証明

#### (1)

$0 = 0 \cdot a$ より明らか.

#### (2)

$0 \mid a$ ならば $k \in \mathbb{Z}$ が存在して $a = 0 \cdot k = 0$ となる.

$a = 0 \implies 0 \mid a$ は (1) より明らか.

#### (3)

$a = a \cdot 1$ より明らか.

#### (4)

$a \mid 1$ ならば $k \in \mathbb{Z}$ が存在して $1 = ak$ と書けるから,
$(a, k) = (1, 1), (-1, -1)$ である.
したがって $|a| = 1$.

$|a| = 1$ 即ち $a = \pm 1$ ならば, (3) および定理 2.2 によって $a \mid 1$ となる.
$\blacksquare$

## 定理 2.4 (反射律)

$a \in \mathbb{Z}$ に対して $a \mid a$ が成り立つ.

### 証明

$a = a \cdot 1$ より明らか.
$\blacksquare$

## 定理 2.5 (推移律)

$a, b, c \in \mathbb{Z}$ に対して $a \mid b$ かつ $b \mid c$ ならば, $a \mid c$ が成り立つ.

### 証明

$a \mid b, \, b \mid c$ ならば, $k, l \in \mathbb{Z}$ が存在して $b = ka, \, c = lb$ と書ける.
このとき $c = l \cdot ka = kla$ で, $kl \in \mathbb{Z}$ だから, $a \mid c$ である.
$\blacksquare$

## 定理 2.6

$a, b \in \mathbb{Z}$ に対し, 次の (a) と (b) は同値である.

(a) $a \mid b$ かつ $b \mid a$.

(b) $|a| = |b|$.

### 証明

#### (a) $\implies$ (b)

$a \mid b$ かつ $b \mid a$ ならば,
$k, l \in \mathbb{Z}$ が存在して $b = ka, \, a = lb$ と書ける.

$a = 0$ ならば $b = k \cdot 0 = 0$ となるから $|a| = |b|$.

$a \neq 0$ ならば $a = lb = l \cdot ka = kla$ より $kl = 1$ となるから,
$(k, l) = (1, 1), (-1, -1)$.
よって $|a| = |lb| = |l||b| = 1 \cdot |b| = |b|$.

#### (b) $\implies$ (a)

$|a| = |b|$ ならば $a = \pm b$ だから,
定理 2.2, 2.4 によって $a \mid b$ かつ $b \mid a$ となる.
$\blacksquare$

## 定理 2.7

$a, b, c \in \mathbb{Z}$ に対し, 以下が成り立つ.

(1) $a \mid b$ かつ $a \mid c$ ならば, $a \mid (b \pm c)$.

(2) $a \mid b$ ならば, $(ac) \mid (bc)$, $a \mid (bc)$.

(3) $(ac) \mid (bc)$ かつ $c \neq 0$ ならば, $a \mid b$.

(4) $(ab) \mid c$ ならば, $a \mid c$ かつ $b \mid c$.

### 証明

#### (1)

$a \mid b$ かつ $a \mid c$ ならば,
$k, l \in \mathbb{Z}$ が存在して $b = ka, \, c = la$ と書けるから,
$b \pm c = (k \pm l)a$ となる.
$k \pm l \in \mathbb{Z}$ だから, $a \mid (b \pm c)$ である.

#### (2)

$a \mid b$ ならば, $k \in \mathbb{Z}$ が存在して $b = ka$ と書けるから,
$bc = kac$ となる.
よって $(ac) \mid (bc)$.
また $bc = kc \cdot a, \, kc \in \mathbb{Z}$ だから, $a \mid bc$.

#### (3)

$(ac) \mid (bc)$ かつ $c \neq 0$ ならば,
$k \in \mathbb{Z}$ が存在して $bc = kac$ と書けるから, $b = ka$.
よって $a \mid b$.

#### (4)

$(ab) \mid c$ ならば, $k \in \mathbb{Z}$ が存在して $c = kab$ と書ける.
$ka, kb \in \mathbb{Z}$ だから, $a \mid c$ かつ $b \mid c$.
$\blacksquare$

## 定理 2.8

$a$ を整数, $m, n$ を非負整数とするとき, 以下が成り立つ.

(1) $m \le n$ ならば, $a^{m} \mid a^{n}$.

(2) $|a| > 1$ ならば, $m \le n \iff a^{m} \mid a^{n}$.

### 証明

#### (1)

$m \le n$ のとき $a^{n - m}$ は整数で, $a^{n} = a^{n - m} \cdot a^{m}$ だから,
$a^{m} \mid a^{n}$ である.

#### (2)

$\implies$ は (1) より明らかだから $\impliedby$ を示す.
$a^{m} \mid a^{n}$ のとき, $k \in \mathbb{Z}$ が存在して $a^{n} = ka^{m}$ となるが,
$|a| > 1$ だから $k \neq 0$.
よって $|k| \ge 1$.
故に

$$|a|^{n} = |a^{n}| = |ka^{m}| = |k||a|^{m} \ge |a|^{m}$$

である.
$|a| > 1$ だから, 底が $1$ より大きい場合の指数函数の狭義単調増加性により, $n \ge m$ が成り立つ.
$\blacksquare$

## 定理 2.9

$a, b \in \mathbb{Z}, \, b \neq 0$ に対し,

$$a \mid b \implies |a| \le |b|.$$

### 証明

$a \mid b$ ならば, $k \in \mathbb{Z}$ が存在して $b = ka$ と書ける.
$b \neq 0$ より $k \neq 0$ だから, $|k| \ge 1$. よって

$$|b| = |ka| = |k||a| \ge |a|$$

となる.
$\blacksquare$
