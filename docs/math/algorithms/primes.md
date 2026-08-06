# primes.rs

対応理論:
[01. 除法の原理とその性質](./01_division.md),
[05. 素数](../number_theory/05_primes.md),
[06. 合同式](../number_theory/06_congruence.md),
[07. フェルマーの小定理](../number_theory/07_fermat_little_theorem.md)

## 概要

`is_prime` は Miller-Rabin 法による素数判定を提供する.
`u64` の範囲 ($< 2^{64}$) において決定的に (確率的判定ではなく確実に) 動作する.

`sieve` はエラトステネスの篩による, 指定範囲内すべての整数の素数判定表を提供する.

---

## 1. is_prime (Miller-Rabin 法)

### 1.1 理論

$n$ を奇数の整数 $> 2$ とする.
また $a$ を $n$ の倍数でない整数とする.
このとき正整数 $s$ と奇数の正整数 $d$ が存在して

$$n - 1 = 2^{s}d \tag{1.1}$$

と表せる.
そこでいま,

$$a_{r} = a^{2^{r}d} \quad (r = 0, 1, \cdots, s - 1) \tag{1.2}$$

によって $a_{0}, a_{1}, \cdots, a_{s - 1}$ を定義する.
このとき

$$a_{0} = a^{d} \tag{1.3}$$

である.
また

$$
    a_{r} = a^{2^{r}d} = a^{2 \cdot 2^{r - 1}d}
                       = (a^{2^{r - 1}d})^{2}
                       = (a_{r - 1})^{2}
                       \quad (r = 1, 2, \cdots, s - 1) \tag{1.4}
$$

が成り立つ.
また $s$ に関する数学的帰納法によって容易にわかるように,

$$
    a^{n - 1} - 1 = a^{2^{s}d} - 1 = (a^{d} - 1)\prod_{r = 0}^{s - 1}(a^{2^{r}d} + 1)
                                   = (a_{0} - 1)\prod_{r = 0}^{s - 1}(a_{r} + 1) \tag{1.5}
$$

が成り立つ.

さていま $n$ が素数であるとする.
このとき[07. フェルマーの小定理](../number_theory/07_fermat_little_theorem.md)の
定理 7.6 系により, $a^{n - 1} \equiv 1 \pmod{n}$, 即ち

$$x \mid (a^{n - 1} - 1) \tag{1.6}$$

が成り立つ.

そこで $n$ が素数であることと $(1.5)$, $(1.6)$ から,
[05. 素数](../number_theory/05_primes.md)の定理 5.3 を繰り返し適用することによって

$$
\begin{aligned}
    a_{0} \equiv 1 &\pmod{n}, \\
    a_{0} \equiv -1 &\pmod{n}, \\
    a_{1} \equiv -1 &\pmod{n}, \\
    &\cdots \\
    a_{s - 1} \equiv -1 &\pmod{n}
\end{aligned}
$$

のいずれかが成り立つことがわかる.
よって[06. 合同式](./06_congruence.md)の定理 6.7 により,

$$
\begin{aligned}
    a_{0} \bmod n &= 1, \\
    a_{0} \bmod n &= n - 1, \\
    a_{1} \bmod n &= n - 1, \\
    &\cdots \\
    a_{s - 1} \bmod n &= n - 1 \tag{1.7}
\end{aligned}
$$

のいずれかが成り立つ
($1 \bmod n = 1, \, (-1) \bmod n = n - 1$ に注意).
いまここで更に

$$b_{r} = a_{r} \bmod n \quad (r = 0, 1, \cdots, s - 1) \tag{1.8}$$

によって $b_{0}, b_{1}, \cdots, b_{s - 1}$ を定義すれば,
$(1.3)$ より

$$b_{0} = a_{0} \bmod n = a^{d} \bmod n \tag{1.9}$$

である.
また $(1.4)$ と[01. 除法の原理とその性質](./01_division.md)の定理 1.6 (2) より,

