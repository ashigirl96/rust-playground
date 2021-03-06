実践Rust入門

[TOC]

# Rustの特徴

### パフォーマンス

- Rustは、g++, gccと同レベルの速度
- Goより3倍速い
- Python3より30倍速い

### マシンコードへのコンパイル

Java(HotSpot仮想マシン)、JS(Node.js環境)は、JITコンパイラを装備している。JITとは、プログラムの実行中に収集したプロファイリング情報をもとに実行中のコードを高速なマシンコードへと徐々に置き換えるもの。

- RustはGolang, C++と同様、マシンコードを生成するので高速

### 静的型付け

- Rustのコンパイラは型推論を行う
- 変数の型宣言はほとんど省略できる

### ゼロコスト抽象化

なぜ、Rustはマシンコードを生成するGolangより速い？

→　**zero-cost abstractions**

abstractionとは、「対象から注目すべき要素を重点的に抜き出して、他は無視する手法」→　再利用性を高め、バグを防ぐ

- オブジェクト指向のポリモーフィズム(同じ名前のメソッドが属する型によって振る舞いが変わる)が抽象化の一つ
  - (Javaの)動的ディスパッチ：実行時にオブジェクトの型を調べ、その方に対応するメソッドを呼ぶ
  - (Rustの)静的ディスパッチ：コンパイル時に分かる型によって呼び出すべきメソッドを決める

- クロージャ(無名関数と自身の環境に補足した値からなるオブジェクト)
  - ガべコレの対象となるデータ構造で実現される

### 安全なシスプロ

C, C++はシスプロに使われてきたが、容易にメモリアクセスが可能で、OS開発に使われてきた。故に安全機構が欠けてきた。

- データの転記のメモリ領域あふれ
- ポインタによる誤った領域へのアクセス
- 初期化前のメモリ領域へのアクセス
- 解放後のメモリ領域へのアクセス
- データへのポインタと関数へのポインタの混同

一般的には、GCで解決するのに対して、Rustはコンパイラが賢いのでそういう問題がなくなる

### 型安全性

C言語と違って、シンタックスエラーどころかセマンティックエラー（プログラムの意味）も出してくれる



# はじめてのRust

## ルーツチェイン

- rustc: rust compoiler
- cargo: build manager && package manager
- std: standard library

