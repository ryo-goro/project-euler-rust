# 06. 合同式

## 概要

合同式の定義と基本的な性質について述べる.

---

## 定義 6.1 (合同式)

$a, b, m$ を整数とする.
$a - b$ が $m$ の倍数であること, 即ち $m \mid (a - b)$ が成り立つことを,
$a$ と $b$ は $m$ を法として**合同**であるといい,

$$a \equiv b \pmod{m} \tag{6.1}$$

と書く.
これの否定は $a \not\equiv b \pmod{m}$ と書く.

## 定理 6.1 (同値律)

$m$ を整数とするとき, 以下が成り立つ.

(1) (反射律) $a \in \mathbb{Z}$ に対し, $a \equiv a \pmod{m}$.

(2) (対称律) $a, b \in \mathbb{Z}$ に対し,
$a \equiv b \pmod{m}$ ならば $b \equiv a \pmod{m}$.

(3) (推移律) $a, b, c \in \mathbb{Z}$ に対し,
$a \equiv b \pmod{m}$ かつ $b \equiv c \pmod{m}$ ならば $a \equiv c \pmod{m}$.

### 証明

#### (1)

定理 2.3 (1) より $m$ は $a - a = 0$ の約数だから明らか.

#### (2)

$a \equiv b \pmod{m}$, 即ち $m \mid (a - b)$ ならば,
定理 2.2 より $m \mid (-(a - b)) = (b - a)$, 即ち $b \equiv a \pmod{m}$ となる.

#### (3)

$a \equiv b \pmod{m}$ かつ $b \equiv c \pmod{m}$ ならば,
$m \mid (a - b)$ かつ $m \mid (b - c)$ だから, 定理 2.7 (1) より
$m \mid ((a - b) + (b - c)) = (a - c)$, 即ち $a \equiv c \pmod{m}$ が成り立つ.
$\blacksquare$

## 定理 6.2

$a, b, m$ を整数とするとき, 次の (a) と (b) は同値である.

(a) $a \equiv b \pmod{m}$.

(b) $a \equiv b \pmod{(-m)}$.

### 証明

(a), (b) はそれぞれ $m \mid (a - b), \, (-m) \mid (a - b)$ を意味する.
定理 2.2 より, これらは同値である.
$\blacksquare$

## 定理 6.3

$a$ と $b$ を整数とするとき, 以下が成り立つ.

(1) $a \equiv b \pmod{0} \iff a = b$.

(2) $a \equiv b \pmod{1}$.

### 証明

#### (1)

$a \equiv b \pmod{0}$ は $0 \mid (a - b)$ を意味する.
定理 2.3 (2) より, これが成り立つのは $a - b = 0$, 即ち $a = b$ となるときに限る.

#### (2)

定理 2.3 (3) によって明らか.
$\blacksquare$

## 定理 6.4
$a, b, c, d, m$ を整数とし,
$a \equiv b \pmod{m}, \, c \equiv d \pmod{m}$ であるとする.
このとき以下が成り立つ.

(1) $a \pm c \equiv b \pm d \pmod{m}$.

(2) $ac \equiv bd \pmod{m}$.

### 証明

#### (1)

$a \equiv b \pmod{m}, \, c \equiv d \pmod{m}$ より
$m \mid (a - b), \, m \mid (c - d)$ だから,
定理 2.7 (1) より $m \mid ((a - b) \pm (c - d)) = ((a \pm c) - (b \pm d))$,
即ち $a \pm c \equiv b \pm d \pmod{m}$ が成り立つ.

#### (2)

$a \equiv b \pmod{m}$ より $m \mid (a - b)$ だから,
定理 2.7 (2) より $m \mid (a - b)c = ac - bc$.
また $c \equiv d \pmod{m}$ より $m \mid (c - d)$ だから,
やはり定理 2.7 (2) より $m \mid b(c - d) = bc - bd$.
よって定理 2.7 (1) より $m \mid ((ac - bc) + (bc - bd)) = ac - bd$,
即ち $ac \equiv bd \pmod{m}$ が成り立つ.
$\blacksquare$

## 定理 6.4 系

$a, b, c, m$ を整数とし, $a \equiv b \pmod{m}$ であるとする.
このとき以下が成り立つ.

(1) $a \pm c \equiv b \pm c \pmod{m}$.

(2) $ac \equiv bc \pmod{m}$.

### 証明

定理 6.1 (1) より $c \equiv c \pmod{m}$ だから,
定理 6.4 により明らかである.
$\blacksquare$

## 定理 6.5

$a, b, c, m$ を整数とし, $c$ と $m$ は互いに素であるとする.
このとき次の (a) と (b) は同値である.

(a) $a \equiv b \pmod{m}$.