$$
    b_{r} = a_{r} \bmod n = (a_{r - 1})^{2} \bmod n
                          = (a_{r - 1} \bmod n)^{2} \bmod n
                          = b_{r - 1}^{2} \bmod n \quad (r = 1, 2, \cdots, s - 1) \tag{1.10}
$$

が成り立つ.
また, $(1.7)$ のいずれかが成り立つということから,

$$
\begin{aligned}
    b_{0} &= 1, \\
    b_{0} &= n - 1, \\
    b_{1} &= n - 1, \\
    &\cdots \\
    b_{s - 1} &= n - 1 \tag{1.11}
\end{aligned}
$$

のいずれかが成り立つ.

以上の議論の対偶を取れば, 次のようになる:

$$
\begin{aligned}
    b_{0} &\neq 1, \\
    b_{0} &\neq n - 1, \\
    b_{1} &\neq n - 1, \\
    &\cdots \\
    b_{s - 1} &\neq n - 1 \tag{1.12}
\end{aligned}
$$

がすべて成り立てば, $n$ は合成数である.

なお $(1.12)$ の不等式がすべて成り立つとき, $a$ は $n$ の合成性の**証人**であるという.
$(1.12)$ の不等式のいずれかが成り立たず (即ち $(1.11)$ のいずれかの等式が成り立ち),
かつ $n$ が合成数であるとき, $n$ を $a$ を基数とする**強擬素数**という.

### 1.2 実装

```rust
const WITNESSES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    for &p in &WITNESSES {
        if n == p {
            return true;
        }

        if n.is_multiple_of(p) {
            return false;
        }
    }

    // n is odd.
    // n - 1 = d * 2^s (s > 0, d is odd)
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;

    'witness: for &a in &WITNESSES {
        // b = a^d % x
        let mut b = modular::mod_pow(a, d, n);

        if b == 1 || b == n - 1 {
            continue;
        }

        for _r in 1..s {
            // b = a^{d * 2^{_r}} % n
            b = modular::mod_mul(b, b, n);

            if b == 1 {
                return false;
            }

            if b == n - 1 {
                continue 'witness;
            }
        }

        return false;
    }

    true
}
```

この実装において, 関数の始めで `0`, `1` は除外されていることと
`WITNESSES` が `2` を要素に持つことから,
```rust
    // n is odd.
    // n - 1 = d * 2^s (s > 0, d is odd)
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
```
に到達した時点で `n` は奇数の整数 $> 2$ である.
また `WITNESSES` の要素はすべて素数であるが,
ここに到達した時点で `n` は `1` でなく, これらの素数のいずれでもないため,
`WITNESSES` の要素はすべてこれ以降 `n` の倍数でない.

`'witness:` の `for` ループ内では,
```rust
        let mut b = modular::mod_pow(a, d, n);
```
によって `b` が $b_{0} = a^{d} \bmod n$ として初期化されている.

```rust
        if b == 1 || b == n - 1 {
            continue;
        }
```
について, $b_{0} = 1, \, b_{0} = n - 1$ のどちらかが成立した場合,
$n$ を合成数と断定できないので現在の `a` に対するループ本体の実行を中断して次のループ本体の実行に進む.
どちらも成立しない場合, 即ち $b_{0} \neq 1$ かつ $b_{0} \neq n - 1$ である場合,
次に $b_{1} \neq n - 1, \cdots, b_{s - 1} \neq n - 1$ がすべて成立するかどうかを順に確認していく:

```rust
        for _r in 1..s {
            // b = a^{d * 2^{_r}} % n
            b = modular::mod_mul(b, b, n);

            if b == 1 {
                return false;
            }

            if b == n - 1 {
                continue 'witness;
            }
        }

        return false;
```
ここでは `_r` が $r$ に対応する
(`r` ではなく `_r` としているのは, ループ本体で実際にはこの変数を使用していないため).
まず $(1.10)$ を利用して `b` の古い値 ($b_{r - 1}$)
から新しい値 ($b_{r}$) を計算している.

明らかに, ある `_r` に対するループ本体の開始時点で,
それまでのループ本体の実行中に $b$ は一度も $n - 1$ になっていない.
即ち, ある `_r` に対するループ本体の開始時点では,
$b_{1} \neq n - 1, \cdots, b_{r - 1} \neq n - 1$ がすべて成立している.