- Linker: コンパイラが出力したオブジェクトファイルやライブラリを結合してABIに則って実行ファイルを出力する。OS向けに用意されてる。Linuxのgcc, binutils. macOSのXcode commandline
- ABI(Application Binary Interface): マシンコードが実行時にOSとやりとりする方法を取り決める仕様
- rustup: Rustのバージョン管理、クロスコンパイル用ターゲットのインスコ、RLSなどの開発支援ツール(ここからrustをインスコする）

## cargoコマンド

- --bin: バイナリパッケージが作られる. ルートはsrc/main.rs。cargo new のデフォルト

- --lib: ライブラリパッケージが作られる. ルートはsrc/lib.rs

- Cargo.toml: package.jsonみたいなやつ

- build: バイナリが生成される

  1. コードの検査：rustcによりエラーがないか検査

  2. コンパイル：問題なければ、rustcによりアセンブリコードへ変換し、オブジェクトファイルが生成

  3. リンカ：rustcがリンカを起動し、オブジェクトファイルとRust標準ライブラリの結合し、実行ファイルを生成 (※ libクレートはここをしない)

  

## 簡単な構文

- 感嘆符！が付いてるのはマクロ。コンパイル時に評価される
- letは右辺の評価式を変数に束縛する
- debug_assert_eq!はデバッグビルド時のみ展開される標準ライブラリ
- ;がなければ、その式で評価した値を返す
- stack.pop.expect("")で、取り出せなかった時そこで強制終了する
- whereはジェネリクスのトレイト境界を決めるもので、この境界内であればどんな値も許容する
- 「なぜならサブタイプの一般的な考え方では、親となる基底型よりも、子である派生型の方が多くの機能を 持つ(子は親の代わりができる)からです」

## バイトニックソートを実装してみる

- pub: tsのexportみたいなもの
- x: &mut [u32]とは、値をポインタ経由で”借用する”
  - mutは変更可能
  - u32は32ビット正整数
  - [u32] はu32のスライス（配列みたいなもの）
    - 数列をコピーせずに直接変更できる
- `#[allow(non_snake_case)]` 関数、変数、ライフタイム、モジュールの識別子がスネークケースでなくても良い。他にもある

### 全順序・半順序

- 半順序集合(Partially ordered set)
  - 反射律: $\forall a: a \leq a$
  - 推移律: $\forall a, b, c: a \leq b \land b \leq c \Rightarrow a \leq c$
  - 反対称律: $\forall a, b: a \leq b \land b \leq b \Rightarrow a = b$
- 全順序集合 := 全順序律 + 半順序
  - 全順序律: $\forall a, b: a \leq b \lor b \leq b$

### トレート境界

```rust
pub fn sort<T: Ord>(x: &mut [T], up: bool)
```

全順序の境界にすることで、NaNを含む全順序じゃないf64を許可しないようにする

```
# error[E0308]のドキュメントを表示する
$ rustc --explain 308
```



```rust
fn sort<T: Ord>(x: &mut [T]) ... {
	...(x, &|a, b| a.cmp(b))
}
```

と、xの要素で`cmp`と比較するには、xの要素が任意で良いわけじゃない。比較できるという保証が必要。型が必要ってことかな？そのためにも境界を設ける必要がある

### enum使う

```rust
sort(&mut x, true)
```

と来たとき、「何がtrueなの？」となるため、enumで定義したインタフェースを作る

インタフェースは、matchで返すものを判断する

```
use SortOrder::Ascending as Asc;
```

と出来る。なんだかTypeScriptのimportみたいだな

```rust
use crate::SortOrder::*;
```

とすると全てインポートしてくる。



### エラーを返す

- Rustは例外を投げることでエラーを起こったことを表さず、戻り値として表現する
- `Result<T, E>` 型が用意されてる。Result型は列挙型で
  - OkとErrの２つのバリアントを持つ
  - `sort(...).is_ok` で、よかったらtrueが帰ってくる

###  構造体

- `struct Student` で構造体を定義する
- `impl Student{ fn new() -> Self { } }`
  - implブロックを使うことで、対象の型に関連関数やメソッドを追加できる

### クロージャー

```rust
|a, b| a.age.cmp(&b.age)
```

と、引数と戻り値の型注釈を付けなくても良い

#### クロージャーの型

```rust
let a = |a, b| a.cmp(&b);
let b = |a, b| a.cmp(&b);
```

としたとき、一見同じように見えるが、別の型となる。クロージャが自分専用の環境を持つことができるから！

- クロージャはそれが置かれる文脈によって、`　Fn, FnMut, FnOnce` トレイトの一部または全てを自動的に実装する

- sort_byでは比較するたびに同じクロージャを使うので、**環境へのアクセスが読み出し専用で、何度でも呼べるFnトレントを選択する**

- `Fn(&T, &T)-> Ordering`

  - 引数としてT型の不変の借用を２つ取り、Ordering型の値を返す

  

### Random Number Generator, RNGを使ってみる

標準ライブラリがないので、`rand_pcg`を使う

乱数のベクトルを生成するとき

- 空のベクトルを用意して、そこに一つ一つ乱数を生成して代入するというのがオーソドックス
- 何らかの値を繰返し生成し、ベクタなような入れ物に収集する　は　イテレータ**と**コレクタ** で表現できる
  - イテレータはデータの生成元で、nextメソッドを呼ばれるたび値（Some）を取り出す。生成元の値が付きたらNoneを返す
    - rng.sample_iter()が無限に生成するイテレータ
      - 無限なのでNoneを返さない
    - take(n)が元のイテレータから最初のn要素を取り出すイテレータを返す
    - collectはイテレータからnextを呼んで値を収集して、ベクタやハッシュマップのようなコレクションに格納する

### 並列ソートの実現

```rust
// spawnで子スレッドを立ち上げ、小スレッドで重い処理を実行
// childがスレッドへのハンドルに束縛される
... = thread::spawn(move || func())

// スレッドのハンドルに対して.joinを呼ぶことで、スレッドの終了を待つ
// クロージャの戻り値はOkでラップされる。エラーなら異常終了でErrが変える
match child.join() {
  Ok(s2) => ...
}
```

- nまでの総和は、`(1..=n).sum` らしい

#### Rayon

Rayonクレートは並列データ処理を簡単かつ安全に実現するためのライブラリ

- **並列イテレータ** ：イテレータチェンを並列に実行する
- **joinメソッド**：再帰的で、分割統括法に適したコードを並列に実行する

バイトニックソートは、数列を再帰的に分割しソートと結合を繰り返す。

`do_sort` で数列の長さが１になるまで２分割するが、閾値より小さいときjoinを使うとオーバーヘッドにあるので、普通の処理を行う

>  要素数が少なくなれば比例して処理時間が少なくなるものと、オーバーヘッドにより処理時間が一定のものはちゃんと比較する



以下のエラーから、comparatorの型Fがstd::marker::Sync(境界)の成約の考慮が必要とわかる

```rust
fn do_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering,
// ↓エラー
    - help: consider further restricting type parameter `F`: `, F: std::marker::Sync`
```



#### SyncトレイトとSendトレイト

マルチスレッドでデータ競合が起こらないことをコンパイル時に確認するのにSyncとSendを使う

Sync境界を付けておくと、comparatorが複数のスレッド間で共有され、並列に実行されるように自動的に実装される

複数のトレイト境界は `F: Sync + Fn(&T, &T)`とつなげる

ある型がSendを実装していると、この値はスレッド間で安全に受け渡しができる`where T: Send`する

#### 所有権

```rust
- second borrow occurs due to use of `x` in closure
second closure is constructed here
```

上記は、このクロージャーで`x` が同時に独占的なアクセスがされているよ、ということ。

> 所有権をすることで、GCなどなしでメモリを管理したり、メモリ安全性を保証したりできる
>
> 所有権の枠組みの元で、ある値に対する可変の参照を同時に２つ作れない

**※　安全に別スレッドに受け渡しするのと、同時にアクセスすることは違う？**

```rust
|| do_sort(&mut x[..mid_point], ...),
|| do_sort(&mut x[mid_point..], ...)
```

と、可変の参照をそれぞれが捕捉しようとしているが、ある値に対する可変の参照は同時に１つしか作れないためコンパイルエラーになる

ならば、クロージャごとに独占的にキャプチャをしようとする

```rust
        let former_x = &mut x[..mid_point];
        let paster_x = &mut x[mid_point..];
        if mid_point >= PARALLEL_THRESHOLD {
            rayon::join(
                || do_sort(former_x, true, comparator),
                || do_sort(paster_x, false, comparator),
```

↑自分はできるとは思わない。結局、former_xも`x`の参照でしか無いし。

著者は「let former_xの行で、xに対する可変の参照が作られ、それが返却される前にlet paster_xの行で再度可変の借用を作ろうとしているから」と述べている。

→sliceモジュールの `split_at_mut()`を使う https://doc.rust-lang.org/std/primitive.slice.html

> 可変の借用に分割し、それぞれの変数に束縛する

```rust
        let (former_x, paster_x) = x.split_at_mut(mid_point);
```

参照であることは問題じゃないのか…？

じゃなかった。もっと根本的に、Copyトレイトっていう所有権を渡せるものが実装されてない型に対してはデータ競合が起こるのでダメ。

# プリミティブ型

型の恩恵として、安全性と効率性がある

**安全性**: エラーの早期発見ができる

**効率性**: データを格納するメモリの領域やメモリ上の表現などを型を透してきめ細やかに制御できる

## 型の分類

**primitive type**: 元々Rustに備わっている型

- **scalar **type: すべてプリミティブ型で、アクセスできるものがない

**user defined type**: ユーザ自身が定義した型

**compound type**: 複数の型の組み合わせ。アクセスできる内部構造を持つ。e.g. [u32]などのスライス型。Vec<u32>のようなベクタ型。

ユニット型：空を表す型で、空のタプル`()`ユニット値をもつ型。何も返さない関数とかはunit型を返しているといえる

アドレス幅の整数型：ターゲットとなるCPUのメモリアドレスのビット幅によってサイズが変わる。

- 符号付き`isize`と符号なし`usize`は配列のインデックスや長さなど広範囲で使われる

- ```rust
  let n1 = 10_000;
  let n2 = 10_000_isize;
  assert_eq!(n1, n2);
  ```

### Reference Type

メモリ安全なポインタ。データが格納されている場所を示す。usizeと同じビット幅の整数で表される。immutable reference type: `&T` . mutable reference type: `&mut T`

所有権システムで、**参照を借用**とも呼ぶ。参照がその元となっているデータを所有していないから

### Raw Pointer Type

メモリ安全でないポインタ。immutable raw pointer`*const T`. mutable raw pointer`*mut T`

用途として

- ポインタを他の言語との間で受け渡しをする
- 所有権システムの管理から外したい

- unsafeブロック内で書くことで「参照外し」「他のポインタ型への変換」ができる



### Fn Pointer

関数ポインタは、入力の型と出力の型を定義した関数のポインタ。ポインタっていうのがへぇって感じ

クロージャは匿名型で、再代入とかできひん。ただ、環境（定義したスコープ内の変数？）を補足しないクロージャは関数ポインタ型になる

## プリミティブな複合型

### tuple type

タプルの要素数をアリティと呼び、コンパイル時に決まり実行時に変更できない

要素を指す可変の参照を得るために、左辺に`ref mut`をつける

### array type

`a2 = [0; 3]`は要素数3で0埋めする

配列の長さはコンパイル時に決まり、実行時変えられない。要素の追加・削除はベクタ（`Vec<T>`型）がある

- なので `let size = 100; let a1 = [1; size]`とかはできない

- `let size= 100; let mut v1 = vec![0; size]`はできる

#### 要素へのアクセス

インデックスを使う方法と、イテレータを使う方法がある

範囲外のアクセスは静的にわかるとき、コンパイルエラーになり、実行時はパニックになる

なので、値を取り出すときはより安全な`get()`メソッドを使う

- get()はインデックスが範囲内のときはSome(&値)を返す

基本的に、インデックスよりイテレータ使うほうがよい

### slice type

`len, iter, get` はスライスのメソッドで、arrayはスライスに含まれる

スライスには不変・可変の参照かBoxというポインタの一種を経由してアクセスする

`begin..end`はbeginは含むが、endは含まない。`begin..=end`はendも含む

mutableなスライスに対して、なにか操作するとき`&mut a1.sort()`というように``&mut` を付けて可変スライスを作るが、実際は型強制によって自動的にスライスが作られる

> ``借用`` は実データを持っておらず、メモリだけ持っている。ライフタイムが死ぬと、メモリから削除されても、借用していた実データはそのままメモリに残る

のに対して、`Box<[T]>`はスライスのライフタイムが尽きると実データがメモリから削除される

### string slice

`str型`はUnicodeの文字で構成された文字列。`str`にはスライスを通じてアクセスする。なので**文字列スライス型**と言われている。型は`&'static str'`で `'static'`はライフタイム指定子と言う

str型はUTF-8で表現したときの長さなので、気をつけなきゃやばい

#### 可変のstr

&strは不変スライス経由のアクセス。&mut strは可変スライス経由のアクセス

- 文字列リテラル(`&'static str'`)から`&mut str`は直接得られない
  - 文字列リテラル→String→&mut str

# ユーザ定義型

- プログラム実行時のメモリ領域（スタック領域とヒープ領域）

について学んでいく

## stack area & heap area

![image-20200426162518239](/Users/reon.nishimura/Library/Application Support/typora-user-images/image-20200426162518239.png)



- `スタック領域` : スレッドごとに用意されるメモリ領域で、関数内のローカル変数やデータが格納されている。関数から抜けたり、スレッドが終了するとスタックの内容は破棄される

  - シンプル
  - サイズがあまり大きくない
    - スタック領域に格納しきれなくなると、スタックオーバーフローが起こる
  - Rustのデフォルト

- `ヒープ領域`:プログラム内で共有されるデータを格納するメモリ領域。必要なときに確保と解放ができる

  - アロケータと呼ばれるライブラリを通して確保・解放を行っているため、その時間が遅くなってしまう
  - 大量のデータが格納できる
  - Boxポインタ、Vec<T>やHashMap<K, V>のような要素数が可変のコレクション型の要素
  - StringやOSStringのような文字の追加や削除が可能な文字列型の要素

  ![image-20200426163001838](/Users/reon.nishimura/Library/Application Support/typora-user-images/image-20200426163001838.png)

## 	標準ライブラリの主な型

ヒープ再割当てなしで格納できる最大の要素数のことを**キャパシティ**という<img src="/Users/reon.nishimura/Library/Application Support/typora-user-images/image-20200426163932635.png" alt="image-20200426163932635"  />![image-20200426163943037](/Users/reon.nishimura/Library/Application Support/typora-user-images/image-20200426163943037.png)

タプルがヒープ領域に移動した後、`t1`は初期化されてない状態になるので、アクセスしようとするとコンパイルエラーになる

`Vec<T>`の`append`は追加する要素が`mut Vec<T>`である必要があり、追加されたあと削除されている。`extend_from_slice`だと、削除されない

![image-20200427002000214](/Users/reon.nishimura/Library/Application Support/typora-user-images/image-20200427002000214.png)

![image-20200427002011153](/Users/reon.nishimura/Library/Application Support/typora-user-images/image-20200427002011153.png)

- `Vec<T>`が余分のスペースを確保しているのに対して、`Box<[T]>`は余分なスペースを持たないヒープ領域の配列みたいな感じ
  - `Vec<T>`の`shrink_tofit()`で余分なメモリを削ぎ落とすことができる

### その他のコレクション

collection typeはベクタのような値の集合を格納する型の総称。

- map: `HashMap`, `BTreeMap`(Btreeを使用した順序付きマップ)
- Set: `HashSetp`, `BTreeSet`(Btreeを使用した順序付きｾｯﾄ)
- Queue: `VecQueue`(循環バッファ), `BinaryHeap`(優先順位付きキュー)
- List: `LinkedList`(双方向連結リスト)



### std::string::String

&strとStringの関係は、immutable sliceとvectorの関係に似てる

![image-20200427005353442](/Users/reon.nishimura/Library/Application Support/typora-user-images/image-20200427005353442.png)

- String: は実データを所有している
- &str: は不変スライスで、実データを所有せず、借用している
  - 文字列リテラルから作られた場合は、スタック領域。
  - Stringから作られた場合は、ヒープ領域に実データをおいて参照する

![image-20200427005633214](/Users/reon.nishimura/Library/Application Support/typora-user-images/image-20200427005633214.png)

#### 関数の引数に文字列を使う場合

- 文字列が不変なら低コスト: `&str`
- 可変なら: `&mut String`
- 可変+所有権: `String`

関数内で`format!`などでString作った場合、それのメモリを返そうとしても、ライフタイムが尽きて削除されるので返せない。

### std::ops::Range

- ６つのRange*型のすべてに対応できるジェネリクスな関数を定義するには、RangeBoundsトレイトをつかう

### Std::option::Option<T>

値があるかわからん型。オプション型はSome(T型の値)とNoneのバリアントを持つ列挙型。

`let mut o1 = Some(10);`でOption<i32>型という認識らしい。Someがオプション型？

`unwrap`はSome(型)を開封し、中の値を取り出す。`.unwrap()`はNoneのときpanicしちゃうのでなるべく使わない

- `unwrap`: Noneだったらpanic
- `unwrap_or_else`: Noneじゃなかったらクロージャの返り値
- `map`: クロージャを適用できる。NoneだったらNoneが返る
- `and_then`: `if`式を定義できる

### std::result::Result<T, E>

`Result` もOk(<T>)とErr(<T>)のバリアントを持つ列挙型

- ()`?`でOk型から値が取れるし、Noneだったらその場で関数からリターンする
- `Result<i32, std::num::ParseIntError>` のように返り値はそれぞれ変えられる
  - 問題なければOk型を返す

#### Option<T>とResult<T, E>の変換

- Option<T>のok_or_else(): Option→Result<T, E>
- Result<T, E>のok(): Result<T, E>→Option<T>



## 新しい型の定義と型エイリアス

型エイリアス `type UserName = String;`

Rustの構造体には、関数型レコードアップデート構文、とか、パターンマッチによる変数の代入とかがある

```typescript
// 関数型レコードアップデート構文
return {
  ...state,
  data: {error: "hoge"},
};

// パターンマッチによる変数展開
const { data } = state;
```

```rust
// 関数型レコードアップデート構文
    let y = Polygon {
        vectexes: vec![(0, 1)],
        stroke_width: x.stroke_width + 3,
        ..x
    };

// パターンマッチによる変数展開
let Polygon { vectexes: vect } = y;
let Polygon { fill, ..} = y;
```

Defaultトレイトを実装することで、値を初期値を決められる `impl Default for Polygon { fn default() -> Self { ...}}`

- ただし、すべてのフィールドがDefaultトレイトが実装されてないと使えない

タプル構造体を使うと、引数の位置をしっかり考慮されるので、同じu32だったとしてもエラーになる



### 型変換

- type cast: asで変換。スカラ型のみ
- transmute: `std::mem::transmute`によるアンセーフな型変換
- type coercion: コンパイラによる暗黙的な型変換

複合型の型変換するための`From`トレイトがある。`v: Vec<u8> = From::from("hello")`

型強制は、「型アノテーションによって、別の型に変換されるもの」

- `let v1: Vec<u8> = vec![3, 4,5 ]`はもともとi32のものをu8にしてる

![image-20200504173350544](/Users/reon.nishimura/Library/Application Support/typora-user-images/image-20200504173350544.png)

- Derefは型強制のトレイト
- ポタインタの弱体化：mutを外す

#　基本構文

- 発散する関数：`fn end_function() -> ! {}` 何も値を返さない

関連関数はclass methodみたいな感じ

**static変数**は、型の明示をしなきゃいけない

static 文で定義されたスタティック変数は定数とは異なり、値が埋め込まれることはなく、 使われるたびに参照されます。