(b) $ac \equiv bc \pmod{m}$.

### 証明

(a), (b) はそれぞれ $m \mid (a - b), \, m \mid (ac - bc) = (a - b)c$ を意味するから,
定理 3.26 系によって本定理が成り立つ.
$\blacksquare$

## 定理 6.6

$a, b, m$ を整数とし, $a \equiv b \pmod{m}$ であるとする.
このとき任意の非負整数 $n$ に対して

$$a^{n} \equiv b^{n} \pmod{m} \tag{6.2}$$

が成り立つ.

### 証明

$n = 0$ のときは定理 2.3 (1) より $m \mid 0 = 1 - 1 = a^{0} - b^{0}$ だから成り立つ.
$n = 1$ のときは仮定そのものだから成り立つ.

いまある整数 $n \ge 1$ に対して $(6.2)$ が成り立つとする.
このとき仮定より $a \equiv b \pmod{m}$ だから, 定理 6.4 (2) により

$$a^{n + 1} \equiv b^{n + 1} \pmod{m}$$

が成り立つ.

以上から, 数学的帰納法により本定理が成り立つ.
$\blacksquare$

## 定理 6.7

$a, b, m$ を整数とし, $m \neq 0$ とする.
このとき次の (a) と (b) は同値である.

(a) $a \equiv b \pmod{m}$.

(b) $a \bmod m = b \bmod m$.

### 証明

$a / m, a \bmod m, b / m, b \bmod m$ をそれぞれ $q, r, q', r'$ と書くと,
$q, r, q', r' \in \mathbb{Z}$ であり,

$$a = mq + r, \; b = mq' + r' \quad (0 \le r, r' < |m|) \tag{6.5}$$

である.

#### (a) $\implies$ (b)

このとき $m \mid (a - b)$ だから, $k \in \mathbb{Z}$ が存在して $a - b = km$ となる.
よって $(6.5)$ より,

$$r - r' + m(q - q') = (mq + r) - (mq' + r') = a - b = km$$

となる.
故に

$$|r - r'| = |km - m(q - q')| = |m||k - (q - q')| \tag{6.6}$$

である.
ここで $0 \le r, r' < |m|$ より $|r - r'| < |m|$ が成り立つから,
$(6.6)$ より $|m||k - (q - q')| < |m|$.
この両辺を $|m| > 0$ で割って $|k - (q - q')| < 1$ を得る.
故に $k - (q - q') = 0$.
これと $(6.6)$ より, $|r - r'| = 0$, 即ち $r = r'$ となる.

#### (b) $\implies$ (a)

このとき $r = r'$ だから, $(6.5)$ より
$a - b = (mq + r) - (mq' + r') = m(q - q')$ となる.
故に $m \mid (a - b)$, 即ち $a \equiv b \pmod{m}$ である.
$\blacksquare$

## 定理 6.7 系 1

$a, b, m$ を整数とし, $0 \le b < |m|$ であるとする.
このとき次の (a) と (b) は同値である.

(a) $a \equiv b \pmod{m}$.

(b) $a \bmod m = b$.

### 証明

このとき $b \bmod m = b$ だから, 定理 6.7 より明らか.
$\blacksquare$

## 定理 6.7 系 2

$a$ と $m$ を整数とし, $m \neq 0$ とする.
このとき次の (a) と (b) は同値である.

(a) $a \equiv 0 \pmod{m}$.

(b) $a \bmod m = 0$.

### 証明

定理 1.3 (1) より $0 \bmod m = 0$ だから, 定理 6.7 より明らか.
$\blacksquare$

## 定理 6.8

$a, b, m, n$ を整数とするとき, 次の (a) と (b) は同値である.

(a) $a \equiv b \pmod{m}$ と $a \equiv b \pmod{n}$ が共に成り立つ.

(b) $a \equiv b \pmod{\operatorname{lcm}(m, n)}$ が成り立つ.

### 証明

(a) は $a - b$ が $m$ と $n$ の公倍数であるという主張であり,
(b) は $a - b$ が $\operatorname{lcm}(m, n)$ の倍数であるという主張である.
定理 4.4 により, これらは同値である.
$\blacksquare$

## 定理 6.8 系

$a, b, m, n$ を整数とし, $m$ と $n$ は互いに素であるとする.
このとき次の (a) と (b) は同値である.

(a) $a \equiv b \pmod{m}$ と $a \equiv b \pmod{n}$ が共に成り立つ.

(b) $a \equiv b \pmod{mn}$ が成り立つ.

### 証明

このとき定理 4.12 系より $\operatorname{lcm}(m, n) = |mn|$ だから,
定理 6.2, 6.8 によって明らかである.
$\blacksquare$