`b == 1`, 即ち $b_{r} = 1$ となった場合, $(1.10)$ と $1 \bmod n = 1 \; (\because n > 1)$ から
$b_{r}, b_{r + 1}, \cdots, b_{s - 1}$ がすべて $1$ となり,
$n > 2$ だからこれらは $n - 1$ ではない.
よってこの時点で $(1.12)$ の不等式がすべて成り立つことが確定するから, $n$ は合成数であると判定できる.

`b == n - 1`, 即ち $b_{r} = n - 1$ となった場合,
$n$ を合成数と断定できないので現在の `a` に対するループ本体の実行を中断して次のループ本体の実行に進む.

このループが中断せずに終了した場合, $(1.12)$ の不等式がすべて成立したということだから,
$n$ は合成数であると判定できる.

関数末尾の `true` に到達した場合
(即ち `WITNESSES` のすべての要素 `a` を使用しても `n` が合成数であると判定できなかった場合) でも,
`n` は `WITNESSES` のすべての要素を基数とする強擬素数の可能性があるので,
数学的には `n` が素数であるとは限らない.
しかし次節に述べる理由によって, この実装では `n` は素数となることが確定する.

### 1.3 決定的に動作する基数集合について

本実装が用いる基数集合

$$\{2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37\}$$

については, **この基数すべてに対して強擬素数となる最小の合成数が
$3.1 \times 10^{23}$ を超えることが, 網羅的な計算検証によって確認されている**
([OEIS: A014233](https://oeis.org/A014233)).
$2^{64} \approx 1.8 \times 10^{19}$ はこの境界よりはるかに小さいため,
`u64` の全範囲でこの基数集合による判定は決定的に正しい.

**この事実は定理として証明されたものではなく, 計算検証によって確認された経験的事実である**という点に注意する
(詳細な基数集合ごとの決定的範囲の一覧は下記参考文献を参照).

### 1.4 計算量

$n$ に対して $O(k \log^{3} n)$ ($k$ は基数の個数, 本実装では $12$ 個で固定).
試し割り ($O(\sqrt{n})$) と比べ, $n$ が大きくなるほど圧倒的に高速.

### 1.5 実装上の注意点

- **オーバーフロー対策**: `mod_pow`, `mod_mul` は `u128` を経由して計算する
  (`u64 × u64` は `u64` の範囲を超え得るため).
  詳細は `modular.rs` を参照.
- **小さい数の早期判定**: `WITNESSES` の各要素 `a` との一致・剰余判定を先に行うことで,
  `a` が `n` の倍数となるケースを回避している.
- **境界値**: `n < 2` は非素数として扱う.

---

## 2. sieve (エラトステネスの篩)

### 2.1 理論

[05. 素数](../number_theory/05_primes.md) の定理 5.1 系により,
$n$ が合成数ならば, $n$ は必ず $\sqrt{n}$ 以下の素因数を持つ.
これにより, $\sqrt{\text{limit}}$ までの素数で篩い落とせば,
`limit` 未満のすべての合成数を除去できる.

### 2.2 実装

```rust
pub fn sieve(limit: usize) -> Vec<bool> {
    if limit <= 1 {
        return vec![false; limit];
    }

    let mut is_prime = vec![true; limit];

    is_prime[0] = false;
    is_prime[1] = false;

    for even in (4..limit).step_by(2) {
        is_prime[even] = false;
    }

    let mut odd = 3;

    while odd * odd < limit {
        if is_prime[odd] {
            for idx in (odd * odd..limit).step_by(2 * odd) {
                is_prime[idx] = false;
            }
        }

        odd += 2;
    }

    is_prime
}
```

### 2.3 計算量

$O(n \log \log n)$ (時間), $O(n)$ (空間).

### 2.4 実装上の注意点

- `is_prime` (Miller-Rabin) と異なり, **篩は指定範囲内すべての整数について
  事前に判定結果を保持する**ため, 大量の判定を行う場合はこちらが有利.
