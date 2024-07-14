# markdown 常用语法

## 数学公式

参考: [DanielGavin - Markdown数学公式语法](https://www.jianshu.com/p/e74eb43960a1)

参考: [katex Supported Functions](https://katex.org/docs/supported.html)

### 行内与独行

1. 行内公式: 将公式插入到本行内, 符号: \$公式内容\$, 如:  \$xyz\$, $xyz$
1. 独行公式: 将公式插入到新的一行内, 并且居中, 符号: \$\$公式内容\$\$, 如: \$\$xyz\$\$, $$xyz$$

### 上标、下标与组合

1. 上标符号, 符号: ^, 如: \$x^4\$, $x^4$
1. 下标符号, 符号: _, 如: \$x_1\$, $x_1$
1. 组合符号, 符号: {}, 如: \${16}_{8}O{2+}_{2}\$, ${16}_{8}O{2+}_{2}$

### 汉字、字体与格式

1. 汉字形式, 符号: \mbox{}, 如: \$V_{\mbox{初始}}\$
   $$V_{\mbox{初始}}$$

1. 字体控制, 符号: \displaystyle, 如: \$\displaystyle \frac{x+y}{y+z}\$
   $$\displaystyle \frac{x+y}{y+z}$$

1. 下划线符号, 符号: \underline, 如: \$\underline{x+y}\$
   $$\underline{x+y}$$

1. 标签, 符号\tag{数字}, 似乎只能用独行形式, 如: \$\$\tag*{hi} x+y^{2x}\$\$, \$\$\tag{hi} x+y^{2x}\$\$
   $$\tag*{hi} x+y^{2x}$$ $$\tag{hi} x+y^{2x}$$

1. 上大括号, 符号: \overbrace{算式}, 如: \$\overbrace{a+b+c+d}^{2.0}\$
   $$\overbrace{a+b+c+d}^{2.0}$$

1. 下大括号, 符号: \underbrace{算式}, 如: \$a+\underbrace{b+c}_{1.0}+d\$
   $$a+\underbrace{b+c}_{1.0}+d$$

1. 上位符号, 符号: \stacrel{上位符号}{基位符号}, 如: \$\vec{x}\stackrel{\mathrm{def}}{=}{x_1,\dots,x_n}\$
   $$\vec{x}\stackrel{\mathrm{def}}{=}{x_1,\dots,x_n}$$

### 占位符

1. 两个quad空格, 符号: \qquad, 如: \$x \qquad y\$, $x \qquad y$
1. quad空格, 符号: \quad, 如: \$x \quad y\$, $x \quad y$
1. 大空格, 符号: \, 如: \$x \ y\$, $x \ y$
1. 中空格, 符号: \, 如: \$x : y\$, $x : y$
1. 小空格, 符号: \,, 如: \$x , y\$, $x , y$
2. 没有空格, 如: \$xy\$, $xy$
3. 紧贴, 符号: \\!, 如: \$x \\! y\$, $x \! y$

### 定界符与组合

1. 括号, 符号: ()\big(\big) \Big(\Big) \bigg(\bigg) \Bigg(\Bigg)

   如: \$()\big(\big) \Big(\Big) \bigg(\bigg) \Bigg(\Bigg)\$
   $$()\big(\big) \Big(\Big) \bigg(\bigg) \Bigg(\Bigg)$$

1. 中括号, 符号: [], 如: \$[x+y]\$, $[x+y]$
1. 大括号, 符号: \{ \}, 如: \${x+y}\$, ${x+y}$
1. 自适应括号, 符号: \left \right, 如: \$\left(x\right)$, $\left(x{yz}\right)\$

   $\left(x\right)$, $\left(x{yz}\right)$

1. 组合公式, 符号: {上位公式 \choose 下位公式}

   如: \${n+1 \choose k}={n \choose k}+{n \choose k-1}\$
   $${n+1 \choose k}={n \choose k}+{n \choose k-1}$$

1. 组合公式, 符号: {上位公式 \atop 下位公式}

   如: \$\sum_{k_0,k_1,\ldots>0 \atop k_0+k_1+\cdots=n}A_{k_0}A_{k_1}\cdots\$
   $$\sum_{k_0,k_1,\ldots>0 \atop k_0+k_1+\cdots=n}A_{k_0}A_{k_1}\cdots$$

### 四则运算

1. 加法运算, 符号: +, 如: \$x+y=z\$ $$x+y=z$$
1. 减法运算, 符号: -, 如: \$x-y=z\$ $$x-y=z$$
1. 加减运算, 符号: \pm, 如: \$x \pm y=z\$ $$x \pm y=z$$
1. 减甲运算, 符号: \mp, 如: \$x \mp y=z\$ $$x \mp y=z$$
1. 乘法运算, 符号: \times, 如: \$x \times y=z\$ $$x \times y=z$$
1. 点乘运算, 符号: \cdot, 如: \$x \cdot y=z\$ $$x \cdot y=z$$
1. 星乘运算, 符号: \ast, 如: \$x \ast y=z\$ $$x \ast y=z$$
1. 除法运算, 符号: \div, 如: \$x \div y=z\$ $$x \div y=z$$
1. 斜法运算, 符号: /, 如: \$x/y=z\$ $$x/y=z$$
1. 分式表示, 符号: \frac{分子}{分母}, 如: \$\frac{x+y}{y+z}\$
   $$\frac{x+y}{y+z}$$

1. 分式表示, 符号: {分子} \voer {分母}, 如: \${x+y} \over {y+z}\$ $${x+y} \over {y+z}$$
1. 绝对值表示, 符号: ||, 如: \$|x+y|\$ $$|x+y|$$

### 高级运算

1. 平均数运算, 符号: \overline{算式}, 如: \$\overline{xyz}\$
   $$\overline{xyz}$$

1. 开二次方运算, 符号: \sqrt, 如: \$\sqrt x\$
   $$\sqrt x$$

1. 开方运算, 符号: \sqrt[开方数]{被开方数}, 如: \$\sqrt[3]{x+y}\$
   $$\sqrt[3]{x+y}$$

1. 对数运算, 符号: \log, 如: \$\log(x)\$
   $$\log(x)$$

1. 极限运算, 符号: \lim, 如: \$\lim^{x \to \infty}_{y \to 0}{\frac{x}{y}}\$
   $$\lim^{x \to \infty}_{y \to 0}{\frac{x}{y}}$$

1. 极限运算, 符号: \displaystyle \lim, 如: \$\displaystyle \lim^{x \to \infty}_{y \to 0}{\frac{x}{y}}\$
   $$\displaystyle \lim^{x \to \infty}_{y \to 0}{\frac{x}{y}}$$

1. 求和运算, 符号: \sum, 如: \$\sum^{x \to \infty}_{y \to 0}{\frac{x}{y}}\$
   $$\sum^{x \to \infty}_{y \to 0}{\frac{x}{y}}$$

1. 求和运算, 符号: \displaystyle \sum, 如: \$\displaystyle \sum^{x \to \infty}_{y \to 0}{\frac{x}{y}}\$
   $$\displaystyle \sum^{x \to \infty}_{y \to 0}{\frac{x}{y}}$$

1. 积分运算, 符号: \int, 如: \$\int^{\infty}_{0}{xdx}\$
   $$\int^{\infty}_{0}{xdx}$$

1. 积分运算, 符号: \displaystyle \int, 如: \$\displaystyle \int^{\infty}_{0}{xdx}\$
   $$\displaystyle \int^{\infty}_{0}{xdx}$$

1. 微分运算, 符号: \partial, 如: \$\frac{\partial x}{\partial y}\$
   $$\frac{\partial x}{\partial y}$$

1. 矩阵表示, 符号: \begin{matrix} \end{matrix}, 如: \$\begin{matrix} a & b \\ c & d \end{matrix}\$
   $$\begin{matrix} a & b \\ c & d \end{matrix}$$

### 逻辑运算

1. 等于运算, 符号: =, 如: \$x+y=z\$
   $$x+y=z$$

1. 大于运算, 符号: >, 如: \$x+y>z\$
   $$x+y>z$$

1. 小于运算, 符号: <, 如: \$x+y<z\$
   $$x+y<z$$

1. 大于等于运算, 符号: \geq, 如: \$x+y \geq z\$
   $$x+y \geq z$$

1. 小于等于运算, 符号: \leq, 如: \$x+y \leq z\$
   $$x+y \leq z$$

1. 不等于运算, 符号: \neq, 如: \$x+y \neq z\$
   $$x+y \neq z$$

1. 不大于等于运算, 符号: \ngeq, 如: \$x+y \ngeq z\$
   $$x+y \ngeq z$$

1. 不大于等于运算, 符号: \not\geq, 如: \$x+y \not\geq z\$
   $$x+y \not\geq z$$

1. 不小于等于运算, 符号: \nleq, 如: \$x+y \nleq z\$
   $$x+y \nleq z$$

1. 不小于等于运算, 符号: \not\leq, 如: \$x+y \not\leq z\$
   $$x+y \not\leq z$$

1. 约等于运算, 符号: \approx, 如: \$x+y \approx z\$
   $$x+y \approx z$$

1. 恒定等于运算, 符号: \equiv, 如: \$x+y \equiv z\$
   $$x+y \equiv z$$

### 集合运算

1. 属于运算, 符号: \in, 如: \$x \in y\$, $x \in y$
1. 不属于运算, 符号: \notin, 如: \$x \notin y\$, $x \notin y$
1. 不属于运算, 符号: \not\in, 如: \$x \not\in y\$, $x \not\in y$
1. 子集运算, 符号: \subset, 如: \$x \subset y\$, $x \subset y$
1. 子集运算, 符号: \supset, 如: \$x \supset y\$, $x \supset y$
1. 真子集运算, 符号: \subseteq, 如: \$x \subseteq y\$, $x \subseteq y$
1. 非真子集运算, 符号: \subsetneq, 如: \$x \subsetneq y\$, $x \subsetneq y$
1. 真子集运算, 符号: \supseteq, 如: \$x \supseteq y\$, $x \supseteq y$
1. 非真子集运算, 符号: \supsetneq, 如: \$x \supsetneq y\$, $x \supsetneq y$
1. 非子集运算, 符号: \not\subset, 如: \$x \not\subset y\$, $x \not\subset y$
1. 非子集运算, 符号: \not\supset, 如: \$x \not\supset y\$, $x \not\supset y$
1. 并集运算, 符号: \cup, 如: \$x \cup y\$, $x \cup y$
1. 交集运算, 符号: \cap, 如: \$x \cap y\$, $x \cap y$
1. 差集运算, 符号: \setminus, 如: \$x \setminus y\$, $x \setminus y$
1. 同或运算, 符号: \bigodot, 如: \$x \bigodot y\$, $x \bigodot y$
1. 同与运算, 符号: \bigotimes, 如: \$x \bigotimes y\$, $x \bigotimes y$
1. 实数集合, 符号: \mathbb{R}, 如: \$\mathbb{R}\$, $\mathbb{R}$
1. 自然数集合, 符号: \mathbb{Z}, 如: \$\mathbb{Z}\$, $\mathbb{Z}$
1. 空集, 符号: \emptyset, 如: \$\emptyset\$, $\emptyset$

### 数学符号

1. 无穷, 符号: \infty, 如: \$\infty\$, $\infty$
1. 虚数, 符号: \imath, 如: \$\imath\$, $\imath$
1. 虚数, 符号: \jmath, 如: \$\jmath\$, $\jmath$
1. 数学符号, 符号\hat{a}, 如: \$\hat{a}\$, $\hat{a}$
1. 数学符号, 符号\check{a}, 如: \$\check{a}\$, $\check{a}$
1. 数学符号, 符号\breve{a}, 如: \$\breve{a}\$, $\breve{a}$
1. 数学符号, 符号\tilde{a}, 如: \$\tilde{a}\$, $\tilde{a}$
1. 数学符号, 符号\bar{a}, 如: \$\bar{a}\$, $\bar{a}$
1. 矢量符号, 符号\vec{a}, 如: \$\vec{a}\$, $\vec{a}$
1. 数学符号, 符号\acute{a}, 如: \$\acute{a}\$, $\acute{a}$
1. 数学符号, 符号\grave{a}, 如: \$\grave{a}\$, $\grave{a}$
1. 数学符号, 符号\mathring{a}, 如: \$\mathring{a}\$, $\mathring{a}$
1. 一阶导数符号, 符号\dot{a}, 如: \$\dot{a}\$, $\dot{a}$
1. 二阶导数符号, 符号\ddot{a}, 如: \$\ddot{a}\$, $\ddot{a}$
1. 上箭头, 符号: \uparrow, 如: \$\uparrow\$, $\uparrow$
1. 上箭头, 符号: \Uparrow, 如: \$\Uparrow\$, $\Uparrow$
1. 下箭头, 符号: \downarrow, 如: \$\downarrow\$, $\downarrow$
1. 下箭头, 符号: \Downarrow, 如: \$\Downarrow\$, $\Downarrow$
1. 左箭头, 符号: \leftarrow, 如: \$\leftarrow\$, $\leftarrow$
1. 左箭头, 符号: \Leftarrow, 如: \$\Leftarrow\$, $\Leftarrow$
1. 右箭头, 符号: \rightarrow, 如: \$\rightarrow\$, $\rightarrow$
1. 右箭头, 符号: \Rightarrow, 如: \$\Rightarrow\$, $\Rightarrow$
1. 底端对齐的省略号, 符号: \ldots, 如: \$1,2,\ldots,n\$, $1,2,\ldots,n$
1. 中线对齐的省略号, 符号: \cdots, 如: \$x_1^2 + x_2^2 + \cdots + x_n^2\$, $x_1^2 + x_2^2 + \cdots + x_n^2$
1. 竖直对齐的省略号, 符号: \vdots, 如: \$\vdots\$, $\vdots$
1. 斜对齐的省略号, 符号: \ddots, 如: \$\ddots\$, $\ddots$

### 希腊字母

| No.  | Lowercase  | Uppercase  |  English  |              IPA              |
| :--: | :--------: | :--------: | :-------: | :---------------------------: |
| $1$  |  $\alpha$  |    $A$     |  $alpha$  |          **/'ælfə/**          |
| $2$  |  $\beta$   |    $B$     |  $beta$   |    **/'bi:tə/or/'beɪtə/**     |
| $3$  |  $\gamma$  |  $\Gamma$  |  $gamma$  |          **/'gæmə/**          |
| $4$  |  $\delta$  |  $\Delta$  |  $delta$  |         **/'deltə/**          |
| $5$  | $\epsilon$ |    $E$     | $epsilon$ |        **/'epsɪlɒn/**         |
| $6$  |  $\zeta$   |    $Z$     |  $zeta$   |         **/'zi:tə/**          |
| $7$  |   $\eta$   |    $H$     |   $eta$   |          **/'i:tə/**          |
| $8$  |  $\theta$  |  $\Theta$  |  $theta$  |         **/'θi:tə/**          |
| $9$  |  $\iota$   |    $I$     |  $iota$   |         **/aɪ'əʊtə/**         |
| $10$ |  $\kappa$  |    $K$     |  $kappa$  |          **/'kæpə/**          |
| $11$ | $\lambda$  | $\lambda$  | $lambda$  |         **/'læmdə/**          |
| $12$ |   $\mu$    |    $M$     |   $mu$    |          **/mju:/**           |
| $13$ |   $\nu$    |    $N$     |   $nu$    |          **/nju:/**           |
| $14$ |   $\xi$    |   $\Xi$    |   $xi$    |   **/ksi/or/'zaɪ/or/'ksaɪ/**  |
| $15$ | $\omicron$ |    $O$     | $omicron$ | **/əu'maikrən/or/'ɑmɪ,krɑn/** |
| $16$ |   $\pi$    |   $\Pi$    |   $pi$    |           **/paɪ/**           |
| $17$ |   $\rho$   |    $P$     |   $rho$   |           **/rəʊ/**           |
| $18$ |  $\sigma$  |  $\Sigma$  |  $sigma$  |         **/'sɪɡmə/**          |
| $19$ |   $\tau$   |    $T$     |   $tau$   |       **/tɔ:/or/taʊ/**        |
| $20$ | $\upsilon$ | $\Upsilon$ | $upsilon$ |  **/'ipsilon/or/'ʌpsilɒn/**   |
| $21$ |   $\phi$   |   $\Phi$   |   $phi$   |           **/faɪ/**           |
| $22$ |   $\chi$   |    $X$     |   $chi$   |           **/kaɪ/**           |
| $23$ |   $\psi$   |   $\Psi$   |   $psi$   |          **/psaɪ/**           |
| $24$ |  $\omega$  |  $\Omega$  |  $omega$  |   **/'əʊmɪɡə/or/oʊ'meɡə/**    |

## test

|  header1   | header2  |
|  ----  | ----  |
| $\omega$     | $\omega$ |