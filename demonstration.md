Proposition: 
$$
(F^k\bot) s = undef \implies  \\
(F^{k+1}\bot) s  = (F\bot \circ S[\![stm]\!]^k)s  = F\bot (S[\![stm]\!]^ks) \\
\forall s \in State, k\ge 1
$$
## Lemma 1
First we prove a useful lemma: $(F\bot) s = undef \implies  B[\![b]\!]s = tt$

Proof (by contraddiction):\
Suppose that $(F\bot) s = undef \land B[\![b]\!]s = ff$, then
$$
\begin{aligned}
(F\bot)s & = cond(B[\![b]\!],\bot \circ S[\![stm]\!],id) s 
\\ & = cond(B[\![b]\!],\bot,id) s
\\ & = 
    \begin{cases}
        \bot s  & \text{if $B[\![b]\!]s = tt$}\\
        id\ s & \text{if $B[\![b]\!]s = ff$}
    \end{cases}
\\ & = id\ s \ne undef &\text{CONTRADDICTION} &&& \square

\end{aligned}
$$

We can generalize this lemma thanks to the monotonicity of $F$:\
$(F\bot)^k s = undef \implies B[\![b]\!]s = tt \ \ \ \forall k \ge 1$ 



## Induction proof
### Base case (n=1):

If $(F\bot) s \ne undef$ then the implication is always true.\
If $(F\bot) s = undef$ we can prove that the second part is true:
$$
\begin{aligned}
    (F^2\bot) s & = cond(B[\![b]\!],F\bot \circ S[\![stm]\!],id) s 
    \\ & = 
    \begin{cases}
        (F\bot \circ S[\![stm]\!])s  & \text{if $B[\![b]\!]s = tt$}\\
        id\ s & \text{if $B[\![b]\!]s = ff$}
    \end{cases}
    \\ & = (F\bot \circ S[\![stm]\!]) s & \text{(by lemma1)}
\end{aligned}
$$

### Induction step:

Inductive hypothesis: \
$(F^k\bot) s = undef \implies 
(F^{k+1}\bot) s  = (F\bot \circ S[\![stm]\!]^k)s$

We need to prove it for $k+1$.


If $(F^{k+1}\bot) s \ne undef$ then the implication is always true.\
If $(F^{k+1}\bot) s = undef$ then it satisfy the hypothesis of inductive hypothesis:
$$
\begin{aligned}
    &(F^{k+1}\bot) s = undef \\
    &\implies (F^{k}\bot) s = undef & \text{(by F monotonicity)} \\
    &\implies (F\bot \circ S[\![stm]\!]^k)s
\end{aligned}
$$

$$
\begin{aligned}
    (F^{k+2}\bot) s 
    & = cond(B[\![b]\!],(F^{k+1}\bot) \circ S[\![stm]\!],id) \\
    & = 
    \begin{cases}
        ((F^{k+1}\bot) \circ S[\![stm]\!]) s & \text{if $B[\![b]\!]s = tt$}\\
        id\ s & \text{if $B[\![b]\!]s = ff$}\\
    \end{cases} \\
    & = ((F^{k+1}\bot) \circ S[\![stm]\!])s & \text{(by lemma1)} \\
    & = (F^{k+1}\bot)(S[\![stm]\!]s) \\
    & = (F\bot \circ S[\![stm]\!]^k)(S[\![stm]\!]s) & \text{(by inductive hypothesys* on $s'=S[\![stm]\!]s$)}\\
    & = (F\bot \circ S[\![stm]\!]^k \circ S[\![stm]\!])s \\
    & = (F\bot \circ S[\![stm]\!]^{k+1})s & \square\\
\end{aligned}
$$


\* Proof that $(F^k\bot)(S[\![stm]\!]s) = undef $:
$$
\begin{aligned}
    (F^{k+1}\bot) s 
    & = cond(B[\![b]\!],(F^k\bot) \circ S[\![stm]\!],id)\\
    & = 
    \begin{cases}
        ((F^{k}\bot) \circ S[\![stm]\!]) s & \text{if $B[\![b]\!]s = tt$}\\
        id\ s & \text{if $B[\![b]\!]s = ff$}\\
    \end{cases} \\
    & = (F^{k}\bot) \circ S[\![stm]\!]) s & \text{(by lemma1)}\\
    & = undef & \text{(by implication hypothesis)}
\end{aligned}
$$